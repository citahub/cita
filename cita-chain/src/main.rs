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
//!     | Queue   | SubModule   | Message Type     |
//!     | ------- | ----------- | ---------------- |
//!     | chain   | Net         | SyncResponse     |
//!     | chain   | Net         | SyncRequest      |
//!     | chain   | Consensus   | BlockWithProof   |
//!     | chain   | Jsonrpc     | Request          |
//!     | chain   | Auth        | BlockTxHashesReq |
//!     | chain   | Executor    | ExecutedResult   |
//!     | chain   | Snapshot    | SnapshotReq      |
//!
//! 2. Publish channel
//!
//!     | Queue | SubModule | Message Type  |
//!     | ----- | --------- | ------------- |
//!     | chain | Chain     | BlockTxHashes |
//!     | chain | Chain     | Status        |
//!     | chain | Chain     | Request       |
//!     | chain | Chain     | Response      |
//!     | chain | Chain     | SyncResponse  |
//!     | chain | Chain     | SnapshotResp  |
//!     | chain | Chain     | LocalSync     |
//!     | chain | Chain     | RichStatus    |
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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_must_use)]
#![feature(custom_attribute)]
#![feature(refcell_replace_swap)]
#![feature(try_from)]
extern crate byteorder;
extern crate clap;
extern crate common_types as types;
extern crate core;
extern crate dotenv;
extern crate error;
extern crate jsonrpc_types;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate proof;
extern crate protobuf;
extern crate pubsub;
extern crate serde_json;
#[macro_use]
extern crate util;

mod forward;
mod block_processor;

use block_processor::BlockProcessor;
use clap::App;
use core::db;
use core::libchain;
use forward::Forward;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time;
use std::time::Duration;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-chain", "CITA:chain");

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
            Snapshot >> SnapshotReq,
        ]),
        tx,
        crx_pub,
    );

    let nosql_path = DataPath::nosql_path();
    trace!("nosql_path is {:?}", nosql_path);
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &nosql_path).unwrap();

    let chain_config = libchain::chain::Config::new(config_path);
    let chain = Arc::new(libchain::chain::Chain::init_chain(
        Arc::new(db),
        chain_config,
    ));

    if let Some(block_tx_hashes) = chain.block_tx_hashes(chain.get_current_height()) {
        chain.delivery_block_tx_hashes(chain.get_current_height(), block_tx_hashes, &ctx_pub);
    }

    let (write_sender, write_receiver) = channel();
    let forward = Forward::new(Arc::clone(&chain), ctx_pub.clone(), write_sender);

    let block_processor = BlockProcessor::new(Arc::clone(&chain), ctx_pub);
    block_processor.broadcast_current_status();

    //chain 读写分离
    //chain 读数据 => 查询数据
    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            forward.dispatch_msg(&key, &msg);
        }
    });

    //chain 写数据 => 添加块
    thread::spawn(move || {
        loop {
            if let Ok(einfo) = write_receiver.recv_timeout(Duration::new(8, 0)) {
                block_processor.set_executed_result(einfo);
            } else {
                // Here will be these status:
                // 1. Executor process restarts, lost cached block information.
                // 2. Executor encountered an invalid block and cleared the block map.
                // 3. Bft restarted, lost chain status information, unable to consensus, unable to generate block.
                //
                // This will trigger:
                // 1. Network retransmits block information or initiates a synchronization request,
                //    and then the executor will receive a block message
                // 2. Bft will receive the latest status of chain
                block_processor.clear_block_map();
                block_processor.broadcast_current_status();
            }
        }
    });

    //garbage collect
    let mut i: u32 = 0;
    loop {
        thread::sleep(time::Duration::from_millis(10_000));
        if i > 100 {
            chain.collect_garbage();
            i = 0;
        }
        i += 1;
    }
}
