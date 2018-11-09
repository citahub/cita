// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use serde_json;
use std::collections::HashMap;
use std::time::Duration;

use cita_crypto::PrivKey;
use cita_types::U256;

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
    let servers = config
        .chains
        .into_iter()
        .map(|c| (c.id, c.servers))
        .collect();
    Config { pkey, servers }
}
