// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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
