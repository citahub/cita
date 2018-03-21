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

use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::time;

use clap::App;

use crypto::PrivKey;
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
    mock_block: &serde_json::Value,
    privkey: &PrivKey,
) {
    use libproto::SignedTransaction;

    let txs: Vec<SignedTransaction> = mock_block["transactions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tx| {
            let contract_address = tx["contract_address"].as_str().unwrap();
            let tx_privkey_str = tx["privkey"].as_str().unwrap();
            let tx_privkey: PrivKey = H256::from_any_str(tx_privkey_str).unwrap().into();
            let code = tx["code"].as_str().unwrap();
            let quota = tx["quota"].as_u64().unwrap();
            let nonce = tx["nonce"].as_u64().unwrap() as u32;
            let valid_until_block = tx["valid_until_block"].as_u64().unwrap();
            info!(
                "address={}, code={}, quota={}, nonce={}",
                contract_address, code, quota, nonce
            );
            Generateblock::generate_tx(
                contract_address,
                code,
                quota,
                nonce,
                valid_until_block,
                &tx_privkey,
            )
        })
        .collect();

    // 构造block
    let (send_data, _block) = Generateblock::build_block_with_proof(&txs, pre_hash, height, privkey);
    info!(
        "===============send block ({} transactions)===============",
        txs.len()
    );
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
    let mut mock_data: serde_json::Value =
        serde_json::from_str(mock_data_string.as_str()).expect("Parse mock data error");

    info!("mock-data-path={}", mock_data_path);
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();

    start_pubsub(
        "consensus",
        routing_key!([Chain >> RichStatus]),
        tx_sub,
        rx_pub,
    );
    let sys_time = Arc::new(Mutex::new(time::SystemTime::now()));

    let privkey: PrivKey = {
        let privkey_str = mock_data["privkey"].as_str().unwrap();
        H256::from_any_str(privkey_str).unwrap().into()
    };
    let mut mock_blocks: HashMap<u64, serde_json::Value> = HashMap::new();
    for block in mock_data["blocks"].as_array_mut().unwrap().into_iter() {
        let block_number = block["number"].as_u64().unwrap();
        mock_blocks.insert(block_number, block.take());
    }
    info!(">> numbers: {:?}", mock_blocks.keys());
    for number in 1..(mock_blocks.len() as u64 + 1) {
        if !mock_blocks.contains_key(&number) {
            error!("Block missing, number={}", number);
            return;
        }
    }

    loop {
        let (key, body) = rx_sub.recv().unwrap();
        info!("received: key={}", key);
        let mut msg = Message::try_from(&body).unwrap();
        match RoutingKey::from(&key) {
            // 接受 chain 发送的 authorities_list
            routing_key!(Chain >> RichStatus) => {
                let rich_status = msg.take_rich_status().unwrap();
                let height = rich_status.height + 1;
                if let Some(mock_block) = mock_blocks.remove(&height) {
                    info!(
                        "send consensus block rich_status.height={} height = {:?}",
                        rich_status.height, height
                    );
                    create_contracts(
                        H256::from_slice(&rich_status.hash),
                        height,
                        tx_pub.clone(),
                        sys_time.clone(),
                        &mock_block,
                        &privkey,
                    );
                } else {
                    warn!("No data for this block height = {:?}", height);
                };
                if mock_blocks.is_empty() {
                    break;
                }
            }
            _ => (),
        }
    }
    info!("[[DONE]]");
}
