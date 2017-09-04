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


use db::{self as db, Writable, ConstKey};
use factory::Factories;
use libchain::block::Block;
use serde_json;
use state::State;
use state_db::StateDB;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use util::{Address, H256, U256};
use util::kvdb::KeyValueDB;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Contract {
    pub nonce: String,
    pub code: String,
    pub storage: HashMap<String, String>,
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
        Genesis {
            spec: spec,
            block: Block::default(),
        }
    }

    pub fn lazy_execute(&mut self, state_db: &StateDB, factories: &Factories) {
        let mut state = State::from_existing(state_db.boxed_clone(), self.block.state_root().clone(), U256::from(0), factories.clone()).expect("state db error");
        self.block.set_version(0);
        self.block.set_parent_hash(self.spec.prevhash);
        self.block.set_timestamp(self.spec.timestamp);
        self.block.set_number(0);

        info!("**** begin **** \n");
        info!("chain first init, to do init contracts on height eq zero");
        for (address, contract) in self.spec.alloc.clone() {
            let address = Address::from_any_str(address.as_str()).unwrap();

            state.new_contract(&address, U256::from(0));
            let _ = state.init_code(&address, contract.code.as_bytes().into()).expect("init code fail");

            for (key, values) in contract.storage.clone() {
                state.set_storage(&address, H256::from_any_str(key.as_ref()).unwrap(), H256::from_any_str(values.as_ref()).unwrap())
                     .expect("init code set_storage fail");
            }
        }
        state.commit().expect("state commit error");
        //query is store in chain
        for (address, contract) in &self.spec.alloc {
            let address = Address::from_any_str(address.as_str()).unwrap();
            for (key, values) in &contract.storage {
                let result = state.storage_at(&address, &H256::from_any_str(key.as_ref()).unwrap());
                info!("address = {:?}, key = {:?}, result = {:?}", address, key, result);
                assert_eq!(H256::from_any_str(values.as_ref()).unwrap(), result.expect("storage error"));
            }
        }

        info!("**** end **** \n");
        let root = state.root().clone();
        trace!("root {:?}", root);
        self.block.set_state_root(root);
    }

    pub fn save_genesis(&mut self, db: &KeyValueDB) -> Result<(), String> {
        let mut batch = db.transaction();
        let hash = self.block.hash();
        let height = self.block.number();
        batch.write(db::COL_HEADERS, &hash, self.block.header());
        batch.write(db::COL_BODIES, &hash, self.block.body());
        batch.write(db::COL_EXTRA, &ConstKey::CurrentHash, &hash);
        batch.write(db::COL_EXTRA, &height, &hash);
        db.write(batch)
    }
}
