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

#![feature(integer_atomics)]

extern crate protobuf;
extern crate logger;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate clap;
extern crate dotenv;
extern crate pubsub;
extern crate cpuprofiler;
extern crate libproto;
extern crate cache_2q;
#[macro_use]
extern crate util;
extern crate cita_crypto as crypto;
extern crate threadpool;
extern crate core as chain_core;
extern crate tx_pool;
extern crate uuid;
extern crate serde_json;
extern crate error;

#[macro_use]
extern crate serde_derive;

pub mod handler;
pub mod verify;
pub mod dispatchtx;
pub mod txwal;
pub mod config;
use clap::App;
use config::Config;
use cpuprofiler::PROFILER;
use dispatchtx::Dispatchtx;
use handler::*;
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicBool};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use util::{Mutex, RwLock};
use util::panichandler::set_panic_handler;
use verify::Verifier;


fn profifer(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
    let start = flag_prof_start;
    let duration = flag_prof_duration;
    thread::spawn(move || {
                      thread::sleep(std::time::Duration::new(start, 0));
                      PROFILER.lock().unwrap().start("./auth.profiler").expect("Couldn't start");
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

    let wal_enable = matches.value_of("tx_pool_wal_enable").unwrap_or("false").parse::<bool>().unwrap();
    let flag_prof_start = config.prof_start;
    let flag_prof_duration = config.prof_duration;

    info!("{} threads are configured for parallel verification", tx_verify_thread_num);
    let threadpool = Arc::new(Mutex::new(threadpool::ThreadPool::new(tx_verify_thread_num)));
    let on_proposal = Arc::new(AtomicBool::new(false));

    profifer(flag_prof_start, flag_prof_duration);

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
    start_pubsub("auth", vec!["consensus.verify_blk_req", "chain.txhashes", "jsonrpc.new_tx_batch", "net.tx"], tx_sub, rx_pub);

    let (single_req_sender, single_req_receiver) = channel();
    let (resp_sender, resp_receiver) = channel();
    let verifier_clone = verifier.clone();
    let cache_clone = cache.clone();
    let resp_sender_main = resp_sender.clone();
    let mut timestamp_receive = SystemTime::now();
    let dispatch_origin = Dispatchtx::new(tx_packet_limit, tx_pool_limit, count_per_batch, buffer_duration, wal_enable);
    let tx_pool_capacity = dispatch_origin.tx_pool_capacity();
    let on_proposal_clone = on_proposal.clone();
    let pool = threadpool.clone();
    thread::spawn(move || loop {
                      timestamp_receive = SystemTime::now();
                      let mut req_grp: Vec<VerifyRequestResponseInfo> = Vec::new();
                      loop {
                          loop {
                              let res_local = single_req_receiver.try_recv();

                              if true == res_local.is_ok() {
                                  let verify_req_info: VerifyRequestResponseInfo = res_local.unwrap();
                                  // verify tx pool flow control
                                  let capacity = tx_pool_capacity.clone();
                                  if tx_pool_limit != 0 && capacity.load(Ordering::SeqCst) == 0 {
                                      process_flow_control_failed(verify_req_info.clone(), &resp_sender_main);
                                      continue;
                                  }

                                  if VerifyResult::VerifyNotBegin != check_verify_request_preprocess(verify_req_info.clone(), verifier_clone.clone(), cache_clone.clone(), &resp_sender_main) {
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
                              if req_grp.len() > 0 && !on_proposal_clone.load(Ordering::SeqCst) {
                                  trace!("main processing: {} reqs are push into req_grp", req_grp.len());
                                  break;
                              } else {
                                  thread::sleep(Duration::new(0, 5000000));
                              }
                          }
                      }
                      trace!("receive verify request for dispatching Time cost {} ns", timestamp_receive.elapsed().unwrap().subsec_nanos());

                      let verifier_clone_for_pool = verifier_clone.clone();
                      let cache_clone_for_pool = cache_clone.clone();
                      let resp_sender_clone = resp_sender_main.clone();
                      pool.lock()
                          .execute(move || { verify_tx_group_service(req_grp, verifier_clone_for_pool, cache_clone_for_pool, resp_sender_clone); });
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
                dispatch.lock().deal_tx(modid, reqid, tx_res, &tx, &txs_pub_clone);
                flag = true;
            } else {
                if true == flag {
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
                dispatch.lock().deal_txs(height, &txs, &txs_pub_clone, block_gas_limit, account_gas_limit);
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
                              handle_remote_msg(msg, on_proposal.clone(), &threadpool, proposal_tx_verify_num_per_thread, verifier.clone(), &single_req_sender, &txs_pub_clone, block_verify_status_hdl_remote.clone(), cache.clone(), &pool_txs_sender, &resp_sender.clone());
                          }
                          Err(err_info) => {
                              error!("Failed to receive message from rx_sub due to {:?}", err_info);
                          }
                      }
                  });

    loop {
        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status.clone(), &pool_tx_sender);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crypto::*;
    use libproto::*;
    use libproto::blockchain::*;
    use protobuf::{Message, RepeatedField};
    use util::{U256, H256};
    use util::Hashable;
    use uuid::Uuid;

    const BLOCK_REQUEST_ID: u64 = 0x0123456789abcdef;

    fn generate_tx(data: Vec<u8>, valid_until_block: u64, privkey: &PrivKey) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);
        let signed_tx = tx.sign(*privkey);
        signed_tx
    }

    fn generate_request(tx: SignedTransaction) -> Request {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = Request::new();
        request.set_un_tx(tx.get_transaction_with_sig().clone());
        request.set_request_id(request_id);
        request
    }

    fn generate_msg_from_request(request: Request) -> Vec<u8> {
        let msg = factory::create_msg(submodules::JSON_RPC, topics::REQUEST, communication::MsgType::REQUEST, request.write_to_bytes().unwrap());
        msg.write_to_bytes().unwrap()
    }

    fn generate_msg(tx: SignedTransaction) -> Vec<u8> {

        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = Request::new();
        request.set_un_tx(tx.get_transaction_with_sig().clone());
        request.set_request_id(request_id);

        let msg = factory::create_msg(submodules::JSON_RPC, topics::REQUEST, communication::MsgType::REQUEST, request.write_to_bytes().unwrap());
        msg.write_to_bytes().unwrap()
    }

    fn generate_blk_msg(tx: SignedTransaction) -> Vec<u8> {
        //create verify message
        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(tx.get_transaction_with_sig().get_transaction().get_valid_until_block());
        let signature = tx.get_transaction_with_sig().get_signature().to_vec();
        req.set_signature(signature);
        let bytes = tx.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash().to_vec();
        req.set_hash(hash);
        req.set_tx_hash(tx.get_tx_hash().to_vec());

        let mut blkreq = VerifyBlockReq::new();
        blkreq.set_id(BLOCK_REQUEST_ID);
        blkreq.set_reqs(RepeatedField::from_slice(&[req]));

        let msg = factory::create_msg(submodules::CONSENSUS, topics::VERIFY_BLK_REQ, communication::MsgType::VERIFY_BLK_REQ, blkreq.write_to_bytes().unwrap());
        msg.write_to_bytes().unwrap()
    }

    fn generate_blk_msg_with_fake_signature(tx: SignedTransaction, pubkey: PubKey) -> Vec<u8> {
        //create verify message
        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(tx.get_transaction_with_sig().get_transaction().get_valid_until_block());
        let mut signature = tx.get_transaction_with_sig().get_signature().to_vec();
        signature[0] = signature[0] + 1;
        req.set_signature(signature[0..16].to_vec());
        let bytes = tx.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash().to_vec();
        req.set_hash(hash);
        req.set_tx_hash(tx.get_tx_hash().to_vec());
        req.set_signer(pubkey.to_vec());

        let mut blkreq = VerifyBlockReq::new();
        blkreq.set_id(BLOCK_REQUEST_ID);
        blkreq.set_reqs(RepeatedField::from_slice(&[req]));

        let msg = factory::create_msg(submodules::CONSENSUS, topics::VERIFY_BLK_REQ, communication::MsgType::VERIFY_BLK_REQ, blkreq.write_to_bytes().unwrap());
        msg.write_to_bytes().unwrap()
    }

    fn generate_sync_blk_hash_msg(height: u64) -> Vec<u8> {
        //prepare and send the block tx hashes to auth
        let mut block_tx_hashes = BlockTxHashes::new();
        block_tx_hashes.set_height(height);
        let mut tx_hashes_in_u8 = Vec::new();

        let u: U256 = 0x123456789abcdef0u64.into();
        let tx_hash_in_h256 = H256::from(u);
        tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());

        let u: U256 = 0x1122334455667788u64.into();
        let tx_hash_in_h256 = H256::from(u);
        tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());

        block_tx_hashes.set_tx_hashes(RepeatedField::from_slice(&tx_hashes_in_u8[..]));

        let msg = factory::create_msg(submodules::CHAIN, topics::BLOCK_TXHASHES, communication::MsgType::BLOCK_TXHASHES, block_tx_hashes.write_to_bytes().unwrap());
        msg.write_to_bytes().unwrap()
    }

    #[test]
    fn verify_sync_block_hash() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let on_proposal = Arc::new(AtomicBool::new(false));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal, &pool, tx_verify_num_per_thread, v.clone(), &req_sender, &tx_pub, c, cache, &pool_txs_sender, &resp_sender);
        assert_eq!(rx_pub.try_recv().is_err(), true);

        let u: U256 = 0x123456789abcdef0u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x1122334455667788u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x3344.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), false);
        assert_eq!(v.read().is_inited(), true);
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_txs_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_txs_receiver);
    }

    #[test]
    fn verify_request_sync_block_hash() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));
        let on_proposal = Arc::new(AtomicBool::new(false));

        let height = 1;
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;

        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal, &pool, tx_verify_num_per_thread, v.clone(), &req_sender, &tx_pub, c, cache, &pool_txs_sender, &resp_sender);

        let u: U256 = 0x123456789abcdef0u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x1122334455667788u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x3344.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);
        assert_eq!(v.read().is_inited(), false);

        let (key, sync_request) = rx_pub.recv().unwrap();
        assert_eq!(key, "auth.blk_tx_hashs_req".to_owned());
        let (_, _, content) = parse_msg(sync_request.as_slice());
        match content {
            MsgClass::BLOCKTXHASHESREQ(req) => {
                assert_eq!(req.get_height(), 0);
            }
            _ => {
                panic!("test failed")
            }
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_txs_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_txs_receiver);
    }

    #[test]
    fn verify_single_tx_request_dispatch_success() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_tx_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        let req = generate_request(tx);
        let request_id = req.get_request_id().to_vec();
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let on_proposal = Arc::new(AtomicBool::new(false));

        handle_remote_msg(generate_msg_from_request(req), on_proposal, &pool, tx_verify_num_per_thread, v.clone(), &req_sender, &tx_pub, c, cache, &pool_txs_sender, &resp_sender);
        let verify_req_info: VerifyRequestResponseInfo = req_receiver.recv().unwrap();
        assert_eq!(verify_req_info.verify_type, VerifyType::SingleVerify);
        if let VerifyRequestID::SingleVerifyRequestID(single_request_id) = verify_req_info.request_id {
            assert_eq!(request_id, single_request_id);
        }

        assert_eq!(submodules::JSON_RPC, verify_req_info.sub_module);
        if let VerifyRequestResponse::AuthRequest(req) = verify_req_info.req_resp {
            assert_eq!(req.get_tx_hash().to_vec().clone(), tx_hash);
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_tx_receiver);
    }

    #[test]
    fn verify_block_tx_request_dispatch_success() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let on_proposal = Arc::new(AtomicBool::new(false));

        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal.clone(), &pool, tx_verify_num_per_thread, v.clone(), &req_sender, &tx_pub, c.clone(), cache.clone(), &pool_txs_sender, &resp_sender);

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg(tx), on_proposal.clone(), &pool, tx_verify_num_per_thread, v.clone(), &req_sender, &tx_pub, c.clone(), cache, &pool_txs_sender, &resp_sender);

        let block_verify_status = c.read();
        assert_eq!(block_verify_status.block_verify_result, VerifyResult::VerifyOngoing);
        assert_eq!(block_verify_status.verify_success_cnt_required, 1);
        assert_eq!(block_verify_status.verify_success_cnt_capture, 0);
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_txs_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_txs_receiver);
    }

    #[test]
    fn handle_verificaton_result_single_tx() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let verify_cache_hashmap = HashMap::new();
        let verify_cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let verifier = Arc::new(RwLock::new(Verifier::new()));
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let on_proposal = Arc::new(AtomicBool::new(false));

        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal.clone(), &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), cache.clone(), &pool_txs_sender, &resp_sender);

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        handle_remote_msg(generate_msg(tx), on_proposal, &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), cache.clone(), &pool_txs_sender, &resp_sender);

        let verify_req_info: VerifyRequestResponseInfo = req_receiver.recv().unwrap();
        let mut req_grp: Vec<VerifyRequestResponseInfo> = Vec::new();
        req_grp.push(verify_req_info);
        verify_tx_group_service(req_grp, verifier, verify_cache, resp_sender);

        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status, &pool_tx_sender);
        let (_, _, resp_msg, _) = pool_tx_receiver.try_recv().unwrap();
        let ok_result = format!("{:?}", Ret::Ok);
        assert_eq!(resp_msg.status, ok_result);
        assert_eq!(tx_hash, resp_msg.hash.to_vec());
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_tx_receiver);
    }

    #[test]
    fn handle_verificaton_result_block_tx() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let verifier = Arc::new(RwLock::new(Verifier::new()));
        let on_proposal = Arc::new(AtomicBool::new(false));

        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal.clone(), &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), cache.clone(), &pool_txs_sender, &resp_sender);

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg(tx), on_proposal, &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), cache, &pool_txs_sender, &resp_sender);
        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status, &pool_tx_sender);

        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => {
                panic!("test failed")
            }
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}, pool_txs_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_tx_receiver, pool_txs_receiver);
    }

    #[test]
    fn block_verificaton_failed() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let verifier = Arc::new(RwLock::new(Verifier::new()));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let on_proposal = Arc::new(AtomicBool::new(false));

        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal.clone(), &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), cache.clone(), &pool_txs_sender, &resp_sender);

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey().clone();
        let tx = generate_tx(vec![1], 99, privkey);

        handle_remote_msg(generate_blk_msg_with_fake_signature(tx, pubkey), on_proposal, &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), cache, &pool_txs_sender, &resp_sender);

        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status, &pool_tx_sender);

        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::BadSig);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => {
                panic!("test failed")
            }
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!("rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}, pool_txs_receiver {:?}", rx_pub, req_receiver, resp_receiver, pool_tx_receiver, pool_txs_receiver);
    }

    #[test]
    fn get_tx_verificaton_from_cache() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let verify_cache_hashmap = HashMap::new();
        let verify_cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let verifier = Arc::new(RwLock::new(Verifier::new()));
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, _) = channel();
        let pool = Mutex::new(threadpool::ThreadPool::new(10));
        let tx_verify_num_per_thread = 30;
        let on_proposal = Arc::new(AtomicBool::new(false));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), on_proposal.clone(), &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), verify_cache.clone(), &pool_txs_sender, &resp_sender);


        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);

        handle_remote_msg(generate_blk_msg(tx.clone()), on_proposal.clone(), &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), verify_cache.clone(), &pool_txs_sender, &resp_sender);
        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status.clone(), &pool_tx_sender);
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => {
                panic!("test failed")
            }
        }

        thread::sleep(Duration::new(0, 9000000));
        // Begin to construct the same tx's verification request
        handle_remote_msg(generate_blk_msg(tx.clone()), on_proposal.clone(), &pool, tx_verify_num_per_thread, verifier.clone(), &req_sender, &tx_pub, block_verify_status.clone(), verify_cache.clone(), &pool_txs_sender, &resp_sender);
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => {
                panic!("test failed")
            }
        }
    }
    #[test]
    fn read_configure_file() {
        let json = r#"{
          "count_per_batch": 30,
          "buffer_duration": 3000000,
          "tx_verify_thread_num": 10,
          "tx_verify_num_per_thread": 300,
          "proposal_tx_verify_num_per_thread": 30,
          "tx_pool_limit": 50000,
          "block_packet_tx_limit": 30000,
          "prof_start": 0,
          "prof_duration": 0
        }"#;

        let value: Config = serde_json::from_str(json).expect("read Error");
        println!("{:?}", value);
        assert_eq!(30, value.count_per_batch);
        assert_eq!(3000000, value.buffer_duration);
        assert_eq!(10, value.tx_verify_thread_num);
        assert_eq!(300, value.tx_verify_num_per_thread);
        assert_eq!(30, value.proposal_tx_verify_num_per_thread);
        assert_eq!(50000, value.tx_pool_limit);
        assert_eq!(30000, value.block_packet_tx_limit);
        assert_eq!(0, value.prof_start);
        assert_eq!(0, value.prof_duration);
    }
}
