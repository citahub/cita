extern crate protobuf;
extern crate sha3;
extern crate util;
extern crate rustc_serialize;
extern crate bincode;
#[macro_use]
extern crate serde_derive;

pub mod blockchain;
pub mod communication;
pub mod request;
pub mod into;

use blockchain::*;
use communication::*;
pub use request::*;
use protobuf::core::parse_from_bytes;
use protobuf::Message;
use util::hash::H256;
use util::sha3::Hashable;
use rustc_serialize::hex::ToHex;
use bincode::{serialize, Infinite};
use std::hash::{Hash, Hasher};
use util::snappy;

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
    TX(Transaction),
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

    pub fn create_msg(sub: u32,
                      top: u16,
                      msg_type: MsgType,
                      content: Vec<u8>)
                      -> communication::Message {
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
    pub fn create_msg_ex(sub: u32,
                         top: u16,
                         msg_type: MsgType,
                         operate: communication::OperateType,
                         origin: u32,
                         content: Vec<u8>)
                         -> communication::Message {
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
        MsgType::TX => MsgClass::TX(parse_from_bytes::<Transaction>(&content_msg).unwrap()),
        MsgType::STATUS => MsgClass::STATUS(parse_from_bytes::<Status>(&content_msg).unwrap()),
        MsgType::MSG => {
            let mut content = Vec::new();
            content.extend_from_slice(&content_msg);
            MsgClass::MSG(content)
        }
    };

    (msg.get_cmd_id(), msg.get_origin(), msg_class)
}

impl blockchain::Transaction {
    pub fn sha3(&self) -> H256 {
        let bytes = self.write_to_bytes().unwrap();
        bytes.sha3()
    }

    pub fn sha3_hex(&self) -> String {
        let bytes = self.write_to_bytes().unwrap();
        bytes.sha3().to_hex()
    }
}

impl Hash for blockchain::Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        self.content.hash(state);
    }
}

impl Eq for blockchain::Transaction {}

impl blockchain::Block {
    pub fn sha3(&self) -> H256 {
        let bytes = self.write_to_bytes().unwrap();
        bytes.sha3()
    }

    pub fn sha3_hex(&self) -> String {
        let bytes = self.write_to_bytes().unwrap();
        bytes.sha3().to_hex()
    }
}

impl blockchain::Content {
    pub fn sha3(&self) -> H256 {
        let bytes = self.write_to_bytes().unwrap();
        bytes.sha3()
    }
}

impl Commit {
    pub fn states_sha3(&self, states: Vec<Vec<u8>>) -> H256 {
        let vec: Vec<u8> = Vec::new();
        let encoded: Vec<u8> = serialize(&states.iter().fold(vec, |mut vec, i| {
            vec.append(&mut i.clone());
            vec
        }), Infinite).unwrap();
        encoded.sha3()
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
}
