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

use helper::{select_topic, RpcMap, TransferType};
use jsonrpc_types::request::{PartialRequest, RequestInfo};
use jsonrpc_types::response::RpcFailure;
use jsonrpc_types::Error;
use libproto::request::Request as ProtoRequest;
use num_cpus;
use serde_json;
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;
use ws::{self as ws, CloseCode, Factory, Handler};

pub struct WsFactory {
    //TODO 定时清理工作
    responses: RpcMap,
    thread_pool: ThreadPool,
    tx: mpsc::Sender<(String, ProtoRequest)>,
}

impl WsFactory {
    pub fn new(
        responses: RpcMap,
        tx: mpsc::Sender<(String, ProtoRequest)>,
        thread_num: usize,
    ) -> WsFactory {
        let thread_number = if thread_num == 0 {
            num_cpus::get()
        } else {
            thread_num
        };
        let thread_pool = ThreadPool::with_name("ws_thread_pool".to_string(), thread_number);
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
            thread_pool: self.thread_pool.clone(),
        }
    }
}

impl Handler for WsHandler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        trace!("Server got message '{}'  post thread_pool deal task ", msg);
        // let this = self.clone();
        let tx = self.tx.clone();
        let response = Arc::clone(&self.responses);
        let sender = self.sender.clone();

        self.thread_pool.execute(move || {
            let mut req_info = RequestInfo::null();

            let _ = serde_json::from_str::<PartialRequest>(&msg.into_text().unwrap())
                .map_err(|err_msg| Error::from(err_msg))
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
    tx: mpsc::Sender<(String, ProtoRequest)>,
}
