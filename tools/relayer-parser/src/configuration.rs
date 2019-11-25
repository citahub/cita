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

use cita_crypto::PrivKey;
use cita_types::U256;
use serde_json;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct UpStream {
    pub url: String,
    pub timeout: Duration,
}

#[derive(Debug, Deserialize, Clone)]
struct Chain {
    pub id: U256,
    pub servers: Vec<UpStream>,
}

#[derive(Debug, Deserialize, Clone)]
struct FileConfig {
    pub private_key: PrivKey,
    pub chains: Vec<Chain>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pkey: PrivKey,
    servers: HashMap<U256, Vec<UpStream>>,
}

impl FileConfig {
    fn load(path: &str) -> Self {
        let file = ::std::fs::File::open(path).expect("open config file failed");
        let reader = ::std::io::BufReader::new(file);
        serde_json::from_reader(reader).expect("serde_json::from_reader failed")
    }
}

impl Config {
    #[inline]
    pub fn get_servers(&self, chain_id: U256) -> Option<&Vec<UpStream>> {
        self.servers.get(&chain_id)
    }
    #[inline]
    pub fn get_private_key(&self) -> &PrivKey {
        &self.pkey
    }
}

pub fn parse_configfile(path: &str) -> Config {
    let config = FileConfig::load(path);
    let pkey = config.private_key;
    let servers = config.chains.into_iter().map(|c| (c.id, c.servers)).collect();
    Config { pkey, servers }
}
