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
extern crate bincode;
extern crate cita_crypto as crypto;
extern crate clap;
extern crate common_types;
extern crate core;
extern crate cpuprofiler;
extern crate dotenv;
#[macro_use]
extern crate libproto;
extern crate proof;
extern crate protobuf;
extern crate rustc_serialize;
extern crate util;

#[macro_use]
extern crate log;
extern crate logger;
extern crate pubsub;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod generate_block;

use std::convert::TryFrom;
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::time;

use clap::App;

use crypto::*;
use generate_block::Generateblock;
use libproto::Message;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use util::H256;

pub type PubType = (String, Vec<u8>);

fn create_contracts(
    pre_hash: H256,
    height: u64,
    pub_sender: Sender<PubType>,
    sys_time: Arc<Mutex<time::SystemTime>>,
    pk: PrivKey,
    mock_data: serde_json::Value,
) {
    use libproto::SignedTransaction;

    let is_multi_sender = false;
    let txs: Vec<SignedTransaction> = mock_data["addresses"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(address, obj)| {
            obj["transactions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|tx_data| {
                    let code = tx_data["code"].as_str().unwrap();
                    let quota = tx_data["quota"].as_u64().unwrap();
                    let nonce = tx_data["nonce"].as_u64().unwrap() as u32;
                    info!(
                        "address={}, code={}, quota={}, nonce={}",
                        address, code, quota, nonce
                    );
                    Generateblock::generate_tx(address.to_owned(), code, quota, nonce, is_multi_sender, pk)
                })
                .collect()
        })
        .fold(vec![], |mut acc, txs: Vec<_>| {
            acc.extend(txs);
            acc
        });

    // 构造block
    let (send_data, _block) = Generateblock::build_block_with_proof(txs, pre_hash, height);
    info!("===============send block===============");
    (*sys_time.lock().unwrap()) = time::SystemTime::now();
    pub_sender
        .send((
            routing_key!(Consensus >> BlockWithProof).into(),
            send_data.clone(),
        ))
        .unwrap();
}

fn main() {
    logger::init();
    info!("CITA:Chain executor mock");

    let matches = App::new("Chain executor mock")
        .version("0.1")
        .author("Cryptape")
        .arg(
            clap::Arg::with_name("mock-data")
                .short("m")
                .long("mock-data")
                .required(true)
                .takes_value(true)
                .help("JSON format mock data"),
        )
        .get_matches();

    let mock_data_path = matches.value_of("mock-data").unwrap();
    let mut mock_data_string = String::new();
    fs::File::open(mock_data_path)
        .expect("Open mock data file error")
        .read_to_string(&mut mock_data_string)
        .expect("Read mock data file error");
    let mock_data: serde_json::Value = serde_json::from_str(mock_data_string.as_str()).expect("Parse mock data error");

    info!("mock-data-path={}", mock_data_path);
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

    loop {
        let (key, body) = rx_sub.recv().unwrap();
        info!("received: key={}", key);
        let mut msg = Message::try_from(&body).unwrap();
        match RoutingKey::from(&key) {
            // 接受chain发送的 authorities_list
            routing_key!(Chain >> RichStatus) => {
                let rich_status = msg.take_rich_status().unwrap();
                let height = rich_status.height + 1;
                info!("send consensus blk . h = {:?}", height);
                create_contracts(
                    H256::from_slice(&rich_status.hash),
                    height,
                    tx_pub.clone(),
                    sys_time.clone(),
                    pk.clone(),
                    mock_data,
                );
                break;
            }
            _ => (),
        }
    }
    info!("[[DONE]]");
}
