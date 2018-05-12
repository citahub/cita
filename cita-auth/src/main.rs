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
//!     | ----- | --------- | ----------------- |
//!     | auth  | Consensus | VerifyBlockReq    |
//!     | auth  | Chain     | BlockTxHashes     |
//!     | auth  | Jsonrpc   | RequestNewTxBatch |
//!     | auth  | Net       | Request           |
//!     | auth  | Snapshot  | SnapshotReq       |
//!     | auth  | Executor  | Miscellaneous     |
//!
//! 2. Publish channel
//!
//!     | Queue | PubModule | SubModule | Message Type      |
//!     | ----- | --------- | --------- | ----------------- |
//!     | auth  | Auth      | Chain     | BlockTxHashesReq  |
//!     | auth  | Auth      | Consensus | VerifyBlockResp   |
//!     | auth  | Auth      | Jsonrpc   | Response          |
//!     | auth  | Auth      | Net       | Request           |
//!     | auth  | Auth      | Consensus | BlockTxs          |
//!     | auth  | Auth      | Snapshot  | SnapshotResp      |
//!     | auth  | Auth      | Executor  | MiscellaneousReq  |
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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]
#![feature(integer_atomics)]
#![feature(try_from)]

extern crate cita_crypto as crypto;
extern crate cita_types;
extern crate clap;
extern crate core as chain_core;
extern crate cpuprofiler;
extern crate dotenv;
extern crate error;
extern crate jsonrpc_types;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate protobuf;
extern crate pubsub;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(test)]
extern crate tempfile;
extern crate threadpool;
extern crate tx_pool;
#[macro_use]
extern crate util;
extern crate uuid;

pub mod handler;
pub mod verifier;
pub mod dispatcher;
pub mod txwal;
pub mod config;
use clap::App;
use config::Config;
use cpuprofiler::PROFILER;
use dispatcher::Dispatcher;
use handler::*;
use libproto::Message;
use libproto::VerifyBlockReq;
use libproto::auth::MiscellaneousReq;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::convert::{Into, TryInto};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime};
use util::{set_panic_handler, Mutex, RwLock};
use verifier::{BlockVerifyStatus, Verifier, VerifyRequestResponseInfo, VerifyResult};

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

