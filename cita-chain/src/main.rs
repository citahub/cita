// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

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

#![feature(try_from)]
#![feature(tool_lints)]

extern crate byteorder;
extern crate cita_types;
extern crate clap;
extern crate common_types as types;
extern crate core;
extern crate dotenv;
extern crate error;
extern crate jsonrpc_types;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate proof;
extern crate pubsub;
extern crate serde_json;
#[macro_use]
extern crate util;

mod block_processor;
mod forward;

use block_processor::BlockProcessor;
use clap::App;
use core::db;
use core::libchain;
use forward::Forward;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time;
use std::time::Duration;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-chain", "CITA:chain");
    info!("Version: {}", get_build_info_str(true));

    let matches = App::new("chain")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-c, --config=[FILE] 'Sets a chain config file'")
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("chain.toml");

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();
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
    let db_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&db_config, &nosql_path).unwrap();

    let chain_config = libchain::chain::Config::new(config_path);
    let chain = Arc::new(libchain::chain::Chain::init_chain(
        Arc::new(db),
        &chain_config,
    ));

    let (write_sender, write_receiver) = channel();
    let forward = Forward::new(Arc::clone(&chain), ctx_pub.clone(), write_sender);

    let block_processor = BlockProcessor::new(Arc::clone(&chain), ctx_pub);

    //chain 读写分离
    //chain 读数据 => 查询数据
    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            forward.dispatch_msg(&key, &msg);
        }
    });

    //chain 写数据 => 添加块
    thread::spawn(move || {
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
    });

    //garbage collect
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        if chain.cache_size().total() > chain_config.cache_size.unwrap() / 2 {
            trace!("cache_manager begin collect garbage...");
            chain.collect_garbage();
        }
    }
}
