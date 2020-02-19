// Copyright Rivtower Technologies LLC.
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

use crate::config::Config;
use crate::get_build_info_str;
use jsonrpc_types::rpc_types::{LicenseInfo as ProtoLicenseInfo, SoftwareVersion};
use jsonrpc_types::ErrorCode;
use libproto::protos::response::Response;
use libproto::Message;
use libproto::Request_oneof_req::{license_info, software_version};
use license::lic_info::LicenseInfo as CitaLicenseInfo;
use serde_json;
use std::fs::File;
use std::io::prelude::Read;
use std::io::ErrorKind;

pub struct Soliloquy {
    config: Config,
    lic_info: ProtoLicenseInfo,
}

impl Soliloquy {
    pub fn new(config: Config) -> Self {
        // Get license info from license file.
        let lic_info = match File::open("cita.lic") {
            Ok(mut file) => {
                let mut buffer = String::new();
                if let Err(e) = file.read_to_string(&mut buffer) {
                    ProtoLicenseInfo {
                        license_type: "Unknow".to_owned(),
                        finger_print: None,
                        expiration_date: None,
                        issuer: None,
                        error_message: Some(format!("Read license file error: {}", e)),
                    }
                } else {
                    match CitaLicenseInfo::from(buffer) {
                        Ok(info) => ProtoLicenseInfo {
                            license_type: format!("{:?}", info.lic_type),
                            finger_print: Some(format!("{:?}", info.finger_print).to_string()),
                            expiration_date: Some(info.end_time.to_string()),
                            issuer: Some(format!("{:?}", info.issuer).to_string()),
                            error_message: None,
                        },
                        Err(e) => ProtoLicenseInfo {
                            license_type: "Unknow".to_owned(),
                            finger_print: None,
                            expiration_date: None,
                            issuer: None,
                            error_message: Some(format!("Parse license file error: {}", e)),
                        },
                    }
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => ProtoLicenseInfo {
                    license_type: "Free Trial".to_owned(),
                    finger_print: None,
                    expiration_date: None,
                    issuer: None,
                    error_message: None,
                },
                _ => ProtoLicenseInfo {
                    license_type: "Unknow".to_owned(),
                    finger_print: None,
                    expiration_date: None,
                    issuer: None,
                    error_message: Some(format!("Open license file error: {}", e)),
                },
            },
        };

        Soliloquy { config, lic_info }
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
                Some(license_info(_)) => {
                    if let Ok(json_license_info) = serde_json::to_value(self.lic_info.clone()) {
                        response.set_license_info(json_license_info.to_string());
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

#[cfg(test)]
pub mod tests {
    use crate::config::Config;
    use crate::soliloquy::Soliloquy;
    use jsonrpc_types::ErrorCode;
    use libproto::Message;
    use libproto::TryInto;

    fn get_response(toml_str: String) -> libproto::response::Response {
        let config = util::parse_config_from_buffer::<Config>(&toml_str)
            .unwrap_or_else(|err| panic!("Error while parsing config: [{}]", err));

        let mut request = libproto::request::Request::new();
        request.set_software_version(true);
        let req_msg: Message = request.into();

        let soliloquy = Soliloquy::new(config.clone());
        let mut res_msg: Message = soliloquy.handle(&req_msg.try_into().unwrap());
        res_msg.take_response().unwrap()
    }

    #[test]
    pub fn test_disable_get_version() {
        let toml_str = r#"
backlog_capacity = 1000

[profile_config]
flag_prof_start = 0
enable = false
flag_prof_duration = 0

[http_config]
allow_origin = "*"
timeout = 3
enable = true
listen_port = "1337"
listen_ip = "0.0.0.0"

[ws_config]
panic_on_internal = true
fragments_grow = true
panic_on_protocol = false
enable = true
in_buffer_capacity = 2048
panic_on_queue = false
fragment_size = 65535
panic_on_timeout = false
method_strict = false
thread_number = 2
panic_on_capacity = false
masking_strict = false
key_strict = false
max_connections = 800
listen_ip = "0.0.0.0"
listen_port = "4337"
queue_size = 200
fragments_capacity = 100
tcp_nodelay = false
shutdown_on_interrupt = true
out_buffer_grow = true
panic_on_io = false
panic_on_new_connection = false
out_buffer_capacity = 2048
encrypt_server = false
in_buffer_grow = true
panic_on_shutdown = false
panic_on_encoding = false

[new_tx_flow_config]
buffer_duration = 30000000
count_per_batch = 30

        "#;

        let response = get_response(toml_str.to_string());
        assert_eq!(response.code, ErrorCode::MethodNotFound.code());
    }

    #[test]
    pub fn test_enable_get_version() {
        let toml_str = r#"
backlog_capacity = 1000
enable_version = true

[profile_config]
flag_prof_start = 0
enable = false
flag_prof_duration = 0

[http_config]
allow_origin = "*"
timeout = 3
enable = true
listen_port = "1337"
listen_ip = "0.0.0.0"

[ws_config]
panic_on_internal = true
fragments_grow = true
panic_on_protocol = false
enable = true
in_buffer_capacity = 2048
panic_on_queue = false
fragment_size = 65535
panic_on_timeout = false
method_strict = false
thread_number = 2
panic_on_capacity = false
masking_strict = false
key_strict = false
max_connections = 800
listen_ip = "0.0.0.0"
listen_port = "4337"
queue_size = 200
fragments_capacity = 100
tcp_nodelay = false
shutdown_on_interrupt = true
out_buffer_grow = true
panic_on_io = false
panic_on_new_connection = false
out_buffer_capacity = 2048
encrypt_server = false
in_buffer_grow = true
panic_on_shutdown = false
panic_on_encoding = false

[new_tx_flow_config]
buffer_duration = 30000000
count_per_batch = 30

        "#;

        let response = get_response(toml_str.to_string());
        assert_eq!(
            response.get_software_version().contains("softwareVersion"),
            true
        );
    }
}
