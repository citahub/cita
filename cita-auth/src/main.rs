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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]
#![feature(integer_atomics)]
#![feature(try_from)]

extern crate cita_crypto as crypto;
extern crate clap;
extern crate core as chain_core;
extern crate cpuprofiler;
extern crate dotenv;
extern crate error;
extern crate jsonrpc_types;
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
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, SystemTime};
use util::{set_panic_handler, Mutex, RwLock};
use verifier::*;

fn profiler(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
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

fn main() {
    micro_service_init!("cita-auth", "CITA:auth");
    // init app
    let matches = App::new("auth")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();
    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        info!("Value for config: {}", c);
        config_path = c;
    }

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

    profiler(flag_prof_start, flag_prof_duration);

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

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub(
        "auth",
        vec![
            "consensus.verify_blk_req",
            "chain.txhashes",
            "jsonrpc.new_tx_batch",
            "net.tx",
        ],
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
                        if req_grp.len() > tx_verify_num_per_thread {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                {
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
    let txs_pub_clone = txs_pub.clone();
    thread::spawn(move || {
        let dispatch = dispatch_clone.clone();
        let mut flag = false;
        loop {
            if let Ok(txinfo) = pool_tx_receiver.try_recv() {
                let (modid, reqid, tx_res, tx) = txinfo;
                dispatch
                    .lock()
                    .deal_tx(modid, reqid, tx_res, &tx, &txs_pub_clone);
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
            if let Ok(txsinfo) = pool_txs_receiver.recv() {
                let (height, txs, block_gas_limit, account_gas_limit) = txsinfo;
                dispatch.lock().deal_txs(
                    height,
                    &txs,
                    &txs_pub_clone,
                    block_gas_limit,
                    account_gas_limit,
                );
            }
        }
    });

    let block_verify_status_hdl_remote = block_verify_status.clone();
    let resp_sender_clone = resp_sender.clone();
    let single_req_sender = single_req_sender.clone();
    let txs_pub_clone = txs_pub.clone();
    let resp_sender = resp_sender_clone.clone();
    thread::spawn(move || loop {
        match rx_sub.recv() {
            Ok((_key, msg)) => {
                let verifier = verifier.clone();
                handle_remote_msg(
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
                );
            }
            Err(err_info) => {
                error!(
                    "Failed to receive message from rx_sub due to {:?}",
                    err_info
                );
            }
        }
    });

    loop {
        handle_verificaton_result(
            &resp_receiver,
            &tx_pub,
            block_verify_status.clone(),
            &pool_tx_sender,
        );
    }
}
