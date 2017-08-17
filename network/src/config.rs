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

extern crate toml;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, RustcDecodable)]
pub struct NetConfig {
    pub id_card: Option<u32>,
    pub port: Option<u64>,
    pub max_peer: Option<u64>,
    pub peers: Option<Vec<PeerConfig>>,
}

#[derive(Debug, RustcDecodable)]
pub struct PeerConfig {
    pub id_card: Option<u32>,
    pub ip: Option<String>,
    pub port: Option<u64>,
}

impl NetConfig {
    pub fn new(path: &str) -> Self {
        let config_file = File::open(path).unwrap();
        let mut fconfig = BufReader::new(config_file);
        let mut content = String::new();
        fconfig.read_to_string(&mut content).unwrap();
        toml::decode_str(&content).unwrap()
    }

    pub fn test_config() -> Self {
        let toml = r#"
            id_card=0
            port = 40000
            max_peer = 1
            [[peers]]
            id_card=0
            ip = "127.0.0.1"
            port = 40000
        "#;

        toml::decode_str(toml).unwrap()
    }
}



#[cfg(test)]
mod test {
    use super::NetConfig;
    extern crate toml;
    #[test]
    fn basics() {
        let toml = r#"
            port = 40000
            max_peer = 2
            [[peers]]
            ip = "127.0.0.1"
            port = 40001
            [[peers]]
            ip = "127.0.0.1"
            port = 40002
        "#;

        let value: NetConfig = toml::decode_str(toml).unwrap();
        println!("{:?}", value);
        assert_eq!(value.port, Some(40000));
    }
}
