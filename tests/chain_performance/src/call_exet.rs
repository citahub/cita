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

use common_types::receipt::LocalizedReceipt;
use core::libchain::chain::Chain;
use core_executor::libexecutor::Genesis;
use core_executor::libexecutor::executor::{Block, Config, Executor};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use util::{H160, H256};
use util::KeyValueDB;

#[allow(unused_variables, dead_code)]
#[derive(Clone)]
pub struct Callexet {
    pub exet: Arc<Executor>,
    pub chain: Arc<Chain>,
}

#[allow(unused_variables, dead_code)]
impl Callexet {
    pub fn new(db: Arc<KeyValueDB>, chain: Arc<Chain>, genesis: Genesis, path: &str) -> Self {
        let executor_config = Config::new(path);
        Callexet {
            exet: Arc::new(Executor::init_executor(db, genesis, executor_config)),
            chain: chain,
        }
    }

    pub fn add_block(&self, block: Block, ctx_pub: &Sender<(String, Vec<u8>)>) {
        self.exet.execute_block(block, ctx_pub);
    }

    pub fn get_height(&self) -> u64 {
        self.exet.get_current_height()
    }

    pub fn get_pre_hash(&self) -> H256 {
        self.exet.get_current_hash()
    }

    pub fn get_contract_address(&self, hash: H256) -> H160 {
        info!("------------------  get_contract_address {:?}", hash);
        let receipt = self.chain.localized_receipt(hash).unwrap();
        match receipt.contract_address {
            Some(contract_address) => contract_address,
            None => panic!("contract_address error"),
        }
    }

    pub fn get_receipt(&self, hash: H256) -> LocalizedReceipt {
        info!(
            "------------------- self.chain.localized_receipt(hash) {:?}",
            hash
        );
        self.chain.localized_receipt(hash).unwrap()
    }
}
