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
extern crate cita_log;
extern crate protobuf;
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

pub mod handler;
pub mod verify;
pub mod cache;

use cache::{VerifyCache, VerifyBlockCache, VerifyResult, BlockVerifyId};
use clap::App;
use cpuprofiler::PROFILER;
use dotenv::dotenv;
use handler::{verify_tx_service, VerifyType, handle_remote_msg, handle_verificaton_result};
use log::LogLevelFilter;
use pubsub::start_pubsub;
use std::env;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use util::RwLock;
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

    // Init logger
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:auth");
    // init app
    let matches = App::new("auth")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-n, --tx_verify_thread_num=[10] 'transaction verification thread count'")
        .args_from_usage("--prof-start=[0] 'Specify the start time of profiling, zero means no profiling'")
        .args_from_usage("--prof-duration=[0] 'Specify the duration for profiling, zero means no profiling'")
        .get_matches();

    let tx_verify_thread_num = matches.value_of("tx_verify_thread_num").unwrap_or("10").parse::<usize>().unwrap();
    let flag_prof_start = matches.value_of("prof-start").unwrap_or("0").parse::<u64>().unwrap();
    let flag_prof_duration = matches.value_of("prof-duration").unwrap_or("0").parse::<u64>().unwrap();
    let threadpool = threadpool::ThreadPool::new(tx_verify_thread_num);

    profifer(flag_prof_start, flag_prof_duration);

    let verifier = Arc::new(RwLock::new(Verifier::new()));
    let cache = Arc::new(RwLock::new(VerifyCache::new(1000)));
    let block_cache = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("auth", vec!["*.verify_req", "chain.txhashes"], tx_sub, rx_pub);

    let (req_sender, req_receiver) = channel();
    let (resp_sender, resp_receiver) = channel();
    let verifier_clone = verifier.clone();
    let block_cache_clone = block_cache.clone();
    let cache_clone = cache.clone();
    thread::spawn(move || loop {
                      let (verify_type, id, verify_req, sub_module) = req_receiver.recv().unwrap();
                      //once one of the verification request within block verification,
                      // the other requests within the same block should be cancelled.
                      let request_id = BlockVerifyId {
                          request_id: id,
                          sub_module: sub_module,
                      };
                      if VerifyType::BlockVerify == verify_type && VerifyResult::VerifyFailed == block_cache_clone.read().get(&request_id).unwrap().block_verify_result {
                          info!("skip the tx verification due to failed already for block id:{} ", id);
                          continue;
                      }
                      let pool = threadpool.clone();
                      let verifier_clone_222 = verifier_clone.clone();
                      let cache_clone_222 = cache_clone.clone();
                      let resp_sender_clone = resp_sender.clone();
                      pool.execute(move || {
                                       let resp = verify_tx_service(verify_req, verifier_clone_222, cache_clone_222);
                                       resp_sender_clone.send((verify_type, id, resp, sub_module)).unwrap();
                                   });
                  });


    let tx_pub_clone = tx_pub.clone();
    let block_cache_clone_111 = block_cache.clone();
    thread::spawn(move || loop {
                      let (key, msg) = rx_sub.recv().unwrap();
                      info!("get key: {} and msg: {:?}", key, msg);
                      let block_cache_clone_222 = block_cache_clone_111.clone();
                      handle_remote_msg(msg, verifier.clone(), req_sender.clone(), tx_pub_clone.clone(), block_cache_clone_222);
                  });

    loop {
        handle_verificaton_result(&resp_receiver, &tx_pub, block_cache.clone());
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
        //create verify message
        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(tx.get_transaction_with_sig().get_transaction().get_valid_until_block());
        let signature = tx.get_transaction_with_sig().get_signature().to_vec();
        req.set_signature(signature);
        let bytes = tx.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash().to_vec();
        req.set_hash(hash);
        req.set_tx_hash(tx.get_tx_hash().to_vec());

        let msg = factory::create_msg(submodules::CONSENSUS, topics::VERIFY_TX_REQ, communication::MsgType::VERIFY_TX_REQ, req.write_to_bytes().unwrap());
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

    fn generate_blk_msg_with_fake_signature(tx: SignedTransaction) -> Vec<u8> {
        //create verify message
        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(tx.get_transaction_with_sig().get_transaction().get_valid_until_block());
        let mut signature = tx.get_transaction_with_sig().get_signature().to_vec();
        signature[0] = signature[0] + 1;
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

    fn generate_tx_verify_request() -> (VerifyTxReq, PubKey) {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey();
        let tx = generate_tx(vec![1], 99, privkey);

        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(tx.get_transaction_with_sig().get_transaction().get_valid_until_block());
        let signature = tx.get_transaction_with_sig().get_signature().to_vec();
        req.set_signature(signature);
        let bytes = tx.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash().to_vec();
        req.set_hash(hash);
        req.set_tx_hash(tx.get_tx_hash().to_vec());

        (req, pubkey.clone())
    }

    #[test]
    fn verify_sync_block_hash() {

        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let c = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), v.clone(), req_sender, tx_pub, c);
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
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let c = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));

        let height = 1;
        handle_remote_msg(generate_sync_blk_hash_msg(height), v.clone(), req_sender, tx_pub, c);

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
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let c = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        handle_remote_msg(generate_msg(tx), v.clone(), req_sender, tx_pub, c);
        let (verify_type, request_id, req, submodule) = req_receiver.recv().unwrap();
        assert_eq!(verify_type, VerifyType::SingleVerify);
        assert_eq!(request_id, 0);
        assert_eq!(submodules::CONSENSUS, submodule);
        assert_eq!(req.get_tx_hash().to_vec().clone(), tx_hash);
    }

    #[test]
    fn verify_block_tx_request_dispatch_success() {

        let (tx_pub, _) = channel();
        let (req_sender, req_receiver) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let c = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        handle_remote_msg(generate_blk_msg(tx), v.clone(), req_sender, tx_pub, c.clone());
        let (verify_type, request_id, req, submodule) = req_receiver.recv().unwrap();
        assert_eq!(verify_type, VerifyType::BlockVerify);
        assert_eq!(request_id, 88);
        assert_eq!(submodules::CONSENSUS, submodule);
        assert_eq!(req.get_tx_hash().to_vec().clone(), tx_hash);
        let request_id = BlockVerifyId {
            request_id: request_id,
            sub_module: submodule,
        };
        let read_guard = c.read();
        if let Some(block_verify_status) = read_guard.get(&request_id) {
            assert_eq!(block_verify_status.block_verify_result, VerifyResult::VerifyOngoing);
            assert_eq!(block_verify_status.verify_success_cnt_required, 1);
            assert_eq!(block_verify_status.verify_success_cnt_capture, 0);

        } else {
            panic!("Test failed for: verify_block_tx_request_dispatch_success");
        }
    }

    #[test]
    fn handle_verificaton_result_single_tx() {

        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_cache = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));
        let verify_cache = Arc::new(RwLock::new(VerifyCache::new(1000)));
        let verifier = Arc::new(RwLock::new(Verifier::new()));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), req_sender, tx_pub.clone(), block_cache.clone());

        let (verify_req, pubkey) = generate_tx_verify_request();
        let resp = verify_tx_service(verify_req, verifier, verify_cache);
        resp_sender.send((VerifyType::SingleVerify, 0, resp, submodules::CONSENSUS)).unwrap();

        handle_verificaton_result(&resp_receiver, &tx_pub, block_cache);

        let (_, resp_msg) = rx_pub.recv().unwrap();
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYTXRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(pubkey.to_vec(), resp.get_signer().to_vec());
            }
            _ => {
                panic!("test failed")
            }
        }
    }

    #[test]
    fn handle_verificaton_result_block_tx() {

        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_cache = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));
        let verify_cache = Arc::new(RwLock::new(VerifyCache::new(1000)));
        let verifier = Arc::new(RwLock::new(Verifier::new()));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), req_sender.clone(), tx_pub.clone(), block_cache.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg(tx), verifier.clone(), req_sender, tx_pub.clone(), block_cache.clone());
        let (_, request_id, verify_req, submodule) = req_receiver.recv().unwrap();

        let resp = verify_tx_service(verify_req, verifier, verify_cache);
        resp_sender.send((VerifyType::BlockVerify, request_id, resp, submodule)).unwrap();

        handle_verificaton_result(&resp_receiver, &tx_pub, block_cache);

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
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_cache = Arc::new(RwLock::new(VerifyBlockCache::new(1000)));
        let verify_cache = Arc::new(RwLock::new(VerifyCache::new(1000)));
        let verifier = Arc::new(RwLock::new(Verifier::new()));

        let height = 0;
        handle_remote_msg(generate_sync_blk_hash_msg(height), verifier.clone(), req_sender.clone(), tx_pub.clone(), block_cache.clone());

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);
        handle_remote_msg(generate_blk_msg_with_fake_signature(tx), verifier.clone(), req_sender, tx_pub.clone(), block_cache.clone());
        let (_, request_id, verify_req, submodule) = req_receiver.recv().unwrap();

        let resp = verify_tx_service(verify_req, verifier, verify_cache);
        resp_sender.send((VerifyType::BlockVerify, request_id, resp, submodule)).unwrap();

        handle_verificaton_result(&resp_receiver, &tx_pub, block_cache);

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
}
