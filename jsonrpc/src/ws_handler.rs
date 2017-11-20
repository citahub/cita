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

use base_hanlder::{BaseHandler, ReqInfo, WsMap};
use jsonrpc_types::{method, Id};
use jsonrpc_types::response::RpcFailure;
use libproto::request as reqlib;
use num_cpus;
use serde_json;
use std::sync::{Arc, mpsc};
use threadpool::ThreadPool;
use util::Mutex;
use ws;
use ws::{Factory, CloseCode, Handler};

pub struct WsFactory {
    //TODO 定时清理工作
    responses: WsMap,
    thread_pool: Arc<Mutex<ThreadPool>>,
    tx: mpsc::Sender<(String, reqlib::Request)>,
}


impl WsFactory {
    pub fn new(responses: WsMap, tx: mpsc::Sender<(String, reqlib::Request)>, thread_num: usize) -> WsFactory {
        let thread_number = if thread_num == 0 { num_cpus::get() / 2 } else { thread_num };
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
            responses: Arc::clone(&self.responses),
            tx: self.tx.clone(),
            thread_pool: Arc::clone(&self.thread_pool),
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
                    this.method_handler.request(rpc).map(|_req| {
                        let request_id = _req.request_id.clone();
                        //let data: communication::Message = _req.into();
                        //this.tx.send((topic, data.write_to_bytes().unwrap()));
                        this.tx.send((topic, _req));
                        let value = (req_info, this.sender.clone());
                        {
                            this.responses.lock().insert(request_id, value);
                        }
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
        info!("WebSocket closing for ({:?}) {} token {}", code, reason, self.sender.token().0);
    }
}


#[derive(Clone)]
pub struct WsHandler {
    responses: WsMap,
    thread_pool: Arc<Mutex<ThreadPool>>,
    method_handler: method::MethodHandler,
    sender: ws::Sender,
    tx: mpsc::Sender<(String, reqlib::Request)>,
}
