// Copyright Rivtower Technologies LLC.
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

use std::cell::RefCell;
use std::sync::Arc;

use crate::genesis::Account;

use cita_trie::MemoryDB;
use cita_types::{Address, U256};

pub struct Miner;

impl Miner {
    pub fn mine(code: Vec<u8>) -> Option<Account> {
        let db = Arc::new(MemoryDB::new(false));
        let state = cita_vm::state::State::new(db).expect("New state failed.");

        let state_data_provider = Arc::new(RefCell::new(state));
        let block_data_provider: Arc<dyn cita_vm::BlockDataProvider> =
            Arc::new(cita_vm::BlockDataProviderMock::default());
        let context = cita_vm::evm::Context::default();
        let config = cita_vm::Config::default();
        let sender = Address::from("0xd6c8454425135d0cfdb7c1fcba0f8a08a5880bf6");

        // Contruct transaction and exec
        let tx = cita_vm::Transaction {
            from: sender,
            to: None,
            value: U256::from(0),
            nonce: U256::from(0),
            gas_limit: 7_999_999,
            gas_price: U256::from(0),
            input: code,
        };
        let _r = cita_vm::exec(
            block_data_provider.clone(),
            state_data_provider.clone(),
            context,
            config,
            tx,
        )
        .expect("Create genesis exec error.");

        // Cal contract address
        let contract_address =
            cita_vm::create_address_from_address_and_nonce(&sender, &U256::from(0));

        if let Some(account) = state_data_provider
            .borrow()
            .get_state_object(&contract_address)
            .expect("Failed to get state object at given address")
        {
            let a = Account {
                nonce: account.nonce,
                code: String::from("0x") + &hex::encode(account.clone().code),
                storage: account.get_storage_changes(),
                value: account.balance,
            };
            return Some(a);
        }
        None
    }
}
