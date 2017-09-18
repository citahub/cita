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

use base_hanlder::{TransferType, ReqInfo};
use jsonrpc_types::response::Output;
use libproto::{parse_msg, display_cmd, MsgClass, Response};
use num_cpus;
use parking_lot::{RwLock, Mutex};
use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use threadpool::ThreadPool;
use ws;

#[derive(Default)]
pub struct MqHandler {
    transfer_type: TransferType,
    thread_pool: Option<ThreadPool>,
    //TODO 定时清理工作
    ws_responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>,
    responses: Arc<RwLock<HashMap<Vec<u8>, Response>>>,
}


impl MqHandler {
    pub fn new() -> Self {
        MqHandler {
            transfer_type: TransferType::ALL,
            thread_pool: None,
            ws_responses: Arc::new(Mutex::new(HashMap::new())),
            responses: Arc::new(RwLock::new(HashMap::new())),
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

    pub fn set_http(&mut self, responses: Arc<RwLock<HashMap<Vec<u8>, Response>>>) {
        self.responses = responses;
    }

    pub fn set_ws(&mut self, ws_responses: Arc<Mutex<HashMap<Vec<u8>, (ReqInfo, ws::Sender)>>>) {
        self.ws_responses = ws_responses;
    }

    pub fn handle(&mut self, key: String, body: Vec<u8>) {
        let (id, _, content_ext) = parse_msg(body.as_slice());
        trace!("routint_key {:?},get msg cmid {:?}", key, display_cmd(id));
        //TODO match
        match content_ext {
            MsgClass::RESPONSE(content) => {
                trace!("from chain response rid {:?}", String::from_utf8(content.request_id.clone()));
                match self.transfer_type {
                    TransferType::HTTP => {
                        self.responses.write().insert(content.request_id.clone(), content);
                    }
                    TransferType::WEBSOCKET => {
                        let ws_responses = self.ws_responses.clone();
                        self.thread_pool.as_ref().map(|pool| {
                                                          pool.execute(move || {
                                                                           if let Some((req_info, sender)) = ws_responses.lock().remove(&content.request_id) {
                                                                               sender.send(serde_json::to_string(&Output::from(content, req_info.id, req_info.jsonrpc)).unwrap());
                                                                           }
                                                                           drop(ws_responses);
                                                                       });
                                                      });
                    }
                    TransferType::ALL => {
                        error!("only start one of websocket and http！");
                    }
                }
            }
            _ => {
                warn!("Unable handle msg {:?}", content_ext);
            }
        }
    }
}
