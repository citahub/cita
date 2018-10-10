extern crate bincode;
extern crate cita_crypto as crypto;
extern crate cita_types;
extern crate clap;
extern crate common_types;
extern crate core;
extern crate dotenv;
extern crate logger;
extern crate proof;
extern crate pubsub;
extern crate rlp;
extern crate rustc_serialize;
extern crate serde_yaml;
extern crate util;

use std::convert::{From, TryFrom};
use std::convert::{Into, TryInto};
use std::str::FromStr;
use std::u8;
use std::{thread, time};

use cita_types::H256;
use config::Config;
use crypto::PrivKey;
use generate_block::BuildBlock;
use libproto::{Message, RichStatus, SignedTransaction};

use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::sync::mpsc::{channel, Receiver, Sender};

pub type PubType = (String, Vec<u8>);
pub type SubType = (String, Vec<u8>);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

const GENESIS_TIMESTAMP: u64 = 1_524_000_000;

pub fn run(config: Config) {
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub(
        "consensus",
        routing_key!([Chain >> RichStatus]),
        tx_sub,
        rx_pub,
    );

    do_loop(config, &rx_sub, &tx_pub);
    info!("PASS");
}

fn do_loop(mut config: Config, rx_sub: &Receiver<SubType>, tx_pub: &Sender<PubType>) {
    while config.blocks.len() > 1 {
        if let Some(rich_status) = receive_rich_status(&rx_sub) {
            info!(">> receive RichStatus.height: {}", rich_status.get_height(),);
            validate_rich_status(&mut config, &rich_status);
            match config.blocks.len() % 4 {
                0 => send_proofed_block(&mut config, &tx_pub, &rich_status),
                1 => {
                    send_signed_proposal(&mut config, &tx_pub, &rich_status);
                    send_proofed_block(&mut config, &tx_pub, &rich_status)
                }
                2 => {
                    send_signed_proposal(&mut config, &tx_pub, &rich_status);
                    thread::sleep(time::Duration::from_secs(4));
                    send_proofed_block(&mut config, &tx_pub, &rich_status);
                }
                3 => {
                    send_signed_proposal(&mut config, &tx_pub, &rich_status);
                    thread::sleep(time::Duration::from_secs(4));

                    // sending an equivalent proposal which has different
                    // `transactions_root` should be ok
                    let hash = H256::from_slice(&rich_status.get_hash());
                    let height = rich_status.get_height() + 1;
                    let transactions = make_transactions(&mut config, &rich_status);
                    let (_, mut signed_proposal) = BuildBlock::build_signed_proposal(
                        &transactions,
                        hash,
                        height,
                        &config.private_key,
                        GENESIS_TIMESTAMP + height * 3,
                    );
                    let fake_transactions_root = hash;
                    signed_proposal
                        .mut_proposal()
                        .mut_block()
                        .mut_header()
                        .set_transactions_root(fake_transactions_root.to_vec());
                    let msg: Message = signed_proposal.clone().into();
                    let key = routing_key!(Consensus >> SignedProposal).into();
                    tx_pub.send((key, msg.try_into().unwrap())).unwrap();

                    send_proofed_block(&mut config, &tx_pub, &rich_status);
                }
                _ => panic!("bomb"),
            }
        }
    }
}

fn receive_rich_status(rx_sub: &Receiver<SubType>) -> Option<RichStatus> {
    let (key, body) = rx_sub.recv().unwrap();
    let mut msg = Message::try_from(body).unwrap();
    if routing_key!(Chain >> RichStatus) != RoutingKey::from(&key) {
        return None;
    }
    let rich_status = msg.take_rich_status().unwrap();
    Some(rich_status)
}

fn validate_rich_status(config: &mut Config, rich_status: &RichStatus) {
    // TODO: need validate hash ?
    config.blocks.remove(&rich_status.get_height());
    assert_eq!(
        &(rich_status.get_height() + 1),
        config.blocks.keys().min_by_key(|x| *x).unwrap(),
    );
}

fn make_transactions(config: &mut Config, rich_status: &RichStatus) -> Vec<SignedTransaction> {
    let height = rich_status.get_height() + 1;
    let block = &config.blocks[&height];
    let raw_transactions = &block["transactions"];
    let transactions: Vec<SignedTransaction> = raw_transactions
        .as_sequence()
        .unwrap()
        .iter()
        .map(|tx| {
            let contract_address = tx["to"].as_str().unwrap();
            let tx_private_key = PrivKey::from_str(tx["privkey"].as_str().unwrap()).unwrap();
            let data = tx["data"].as_str().unwrap();
            let quota = tx["quota"].as_u64().unwrap();
            let nonce = tx["nonce"].as_u64().unwrap() as u32;
            let valid_until_block = tx["valid_until_block"].as_u64().unwrap();
            BuildBlock::build_tx(
                contract_address,
                data,
                quota,
                nonce,
                valid_until_block,
                &tx_private_key,
            )
        })
        .collect();
    transactions
}

fn send_signed_proposal(config: &mut Config, tx_pub: &Sender<PubType>, rich_status: &RichStatus) {
    let height = rich_status.get_height() + 1;
    let hash = H256::from_slice(&rich_status.get_hash());
    let transactions = make_transactions(config, rich_status);
    let (body, _) = BuildBlock::build_signed_proposal(
        &transactions,
        hash,
        height,
        &config.private_key,
        GENESIS_TIMESTAMP + height * 3,
    );
    let key = routing_key!(Consensus >> SignedProposal).into();
    tx_pub.send((key, body)).unwrap();
}

fn send_proofed_block(config: &mut Config, tx_pub: &Sender<PubType>, rich_status: &RichStatus) {
    let height = rich_status.get_height() + 1;
    let hash = H256::from_slice(&rich_status.get_hash());
    let transactions = make_transactions(config, rich_status);
    let (body, _) = BuildBlock::build_block_with_proof(
        &transactions,
        hash,
        height,
        &config.private_key,
        GENESIS_TIMESTAMP + height * 3,
    );
    let key = routing_key!(Consensus >> BlockWithProof).into();
    tx_pub.send((key, body)).unwrap();
}
