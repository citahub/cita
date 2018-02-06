use Source;
use connection::Connection;
use libproto::{Message, MsgClass, Response};
use std::convert::{TryFrom, TryInto};
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

    pub fn receiver(&self, source: Source, payload: (String, Vec<u8>)) {
        let (key, data) = payload;
        trace!("Network receive Msg from {:?}/{}", source, key);
        match source {
            // Come from MQ
            Source::LOCAL => match &key[..] {
                "chain.status" => {
                    self.tx_sync.send((source, data));
                }
                "chain.blk" => {
                    self.con
                        .broadcast_rawbytes("net.sync_resp".to_string(), &data);
                }
                "jsonrpc.net" => {
                    self.reply_rpc(&data);
                }
                _ => {
                    error!("Unexpected key {} from {:?}", key, source);
                }
            },
            // Come from Netserver
            Source::REMOTE => match &key[..] {
                "net.sync_status" | "net.sync_resp" => {
                    self.tx_sync.send((source, data));
                }
                "net.sync_req" => {
                    self.tx_pub.send(("net.sync".to_string(), data));
                }
                "auth.tx" => {
                    self.tx_new_tx.send(("net.tx".to_string(), data));
                }
                "consensus.msg" => {
                    self.tx_consensus.send(("net.msg".to_string(), data));
                }
                _ => {
                    error!("Unexpected key {} from {:?}", key, source);
                }
            },
        }
    }

    pub fn reply_rpc(&self, data: &[u8]) {
        let mut msg = Message::try_from(data).unwrap();
        let content = msg.take_content();
        match content {
            MsgClass::Request(mut ts) => {
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
                    let ms: Message = response.into();
                    self.tx_pub
                        .send(("chain.rpc".to_string(), ms.try_into().unwrap()))
                        .unwrap();
                }
            }
            _ => {
                warn!("receive: unexpected data type = {:?}", content);
            }
        }
    }
}
