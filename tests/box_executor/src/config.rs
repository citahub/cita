extern crate bincode;
extern crate cita_crypto as crypto;
extern crate cita_types;
extern crate clap;
extern crate dotenv;
extern crate pubsub;
extern crate rlp;
extern crate rustc_serialize;
extern crate serde_yaml;
extern crate util;

use crypto::PrivKey;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::str::FromStr;

pub struct Config {
    pub private_key: PrivKey,
    pub blocks: HashMap<u64, serde_yaml::Value>,
}

impl Config {
    // Initialize Config from given command line options
    pub fn init(path: &str) -> Self {
        let mut yaml_str = String::new();
        let _ = fs::File::open(path)
            .unwrap()
            .read_to_string(&mut yaml_str)
            .unwrap();
        let mut yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str.as_str()).unwrap();

        let blocks = {
            let mut mock_blocks: HashMap<u64, serde_yaml::Value> = HashMap::new();
            for block in yaml["blocks"].as_sequence_mut().unwrap().iter() {
                let block_number = block["number"].as_u64().unwrap();
                mock_blocks.insert(block_number, block.clone());
            }
            Self::detect_missing_blocks(&mock_blocks);
            mock_blocks
        };
        let private_key: PrivKey = {
            let val = &yaml;
            let val = val["privkey"].as_str().unwrap();
            PrivKey::from_str(val).unwrap()
        };

        Config {
            private_key,
            blocks,
        }
    }

    fn detect_missing_blocks(blocks: &HashMap<u64, serde_yaml::Value>) {
        for number in 1..=blocks.len() as u64 {
            if !blocks.contains_key(&number) {
                panic!("missing block-{}", number);
            }
        }
    }
}
