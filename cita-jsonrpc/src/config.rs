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

use serde_json::from_reader;
use std::convert::Into;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use ws::Settings;

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;
    // Read the JSON contents of the file as an instance of `User`.
    let u = from_reader(file)?;

    // Return the `User`.
    Ok(u)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub backlog_capacity: usize,
    pub profile_config: ProfileConfig,
    pub http_config: HttpConfig,
    pub ws_config: WsConfig,
    pub new_tx_flow_config: NewTxFlowConfig,
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
}
