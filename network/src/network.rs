use Source;
use config::NetConfig;
use connection::Connection;
use libproto::{Response, Request, communication, submodules, topics, cmd_id};
use libproto::communication::MsgType;
use notify::DebouncedEvent;
use protobuf::Message;
use protobuf::core::parse_from_bytes;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use util::RwLock;


pub struct NetWork {
    con: Arc<RwLock<Connection>>,
    tx_pub: Sender<(String, Vec<u8>)>,
    tx_sync: Sender<(Source, communication::Message)>,
    tx_new_tx: Sender<(String, Vec<u8>)>,
}

impl NetWork {
    pub fn new(con: Arc<RwLock<Connection>>, tx_pub: Sender<(String, Vec<u8>)>, tx_sync: Sender<(Source, communication::Message)>, tx_new_tx: Sender<(String, Vec<u8>)>) -> Self {
        NetWork {
            con: con,
            tx_pub: tx_pub,
            tx_sync: tx_sync,
            tx_new_tx: tx_new_tx,
        }
    }

    pub fn receiver(&self, source: Source, data: Vec<u8>) {
        trace!("receiver: from {:?}", source);
        let (topic, msg) = NetWork::parse_msg(&data);
        match source {
            Source::LOCAL => {
                //send other node
                if topic == "net.status".to_string() {
                    //sync
                    self.tx_sync.send((source, msg));

                } else if topic == "chain.rpc".to_string() {
                    //reply rpc
                    self.reply_rpc(msg.get_content());

                } else if topic != "".to_string() {
                    self.con.read().broadcast(msg);
                }
            }

            Source::REMOTE => {
                //send mq
                if topic == "net.status".to_string() || topic == "net.blk".to_string() {
                    //sync
                    self.tx_sync.send((source, msg));

                } else if topic == "net.tx".to_string() {

                    self.tx_new_tx.send((topic, data));

                } else if topic != "".to_string() {
                    self.tx_pub.send((topic, data));

                }
            }
        }
    }


    pub fn manage_connect(&self, config_path: &str, rx: Receiver<DebouncedEvent>) {
        self.con.read().connect();
        let config = String::from(config_path);

        let con = self.con.clone();
        thread::spawn(move || loop {
                          con.write().del_peer();
                          thread::sleep(Duration::from_millis(15000));
                      });

        let con = self.con.clone();
        thread::spawn(move || loop {
                          match rx.recv() {
                              Ok(event) => {
                                  match event {
                                      DebouncedEvent::Create(path_buf) => {
                                          if path_buf.is_file() {
                                              let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                                              if file_name == config.as_str() {
                                                  info!("file {} change", file_name);
                                                  let config = NetConfig::new(&config.as_str());
                                                  {
                                                      let mut con = con.write();
                                                      con.update(&config);
                                                      con.connect();
                                                  }
                                              }
                                          }
                                      }
                                      _ => (),
                                  }
                              }
                              Err(e) => info!("watch error: {:?}", e),
                          }
                      });
    }

    pub fn reply_rpc(&self, msg: &[u8]) {
        let mut ts = parse_from_bytes::<Request>(msg).unwrap();
        let mut response = Response::new();
        response.set_request_id(ts.take_request_id());
        if ts.has_peercount() {
            let peercount = self.con.read().peers_pair.iter().filter(|x| x.2.as_ref().read().is_some()).count();
            response.set_peercount(peercount as u32);
            let ms: communication::Message = response.into();
            self.tx_pub.send(("chain.rpc".to_string(), ms.write_to_bytes().unwrap())).unwrap();
        }
    }

    pub fn parse_msg(payload: &[u8]) -> (String, communication::Message) {
        if let Ok(mut msg) = parse_from_bytes::<communication::Message>(payload) {
            let mut topic = String::new();
            let t = msg.get_field_type();
            let cid = msg.get_cmd_id();
            if cid == cmd_id(submodules::JSON_RPC, topics::REQUEST) && t == MsgType::REQUEST {
                topic = "chain.rpc".to_string();

            } else if cid == cmd_id(submodules::AUTH, topics::REQUEST) && t == MsgType::REQUEST {
                trace!("AUTH broadcast tx");
                topic = "net.tx".to_string();

            } else if cid == cmd_id(submodules::CHAIN, topics::NEW_STATUS) && t == MsgType::STATUS {
                trace!("CHAIN pub status");
                topic = "net.status".to_string();

            } else if cid == cmd_id(submodules::CHAIN, topics::SYNC_BLK) && t == MsgType::SYNC_REQ {
                trace!("CHAIN sync blk");
                topic = "net.sync".to_string();

            } else if cid == cmd_id(submodules::CHAIN, topics::NEW_BLK) && t == MsgType::SYNC_RES {
                trace!("CHAIN sync blk");
                topic = "net.blk".to_string();

            } else if (cid == cmd_id(submodules::CONSENSUS, topics::CONSENSUS_MSG) && t == MsgType::MSG) || (cid == cmd_id(submodules::CONSENSUS, topics::NEW_PROPOSAL) && t == MsgType::MSG) {
                trace!("CONSENSUS pub msg");
                topic = "net.msg".to_string();

            } else {
                topic = "".to_string();
                msg = communication::Message::new();

            }
            (topic, msg)
        } else {
            ("".to_string(), communication::Message::new())

        }
    }
}
