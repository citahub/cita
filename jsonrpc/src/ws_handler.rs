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

use base_hanlder::{BaseHandler, ReqInfo};
use jsonrpc_types::{method, Id};
use jsonrpc_types::response::RpcFailure;
use libproto::communication;
use num_cpus;
use parking_lot::Mutex;
use protobuf::Message;
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use ws;
use ws::{Factory, CloseCode, Handler};

pub struct WsFactory {
    //TODO 定时清理工作
    responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>,
    thread_pool: Arc<Mutex<ThreadPool>>,
    tx: Sender<(String, Vec<u8>)>,
}


impl WsFactory {
    pub fn new(responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>, tx: Sender<(String, Vec<u8>)>, thread_num: usize) -> WsFactory {
        let mut thread_number: usize = 0 as usize;
        if thread_num == 0 {
            thread_number = num_cpus::get() * 2;
        } else {
            thread_number = thread_num;
        }
        let thread_pool = Arc::new(Mutex::new(ThreadPool::new_with_name("ws_thread_pool".to_string(), thread_number)));
        WsFactory {
            responses: responses,
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
        let this = self.clone();
        self.thread_pool.lock().execute(move || {
            let mut req_id = Id::Null;
            let mut jsonrpc_version = None;
            let err = match WsHandler::into_rpc(msg.into_text().unwrap()) {
                Err(err) => Err(err),
                Ok(rpc) => {
                    req_id = rpc.id.clone();
                    jsonrpc_version = rpc.jsonrpc.clone();
                    let topic = WsHandler::select_topic(&rpc.method);
                    let req_info = ReqInfo {
                        jsonrpc: jsonrpc_version.clone(),
                        id: req_id.clone(),
                    };
                    this.method_handler.from_req(rpc).map(|_req| {
                                                              let request_id = _req.request_id.clone();
                                                              let data: communication::Message = _req.into();
                                                              let _ = this.tx.send((topic, data.write_to_bytes().unwrap()));
                                                              this.responses.lock().insert(request_id, (req_info, this.sender.clone()));
                                                              ()
                                                          })
                }
            };
            //TODO 错误返回
            if let Err(err) = err {
                let _ = this.sender
                            .send(serde_json::to_string(&RpcFailure::from_options(req_id, jsonrpc_version, err)).unwrap());
            }
        });
        //
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        trace!("WebSocket closing for ({:?}) {} token {}", code, reason, self.sender.token().0);
    }
}


#[derive(Clone)]
pub struct WsHandler {
    responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>,
    thread_pool: Arc<Mutex<ThreadPool>>,
    method_handler: method::MethodHandler,
    sender: ws::Sender,
    tx: Sender<(String, Vec<u8>)>,
}
