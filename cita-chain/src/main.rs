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
//! 订阅频道(Queue) | Rounting_key | 消息类型(Message Type) | topic
//! ----- | ----- | ----- | -----
//! chain | Chain | SyncResponse | "cita"
//! chain | Net | SyncResponse | "cita"
//! chain | Net | SyncRequest | "cita"
//! chain | Consensus | BlockWithProof | "cita"
//! chain | Jsonrpc | Request | "cita"
//! chain | Auth | BlockTxHashesReq | "cita"
//! chain | Executor | ExecutedResult | "cita"
//!
//! ### Key behavior
//!
//! the key struct
//!
//! ```rust
//! // Database read and write and cache
//! pub struct Chain {
//!    blooms_config: bc::Config,
//!    pub current_header: RwLock<Header>,
//!    pub is_sync: AtomicBool,
//!    // Max height in block map
//!    pub max_height: AtomicUsize,
//!    pub max_store_height: AtomicUsize,
//!    pub block_map: RwLock<BTreeMap<u64, BlockInQueue>>,
//!    pub db: Arc<KeyValueDB>,
//!    pub state_db: StateDB,
//!
//!    // block cache
//!    pub block_headers: RwLock<HashMap<BlockNumber, Header>>,
//!    pub block_bodies: RwLock<HashMap<BlockNumber, BlockBody>>,
//!
//!    // extra caches
//!    pub block_hashes: RwLock<HashMap<H256, BlockNumber>>,
//!    pub transaction_addresses: RwLock<HashMap<TransactionId, TransactionAddress>>,
//!    pub blocks_blooms: RwLock<HashMap<LogGroupPosition, BloomGroup>>,
//!    pub block_receipts: RwLock<HashMap<H256, BlockReceipts>>,
//!    pub nodes: RwLock<Vec<Address>>,
//!
//!    pub block_gas_limit: AtomicUsize,
//!    pub account_gas_limit: RwLock<ProtoAccountGasLimit>,
//!
//!    cache_man: Mutex<CacheManager<CacheId>>,
//!    polls_filter: Arc<Mutex<PollManager<PollFilter>>>,
//!
//!    /// Proof type
//!    pub prooftype: u8,
//! }
//!
//! // Message forwarding and query data
//! pub struct Forward {
//!    write_sender: Sender<ExecutedResult>,
//!    chain: Arc<Chain>,
//!    ctx_pub: Sender<(String, Vec<u8>)>,
//! }
//!
//! // Processing blocks and transaction storage
//! pub struct BlockProcessor {
//!    chain: Arc<Chain>,
//!    ctx_pub: Sender<(String, Vec<u8>)>,
//! }
//! ```
//!
//! Construct a caching mechanism with `RowLock<Vec<.. >>` or `RowLock<HashMap<.. >>` and clean it regularly.
//!
//! `forward` listen to the message bus, handle read commands or forward write commands according to message key.
//!
//! `blockprocessor` processing according to the forwarded information.
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

fn main() {
    micro_service_init!("cita-chain", "CITA:chain");
    let matches = App::new("chain")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-c, --config=[FILE] 'Sets a chain config file'")
        .get_matches();

    let mut config_path = "chain.toml";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub(
        "chain",
        routing_key!([
            Chain >> SyncResponse,
            Net >> SyncResponse,
            Net >> SyncRequest,
            Consensus >> BlockWithProof,
            Jsonrpc >> Request,
            Auth >> BlockTxHashesReq,
            Executor >> ExecutedResult,
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
            if let Ok(einfo) = write_receiver.recv_timeout(Duration::new(18, 0)) {
                block_processor.set_executed_result(einfo);
            } else {
                //here maybe need send blockbody when max_store_height > max_height
                block_processor.broadcast_current_block();
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
