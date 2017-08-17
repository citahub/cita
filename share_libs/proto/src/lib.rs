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

extern crate protobuf;
extern crate util;
extern crate rustc_serialize;
extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate cita_crypto;

pub mod blockchain;
pub mod communication;
pub mod request;
pub mod into;

use blockchain::*;
use communication::*;
use protobuf::Message;
use protobuf::core::parse_from_bytes;
pub use request::*;
use rustc_serialize::hex::ToHex;
use util::{H256, Hashable, H520, merklehash};
use util::snappy;
use cita_crypto::{sign, PrivKey, recover, Signature, KeyPair, SIGNATURE_BYTES_LEN};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct State(pub Vec<Vec<u8>>);

pub type TopicMessage = (String, communication::Message);

pub mod submodules {
    pub const JSON_RPC: u32 = 1;
    pub const NET: u32 = 2;
    pub const CHAIN: u32 = 3;
    pub const CONSENSUS: u32 = 4;
    pub const CONSENSUS_CMD: u32 = 5;
}

pub mod topics {
    pub const DEFAULT: u16 = 0;
    pub const REQUEST: u16 = 1;
    pub const NEW_BLK: u16 = 2;
    pub const NEW_STATUS: u16 = 3;
    pub const SYNC_BLK: u16 = 4;
    pub const RESPONSE: u16 = 5;
    pub const NEW_TX: u16 = 6;
    pub const TX_RESPONSE: u16 = 7;
    pub const CONSENSUS_MSG: u16 = 8;
    pub const NEW_PROPOSAL: u16 = 9;
}

#[derive(Debug)]
pub enum MsgClass {
    REQUEST(Request),
    RESPONSE(Response),
    HEADER(BlockHeader),
    BODY(BlockBody),
    BLOCK(Block),
    TX(SignedTransaction),
    TXRESPONSE(TxResponse),
    STATUS(Status),
    MSG(Vec<u8>),
}

pub fn topic_to_string(top: u16) -> &'static str {
    match top {
        topics::DEFAULT => "default",
        topics::REQUEST => "request",
        topics::NEW_BLK => "new_blk",
        topics::NEW_STATUS => "new_status",
        topics::SYNC_BLK => "sync_blk",
        topics::RESPONSE => "response",
        topics::NEW_TX => "new_tx",
        topics::TX_RESPONSE => "tx_response",
        topics::CONSENSUS_MSG => "consensus_msg",
        topics::NEW_PROPOSAL => "new_proposal",
        _ => "",
    }
}

pub fn id_to_key(id: u32) -> &'static str {
    match id {
        submodules::JSON_RPC => "json_rpc",
        submodules::NET => "net",
        submodules::CHAIN => "chain",
        submodules::CONSENSUS => "consensus",
        submodules::CONSENSUS_CMD => "consensus_cmd",
        _ => "",
    }
}

pub fn key_to_id(key: &str) -> u32 {
    if key.starts_with("jsonrpc") {
        submodules::JSON_RPC
    } else if key.starts_with("net") {
        submodules::NET
    } else if key.starts_with("chain") {
        submodules::CHAIN
    } else if key.starts_with("consensus_cmd") {
        submodules::CONSENSUS_CMD
    } else if key.starts_with("consensus") {
        submodules::CONSENSUS
    } else {
        0
    }
}

pub fn de_cmd_id(cmd_id: u32) -> (u32, u16) {
    let mut submodule = cmd_id >> 16;
    let sub = submodule;
    submodule = submodule << 16;
    let topic = (cmd_id - submodule) as u16;
    (sub, topic)
}

pub fn display_cmd(cmd_id: u32) -> (&'static str, &'static str) {
    let cd = de_cmd_id(cmd_id);
    (id_to_key(cd.0), topic_to_string(cd.1))
}

pub fn cmd_id(submodule: u32, topic: u16) -> u32 {
    (submodule << 16) + topic as u32
}

pub mod factory {
    use super::*;
    pub const ZERO_ORIGIN: u32 = 99999;

    pub fn create_msg(sub: u32, top: u16, msg_type: MsgType, content: Vec<u8>) -> communication::Message {
        let mut msg = communication::Message::new();
        msg.set_cmd_id(cmd_id(sub, top));
        msg.set_field_type(msg_type);
        msg.set_operate(communication::OperateType::BROADCAST);
        msg.set_origin(ZERO_ORIGIN);
        //compress data
        msg.set_content(snappy::cita_compresse(content));
        msg
    }

    ///for crate_msg extral version
    pub fn create_msg_ex(sub: u32, top: u16, msg_type: MsgType, operate: communication::OperateType, origin: u32, content: Vec<u8>) -> communication::Message {
        let mut msg = factory::create_msg(sub, top, msg_type, content);
        msg.set_origin(origin);
        msg.set_operate(operate);
        msg
    }
}

type CmdId = u32;
type Origin = u32;

