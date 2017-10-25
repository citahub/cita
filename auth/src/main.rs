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

#[macro_use]
extern crate log;
extern crate clap;
extern crate dotenv;
extern crate pubsub;
extern crate cpuprofiler;
extern crate libproto;
extern crate cache_2q;
extern crate util;
extern crate cita_crypto as crypto;
extern crate threadpool;
extern crate core as chain_core;
extern crate tx_pool;
extern crate uuid;
extern crate serde_json;
extern crate error;


pub mod handler;
pub mod verify;
pub mod dispatchtx;
pub mod txwal;

use clap::App;
use cpuprofiler::PROFILER;
use dispatchtx::Dispatchtx;
use dotenv::dotenv;
use handler::*;
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use util::{Mutex, H256, RwLock};
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
    dotenv().ok();
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");

    //exit process when panic
    set_panic_handler();

    // Init logger
    logger::init();
    info!("CITA:auth");
    // init app
    let matches = App::new("auth")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-n, --tx_verify_thread_num=[10] 'transaction verification thread count'")
        .args_from_usage("-v, --tx_verify_num_per_thread=[30] 'transaction verification thread count'")
        .args_from_usage("-c, --tx_pool_limit=[50000] 'tx pool's capacity'")
        .args_from_usage("-w, --tx_pool_wal_enable=[false] ' Transaction pool persistent default closure'")
        .args_from_usage("-p, --block_packet_tx_limit=[30000] 'block's tx limit'")
        .args_from_usage("--prof-start=[0] 'Specify the start time of profiling, zero means no profiling'")
        .args_from_usage("--prof-duration=[0] 'Specify the duration for profiling, zero means no profiling'")
        .get_matches();

    let count_per_batch = matches.value_of("count_per_batch").unwrap_or("30").parse::<usize>().unwrap();
    let buffer_duration = matches.value_of("buffer_duration").unwrap_or("30000000").parse::<u32>().unwrap();
    let tx_verify_thread_num = matches.value_of("tx_verify_thread_num").unwrap_or("10").parse::<usize>().unwrap();
    let tx_verify_num_per_thread = matches.value_of("tx_verify_num_per_thread").unwrap_or("30").parse::<usize>().unwrap();
    let tx_pool_limit = matches.value_of("tx_pool_limit").unwrap_or("50000").parse::<usize>().unwrap();
    let tx_packet_limit = matches.value_of("block_packet_tx_limit").unwrap_or("30000").parse::<usize>().unwrap();
    let wal_enable = matches.value_of("tx_pool_wal_enable").unwrap_or("false").parse::<bool>().unwrap();
    let flag_prof_start = matches.value_of("prof-start").unwrap_or("0").parse::<u64>().unwrap();
    let flag_prof_duration = matches.value_of("prof-duration").unwrap_or("0").parse::<u64>().unwrap();
    info!("{} threads are configured for parallel verification", tx_verify_thread_num);
    let threadpool = threadpool::ThreadPool::new(tx_verify_thread_num);

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
    let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("auth", vec!["consensus.verify_req", "chain.txhashes", "jsonrpc.new_tx_batch", "net.tx"], tx_sub, rx_pub);

    let (block_req_sender, block_req_receiver) = channel();
    let (single_req_sender, single_req_receiver) = channel();
    let (resp_sender, resp_receiver) = channel();
    let verifier_clone = verifier.clone();
    let block_verify_status_clone = block_verify_status.clone();
    let cache_clone = cache.clone();
    let resp_sender_main = resp_sender.clone();
    let tx_pub_block_res = tx_pub.clone();
    let mut timestamp_receive = SystemTime::now();
    thread::spawn(move || loop {
                      timestamp_receive = SystemTime::now();
                      let mut req_grp: Vec<VerifyReqInfo> = Vec::new();
                      loop {
                          loop {
                              let res_local = block_req_receiver.try_recv();
                              if true == res_local.is_ok() {
                                  let (verify_type, request_id, verify_req, sub_module, now, origin) = res_local.unwrap();
                                  {
                                      let block_verify_status_gurard = block_verify_status_clone.read();
                                      if VerifyResult::VerifyFailed == block_verify_status_gurard.block_verify_result {
                                          trace!("skip the block tx verification due to failed already for block verification request id:{:?}.", request_id);
                                          continue;
                                      }

                                      if request_id != block_verify_status_gurard.request_id {
                                          trace!("skip the tx verification due to block verification with request id {:?} has been expired.", request_id);
                                          continue;
                                      }
                                  }
                                  let verify_req_info = VerifyReqInfo {
                                      req: verify_req,
                                      info: (verify_type, request_id, sub_module, now, origin),
                                  };
                                  let preproc_res = check_verify_request_preprocess(verify_req_info.clone(), verifier_clone.clone(), cache_clone.clone(), resp_sender_main.clone());
                                  if VerifyResult::VerifySucceeded == preproc_res {
                                      info!("check_verify_request_preprocess is VerifySucceeded, and {} reqs have been pushed into req_grp in main loop", req_grp.len());
                                      let mut block_verify_status_gurard = block_verify_status_clone.write();
                                      block_verify_status_gurard.cache_hit += 1;

                                      continue;
                                  } else if VerifyResult::VerifyFailed == preproc_res {
                                      let mut block_verify_status_gurard = block_verify_status_clone.write();
                                      block_verify_status_gurard.block_verify_result = VerifyResult::VerifyFailed;
                                      publish_block_verification_fail_result(request_id, &H256::from_slice(verify_req_info.req.get_tx_hash()), cache_clone.clone(), &tx_pub_block_res);
                                      info!("block verify failed for request id:{:?}", request_id);
                                      continue;
                                  }
                                  req_grp.push(verify_req_info);
                                  if req_grp.len() > tx_verify_num_per_thread {
                                      trace!(" {} reqs pushed in req_grp get the threshold value:{}", req_grp.len(), tx_verify_num_per_thread);
                                      break;
                                  }
                              } else {
                                  break;
                              }
                          }

                          loop {
                              let res_local = single_req_receiver.try_recv();
                              if true == res_local.is_ok() {
                                  let (verify_type, id, verify_req, sub_module, now, origin) = res_local.unwrap();
                                  let verify_req_info = VerifyReqInfo {
                                      req: verify_req,
                                      info: (verify_type, id, sub_module, now, origin),
                                  };
                                  if VerifyResult::VerifyNotBegin != check_verify_request_preprocess(verify_req_info.clone(), verifier_clone.clone(), cache_clone.clone(), resp_sender_main.clone()) {
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
                          if req_grp.len() > 0 {
                              trace!("main processing: {} reqs are push into req_grp", req_grp.len());
                              break;
                          } else {
                              thread::sleep(Duration::new(0, 5000000));
                          }
                      }
                      trace!("receive verify request for dispatching Time cost {} ns", timestamp_receive.elapsed().unwrap().subsec_nanos());

                      let pool = threadpool.clone();
                      let verifier_clone_for_pool = verifier_clone.clone();
                      let cache_clone_for_pool = cache_clone.clone();
                      let resp_sender_clone = resp_sender_main.clone();
                      pool.execute(move || { verify_tx_group_service(req_grp, verifier_clone_for_pool, cache_clone_for_pool, resp_sender_clone); });
                  });

    let (pool_tx_sender, pool_tx_recver) = channel();
    let (pool_txs_sender, pool_txs_recver) = channel();
    let txs_pub = tx_pub.clone();

    let dispatch_origin = Dispatchtx::new(tx_packet_limit, tx_pool_limit, count_per_batch, buffer_duration, wal_enable);
    let dispatch = Arc::new(Mutex::new(dispatch_origin));
    let dispatch_clone = dispatch.clone();
    let txs_pub_clone = txs_pub.clone();
    thread::spawn(move || {
        let dispatch = dispatch_clone.clone();
        let mut flag = false;
        loop {
            if let Ok(txinfo) = pool_tx_recver.try_recv() {
                let (modid, reqid, tx_res, tx, _) = txinfo;
                dispatch.lock().deal_tx(modid, reqid, tx_res, &tx, txs_pub.clone());
                flag = true;
            } else {
                if true == flag {
                    dispatch.lock().wait_timeout_process(txs_pub.clone());
                    flag = false;
                }
                thread::sleep(Duration::new(0, buffer_duration));
            }
        }
    });

    thread::spawn(move || {
        let dispatch = dispatch.clone();
        loop {
            if let Ok(txsinfo) = pool_txs_recver.recv() {
                let (height, txs, block_gas_limit, account_gas_limit) = txsinfo;
                dispatch.lock().deal_txs(height, &txs, txs_pub_clone.clone(), block_gas_limit, account_gas_limit);
            }
        }
    });

    let tx_pub_clone = tx_pub.clone();
    let block_verify_status_hdl_remote = block_verify_status.clone();
    let batch_new_tx_pool_clone = batch_new_tx_pool.clone();
    let resp_sender_clone = resp_sender.clone();
    thread::spawn(move || loop {
                      match rx_sub.recv() {
                          Ok((_key, msg)) => {
                              let verifier = verifier.clone();
                              let block_req_sender = block_req_sender.clone();
                              let single_req_sender = single_req_sender.clone();
                              let tx_pub_clone = tx_pub_clone.clone();
                              let resp_sender = resp_sender_clone.clone();
                              handle_remote_msg(msg, verifier.clone(), block_req_sender, single_req_sender, tx_pub_clone, block_verify_status_hdl_remote.clone(), cache.clone(), batch_new_tx_pool_clone.clone(), pool_txs_sender.clone(), resp_sender.clone());
                          }
                          Err(err_info) => {
                              error!("Failed to receive message from rx_sub due to {:?}", err_info);
                          }
                      }
                  });

    loop {
        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status.clone(), batch_new_tx_pool.clone(), pool_tx_sender.clone());
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

    fn generate_tx(data: Vec<u8>, valid_until_block: u64, privkey: &PrivKey) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);
        let signed_tx = tx.sign(*privkey);
        signed_tx
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
        blkreq.set_id(88);
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
        blkreq.set_id(88);
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
        let (req_sender, _) = channel();
        let (resp_sender, _) = channel();
        let (block_req_sender, _) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), v.clone(), block_req_sender.clone(), req_sender, tx_pub, c, cache, batch_new_tx_pool, pool_txs_sender, resp_sender);
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
    }

    #[test]
    fn verify_request_sync_block_hash() {

        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (resp_sender, _) = channel();
        let (block_req_sender, _) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));

        let height = 1;
        handle_remote_msg(generate_sync_blk_hash_msg(height), v.clone(), block_req_sender.clone(), req_sender, tx_pub, c, cache, batch_new_tx_pool, pool_txs_sender, resp_sender);

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

        let (key, sync_request) = rx_pub.try_recv().unwrap();
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
    }

    #[test]
    fn verify_single_tx_request_dispatch_success() {

        let (tx_pub, _) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, _) = channel();
        let (block_req_sender, _) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();

        handle_remote_msg(generate_msg(tx), v.clone(), block_req_sender.clone(), req_sender, tx_pub, c, cache, batch_new_tx_pool, pool_txs_sender, resp_sender);
        let (verify_type, request_id, req, submodule, _, _) = req_receiver.recv().unwrap();
        assert_eq!(verify_type, VerifyType::SingleVerify);
        assert_eq!(request_id, 0);
        assert_eq!(submodules::JSON_RPC, submodule);
        assert_eq!(req.get_tx_hash().to_vec().clone(), tx_hash);
    }

    #[test]
    fn verify_block_tx_request_dispatch_success() {

        let (tx_pub, _) = channel();
        let (req_sender, _) = channel();
        let (resp_sender, _) = channel();
        let (block_req_sender, block_req_receiver) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), v.clone(), block_req_sender.clone(), req_sender.clone(), tx_pub.clone(), c.clone(), cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender.clone(), resp_sender.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        handle_remote_msg(generate_blk_msg(tx), v.clone(), block_req_sender.clone(), req_sender, tx_pub, c.clone(), cache, batch_new_tx_pool, pool_txs_sender, resp_sender);
        let (verify_type, request_id, req, submodule, _, _) = block_req_receiver.recv().unwrap();
        assert_eq!(verify_type, VerifyType::BlockVerify);
        assert_eq!(request_id, 88);
        assert_eq!(submodules::CONSENSUS, submodule);
        assert_eq!(req.get_tx_hash().to_vec().clone(), tx_hash);

        let block_verify_status = c.read();
        assert_eq!(block_verify_status.block_verify_result, VerifyResult::VerifyOngoing);
        assert_eq!(block_verify_status.verify_success_cnt_required, 1);
        assert_eq!(block_verify_status.verify_success_cnt_capture, 0);
    }

    #[test]
    fn handle_verificaton_result_single_tx() {
        let (tx_pub, _) = channel();
        let (block_req_sender, _) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), block_req_sender.clone(), req_sender.clone(), tx_pub.clone(), block_verify_status.clone(), cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender.clone(), resp_sender.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        handle_remote_msg(generate_msg(tx), verifier.clone(), block_req_sender.clone(), req_sender, tx_pub.clone(), block_verify_status.clone(), cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender, resp_sender.clone());

        let (_, _, verify_req, _, _, _) = req_receiver.try_recv().unwrap();
        //let resp = verify_tx_service(verify_req, verifier, verify_cache);
        let mut req_grp: Vec<VerifyReqInfo> = Vec::new();
        let verify_req_info = VerifyReqInfo {
            req: verify_req,
            info: (VerifyType::SingleVerify, 0, submodules::JSON_RPC, SystemTime::now(), 0),
        };
        req_grp.push(verify_req_info);
        verify_tx_group_service(req_grp, verifier, verify_cache, resp_sender);

        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status, batch_new_tx_pool, pool_tx_sender);
        let (_, _, resp_msg, _, _) = pool_tx_receiver.try_recv().unwrap();
        let ok_result = format!("{:?}", Ret::Ok);
        assert_eq!(resp_msg.status, ok_result);
        assert_eq!(tx_hash, resp_msg.hash.to_vec());
    }

    #[test]
    fn handle_verificaton_result_block_tx() {

        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (block_req_sender, block_req_receiver) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, _) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), block_req_sender.clone(), req_sender.clone(), tx_pub.clone(), block_verify_status.clone(), cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender.clone(), resp_sender.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg(tx), verifier.clone(), block_req_sender.clone(), req_sender, tx_pub.clone(), block_verify_status.clone(), cache, batch_new_tx_pool.clone(), pool_txs_sender, resp_sender.clone());
        let (_, request_id, verify_req, submodule, _, _) = block_req_receiver.recv().unwrap();

        let mut req_grp: Vec<VerifyReqInfo> = Vec::new();
        let verify_req_info = VerifyReqInfo {
            req: verify_req,
            info: (VerifyType::BlockVerify, request_id, submodule, SystemTime::now(), 0),
        };
        req_grp.push(verify_req_info);
        verify_tx_group_service(req_grp, verifier, verify_cache, resp_sender);

        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status, batch_new_tx_pool, pool_tx_sender);

        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), request_id);
            }
            _ => {
                panic!("test failed")
            }
        }
    }

    #[test]
    fn block_verificaton_failed() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (block_req_sender, block_req_receiver) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, _) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), block_req_sender.clone(), req_sender.clone(), tx_pub.clone(), block_verify_status.clone(), cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender.clone(), resp_sender.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey().clone();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg_with_fake_signature(tx, pubkey), verifier.clone(), block_req_sender.clone(), req_sender, tx_pub.clone(), block_verify_status.clone(), cache, batch_new_tx_pool.clone(), pool_txs_sender, resp_sender.clone());
        let (_, request_id, verify_req, submodule, _, _) = block_req_receiver.recv().unwrap();

        let mut req_grp: Vec<VerifyReqInfo> = Vec::new();
        let verify_req_info = VerifyReqInfo {
            req: verify_req,
            info: (VerifyType::BlockVerify, request_id, submodule, SystemTime::now(), 0),
        };
        req_grp.push(verify_req_info);
        verify_tx_group_service(req_grp, verifier, verify_cache, resp_sender);

        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status, batch_new_tx_pool, pool_tx_sender);
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::BadSig);
                assert_eq!(resp.get_id(), request_id);
            }
            _ => {
                panic!("test failed")
            }
        }
    }

    #[test]
    fn get_tx_verificaton_from_cache() {

        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (block_req_sender, block_req_receiver) = channel();
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
        let batch_new_tx_pool = Arc::new(Mutex::new(HashMap::new()));
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, _) = channel();

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), block_req_sender.clone(), req_sender.clone(), tx_pub.clone(), block_verify_status.clone(), verify_cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender.clone(), resp_sender.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg(tx.clone()), verifier.clone(), block_req_sender.clone(), req_sender.clone(), tx_pub.clone(), block_verify_status.clone(), verify_cache.clone(), batch_new_tx_pool.clone(), pool_txs_sender.clone(), resp_sender.clone());
        let (_, request_id, verify_req, submodule, _, _) = block_req_receiver.recv().unwrap();

        let mut req_grp: Vec<VerifyReqInfo> = Vec::new();
        let verify_req_info = VerifyReqInfo {
            req: verify_req,
            info: (VerifyType::BlockVerify, request_id, submodule, SystemTime::now(), 0),
        };
        req_grp.push(verify_req_info.clone());
        verify_tx_group_service(req_grp, verifier.clone(), verify_cache.clone(), resp_sender.clone());
        handle_verificaton_result(&resp_receiver, &tx_pub, block_verify_status.clone(), batch_new_tx_pool.clone(), pool_tx_sender.clone());
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), request_id);
            }
            _ => {
                panic!("test failed")
            }
        }
        //Begin to construct the same tx's verification request
        handle_remote_msg(generate_blk_msg(tx), verifier.clone(), block_req_sender.clone(), req_sender, tx_pub.clone(), block_verify_status.clone(), verify_cache, batch_new_tx_pool.clone(), pool_txs_sender, resp_sender.clone());
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), request_id);
            }
            _ => {
                panic!("test failed")
            }
        }

    }
}
