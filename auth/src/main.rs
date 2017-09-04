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
extern crate cita_crypto as crypto;
extern crate util;
pub mod handler;
pub mod verifier;

use clap::App;
use cpuprofiler::PROFILER;
use dotenv::dotenv;
use handler::handle_msg;
use log::LogLevelFilter;
use pubsub::start_pubsub;
use std::env;
use std::sync::mpsc::channel;
use std::thread;
use verifier::Verifier;

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
        .args_from_usage("--prof-start=[0] 'Specify the start time of profiling, zero means no profiling'")
        .args_from_usage("--prof-duration=[0] 'Specify the duration for profiling, zero means no profiling'")
        .get_matches();

    let flag_prof_start = matches.value_of("prof-start").unwrap_or("0").parse::<u64>().unwrap();
    let flag_prof_duration = matches.value_of("prof-duration").unwrap_or("0").parse::<u64>().unwrap();

    profifer(flag_prof_start, flag_prof_duration);

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("auth", vec!["*.verify_req", "*.verify_req_batch", "chain.status"], tx_sub, rx_pub);


    let verifier = Verifier::new();
    loop {
        let (key, msg) = rx_sub.recv().unwrap();
        info!("get {} : {:?}", key, msg);
        handle_msg(msg, &tx_pub, &verifier);

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
    pub fn generate_tx(data: Vec<u8>, valid_until_block: u64, privkey: &PrivKey) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);

        let mut uv_tx = UnverifiedTransaction::new();
        uv_tx.set_transaction(tx);

        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_transaction_with_sig(uv_tx);
        signed_tx.sign(*privkey);

        signed_tx
    }




    #[test]
    fn verify_tx() {
        //create verify message
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey();
        let tx = generate_tx(vec![1], 999, privkey);
        let mut msg = VerifyReqMsg::new();
        msg.set_valid_until_block(999);

        let signature = tx.get_transaction_with_sig().get_signature().to_vec();
        msg.set_signature(signature);


        let bytes = tx.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash().to_vec();

        msg.set_tx_hash(hash.to_vec());
        msg.set_hash(hash);



        let mut vmsg = VerifyReq::new();
        vmsg.set_reqs(RepeatedField::from_slice(&[msg]));


        //verify tx

        let mut v = Verifier::new();
        v.set_height(5);
        let (tx_sub, rx_sub) = channel();
        let (tx_pub, rx_pub) = channel();
        let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_REQ, communication::MsgType::VERIFY_REQ, vmsg.write_to_bytes().unwrap());
        tx_sub.send(("auth.verify_req".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        let (_, msg) = rx_sub.recv().unwrap();
        handle_msg(msg, &tx_pub, &v);
        let (key1, resp_msg) = rx_pub.recv().unwrap();
        println!("get {} : {:?}", key1, resp_msg);
        let (_, _, content) = parse_msg(resp_msg.as_slice());
        match content {
            MsgClass::VERIFYRESP(resps) => {
                for resp in resps.get_resps() {
                    assert_eq!(resp.get_ret(), Ret::Ok);
                    assert_eq!(pubkey.to_vec(), resp.get_signer());
                }
            }

            _ => {}
        }
    }
}
