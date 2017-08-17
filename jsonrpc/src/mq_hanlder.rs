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

#![allow(deprecated,unused_assignments, unused_must_use)]
use amqp::{Consumer, Channel, protocol, Basic};
use base_hanlder::TransferType;
use jsonrpc_types::response::{RpcSuccess, ResponseBody};
use libproto::{submodules, topics, parse_msg, cmd_id, display_cmd, MsgClass, blockchain, request};
use num_cpus;
use parking_lot::{RwLock, Mutex};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use threadpool::ThreadPool;
use util::hash::H256;
use ws;
use ws_handler::ReqInfo;


#[derive(Default)]
pub struct MqHandler {
    transfer_type: TransferType,
    thread_pool: Option<ThreadPool>,
    ws_tx_responses: Arc<Mutex<HashMap<H256, (ReqInfo, ws::Sender)>>>,
    ws_responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>,

    responses: Arc<RwLock<HashMap<Vec<u8>, request::Response>>>,
    tx_responses: Arc<RwLock<HashMap<H256, blockchain::TxResponse>>>,
}


impl MqHandler {
    pub fn new() -> Self {
        MqHandler {
            transfer_type: TransferType::ALL,
            thread_pool: None,
            ws_tx_responses: Arc::new(Mutex::new(HashMap::new())),
            ws_responses: Arc::new(Mutex::new(HashMap::new())),
            responses: Arc::new(RwLock::new(HashMap::new())),
            tx_responses: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set_http_or_ws(&mut self, transfer_type: TransferType, thread_num: usize) {
        self.transfer_type = transfer_type;
        if self.transfer_type == TransferType::WEBSOCKET {
            let mut num_cpus = 0;
            if thread_num == 0 {
                num_cpus = 2 * num_cpus::get();
            } else {
                num_cpus = thread_num;
            }
            self.thread_pool = Some(ThreadPool::new_with_name("MqHandler".to_string(), num_cpus));
        }
    }

    pub fn set_http(&mut self, tx_responses: Arc<RwLock<HashMap<H256, blockchain::TxResponse>>>, responses: Arc<RwLock<HashMap<Vec<u8>, request::Response>>>) {
        self.responses = responses;
        self.tx_responses = tx_responses;
    }

    pub fn set_ws(&mut self, ws_tx_responses: Arc<Mutex<HashMap<H256, (ReqInfo, ws::Sender)>>>, ws_responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>) {
        self.ws_tx_responses = ws_tx_responses;
        self.ws_responses = ws_responses;
    }
}

impl Consumer for MqHandler {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, _: protocol::basic::BasicProperties, body: Vec<u8>) {
        let (id, _, content_ext) = parse_msg(body.as_slice());
        trace!("routint_key {:?},get msg cmid {:?}", deliver.routing_key, display_cmd(id));
        //TODO match

        if id == cmd_id(submodules::CHAIN, topics::RESPONSE) {
            if let MsgClass::RESPONSE(content) = content_ext {
                //TODO ws 并不能一下子开启两个服务。
                if self.transfer_type == TransferType::HTTP {
                    let mut responses = self.responses.write();
                    trace!("from chain response rid {:?}", content.request_id.clone());
                    responses.insert(content.request_id.clone(), content);

                } else if self.transfer_type == TransferType::WEBSOCKET {
                    //TODO
                    let ws_responses = self.ws_responses.clone();
                    self.thread_pool.as_ref().map(|pool| {
                        pool.execute(move || {
                            let pair = ws_responses.lock().remove(&content.request_id);
                            drop(ws_responses);
                            if let Some(pair) = pair {
                                let rpc_success = RpcSuccess {
                                    jsonrpc: pair.0.jsonrpc.clone(),
                                    id: pair.0.id.clone(),
                                    result: ResponseBody::from(content.result.expect("chain response error")), //TODO
                                };
                                let data = serde_json::to_string(&rpc_success).unwrap();
                                pair.1.send(data);
                            }
                        })
                    });
                }
            } else {
                warn!("from chain Unable to parse right {:?}", content_ext);
            }
        } else if id == cmd_id(submodules::CONSENSUS, topics::TX_RESPONSE) {
            if let MsgClass::TXRESPONSE(content) = content_ext {
                if self.transfer_type == TransferType::HTTP {
                    let mut tx_responses = self.tx_responses.write();
                    trace!("from chain response rid {:?}", content.hash.clone());
                    tx_responses.insert(H256::from(content.hash.clone().as_slice()), content);

                } else if self.transfer_type == TransferType::WEBSOCKET {
                    //TODO ws
                    let ws_tx_responses = self.ws_tx_responses.clone();
                    self.thread_pool.as_ref().map(|pool| {
                        pool.execute(move || {
                            let pair = ws_tx_responses.lock().remove(&H256::from(content.hash.clone().as_slice()));
                            drop(ws_tx_responses);
                            if let Some(pair) = pair {
                                let rpc_success = RpcSuccess {
                                    jsonrpc: pair.0.jsonrpc.clone(),
                                    id: pair.0.id.clone(),
                                    result: ResponseBody::from(content), //TODO
                                };
                                let data = serde_json::to_string(&rpc_success).unwrap();
                                let _ = pair.1.send(data);
                            }
                        })
                    });
                }

            } else {
                warn!("from chain Unable to parse right: {:?} ", content_ext);
            }
        } else {
            // warn!("Unable handle msg {:?}", content_ext);
        }
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}
