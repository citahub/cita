// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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
extern crate chrono;
extern crate cita_crypto as crypto;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate proof;
extern crate pubsub;
#[macro_use]
extern crate serde_derive;
extern crate cita_types as types;
extern crate serde_yaml;
extern crate util;

use bincode::{serialize, Infinite};
use clap::App;
use crypto::{CreateKey, KeyPair, PrivKey, Sign, Signature};
use libproto::blockchain::{Block, BlockBody, BlockTxs, BlockWithProof};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use proof::BftProof;
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::convert::{Into, TryFrom, TryInto};
use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use types::{Address, H256};
use util::Hashable;

pub type PubType = (String, Vec<u8>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

fn build_proof(height: u64, sender: Address, privkey: &PrivKey) -> BftProof {
    let mut proof = BftProof::default();
    proof.height = height as usize;
    proof.round = 0;
    proof.proposal = H256::default();

    let mut commits = HashMap::new();
    let message = serialize(
        &(
            proof.height,
            proof.round,
            Step::Precommit,
            sender,
            Some(proof.proposal),
        ),
        Infinite,
    )
    .unwrap();

    let signature = Signature::sign(privkey, &message.crypt_hash()).unwrap();
    commits.insert((*sender).into(), signature);
    proof.commits = commits;
    proof
}

fn build_block(
    //txs: &Vec<SignedTransaction>,
    body: &BlockBody,
    pre_block_hash: H256,
    height: u64,
    privkey: &PrivKey,
    time_stamp: u64,
) -> (Vec<u8>, BlockWithProof) {
    let sender = KeyPair::from_privkey(*privkey).unwrap().address();
    let mut block = Block::new();
    let proof = build_proof(height, sender, privkey);
    let transaction_root = body.transactions_root().to_vec();
    let mut proof_blk = BlockWithProof::new();

    let mut previous_proof = proof.clone();
    previous_proof.height = height as usize - 1;
    block.mut_header().set_timestamp(time_stamp);
    block.mut_header().set_height(height);
    block.mut_header().set_prevhash(pre_block_hash.0.to_vec());
    block.mut_header().set_proof(previous_proof.into());
    block.mut_header().set_transactions_root(transaction_root);
    block.set_body(body.clone());

    proof_blk.set_blk(block);
    proof_blk.set_proof(proof.into());

    let msg: Message = proof_blk.clone().into();
    (msg.try_into().unwrap(), proof_blk)
}

fn send_block(
    pre_block_hash: H256,
    height: u64,
    pub_sender: &Sender<PubType>,
    timestamp: u64,
    block_txs: &BlockTxs,
    privkey: &PrivKey,
) {
    // let txs = &block_txs.body.get_ref().transactions.clone().into_vec();
    let (send_data, _block) = build_block(
        &block_txs.body.get_ref(),
        pre_block_hash,
        height,
        privkey,
        timestamp,
    );
    pub_sender
        .send((
            routing_key!(Consensus >> BlockWithProof).into(),
            send_data.clone(),
        ))
        .unwrap();
}

fn main() {
    logger::init_config("consensus_mock");
    info!("CITA: Consensus Mock");

    // set up the clap to receive info from CLI
    let matches = App::new("consensus mock")
        .version("0.1")
        .author("Cryptape")
        .about("Mock the process of consensus")
        .arg(
            clap::Arg::with_name("interval")
                .short("i")
                .long("interval(seconds) of block generating, default: 3")
                .required(true)
                .takes_value(true)
                .help("Set the path of mock data in YAML format"),
        )
        .get_matches();

    let default_interval = 3;
    // get the mock data and parse it to serde_yaml format
    let interval = value_t!(matches, "interval", u64).unwrap_or(default_interval);
    let key_pair = KeyPair::gen_keypair();
    let pk_miner = key_pair.privkey();

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();

    start_pubsub(
        "consensus",
        routing_key!([Auth >> BlockTxs, Chain >> RichStatus,]),
        tx_sub,
        rx_pub,
    );

    let mut received_block_txs: HashMap<usize, BlockTxs> = HashMap::new();

    let mut send_height = 0;
    let interval_duration = Duration::new(interval, 0);
    let mut last_new_block_at = Instant::now();
    loop {
        match rx_sub.recv_timeout(interval_duration) {
            Ok((key, body)) => {
                let routing_key = RoutingKey::from(&key);
                let mut msg = Message::try_from(body).unwrap();

                match routing_key {
                    routing_key!(Auth >> BlockTxs) => {
                        // add received block
                        let block_txs = msg.take_block_txs().unwrap();
                        let height = block_txs.get_height() as usize;
                        received_block_txs.insert(height, block_txs);
                    }
                    routing_key!(Chain >> RichStatus) => {
                        // update rich status
                        let rich_status = msg.take_rich_status().unwrap();
                        if rich_status.height < send_height {
                            continue;
                        }

                        // sleep until hit inteval
                        let seconds_since_last = last_new_block_at.elapsed().as_secs();
                        if seconds_since_last < interval {
                            sleep(Duration::from_secs(interval - seconds_since_last));
                        } else {
                            last_new_block_at = Instant::now();
                        }

                        // current timestamp
                        let timestamp = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("get timestamp error")
                            .as_secs();

                        if let Some(block_txs) =
                            received_block_txs.remove(&(rich_status.height as usize))
                        {
                            send_height = rich_status.height + 1;
                            send_block(
                                H256::from_slice(&rich_status.hash),
                                send_height,
                                &tx_pub,
                                timestamp,
                                &block_txs,
                                &pk_miner,
                            );
                        } else {
                            warn!(
                                "No received block_txs at rich_status_height = {:?}",
                                rich_status.height
                            );
                        }
                        trace!("get new local status {:?}", rich_status);
                    }
                    _ => {}
                }
            }
            Err(err) => {
                if err != RecvTimeoutError::Timeout {
                    error!("consensus err {:?}", err)
                }
            }
        }
    }
}