// Main entry for Auth micro-service, It runs as a stand-alone process in CITA system.
// Auth is a multi-threads process, and using channel to communicate with each other.
// So, understanding the main function of each thread and the channel net between
// them, is very useful for you to understand the code of Auth.
//
// Threads:
// thread-1: verify_tx_group_service
// thread-2: deal_tx
// thread-3: deal_txs
// thread-4: clean_tx_pool
// thread-5: handle_remote_msg
// thread-6 (the main thread): handle_verification_result
//
// Channels (each channel is multi-producer, single consummer):
// channel-1: tx_sub, rx_sub (tx_sub connect to producers, and rx_sub connect to consummer)
//       tx_sub <--  MQ
//       rx_sub --> handle_remote_msg
// channel-2: tx_pub, rx_pub (tx_pub connect to producers, and rx_pub connect to consummer)
//       tx_pub <-- handle_remote_msg
//              <-- handle_verification_result
//              <-- deal_txs (Auth >> BlockTxs)
//              <-- deal_tx (Auth >> Response, Auth >> Request)
//       rx_pub --> MQ
// channel-3: single_req_sender, single_req_receiver
//       single_req_sender <-- handle_remote_msg
//       single_req_receiver --> verify_tx_group_service
// channel-4: resp_sender, resp_receiver
//       resp_sender <-- handle_remote_msg
//                   <-- verify_tx_group_service
//       resp_receiver --> handle_verification_result
// channel-5: pool_tx_sender, pool_tx_receiver
//       pool_tx_sender <-- handle_verification_result
//       pool_tx_receiver --> deal_tx
// channel-6: pool_txs_sender, pool_txs_receiver
//       pool_txs_sender <-- handle_remote_msg
//       pool_txs_receiver --> deal_txs
fn main() {
    micro_service_init!("cita-auth", "CITA:auth");
    info!("Version: {}", get_build_info_str(true));

    // init app
    let matches = App::new("auth")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();
    let config_path = matches.value_of("config").unwrap_or("config");

    let config = Config::new(config_path);

    let count_per_batch = config.count_per_batch;
    let buffer_duration = config.buffer_duration;
    let tx_packet_limit = config.block_packet_tx_limit;
    let tx_verify_thread_num = config.tx_verify_thread_num;
    let tx_verify_num_per_thread = config.tx_verify_num_per_thread;
    let proposal_tx_verify_num_per_thread = config.proposal_tx_verify_num_per_thread;
    let tx_pool_limit = config.tx_pool_limit;

    let wal_enable = matches
        .value_of("tx_pool_wal_enable")
        .unwrap_or("false")
        .parse::<bool>()
        .unwrap();
    let flag_prof_start = config.prof_start;
    let flag_prof_duration = config.prof_duration;

    info!(
        "{} threads are configured for parallel verification",
        tx_verify_thread_num
    );
    let threadpool = threadpool::ThreadPool::new(tx_verify_thread_num);
    let on_proposal = Arc::new(AtomicBool::new(false));

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();

    let verifier = Arc::new(RwLock::new(Verifier::new()));

    let verify_cache = HashMap::new();
    let cache = Arc::new(RwLock::new(verify_cache));
    let block_verify_status = BlockVerifyStatus {
        request_id: 0,
        block_verify_result: VerifyResult::VerifyNotBegin,
        verify_success_cnt_required: 0,
        verify_success_cnt_capture: 0,
        cache_hit: 0,
    };
    let block_verify_status = Arc::new(RwLock::new(block_verify_status));

    // Start publish and subcribe message from MQ.
    // The CITA system runs in a logic nodes, and it contains some components
    // which we called micro-service at their running time.
    // All micro-services connect to a MQ, as this design can keep them loose
    // coupling with each other.
    start_pubsub(
        "auth",
        routing_key!([
            Consensus >> VerifyBlockReq,
            Chain >> BlockTxHashes,
            Jsonrpc >> RequestNewTxBatch,
            Net >> Request,
            Snapshot >> SnapshotReq,
            Executor >> Miscellaneous,
        ]),
        tx_sub,
        rx_pub,
    );

    let (single_req_sender, single_req_receiver) = channel();
    let (resp_sender, resp_receiver) = channel();
    let verifier_clone = verifier.clone();
    let cache_clone = cache.clone();
    let resp_sender_main = resp_sender.clone();
    let mut timestamp_receive = SystemTime::now();
    let dispatch_origin = Dispatcher::new(
        tx_packet_limit,
        tx_pool_limit,
        count_per_batch,
        buffer_duration,
        wal_enable,
    );
    let tx_pool_capacity = dispatch_origin.tx_pool_capacity();
    let on_proposal_clone = on_proposal.clone();
    let pool = threadpool.clone();
    thread::spawn(move || {
        loop {
            timestamp_receive = SystemTime::now();
            let mut req_grp: Vec<VerifyRequestResponseInfo> = Vec::new();
            loop {
                loop {
                    // Receive a message from single_req channel. All messages in this channel
                    // are sent by handle_remote_msg.
                    let res_local = single_req_receiver.try_recv();

                    if res_local.is_ok() {
                        let verify_req_info: VerifyRequestResponseInfo = res_local.unwrap();
                        // verify tx pool flow control
                        let capacity = tx_pool_capacity.clone();
                        if tx_pool_limit != 0 && capacity.load(Ordering::SeqCst) == 0 {
                            process_flow_control_failed(verify_req_info.clone(), &resp_sender_main);
                            continue;
                        }

                        if VerifyResult::VerifyNotBegin
                            != check_verify_request_preprocess(
                                verify_req_info.clone(),
                                verifier_clone.clone(),
                                cache_clone.clone(),
                                &resp_sender_main,
                            ) {
                            continue;
                        }
                        req_grp.push(verify_req_info);

                        // Catch a group of req before starting to verify.
                        // Set the value of "tx_verify_num_per_thread" in a logic node config file
                        // (file likes node0/auth.toml). See node configs for more detail.
                        if req_grp.len() > tx_verify_num_per_thread {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                {
                    // FIXME: has concurrency risk here? "on_proposal_clone" can be setted to
                    // "ture" just after this conditional judgment in thread handle_remote_msg.
                    if !req_grp.is_empty() && !on_proposal_clone.load(Ordering::SeqCst) {
                        trace!(
                            "main processing: {} reqs are push into req_grp",
                            req_grp.len()
                        );
                        break;
                    } else {
                        thread::sleep(Duration::new(0, 5_000_000));
                    }
                }
            }
            trace!(
                "receive verify request for dispatching Time cost {} ns",
                timestamp_receive.elapsed().unwrap().subsec_nanos()
            );

            let verifier_clone_for_pool = verifier_clone.clone();
            let cache_clone_for_pool = cache_clone.clone();
            let resp_sender_clone = resp_sender_main.clone();
            pool.execute(move || {
                verify_tx_group_service(
                    req_grp,
                    verifier_clone_for_pool,
                    cache_clone_for_pool,
                    resp_sender_clone,
                );
            });
        }
    });

    let (pool_tx_sender, pool_tx_receiver) = channel();
    let (pool_txs_sender, pool_txs_receiver) = channel();
    let txs_pub = tx_pub.clone();

    let dispatch = Arc::new(Mutex::new(dispatch_origin));
    let dispatch_clone = dispatch.clone();
    let clear = dispatch_clone.clone();
    let txs_pub_clone = txs_pub.clone();
    let clear_txs_pool = Arc::new(AtomicBool::new(false));
    let verifier_for_tx_pool = verifier.clone();

    thread::spawn(move || {
        let dispatch = dispatch_clone.clone();
        let mut flag = false;
        loop {
            // Receive a message from pool_tx channel. All messages in this channel
            // are sent by handle_verification_result.
            if let Ok(txinfo) = pool_tx_receiver.try_recv() {
                let (key, reqid, tx_res, tx) = txinfo;
                dispatch.lock().deal_tx(
                    key,
                    reqid,
                    tx_res,
                    &tx,
                    &txs_pub_clone,
                    verifier_for_tx_pool.clone(),
                );
                flag = true;
            } else {
                if flag {
                    dispatch.lock().wait_timeout_process(&txs_pub_clone);
                    flag = false;
                }
                thread::sleep(Duration::new(0, buffer_duration));
            }
        }
    });

    let txs_pub_clone = txs_pub.clone();
    thread::spawn(move || {
        let dispatch = dispatch.clone();
        loop {
            // Receive a message from pool_txs channel. All messages in this channel
            // are sent by handle_remote_msg.
            if let Ok(txsinfo) = pool_txs_receiver.recv() {
                let (height, txs, block_gas_limit, account_gas_limit, check_quota) = txsinfo;
                dispatch.lock().deal_txs(
                    height,
                    &txs,
                    &txs_pub_clone,
                    block_gas_limit,
                    account_gas_limit,
                    check_quota,
                );
            }
        }
    });
    let clear_txs_pool_clone = clear_txs_pool.clone();
    thread::spawn(move || {
        // Thread wait for a command to clear txs pool and wal.
        // Actually, when the system get a "snapshot" command, the value of
        // "clear_txs_pool" is setted to true.
        let clear_clone = clear.clone();
        loop {
            if clear_txs_pool_clone.load(Ordering::SeqCst) {
                clear_clone.lock().clear_txs_pool(tx_packet_limit);
            }
            thread::sleep(Duration::new(0, buffer_duration));
        }
    });

    let verifier_clone = verifier.clone();
    let txs_pub_clone = txs_pub.clone();
    thread::spawn(move || {
        let time_interval = Duration::from_secs(3);
        loop {
            let sig;
            {
                if verifier_clone.read().get_chain_id().is_some() {
                    sig = true;
                } else {
                    sig = false;
                }
            }
            if !sig {
                let msg: Message = MiscellaneousReq::new().into();
                txs_pub_clone
                    .send((
                        routing_key!(Auth >> MiscellaneousReq).into(),
                        msg.try_into().unwrap(),
                    ))
                    .unwrap();
            }
            thread::sleep(time_interval);
        }
        // verifier_clone.read().ask_chain_id(&txs_pub_clone);
    });

    let block_verify_status_hdl_remote = block_verify_status.clone();
    let resp_sender_clone = resp_sender.clone();
    let single_req_sender = single_req_sender.clone();
    let txs_pub_clone = txs_pub.clone();
    let resp_sender = resp_sender_clone.clone();

    thread::spawn(move || {
        let mut block_reqs: Option<VerifyBlockReq> = None;
        loop {
            // Receive a message from sub channel. All messages in this channel
            // are sent by MQ. This thread runs as a message goalkeeper for Auth,
            // and all messages from outside Auth come here first.
            match rx_sub.recv() {
                Ok((key, msg)) => {
                    let verifier = verifier.clone();
                    handle_remote_msg(
                        key,
                        msg,
                        on_proposal.clone(),
                        &threadpool,
                        proposal_tx_verify_num_per_thread,
                        verifier.clone(),
                        &single_req_sender,
                        &txs_pub_clone,
                        block_verify_status_hdl_remote.clone(),
                        cache.clone(),
                        &pool_txs_sender,
                        &resp_sender.clone(),
                        clear_txs_pool.clone(),
                        &mut block_reqs,
                    );
                }
                Err(err_info) => {
                    error!("Failed to receive message from MQ due to {:?}", err_info);
                }
            }
        }
    });

    profiler(flag_prof_start, flag_prof_duration);

    loop {
        handle_verification_result(
            &resp_receiver,
            &tx_pub,
            block_verify_status.clone(),
            &pool_tx_sender,
        );
    }
}
