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

use clap::App;
use cpuprofiler::PROFILER;
use dotenv::dotenv;
use log::LogLevelFilter;
use pubsub::start_pubsub;
use std::env;
use std::sync::mpsc::channel;
use std::thread;
use verify::Verifier;
use cache::{VerifyCache, VerifyBlockCache, VerifyResult};
use handler::{verify_tx_service, VerifyType, handle_remote_msg, handle_verificaton_result};
use util::RwLock;
use std::sync::Arc;

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
//    let (resp_sender_single, resp_receiver_single) = channel();
//    let (resp_sender_block, resp_receiver_block) = channel();
    let (resp_sender, resp_receiver) = channel();
    let verifier_clone = verifier.clone();
    let block_cache_clone = block_cache.clone();
    let cache_clone = cache.clone();
    thread::spawn(move || loop {
        let (verify_type, id , verify_req, sub_module) = req_receiver.recv().unwrap();
        trace!("(verify_type, id , verify_req, sub_module) is:{:?}, {:?}, {:?}, {:?}", verify_type, id , verify_req, sub_module);
        //once one of the verification request within block verification,
        // the other requests within the same block should be cancelled.
        if VerifyType::BlockVerify == verify_type &&
            VerifyResult::VerifyFailed == block_cache_clone.read().get(id).unwrap().block_verify_result {
            trace!("skip the tx verification due to failed already for block id:{} ", id);
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
        blkreq.set_id(0);
        blkreq.set_reqs(RepeatedField::from_slice(&[req]));

        let msg = factory::create_msg(submodules::CONSENSUS, topics::VERIFY_BLK_REQ, communication::MsgType::VERIFY_BLK_REQ, blkreq.write_to_bytes().unwrap());
        msg.write_to_bytes().unwrap()
    }

    #[test]
    fn verify_tx_ok() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey();
        let tx = generate_tx(vec![1], 99, privkey);

        let (tx_pub, rx_pub) = channel();
        //verify tx
        let mut v = Verifier::new();
        let mut c = VerifyCache::new(1000);
        v.update_hashes(0, vec![], &tx_pub);

        handle_msg(generate_msg(tx), &tx_pub, &mut v, &mut c);
        let (key1, resp_msg) = rx_pub.recv().unwrap();
        println!("get {} : {:?}", key1, resp_msg);
        assert_eq!(key1, "verify_tx_consensus".to_owned());
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYTXRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(pubkey.to_vec(), resp.get_signer());
            }
            _ => {panic!("test failed")}
        }
    }

    #[test]
    fn verify_tx_cache() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey();
        let tx = generate_tx(vec![1], 99, privkey);

        let (tx_pub, rx_pub) = channel();
        //verify tx
        let mut v = Verifier::new();
        let mut c = VerifyCache::new(1000);
        v.update_hashes(0, vec![], &tx_pub);

        handle_msg(generate_msg(tx.clone()), &tx_pub, &mut v, &mut c);
        let (key1, resp_msg) = rx_pub.recv().unwrap();
        println!("get {} : {:?}", key1, resp_msg);
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYTXRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(pubkey.to_vec(), resp.get_signer());
            }
            _ => {panic!("test failed")}
        }

        handle_msg(generate_msg(tx), &tx_pub, &mut v, &mut c);
        let (key1, resp_msg) = rx_pub.recv().unwrap();
        println!("get {} : {:?}", key1, resp_msg);
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYTXRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(pubkey.to_vec(), resp.get_signer());
            }
            _ => {panic!("test failed")}
        }
    }

    #[test]
    fn verify_blk_ok() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let tx = generate_tx(vec![1], 99, privkey);

        let (tx_pub, rx_pub) = channel();
        //verify tx
        let mut v = Verifier::new();
        let mut c = VerifyCache::new(1000);
        v.update_hashes(0, vec![], &tx_pub);

        handle_msg(generate_blk_msg(tx), &tx_pub, &mut v, &mut c);
        let (key1, resp_msg) = rx_pub.recv().unwrap();
        println!("get {} : {:?}", key1, resp_msg);
        assert_eq!(key1, "verify_blk_consensus".to_owned());
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYBLKRESP(resp) => {
                assert_eq!(resp.get_ret(), Ret::Ok);
                assert_eq!(resp.get_id(), 0);
            }
            _ => {panic!("test failed")}
        }
    }
}
