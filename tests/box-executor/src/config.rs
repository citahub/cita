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

extern crate cita_crypto as crypto;

use crate::crypto::PrivKey;
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
