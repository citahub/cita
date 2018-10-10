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

use cita_types::traits::ConvertType;
use cita_types::{clean_0x, Address, H256, U256};
use crypto::digest::Digest;
use crypto::md5::Md5;
use db::{self as db, Writable};
use factory::Factories;
use libexecutor::block::Block;
use libexecutor::extras::*;
use rustc_hex::FromHex;
use serde_json;
use state::State;
use state_db::StateDB;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use util::kvdb::KeyValueDB;
#[cfg(feature = "privatetx")]
use zktx::set_param_path;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Contract {
    pub nonce: String,
    pub code: String,
    pub storage: HashMap<String, String>,
    pub value: Option<U256>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Spec {
    pub alloc: HashMap<String, Contract>,
    pub prevhash: H256,
    pub timestamp: u64,
}

#[derive(Debug, PartialEq)]
pub struct Genesis {
    pub spec: Spec,
    pub block: Block,
}

impl Genesis {
    pub fn init(path: &str) -> Genesis {
        let config_file = File::open(path).unwrap();
        let fconfig = BufReader::new(config_file);
        let spec: Spec = serde_json::from_reader(fconfig).expect("Failed to load genesis.");

        // check resource with pre hash in genesis
        // default pre hash is zero
        let mut pre_hash = H256::zero();
        // resource folder at the same place with genesis file
        let resource_path = Path::new(path).parent().unwrap().join("resource");
        #[cfg(feature = "privatetx")]
        {
            set_param_path(resource_path.join("PARAMS").to_str().unwrap());
        }
        if resource_path.exists() {
            let file_list_path = resource_path.join("file_list");
            if file_list_path.exists() {
                let file_list = File::open(file_list_path).unwrap();
                let mut buf_reader = BufReader::new(file_list);
                let mut contents = String::new();
                buf_reader.read_to_string(&mut contents).unwrap();
                let mut hasher = Md5::new();
                for p in contents.lines() {
                    let path = resource_path.join(p);
                    let file = File::open(path).unwrap();
                    let mut buf_reader = BufReader::new(file);
                    let mut buf = Vec::new();
                    buf_reader.read_to_end(&mut buf).unwrap();
                    hasher.input(&buf);
                }
                let mut hash_str = "0x00000000000000000000000000000000".to_string();
                hash_str += &hasher.result_str();
                info!("resource hash {}", hash_str);
                pre_hash = H256::from_unaligned(hash_str.as_str()).unwrap();
            }
        }

        assert_eq!(pre_hash, spec.prevhash);

        Genesis {
            spec,
            block: Block::default(),
        }
    }

    pub fn lazy_execute(
        &mut self,
        state_db: &StateDB,
        factories: &Factories,
    ) -> Result<(), String> {
        let mut state = State::from_existing(
            state_db.boxed_clone_canon(&self.spec.prevhash),
            *self.block.state_root(),
            U256::from(0),
            factories.clone(),
        )
        .expect("state db error");
        self.block.set_version(0);
        self.block.set_parent_hash(self.spec.prevhash);
        self.block.set_timestamp(self.spec.timestamp);
        self.block.set_number(0);

        info!("This is the first time to init executor, and it will init contracts on height 0");
        trace!("**** begin **** \n");
        for (address, contract) in self.spec.alloc.clone() {
            let address = Address::from_unaligned(address.as_str()).unwrap();
            state.new_contract(&address, U256::from(0), U256::from(0));
            {
                state
                    .init_code(&address, clean_0x(&contract.code).from_hex().unwrap())
                    .expect("init code fail");
                if let Some(value) = contract.value {
                    state
                        .add_balance(&address, &value)
                        .expect("init balance fail");
                }
            }
            for (key, values) in contract.storage.clone() {
                state
                    .set_storage(
                        &address,
                        H256::from_unaligned(key.as_ref()).unwrap(),
                        H256::from_unaligned(values.as_ref()).unwrap(),
                    )
                    .expect("init code set_storage fail");
            }
        }
        state.commit().expect("state commit error");
        //query is store in chain
        for (address, contract) in &self.spec.alloc {
            let address = Address::from_unaligned(address.as_str()).unwrap();
            for (key, values) in &contract.storage {
                let result =
                    state.storage_at(&address, &H256::from_unaligned(key.as_ref()).unwrap());
                trace!(
                    "address = {:?}, key = {:?}, result = {:?}",
                    address,
                    key,
                    result
                );
                assert_eq!(
                    H256::from_unaligned(values.as_ref()).unwrap(),
                    result.expect("storage error")
                );
            }
        }

        trace!("**** end **** \n");
        let root = *state.root();
        trace!("root {:?}", root);
        self.block.set_state_root(root);

        self.save(state, state_db.journal_db().backing())
    }

    fn save(&mut self, state: State<StateDB>, db: &Arc<KeyValueDB>) -> Result<(), String> {
        let mut batch = db.transaction();
        let hash = self.block.hash();
        let height = self.block.number();
        //初始化的时候需要获取头部信息
        batch.write(db::COL_HEADERS, &hash, self.block.header());
        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        batch.write(db::COL_EXTRA, &height, &hash);
        let mut state_db = state.drop().1;
        state_db
            .journal_under(&mut batch, height, &hash)
            .expect("DB commit failed");
        state_db.sync_cache(&[], &[], true);
        db.write(batch)
    }
}

#[cfg(test)]
mod test {
    use cita_types::{H256, U256};
    use libexecutor::genesis::{Contract, Spec};
    use serde_json;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_spec() {
        let genesis = json!({
            "timestamp": 1524000000,
            "alloc": {
                "0xffffffffffffffffffffffffffffffffff021019": {
                    "nonce": "1",
                    "code": "0x6060604052600436106100745763",
                    "storage": {
                        "0x00": "0x013241b2",
                        "0x01": "0x02",
                    }
                },
                "0x000000000000000000000000000000000a3241b6": {
                    "nonce": "1",
                    "code": "0x6060604052600436106100745763",
                    "value": "0x10000000",
                    "storage": {}
                },
            },
            "prevhash": "0x0000000000000000000000000000000000000000000000000000000000000000",
        });
        let spec = Spec {
            prevhash: H256::from_str(
                "0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
            timestamp: 1524000000,
            alloc: [
                (
                    "0xffffffffffffffffffffffffffffffffff021019".to_owned(),
                    Contract {
                        nonce: "1".to_owned(),
                        code: "0x6060604052600436106100745763".to_owned(),
                        value: None,
                        storage: [
                            ("0x00".to_owned(), "0x013241b2".to_owned()),
                            ("0x01".to_owned(), "0x02".to_owned()),
                        ]
                            .iter()
                            .cloned()
                            .collect(),
                    },
                ),
                (
                    "0x000000000000000000000000000000000a3241b6".to_owned(),
                    Contract {
                        nonce: "1".to_owned(),
                        code: "0x6060604052600436106100745763".to_owned(),
                        value: Some(U256::from(0x10000000)),
                        storage: HashMap::new(),
                    },
                ),
            ]
                .iter()
                .cloned()
                .collect(),
        };
        assert_eq!(serde_json::from_value::<Spec>(genesis).unwrap(), spec);
    }
}
