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
use libproto::Message;
use libproto::Request_oneof_req::software_version;
use serde_json;

pub struct Soliloquy {
    config: Config,
}

impl Soliloquy {
    pub fn new(config: Config) -> Self {
        Soliloquy { config }
    }

    pub fn handle(&self, msg_bytes: &[u8]) -> Message {
        let maybe_msg: Result<Message, _> = libproto::TryFrom::try_from(msg_bytes);
        let mut response = Response::new();

        if let Ok(Some(req)) = maybe_msg.map(|mut msg| msg.take_request()) {
            response.set_request_id(req.request_id);
            let enabled_version = self.config.enable_version.unwrap_or(false);
            debug!("getVersion enabled:{}", enabled_version);

            match req.req {
                Some(software_version(_)) if enabled_version => {
                    let version = get_build_info_str(true);
                    let vec: Vec<&str> = version.split('-').collect();
                    let version = vec[0].to_string();
                    if let Ok(json_ver) = serde_json::to_value(SoftwareVersion::new(version)) {
                        response.set_software_version(json_ver.to_string());
                    } else {
                        response.set_code(ErrorCode::InternalError.code());
                        response.set_error_msg(ErrorCode::InternalError.description());
                    }
                }
                _ => {
                    response.set_code(ErrorCode::MethodNotFound.code());
                    response.set_error_msg(ErrorCode::MethodNotFound.description());
                }
            }
        } else {
            warn!("receive unexpected data");
            response.set_code(ErrorCode::InvalidRequest.code());
            response.set_error_msg(ErrorCode::InvalidRequest.description());
        }

        response.into()
    }
}
