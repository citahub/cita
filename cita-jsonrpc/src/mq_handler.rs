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

use helper::{RpcMap, TransferType};
use jsonrpc_types::response::Output;
use libproto::Message;
use libproto::router::{MsgType, RoutingKey, SubModules};
use serde_json;
use std::convert::TryFrom;

#[derive(Default)]
pub struct MqHandler {
    responses: RpcMap,
}

impl MqHandler {
    pub fn new(responses: RpcMap) -> Self {
        MqHandler {
            responses: responses,
        }
    }

    pub fn handle(&mut self, key: &str, body: &[u8]) {
        let mut msg = Message::try_from(body).unwrap();
        trace!("get msg from routint_key {}", key);

        match RoutingKey::from(key) {
            routing_key!(Auth >> Response)
            | routing_key!(Chain >> Response)
            | routing_key!(Executor >> Response)
            | routing_key!(Net >> Response) => {
                let content = msg.take_response().unwrap();
                trace!("from response request_id {:?}", content.request_id);
                let value = { self.responses.lock().remove(&content.request_id) };
                if let Some(val) = value {
                    match val {
                        TransferType::HTTP((req_info, sender)) => {
                            let _ = sender.send(Output::from(content, req_info.id, req_info.jsonrpc));
                        }
                        TransferType::WEBSOCKET((req_info, sender)) => {
                            let _ = sender.send(
                                serde_json::to_string(&Output::from(content, req_info.id, req_info.jsonrpc)).unwrap(),
                            );
                        }
                    }
                } else {
                    warn!("receive lost request_id {:?}", content.request_id);
                }
            }
            _ => {
                warn!("receive unexpect key {}", key);
            }
        }
    }
}
