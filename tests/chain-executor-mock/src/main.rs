// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cita_crypto as crypto;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;
#[macro_use]
extern crate serde_derive;

mod generate_block;

use pubsub::channel::{self, Sender};
use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time;
use std::{fs, u8};

use crate::crypto::{CreateKey, KeyPair, PrivKey};
use crate::generate_block::BuildBlock;
use cita_types::traits::LowerHex;
use cita_types::{H256, U256};
use clap::App;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use libproto::TryFrom;
use pubsub::start_pubsub;

pub type PubType = (String, Vec<u8>);

const GENESIS_TIMESTAMP: u64 = 1_524_000_000;

fn main() {
    dotenv::dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
    logger::init_config(&logger::LogFavour::File("chain-executor-mock"));
    info!("CITA:Chain executor mock");

    let matches = App::new("Chain executor mock")
        .version("0.1.0")
        .author("Rivtower")
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
    let (tx_sub, rx_sub) = channel::unbounded();
    let (tx_pub, rx_pub) = channel::unbounded();

    start_pubsub(
        "consensus",
        routing_key!([Chain >> RichStatus]),
        tx_sub,
        rx_pub,
    );
    let amqp_url = std::env::var("AMQP_URL").expect("AMQP_URL empty");
    info!("AMQP_URL={}", amqp_url);
    let sys_time = Arc::new(Mutex::new(time::SystemTime::now()));

    let privkey = mock_data["privkey"]
        .as_str()
        .and_then(|p| PrivKey::from_str(p).ok())
        .unwrap();
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
    loop {
        let (key, body) = rx_sub.recv().unwrap();
        info!("received: key={}", key);
        let mut msg = Message::try_from(&body).unwrap();
        // Receive authorities_list from chain
        if RoutingKey::from(&key) == routing_key!(Chain >> RichStatus) {
            let rich_status = msg.take_rich_status().unwrap();
            let height = rich_status.height + 1;

            // Remove previous block
            if mock_blocks.remove(&rich_status.height).is_some() {
                let current_height = rich_status.height as u8;
                info!("current height-{:?}", current_height);
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
                warn!("no data for this block height = {:?}", height);
            };
            if mock_blocks.is_empty() {
                warn!("break for empty...");
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

    // Build block
    let (send_data, _block) = BuildBlock::build_block_with_proof(
        &txs[..],
        pre_hash,
        height,
        privkey,
        GENESIS_TIMESTAMP + height * 3,
    );
    info!("send block ({} transactions)", txs.len());
    *sys_time.lock().unwrap() = time::SystemTime::now();
    pub_sender
        .send((
            routing_key!(Consensus >> BlockWithProof).into(),
            send_data.clone(),
        ))
        .unwrap();
}
