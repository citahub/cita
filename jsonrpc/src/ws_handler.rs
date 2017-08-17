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
use base_hanlder::BaseHandler;
use jsonrpc_types::Id;
use jsonrpc_types::method;
use jsonrpc_types::request::Version;
use jsonrpc_types::response::RpcFailure;
use libproto::TopicMessage;
use libproto::communication;
use num_cpus;
use parking_lot::Mutex;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use util::hash::H256;
use ws;
use ws::{Factory, CloseCode, Handler};

pub struct WsFactory {
	//TODO 定时清理工作
    tx_responses: Arc<Mutex<HashMap<H256, (ReqInfo, ws::Sender)>>>,
    responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>,
    thread_pool: Arc<Mutex<ThreadPool>>,
    tx: Sender<TopicMessage>,
}


impl WsFactory {
    pub fn new(tx_responses: Arc<Mutex<HashMap<H256, (ReqInfo, ws::Sender)>>>, responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>, tx: Sender<TopicMessage>, thread_num: usize) -> WsFactory {
        let mut thread_number: usize = 0 as usize;
        if thread_num == 0 {
            thread_number = num_cpus::get() * 2;
        } else {
            thread_number = thread_num;
        }
        let thread_pool = Arc::new(Mutex::new(ThreadPool::new_with_name("ws_thread_pool".to_string(), thread_number)));
        WsFactory {
            responses: responses,
            tx_responses: tx_responses,
            thread_pool: thread_pool,
            tx: tx,
        }
    }
}


impl Factory for WsFactory {
    type Handler = WsHandler;
    fn connection_made(&mut self, ws: ws::Sender) -> WsHandler {
        WsHandler {
            sender: ws,
            tx_responses: self.tx_responses.clone(),
            responses: self.responses.clone(),
            tx: self.tx.clone(),
            thread_pool: self.thread_pool.clone(),
            method_handler: method::MethodHandler,
        }
    }
}


impl BaseHandler for WsHandler {}

impl Handler for WsHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        trace!("Server got message '{}'  post thread_pool deal task ", msg);

        let _self = self.clone();
        self.thread_pool.lock().execute(move || {
            let req_id = Id::Null;
            let jsonrpc_version = None;

            let err = match WsHandler::into_json(msg.into_text().unwrap()) {
                Err(err) => Err(err),
                Ok(rpc) => {
                    let req_id = rpc.id.clone();
                    let jsonrpc_version = rpc.jsonrpc.clone();
                    let topic = WsHandler::select_topic(&rpc.method);
                    let req_info = ReqInfo {
                        jsonrpc: jsonrpc_version.clone(),
                        id: req_id.clone(),
                    };

                    let err = _self.method_handler.from_req(rpc).map(|req_type| {
                        match req_type {
                            method::RpcReqType::TX(tx_req) => {
                                let hash = tx_req.crypt_hash();
                                let data: communication::Message = tx_req.into();
                                let _ = _self.tx.send((topic, data));
                                _self.tx_responses.lock().insert(hash, (req_info, _self.sender.clone()));
                            }
                            method::RpcReqType::REQ(_req) => {
                                let key = _req.request_id.clone();
                                let data: communication::Message = _req.into();
                                let _ = _self.tx.send((topic, data));

                                _self.responses.lock().insert(key, (req_info, _self.sender.clone()));
                            }
                        }
                        ()
                    });
                    err
                }
            };
            //TODO 错误返回
            if let Err(err) = err {
                let _ = _self.sender
                             .send(serde_json::to_string(&RpcFailure::from_options(req_id, jsonrpc_version, err)).unwrap());
            }
        });

        //TODO 错误返回
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        trace!("WebSocket closing for ({:?}) {} token {}", code, reason, self.sender.token().0);
    }
}


#[derive(Clone)]
pub struct WsHandler {
    tx_responses: Arc<Mutex<HashMap<H256, (ReqInfo, ws::Sender)>>>,
    responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>,
    thread_pool: Arc<Mutex<ThreadPool>>,
    method_handler: method::MethodHandler,
    sender: ws::Sender,
    tx: Sender<TopicMessage>,
}



#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub jsonrpc: Option<Version>,
    pub id: Id,
}

unsafe impl Send for ReqInfo {}

impl ReqInfo {
    pub fn new(jsonrpc: Option<Version>, id: Id) -> ReqInfo {
        ReqInfo { jsonrpc: jsonrpc, id: id }
    }
}