pub fn parse_msg(msg: &[u8]) -> (CmdId, Origin, MsgClass) {
    let mut msg = parse_from_bytes::<communication::Message>(msg.as_ref()).unwrap();
    let content_msg = msg.take_content();
    let content_msg = snappy::cita_decompress(content_msg);
    let msg_class = match msg.get_field_type() {
        MsgType::REQUEST => MsgClass::REQUEST(parse_from_bytes::<Request>(&content_msg).unwrap()),
        MsgType::RESPONSE => {
            MsgClass::RESPONSE(parse_from_bytes::<Response>(&content_msg).unwrap())
        }
        MsgType::TX_RESPONSE => {
            MsgClass::TXRESPONSE(parse_from_bytes::<TxResponse>(&content_msg).unwrap())
        }
        MsgType::HEADER => MsgClass::HEADER(parse_from_bytes::<BlockHeader>(&content_msg).unwrap()),
        MsgType::BODY => MsgClass::BODY(parse_from_bytes::<BlockBody>(&content_msg).unwrap()),
        MsgType::BLOCK => MsgClass::BLOCK(parse_from_bytes::<Block>(&content_msg).unwrap()),
        MsgType::TX => MsgClass::TX(parse_from_bytes::<SignedTransaction>(&content_msg).unwrap()),
        MsgType::STATUS => MsgClass::STATUS(parse_from_bytes::<Status>(&content_msg).unwrap()),
        MsgType::MSG => {
            let mut content = Vec::new();
            content.extend_from_slice(&content_msg);
            MsgClass::MSG(content)
        }
    };

    (msg.get_cmd_id(), msg.get_origin(), msg_class)
}


impl blockchain::SignedTransaction {
    pub fn sign(&mut self, sk: PrivKey) {
        let keypair = KeyPair::from_privkey(sk).unwrap();
        let pubkey = keypair.pubkey();

        let bytes = self.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash();
        let signature = sign(&sk, &H256::from(hash)).unwrap();
        self.mut_transaction_with_sig().set_signature(signature.to_vec());
        self.mut_transaction_with_sig().set_crypto(Crypto::SECP);
        self.set_signer(pubkey.to_vec());
        let bytes = self.get_transaction_with_sig().write_to_bytes().unwrap();
        self.set_tx_hash(bytes.crypt_hash().to_vec());
    }

    pub fn recover(&mut self) -> bool {
        let mut ret = true;
        let bytes = self.get_transaction_with_sig().get_transaction().write_to_bytes().unwrap();
        let hash = bytes.crypt_hash();
        if self.get_transaction_with_sig().get_signature().len() != SIGNATURE_BYTES_LEN {
            ret = false;
        } else {
            match self.get_transaction_with_sig().get_crypto() {
                Crypto::SECP => {
                    let signature: Signature = H520::from_slice(self.get_transaction_with_sig().get_signature()).into();
                    match recover(&signature, &hash) {
                        Ok(pubkey) => {
                            self.set_signer(pubkey.to_vec());
                        },
                        _ => {ret = false;},
                    }
                },
                _ => {ret = false;},
            }
        }

        let bytes = self.get_transaction_with_sig().write_to_bytes().unwrap();
        self.set_tx_hash(bytes.crypt_hash().to_vec());        
        ret
    }

    pub fn crypt_hash(&self) -> H256 {
        let bytes = self.get_transaction_with_sig().write_to_bytes().unwrap();
        bytes.crypt_hash()
    }
}

impl Block {
    pub fn crypt_hash(&self) -> H256 {
        self.get_header().crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        self.get_header().crypt_hash_hex()
    }
}

impl BlockHeader {
    pub fn crypt_hash(&self) -> H256 {
        let bytes = self.write_to_bytes().unwrap();
        bytes.crypt_hash()
    }

    pub fn crypt_hash_hex(&self) -> String {
        let bytes = self.write_to_bytes().unwrap();
        bytes.crypt_hash().to_hex()
    }
}

impl BlockBody {
    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.get_transactions().iter().map(|ts| H256::from_slice(ts.get_tx_hash())).collect()
    }

    pub fn transactions_root(&self) -> H256 {
        merklehash::complete_merkle_root_raw(self.transaction_hashes().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_id_works() {
        assert_eq!(cmd_id(submodules::JSON_RPC, topics::NEW_TX), 0x10006);
        assert_eq!(cmd_id(submodules::CHAIN, topics::NEW_TX), 0x30006);
    }

    #[test]
    fn create_tx() {
        let test1_privkey = H256::random();
        let keypair = KeyPair::from_privkey(H256::from(test1_privkey)).unwrap();
        let pv = keypair.privkey();

        let data = vec![1];
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_to("123".to_string());
        tx.set_valid_until_block(99999);

        let mut uv_tx = UnverifiedTransaction::new();
        uv_tx.set_transaction(tx);

        let mut signed_tx = SignedTransaction::new();
        signed_tx.set_transaction_with_sig(uv_tx);
        signed_tx.sign(pv.clone());

        println!("{}", signed_tx.write_to_bytes().unwrap().to_hex())
    }
}
