extern crate toml;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, RustcDecodable)]
pub struct NetConfig {
    pub id_card:Option<u32>,
    pub port: Option<u64>,
    pub max_peer: Option<u64>,
    pub peers: Option<Vec<PeerConfig>>,
}

#[derive(Debug, RustcDecodable)]
pub struct PeerConfig {
    pub id_card:Option<u32>,
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
