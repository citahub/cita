// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::helper::{select_topic, RpcMap, TransferType};
use jsonrpc_proto::complete::CompleteInto;
use jsonrpc_types::rpc_request::{PartialRequest, RequestInfo};
use jsonrpc_types::rpc_response::RpcFailure;
use jsonrpc_types::Error;
use libproto::request::Request as ProtoRequest;
use num_cpus;
use pubsub::channel::Sender;
use serde_json;
use std::sync::Arc;
use threadpool::ThreadPool;
use ws::{self as ws, CloseCode, Factory, Handler};

pub struct WsFactory {
    //TODO 定时清理工作
    responses: RpcMap,
    thread_pool: ThreadPool,
    tx: Sender<(String, ProtoRequest)>,
}

impl WsFactory {
    pub fn new(
        responses: RpcMap,
        tx: Sender<(String, ProtoRequest)>,
        thread_num: usize,
    ) -> WsFactory {
        let thread_number = if thread_num == 0 {
            num_cpus::get()
        } else {
            thread_num
        };
        let thread_pool = ThreadPool::with_name("ws_thread_pool".to_string(), thread_number);
        WsFactory {
            responses,
            thread_pool,
            tx,
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
            thread_pool: self.thread_pool.clone(),
        }
    }
}

impl Handler for WsHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        trace!("Server got message '{}'  post thread_pool deal task ", msg);
        let tx = self.tx.clone();
        let response = Arc::clone(&self.responses);
        let sender = self.sender.clone();

        self.thread_pool.execute(move || {
            let mut req_info = RequestInfo::null();

            let _ = serde_json::from_str::<PartialRequest>(&msg.into_text().unwrap())
                .map_err(Error::from)
                .and_then(|part_req| {
                    req_info = part_req.get_info();
                    part_req.complete_and_into_proto().map(|(full_req, req)| {
                        let request_id = req.request_id.clone();
                        let topic = select_topic(&full_req.get_method());
                        let _ = tx.send((topic, req));
                        let value = (req_info.clone(), sender.clone());
                        {
                            response
                                .lock()
                                .insert(request_id, TransferType::WEBSOCKET(value));
                        }
                    })
                })
                .map_err(|err| {
                    // TODO 错误返回
                    sender.send(
                        serde_json::to_string(&RpcFailure::from_options(req_info, err)).unwrap(),
                    )
                });
        });
        //
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        error!(
            "WebSocket closing for ({:?}) {} token {}",
            code,
            reason,
            self.sender.token().0
        );
    }
}

#[derive(Clone)]
pub struct WsHandler {
    responses: RpcMap,
    thread_pool: ThreadPool,
    sender: ws::Sender,
    tx: Sender<(String, ProtoRequest)>,
}
