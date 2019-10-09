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

use std::convert::Into;
use ws::Settings;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub backlog_capacity: usize,
    pub enable_version: Option<bool>,
    pub profile_config: ProfileConfig,
    pub http_config: HttpConfig,
    pub ws_config: WsConfig,
    pub new_tx_flow_config: NewTxFlowConfig,
}

impl Config {
    pub fn new(path: &str) -> Self {
        parse_config!(Config, path)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NewTxFlowConfig {
    pub count_per_batch: usize,
    pub buffer_duration: u32, //in unit of ns
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProfileConfig {
    pub enable: bool,
    pub flag_prof_start: u64,
    pub flag_prof_duration: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WsConfig {
    pub enable: bool,
    pub thread_number: usize,
    pub listen_ip: String,
    pub listen_port: String,

    max_connections: usize,
    queue_size: usize,
    panic_on_new_connection: bool,
    panic_on_shutdown: bool,
    fragments_capacity: usize,
    fragments_grow: bool,
    fragment_size: usize,
    in_buffer_capacity: usize,
    in_buffer_grow: bool,
    out_buffer_capacity: usize,
    out_buffer_grow: bool,
    panic_on_internal: bool,
    panic_on_capacity: bool,
    panic_on_protocol: bool,
    panic_on_encoding: bool,
    panic_on_queue: bool,
    panic_on_io: bool,
    panic_on_timeout: bool,
    shutdown_on_interrupt: bool,
    masking_strict: bool,
    key_strict: bool,
    method_strict: bool,
    encrypt_server: bool,
    tcp_nodelay: bool,
}

impl Into<Settings> for WsConfig {
    fn into(self) -> Settings {
        Settings {
            max_connections: self.max_connections,
            queue_size: self.queue_size,
            panic_on_new_connection: self.panic_on_new_connection,
            panic_on_shutdown: self.panic_on_shutdown,
            fragments_capacity: self.fragments_capacity,
            fragments_grow: self.fragments_grow,
            fragment_size: self.fragment_size,
            in_buffer_capacity: self.in_buffer_capacity,
            in_buffer_grow: self.in_buffer_grow,
            out_buffer_capacity: self.out_buffer_capacity,
            out_buffer_grow: self.out_buffer_grow,
            panic_on_internal: self.panic_on_internal,
            panic_on_capacity: self.panic_on_capacity,
            panic_on_protocol: self.panic_on_protocol,
            panic_on_encoding: self.panic_on_encoding,
            panic_on_queue: self.panic_on_queue,
            panic_on_io: self.panic_on_io,
            panic_on_timeout: self.panic_on_timeout,
            shutdown_on_interrupt: self.shutdown_on_interrupt,
            masking_strict: self.masking_strict,
            key_strict: self.key_strict,
            method_strict: self.method_strict,
            encrypt_server: self.encrypt_server,
            tcp_nodelay: self.tcp_nodelay,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HttpConfig {
    pub enable: bool,
    pub thread_number: Option<usize>,
    pub listen_ip: String,
    pub listen_port: String,
    pub timeout: u64,
    pub allow_origin: Option<String>,
}
