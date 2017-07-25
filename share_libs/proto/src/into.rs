use blockchain;
use communication;
use request;
use submodules;
use topics;
use protobuf::Message;
use super::*;

impl Into<communication::Message> for request::Request {
    fn into(self) -> communication::Message {
        let msg = factory::create_msg(submodules::JSON_RPC,
                                      topics::REQUEST,
                                      communication::MsgType::REQUEST,
                                      self.write_to_bytes().unwrap());
        msg
    }
}

impl Into<communication::Message> for blockchain::Transaction {
    fn into(self) -> communication::Message {
        let msg = factory::create_msg(submodules::JSON_RPC,
                                      topics::NEW_TX,
                                      communication::MsgType::TX,
                                      self.write_to_bytes().unwrap());
        msg
    }
}

impl Into<communication::Message> for request::Response {
    fn into(self) -> communication::Message {
        let msg = factory::create_msg(submodules::CHAIN,
                                      topics::RESPONSE,
                                      communication::MsgType::RESPONSE,
                                      self.write_to_bytes().unwrap());
        msg
    }
}

