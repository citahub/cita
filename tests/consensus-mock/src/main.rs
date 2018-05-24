#![feature(try_from)]
extern crate bincode;
extern crate chrono;
extern crate cita_crypto as crypto;
extern crate cita_types;
extern crate clap;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate proof;
extern crate pubsub;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate util;

use bincode::{serialize, Infinite};
use chrono::NaiveDateTime;
use cita_types::{Address, H256};
use clap::{App, ArgMatches};
use crypto::{CreateKey, KeyPair, PrivKey, Sign, Signature};
use libproto::blockchain::{Block, BlockBody, BlockTxs, BlockWithProof};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use proof::TendermintProof;
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::convert::{Into, TryFrom, TryInto};
use std::fs;
use std::io::Read;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use util::Hashable;

pub type PubType = (String, Vec<u8>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

fn build_proof(height: u64, sender: Address, privkey: &PrivKey) -> TendermintProof {
    let mut proof = TendermintProof::default();
    proof.height = (height - 1) as usize;
    proof.round = 0;
    proof.proposal = H256::default();

    let mut commits = HashMap::new();
    let message = serialize(
        &(
            proof.height,
            proof.round,
            Step::Precommit,
            sender.clone(),
            Some(proof.proposal.clone()),
        ),
        Infinite,
    ).unwrap();

    let signature = Signature::sign(privkey, &message.crypt_hash().into()).unwrap();
    commits.insert((*sender).into(), signature.into());
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
    let sender = KeyPair::from_privkey(*privkey).unwrap().address().clone();
    let mut block = Block::new();
    let proof = build_proof(height, sender, privkey);
    let transaction_root = body.transactions_root().to_vec();
    let mut proof_blk = BlockWithProof::new();

    block.mut_header().set_timestamp(time_stamp);
    block.mut_header().set_height(height);
    block.mut_header().set_prevhash(pre_block_hash.0.to_vec());
    block.mut_header().set_proof(proof.clone().into());
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
    pub_sender: Sender<PubType>,
    mock_block: &serde_yaml::Value,
    block_txs: &BlockTxs,
    privkey: &PrivKey,
) {
    let time_stamp = mock_block["time_stamp"].as_str().unwrap();
    let time_stamp = NaiveDateTime::parse_from_str(time_stamp, "%Y-%m-%d %H:%M:%S.%f").unwrap();
    let time_stamp = time_stamp.timestamp_millis() as u64;
    // let txs = &block_txs.body.get_ref().transactions.clone().into_vec();
    let (send_data, _block) = build_block(
        &block_txs.body.get_ref(),
        pre_block_hash,
        height,
        privkey,
        time_stamp,
    );
    pub_sender
        .send((
            routing_key!(Consensus >> BlockWithProof).into(),
            send_data.clone(),
        ))
        .unwrap();
}

fn get_mock_data<'a>(matches: &'a ArgMatches) -> serde_yaml::Value {
    // get the path of mock-data file
    let mock_data_path = matches.value_of("mock-data").unwrap();

    // read mock-data from the corresponding file
    // and convert it to serde_yaml format
    let mut mock_data = String::new();
    fs::File::open(mock_data_path)
        .expect("Failed to open mock data file")
        .read_to_string(&mut mock_data)
        .expect("Failed to read mock data");
    let mock_data: serde_yaml::Value = serde_yaml::from_str(mock_data.as_str())
        .expect("Failed to parse the mock data from yaml format");

    mock_data
}

fn parse_mock_data<'a>(
    mock_data: &'a mut serde_yaml::Value,
) -> (PrivKey, HashMap<u64, &'a serde_yaml::Value>) {
    // get the private-key of the miner
    let pk_miner: PrivKey = {
        let pk_str = mock_data["privkey"].as_str().unwrap();
        PrivKey::from_str(pk_str).unwrap()
    };

    // get the detailed block information
    let mut mock_blocks: HashMap<u64, &serde_yaml::Value> = HashMap::new();
    for block in mock_data["blocks"].as_sequence_mut().unwrap().into_iter() {
        let block_number = block["number"].as_u64().unwrap();
        mock_blocks.insert(block_number, block);
    }
    info!(">> numbers: {:?}", mock_blocks.keys());

    // verify the existence of all blocks
    for number in 1..(mock_blocks.len() as u64 + 1) {
        if !mock_blocks.contains_key(&number) {
            error!("Block missing, number={}", number);
            panic!("Exit because of missing block");
        }
    }

    (pk_miner, mock_blocks)
}

fn main() {
    logger::init();
    info!("CITA: Consensus Mock");

    // set up the clap to receive info from CLI
    let matches = App::new("consensus mock")
        .version("0.1")
        .author("Cryptape")
        .about("Mock the process of consensus")
        .arg(
            clap::Arg::with_name("mock-data")
                .short("m")
                .long("mock-data")
                .required(true)
                .takes_value(true)
                .help("Set the path of mock data in YAML format"),
        )
        .get_matches();

    // get the mock data and parse it to serde_yaml format
    let mut mock_data = get_mock_data(&matches);
    let (pk_miner, mut mock_blocks) = parse_mock_data(&mut mock_data);

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();

    start_pubsub(
        "consensus",
        routing_key!([Auth >> BlockTxs, Chain >> RichStatus,]),
        tx_sub,
        rx_pub,
    );

    let mut received_block_txs: HashMap<usize, BlockTxs> = HashMap::new();

    loop {
        let (key, body) = rx_sub.recv().unwrap();
        let routing_key = RoutingKey::from(&key);
        let mut msg = Message::try_from(body).unwrap();

        match routing_key {
            routing_key!(Auth >> BlockTxs) => {
                let block_txs = msg.take_block_txs().unwrap();
                let height = block_txs.get_height() as usize;
                received_block_txs.insert(height, block_txs);
            }
            routing_key!(Chain >> RichStatus) => {
                let rich_status = msg.take_rich_status().unwrap();
                trace!("get new local status {:?}", rich_status.height);
                let rich_status_height = rich_status.height;
                let height = rich_status_height + 1;
                if let Some(block_txs) = received_block_txs.remove(&(rich_status_height as usize)) {
                    if let Some(mock_block) = mock_blocks.remove(&height) {
                        send_block(
                            H256::from_slice(&rich_status.hash),
                            height,
                            tx_pub.clone(),
                            &mock_block,
                            &block_txs,
                            &pk_miner,
                        );
                    } else {
                        warn!("No mock_block at height = {:?}", height);
                    }
                } else {
                    warn!(
                        "No received block_txs at rich_status_height = {:?}",
                        rich_status_height
                    );
                }
            }
            _ => {}
        }
    }
}
