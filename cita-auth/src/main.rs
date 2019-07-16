// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

//! # Summary
//!
//!   One of CITA's core components, transaction pool management,
//!   packaging transactions to consensus modules, verifying the validity of transactions,
//!   verifying the validity of synchronized blocks, remote proposals.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     | Queue | PubModule | Message Type      |
//!     | ----- | --------- | ------------------|
//!     | auth  | Consensus | VerifyBlockReq    |
//!     | auth  | Chain     | BlockTxHashes     |
//!     | auth  | Executor  | BlackList         |
//!     | auth  | Jsonrpc   | RequestNewTxBatch |
//!     | auth  | Net       | Request           |
//!     | auth  | Snapshot  | SnapshotReq       |
//!     | auth  | Executor  | Miscellaneous     |
//!     | auth  | Net       | GetBlockTxn       |
//!     | auth  | Net       | BlockTxn          |
//!
//! 2. Publish channel
//!
//!     | Queue | PubModule | SubModule | Message Type     |
//!     | ----- | --------- | --------- | ---------------- |
//!     | auth  | Auth      | Chain     | BlockTxHashesReq |
//!     | auth  | Auth      | Consensus | VerifyBlockResp  |
//!     | auth  | Auth      | Jsonrpc   | Response         |
//!     | auth  | Auth      | Net       | Request          |
//!     | auth  | Auth      | Consensus | BlockTxs         |
//!     | auth  | Auth      | Snapshot  | SnapshotResp     |
//!     | auth  | Auth      | Executor  | MiscellaneousReq |
//!     | auth  | Auth      | Net       | GetBlockTxn      |
//!     | auth  | Auth      | Net       | BlockTxn         |
//!
//! ### Key behavior
//!
//! the key struct:
//!
//! - [`Dispatcher`]
//! - [`Pool`]
//! - [`TxWal`]
//! - [`Verifier`]
//! - [`handle module`]
//!
//! [`Dispatcher`]: ./dispatcher/struct.Dispatcher.html
//! [`Pool`]: ../tx_pool/pool/struct.Pool.html
//! [`TxWal`]: ./txwal/struct.TxWal.html
//! [`Verifier`]: ./verifier/struct.Verifier.html
//! [`handle module`]: ./handler/index.html
//!

extern crate cita_crypto as crypto;
extern crate core as chain_core;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate tempfile;
#[macro_use]
extern crate util;
extern crate db as cita_db;
extern crate hashable;

use balance_verify_req_publisher::BalanceVerifyReqPublisher;
use batch_forward::BatchForward;
use clap::App;
use config::Config;
use cpuprofiler::PROFILER;
use dispatcher::Dispatcher;
use handler::MsgHandler;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::channel;
use pubsub::start_pubsub;
use std::thread;
use util::set_panic_handler;

mod balance_verify_req_publisher;
pub mod batch_forward;
pub mod block_txn;
pub mod block_verify;
pub mod config;
pub mod dispatcher;
pub mod handler;
pub mod history;
mod transaction_verify;
pub mod txwal;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn profiler(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
    if flag_prof_duration != 0 {
        let start = flag_prof_start;
        let duration = flag_prof_duration;
        thread::spawn(move || {
            thread::sleep(std::time::Duration::new(start, 0));
            PROFILER
                .lock()
                .unwrap()
                .start("./auth.profiler")
                .expect("Couldn't start");
            thread::sleep(std::time::Duration::new(duration, 0));
            PROFILER.lock().unwrap().stop().unwrap();
        });
    }
}

fn main() {
    // init app
    let matches = App::new("auth")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                          -s, --stdout 'Log to console'",
        )
        .get_matches();

    let stdout = matches.is_present("stdout");
    micro_service_init!("cita-auth", "CITA:auth", stdout);
    info!("Version: {}", get_build_info_str(true));

    let config_path = matches.value_of("config").unwrap_or("auth.toml");

    let config = Config::new(config_path);

    let count_per_batch = config.count_per_batch;
    let buffer_duration = config.buffer_duration;
    let wal_enable = config.wal_enable;

    // start profiler
    let flag_prof_start = config.prof_start;
    let flag_prof_duration = config.prof_duration;
    profiler(flag_prof_start, flag_prof_duration);

    // Start publish and subcribe message from MQ.
    // The CITA system runs in a logic nodes, and it contains some components
    // which we called micro-service at their running time.
    // All micro-services connect to a MQ, as this design can keep them loose
    // coupling with each other.
    let (tx_sub, rx_sub) = channel::unbounded();
    let (tx_pub, rx_pub) = channel::unbounded();
    start_pubsub(
        "auth",
        routing_key!([
            Consensus >> VerifyBlockReq,
            Chain >> BlockTxHashes,
            Executor >> BlackList,
            Jsonrpc >> RequestNewTxBatch,
            Net >> Request,
            Snapshot >> SnapshotReq,
            Executor >> Miscellaneous,
            Net >> GetBlockTxn,
            Net >> BlockTxn,
            Executor >> BalanceVerifyRes,
        ]),
        tx_sub,
        rx_pub,
    );

    // a single thread to batch forward transactions
    let tx_pub_forward = tx_pub.clone();
    let (tx_request, rx_request) = channel::unbounded();
    thread::spawn(move || {
        let mut batch_forward =
            BatchForward::new(count_per_batch, buffer_duration, rx_request, tx_pub_forward);
        batch_forward.run();
    });

    // a single thread to batch publish verify request
    let tx_req_pub = tx_pub.clone();
    let (tx_balance_verify_tx, rx_balance_verify_tx) = channel::unbounded();
    thread::spawn(move || {
        let mut bv_req_publisher = BalanceVerifyReqPublisher::new(
            count_per_batch,
            buffer_duration,
            rx_balance_verify_tx,
            tx_req_pub,
        );
        bv_req_publisher.run();
    });

    let dispatcher = Dispatcher::new(wal_enable);

    // handle message from MQ
    let mut msg_handler = MsgHandler::new(
        rx_sub,
        tx_pub,
        dispatcher,
        tx_request,
        tx_balance_verify_tx,
        config,
    );
    msg_handler.handle_remote_msg();
}
