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

#![feature(try_from)]
#![feature(tool_lints)]

extern crate bincode;
extern crate cita_crypto as crypto;
extern crate cita_types;
extern crate clap;
extern crate common_types;
extern crate core;
extern crate cpuprofiler;
extern crate dotenv;
#[macro_use]
extern crate libproto;
extern crate proof;
extern crate rustc_serialize;
extern crate util;

#[macro_use]
extern crate logger;
extern crate pubsub;
#[macro_use]
extern crate serde_derive;

mod generate_block;

use cita_types::H256;
use clap::App;
use crypto::*;
use generate_block::Generateblock;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use pubsub::start_pubsub;
use std::convert::TryFrom;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time;

pub type PubType = (String, Vec<u8>);

#[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
fn create_contract(
    block_tx_num: u64,
    pre_hash: H256,
    flag: i32,
    h: u64,
    pub_sender: &Sender<PubType>,
    sys_time: &Arc<Mutex<time::SystemTime>>,
    quota: u64,
    flag_multi_sender: i32,
    pk: PrivKey,
) {
    let code = match flag {
        1 => {
            "60606040523415600e57600080fd5b\
             5b5b5b60948061001f6000396000f3\
             0060606040526000357c0100000000\
             000000000000000000000000000000\
             000000000000000000900463ffffff\
             ff1680635524107714603d575b6000\
             80fd5b3415604757600080fd5b605b\
             600480803590602001909190505060\
             5d565b005b806000819055505b5056\
             00a165627a7a72305820c471b43766\
             26da2540b2374e8b4110501051c426\
             ff46814a6170ce9e219e49a80029"
        }
        0 => "0000000000000000000000000000000000000000",
        _ => "4f2be91f",
    };

    let contract_address = match flag {
        1 => "",
        0 => common_types::reserved_addresses::STORE_ADDRESS,
        _ => "0000000000000000000000000000000082720029",
    };
    let mut txs = Vec::new();
    for _ in 0..block_tx_num - 1 {
        let tx = Generateblock::generate_tx(
            code,
            contract_address.to_string().clone(),
            quota,
            flag_multi_sender,
            pk,
        );
        txs.push(tx);
    }
    let tx = Generateblock::generate_tx(
        code,
        contract_address.to_string().clone(),
        quota,
        flag_multi_sender,
        pk,
    );
    txs.push(tx);

    // 构造block
    let (send_data, _block) = Generateblock::build_block_with_proof(txs, pre_hash, h);
    info!("===============send block===============");
    let mut sys_time_lock = sys_time.lock().unwrap();
    *sys_time_lock = time::SystemTime::now();
    pub_sender
        .send((
            routing_key!(Consensus >> BlockWithProof).into(),
            send_data.clone(),
        ))
        .unwrap();
}

fn main() {
    logger::init();
    info!("CITA:Chain Performance by MQ");

    let matches = App::new("Chain Performance by MQ")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Chain Performance by MQ powered by Rust")
        .arg_from_usage("--totaltx=[20000] 'transation num in one block'")
        .arg_from_usage(
            "--times=[0] 'how many times to send block, i.e. block-height. 0 means limitless'",
        )
        .arg_from_usage("--quota=[1000] 'transation quota'")
        .arg_from_usage("--flag_multi_sender=[0] 'Multi sender or not'")
        .arg_from_usage(
            "--flag_tx_type=[1] 'tx type: 0 is store, 1 is creating contract, 2 is call contract'",
        )
        .get_matches();

    let totaltx = matches
        .value_of("totaltx")
        .unwrap_or("40000")
        .parse::<u64>()
        .unwrap();
    let times = matches
        .value_of("times")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap();
    let flag_multi_sender = matches
        .value_of("flag_multi_sender")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    let quota = matches
        .value_of("quota")
        .unwrap_or("1000")
        .parse::<u64>()
        .unwrap();
    let flag_tx_type = matches
        .value_of("flag_tx_type")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();

    let mut send_flag = true;
    let mut height = 0;
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    let keypair = KeyPair::gen_keypair();
    let pk = keypair.privkey();

    start_pubsub(
        "consensus",
        routing_key!([Chain >> RichStatus]),
        tx_sub,
        rx_pub,
    );
    let sys_time = Arc::new(Mutex::new(time::SystemTime::now()));

    let mut count_times: u64 = 0;

    loop {
        let (key, body) = rx_sub.recv().unwrap();
        let mut msg = Message::try_from(&body).unwrap();
        if let routing_key!(Chain >> RichStatus) = RoutingKey::from(&key) {
            //接受chain发送的 authorities_list
            let rich_status = msg.take_rich_status().unwrap();
            info!("get new local status {:?}", rich_status.height);
            if !send_flag && rich_status.height == height {
                let start_time = sys_time.lock().unwrap();
                let end_time = time::SystemTime::now();
                let diff = end_time
                    .duration_since(*start_time)
                    .expect("SystemTime::duration_since failed");
                let mut secs = diff.as_secs();
                let nanos = diff.subsec_nanos();
                secs = secs * 1000 + u64::from(nanos / 1_000_000);
                let tps = if secs > 0 {
                    totaltx * 1000 / secs
                } else {
                    totaltx
                };

                info!(
                    "tx_num = {}, use time = {} ms, tps = {}",
                    totaltx, secs, tps
                );

                if times != 0 {
                    count_times += 1;
                    if count_times >= times {
                        break;
                    }
                }

                send_flag = true;
            }
            if send_flag {
                height = rich_status.height + 1;
                info!("send consensus blk . h = {:?}", height);
                create_contract(
                    totaltx,
                    H256::from_slice(&rich_status.hash),
                    flag_tx_type,
                    height,
                    &tx_pub,
                    &Arc::clone(&sys_time),
                    quota,
                    flag_multi_sender,
                    *pk,
                );
                send_flag = false;
            }
        }
    }
}
