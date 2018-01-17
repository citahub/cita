use Source;
use connection::Connection;
use libproto::{cmd_id, communication, parse_msg, submodules, topics, MsgClass, Response};
use protobuf::Message;
use std::sync::Arc;
use std::sync::mpsc::Sender;

pub struct NetWork {
    con: Arc<Connection>,
    tx_pub: Sender<(String, Vec<u8>)>,
    tx_sync: Sender<(Source, Vec<u8>)>,
    tx_new_tx: Sender<(String, Vec<u8>)>,
    tx_consensus: Sender<(String, Vec<u8>)>,
}

impl NetWork {
    pub fn new(
        con: Arc<Connection>,
        tx_pub: Sender<(String, Vec<u8>)>,
        tx_sync: Sender<(Source, Vec<u8>)>,
        tx_new_tx: Sender<(String, Vec<u8>)>,
        tx_consensus: Sender<(String, Vec<u8>)>,
    ) -> Self {
        NetWork {
            con: con,
            tx_pub: tx_pub,
            tx_sync: tx_sync,
            tx_new_tx: tx_new_tx,
            tx_consensus: tx_consensus,
        }
    }

    pub fn receiver(&self, source: Source, data: Vec<u8>) {
        trace!("receiver: from {:?}", source);
        let (topic, content) = NetWork::parse_topic(&data);
        match source {
            Source::LOCAL => {
                //send other node
                if topic == "net.status" {
                    //sync
                    self.tx_sync.send((source, data));
                } else if topic == "chain.rpc" {
                    //reply rpc
                    self.reply_rpc(content);
                } else if topic != "" {
                    self.con.broadcast_rawbytes(&data);
                }
            }

            Source::REMOTE => {
                //send mq
                if topic == "net.status" || topic == "net.blk" {
                    //sync
                    self.tx_sync.send((source, data));
                } else if topic == "net.tx" {
                    self.tx_new_tx.send((topic, data));
                } else if topic == "net.msg" {
                    self.tx_consensus.send((topic, data));
                } else if topic != "" {
                    self.tx_pub.send((topic, data));
                }
            }
        }
    }

    pub fn reply_rpc(&self, msg_class: MsgClass) {
        match msg_class {
            MsgClass::REQUEST(mut ts) => {
                let mut response = Response::new();
                response.set_request_id(ts.take_request_id());
                if ts.has_peercount() {
                    let peercount = self.con
                        .peers_pair
                        .read()
                        .iter()
                        .filter(|x| x.2.is_some())
                        .count();
                    response.set_peercount(peercount as u32);
                    let ms: communication::Message = response.into();
                    self.tx_pub
                        .send(("chain.rpc".to_string(), ms.write_to_bytes().unwrap()))
                        .unwrap();
                }
            }
            _ => {
                warn!("receive: unexpected data type = {:?}", msg_class);
            }
        }
    }

    pub fn parse_topic(data: &[u8]) -> (String, MsgClass) {
        let (cid, _, content) = parse_msg(data);
        let topic = match content {
            MsgClass::REQUEST(_) => {
                if cid == cmd_id(submodules::JSON_RPC, topics::REQUEST) {
                    "chain.rpc"
                } else if cid == cmd_id(submodules::AUTH, topics::REQUEST) {
                    "net.tx"
                } else {
                    ""
                }
            }
            MsgClass::STATUS(_) => {
                if cid == cmd_id(submodules::CHAIN, topics::NEW_STATUS) {
                    "net.status"
                } else {
                    ""
                }
            }
            MsgClass::SYNCREQUEST(_) => {
                if cid == cmd_id(submodules::CHAIN, topics::SYNC_BLK) {
                    "net.sync"
                } else {
                    ""
                }
            }
            MsgClass::SYNCRESPONSE(_) => {
                if cid == cmd_id(submodules::CHAIN, topics::NEW_BLK) {
                    "net.blk"
                } else {
                    ""
                }
            }
            MsgClass::MSG(_) => {
                if cid == cmd_id(submodules::CONSENSUS, topics::CONSENSUS_MSG)
                    || cid == cmd_id(submodules::CONSENSUS, topics::NEW_PROPOSAL)
                {
                    "net.msg"
                } else {
                    ""
                }
            }
            _ => "",
        }.to_string();
        (topic, content)
    }
}
