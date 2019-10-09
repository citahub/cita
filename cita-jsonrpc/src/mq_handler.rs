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

use crate::helper::{RpcMap, TransferType};
use jsonrpc_proto::response::OutputExt;
use jsonrpc_types::rpc_response::Output;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use libproto::TryFrom;
use serde_json;

#[derive(Default)]
pub struct MqHandler {
    responses: RpcMap,
}

impl MqHandler {
    pub fn new(responses: RpcMap) -> Self {
        MqHandler { responses }
    }

    pub fn handle(&mut self, key: &str, body: &[u8]) -> Result<(), ()> {
        trace!("get msg from routing_key {}", key);

        let mut msg = Message::try_from(body).map_err(|e| {
            error!("try_from: {:?}", e);
        })?;

        match RoutingKey::from(key) {
            routing_key!(Auth >> Response)
            | routing_key!(Chain >> Response)
            | routing_key!(Executor >> Response)
            | routing_key!(Jsonrpc >> Response)
            | routing_key!(Net >> Response) => {
                let content = msg.take_response().ok_or_else(|| {
                    error!("empty response message");
                })?;

                let resp = {
                    let request_id = &content.request_id;
                    trace!("from response request_id {:?}", request_id);
                    self.responses.lock().remove(request_id).ok_or_else(|| {
                        warn!("receive lost request_id {:?}", request_id);
                    })?
                };

                match resp {
                    TransferType::HTTP((req_info, sender)) => {
                        sender
                            .send(Output::from_res_info(content, req_info))
                            .map_err(|e| {
                                error!("http: {:?}", e);
                            })?;
                    }
                    TransferType::WEBSOCKET((req_info, sender)) => {
                        let json_body =
                            serde_json::to_string(&Output::from_res_info(content, req_info))
                                .map_err(|e| {
                                    error!("ws: {:?}", e);
                                })?;
                        sender.send(json_body).map_err(|e| {
                            error!("ws: {:?}", e);
                        })?;
                    }
                };
            }
            _ => {
                warn!("receive unexpect key {}", key);
            }
        };
        Ok(())
    }
}
