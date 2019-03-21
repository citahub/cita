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

use config::Config;
use get_build_info_str;
use jsonrpc_types::rpctypes::SoftwareVersion;
use jsonrpc_types::ErrorCode;
use libproto::protos::response::Response;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use libproto::TryInto;
use pubsub::channel::Sender;
use serde_json;

pub struct Responser {
    tx: Sender<(String, Vec<u8>)>,
    config: Config,
}

impl Responser {
    pub fn new(config: Config, tx: Sender<(String, Vec<u8>)>) -> Responser {
        Responser { tx, config }
    }

    pub fn reply_request(&self, _: RoutingKey, mut msg: Message) {
        let req_opt = msg.take_request();

        if let Some(mut req) = req_opt {
            let mut response = Response::new();
            response.set_request_id(req.take_request_id());

            match req.req.unwrap() {
                libproto::Request_oneof_req::software_version(_) => {
                    let enabled = self.config.enable_version.unwrap_or(false);
                    debug!("getVersion enabled:{}", enabled);
                    if enabled {
                        let version = get_build_info_str(true);
                        let vec: Vec<&str> = version.split("-").collect();
                        let version = vec[0].to_string();
                        response.set_software_version(
                            serde_json::to_value(SoftwareVersion::new(version))
                                .unwrap()
                                .to_string(),
                        );
                    } else {
                        response.set_code(ErrorCode::MethodNotFound.code());
                        response.set_error_msg(ErrorCode::MethodNotFound.description());
                    }
                }

                _ => {
                    warn!("receive unexpected request");
                    response.set_code(ErrorCode::MethodNotFound.code());
                    response.set_error_msg(ErrorCode::MethodNotFound.description());
                }
            }

            let message: Message = response.into();
            self.tx
                .send((
                    routing_key!(Jsonrpc >> Response).into(),
                    message.try_into().unwrap(),
                ))
                .unwrap();
        } else {
            warn!("receive unexpected data");
        }
    }
}
