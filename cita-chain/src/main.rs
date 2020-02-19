// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! ## Summary
//! One of CITA's core components that processing blocks and transaction storage,
//! provides queries, caches query records, and more.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     | Queue   | PubModule   | Message Type     |
//!     | ------- | ----------- | ---------------- |
//!     | chain   | Net         | SyncResponse     |
//!     | chain   | Net         | SyncRequest      |
//!     | chain   | Consensus   | BlockWithProof   |
//!     | chain   | Jsonrpc     | Request          |
//!     | chain   | Auth        | BlockTxHashesReq |
//!     | chain   | Executor    | ExecutedResult   |
//!     | chain   | Snapshot    | SnapshotReq      |
//!     | chain   | Executor    | StateSignal      |
//!
//! 2. Publish channel
//!
//!     | Queue | PubModule | SubModule     | Message Type  |
//!     | ----- | --------- | ------------- | ------------- |
//!     | chain | Chain     | Auth          | BlockTxHashes |
//!     | chain | Chain     | Net           | Status        |
//!     | chain | Chain     | Executor      | Request       |
//!     | chain | Chain     | Executor      | StateSignal   |
//!     | chain | Chain     | Jsonrpc       | Response      |
//!     | chain | Chain     | Net           | SyncResponse  |
//!     | chain | Chain     | Snapshot      | SnapshotResp  |
//!     | chain | Chain     | Executor      | LocalSync     |
//!     | chain | Chain     | Consensus     | RichStatus    |
//!     | chain | Chain     | Executor      | RichStatus    |
//!
//! ### Key behavior
//!
//! the key struct:
//!
//! - [`Chain`]
//! - `Forward`: `forward::Forward`
//! - `BlockProcessor`: `block_processor::BlockProcessor`
//!
//! Construct a caching mechanism with `RowLock<Vec<.. >>` or `RowLock<HashMap<.. >>` and clean it regularly.
//!
//! `Forward` listen to the message bus, handle read commands or forward write commands according to message key.
//!
//! `BlockProcessor` processing according to the forwarded information.
//!
//! [`Chain`]: ../core/libchain/chain/struct.Chain.html
//!

extern crate common_types as types;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;
#[macro_use]
extern crate util;

mod block_processor;
mod forward;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use util::set_panic_handler;

use crate::block_processor::BlockProcessor;
use crate::forward::Forward;

use cita_db::{Config as DatabaseConfig, RocksDB, NUM_COLUMNS};
use cita_directories::DataPath;
use clap::App;
use core::libchain;
use libproto::router::{MsgType, RoutingKey, SubModules};
use license::lic_cfg::LICENSE_CONFIG;
use license::lic_verify::LicVerify;
use pubsub::{channel, start_pubsub};
use std::process::exit;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    let matches = App::new("chain")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Rivtower")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a chain config file'
                          -s, --stdout 'Log to console'",
        )
        .get_matches();

    let stdout = matches.is_present("stdout");
    micro_service_init!("cita-chain", "CITA:chain", stdout);
    info!("Version: {}", get_build_info_str(true));

    let config_path = matches.value_of("config").unwrap_or("chain.toml");

    let (tx, rx) = channel::unbounded();
    let (ctx_pub, crx_pub) = channel::unbounded();
    start_pubsub(
        "chain",
        routing_key!([
            Net >> SyncResponse,
            Net >> SyncRequest,
            Consensus >> BlockWithProof,
            Jsonrpc >> Request,
            Auth >> BlockTxHashesReq,
            Executor >> ExecutedResult,
            Executor >> StateSignal,
            Snapshot >> SnapshotReq,
        ]),
        tx,
        crx_pub,
    );

    let nosql_path = DataPath::nosql_path();
    trace!("nosql_path is {:?}", nosql_path);
    let db_config = DatabaseConfig::with_category_num(NUM_COLUMNS);
    let db = RocksDB::open(&nosql_path, &db_config).expect("Open DB failed unexpected.");

    let chain_config = libchain::chain::Config::new(config_path);
    let chain = Arc::new(libchain::chain::Chain::init_chain(
        Arc::new(db),
        chain_config,
    ));

    let mut lic_verify = LicVerify::new(LICENSE_CONFIG, ctx_pub.clone()).unwrap_or_else(|e| {
        error!("New license verify error: {}", e);
        exit(1);
    });

    let (write_sender, write_receiver) = channel::unbounded();
    let forward = Forward::new(Arc::clone(&chain), ctx_pub.clone(), write_sender);

    let block_processor = BlockProcessor::new(Arc::clone(&chain), ctx_pub, lic_verify.client());

    // Run verify cita license
    thread::spawn(move || lic_verify.run());

    // Two threads, one for reading, one for writing
    // Read: dispatch msg
    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            forward.dispatch_msg(&key, &msg);
        }
    });

    // Write: add block
    let mut timeout_factor = 0u8;
    loop {
        if let Ok(einfo) = write_receiver
            .recv_timeout(Duration::new(18 * (2u64.pow(u32::from(timeout_factor))), 0))
        {
            block_processor.set_executed_result(&einfo);
            timeout_factor = 0;
        } else if !*block_processor.chain.is_snapshot.read() {
            // Here will be these status:
            // 1. Executor process restarts, lost cached block information.
            // 2. Executor encountered an invalid block and cleared the block map.
            // 3. Bft restarted, lost chain status information, unable to consensus, unable to generate block.
            //
            // This will trigger:
            // 1. Network retransmits block information or initiates a synchronization request,
            //    and then the executor will receive a block message
            // 2. Bft will receive the latest status of chain
            info!("Chain enters the timeout retransmission phase");
            block_processor.reset_max_store_height();
            block_processor.signal_to_executor();
            block_processor.broadcast_current_status();
            if timeout_factor < 6 {
                timeout_factor += 1
            }
        }
    }
}
