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
extern crate cita_types;
extern crate clap;
#[macro_use]
extern crate libproto;
extern crate proof;
extern crate rustc_serialize;
extern crate util;

#[macro_use]
extern crate logger;
extern crate pubsub;
extern crate rlp;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate serde_yaml;

mod generate_block;

use std::collections::HashMap;
use std::convert::{From, TryFrom};
use std::env;
use std::io::Read;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::time;
use std::{fs, u8};

use clap::App;

use cita_types::traits::LowerHex;
use cita_types::{H256, U256};
use crypto::{CreateKey, KeyPair, PrivKey};
use generate_block::BuildBlock;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use pubsub::start_pubsub;

pub type PubType = (String, Vec<u8>);

const GENESIS_TIMESTAMP: u64 = 1_524_000_000;

fn main() {
    dotenv::dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
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
                .help("YAML format mock data"),
        )
        .get_matches();

    let mock_data_path = matches.value_of("mock-data").unwrap();
    let mut mock_data_string = String::new();
    fs::File::open(mock_data_path)
        .expect("Open mock data file error")
        .read_to_string(&mut mock_data_string)
        .expect("Read mock data file error");
    let mut mock_data: serde_yaml::Value =
        serde_yaml::from_str(mock_data_string.as_str()).expect("Parse mock data error");

    info!("mock-data-path={}", mock_data_path);
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();

    start_pubsub(
        "consensus",
        routing_key!([Chain >> RichStatus]),
        tx_sub,
        rx_pub,
    );
    let amqp_url = std::env::var("AMQP_URL").expect("AMQP_URL empty");
    info!("AMQP_URL={}", amqp_url);
    let sys_time = Arc::new(Mutex::new(time::SystemTime::now()));

    let privkey: PrivKey = {
        let privkey_str = mock_data["privkey"].as_str().unwrap();
        PrivKey::from_str(privkey_str).unwrap()
    };
    let mut mock_blocks: HashMap<u64, &serde_yaml::Value> = HashMap::new();
    for block in mock_data["blocks"].as_sequence_mut().unwrap() {
        let block_number = block["number"].as_u64().unwrap();
        mock_blocks.insert(block_number, block);
    }
    {
        let mut numbers = mock_blocks.keys().collect::<Vec<&u64>>();
        numbers.sort();
        info!(">> numbers: {:?}", numbers);
    }
    for number in 1..=mock_blocks.len() as u64 {
        if !mock_blocks.contains_key(&number) {
            error!("Block missing, number={}", number);
            return;
        }
    }
    let mut repeat = 0u8;
    let mut _current_height = 0u8;

    loop {
        let (key, body) = rx_sub.recv().unwrap();
        info!("received: key={}", key);
        let mut msg = Message::try_from(&body).unwrap();
        // 接受 chain 发送的 authorities_list
        if RoutingKey::from(&key) == routing_key!(Chain >> RichStatus) {
            let rich_status = msg.take_rich_status().unwrap();
            let height = rich_status.height + 1;

            // Remove previous block
            if mock_blocks.remove(&rich_status.height).is_some() {
                _current_height = rich_status.height as u8;
                repeat = 0;
            } else if repeat < u8::MAX {
                repeat += 1;
            }

            if repeat >= 3 {
                warn!("the {} block can't generate", height);
            }

            if let Some(mock_block) = mock_blocks.get(&height) {
                info!(
                    "send consensus block rich_status.height={} height = {:?}",
                    rich_status.height, height
                );
                send_block(
                    H256::from_slice(&rich_status.hash),
                    height,
                    &tx_pub,
                    &sys_time.clone(),
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
    }
    info!("[[DONE]]");
}

// Build the block from transactions, then send it to MQ
fn send_block(
    pre_hash: H256,
    height: u64,
    pub_sender: &Sender<PubType>,
    sys_time: &Arc<Mutex<time::SystemTime>>,
    mock_block: &serde_yaml::Value,
    privkey: &PrivKey,
) {
    use libproto::SignedTransaction;

    let txs: Vec<SignedTransaction> = mock_block["transactions"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|tx| {
            let contract_address = tx["to"].as_str().unwrap();
            let tx_privkey_str = tx["privkey"].as_str().unwrap();
            let tx_privkey: PrivKey = PrivKey::from_str(tx_privkey_str).unwrap();
            let data = tx["data"].as_str().unwrap();
            let quota = tx["quota"].as_u64().unwrap();
            let nonce = tx["nonce"].as_u64().unwrap() as u32;
            let valid_until_block = tx["valid_until_block"].as_u64().unwrap();

            let sender = KeyPair::from_privkey(*privkey).unwrap().address();
            info!(
                "sender={}, contract_address={}",
                sender.lower_hex(),
                BuildBlock::build_contract_address(&sender, &U256::from(nonce)).lower_hex()
            );
            info!(
                "address={}, quota={}, nonce={}",
                contract_address, quota, nonce
            );
            BuildBlock::build_tx(
                contract_address,
                data,
                quota,
                nonce,
                valid_until_block,
                &tx_privkey,
            )
        })
        .collect();

    // 构造block
    let (send_data, _block) = BuildBlock::build_block_with_proof(
        &txs[..],
        pre_hash,
        height,
        privkey,
        GENESIS_TIMESTAMP + height * 3,
    );
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
