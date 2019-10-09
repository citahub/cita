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

use crate::header::BlockNumber;
use cita_types::{Address, H256, U256};
use cita_vm::evm::Context as EVMContext;
use std::sync::Arc;

pub type LastHashes = Vec<H256>;

#[derive(Debug, Clone)]
pub struct Context {
    pub block_number: BlockNumber,
    pub coin_base: Address,
    pub timestamp: u64,
    pub difficulty: U256,
    pub last_hashes: Arc<LastHashes>,
    pub quota_used: U256,
    pub block_quota_limit: U256,
    pub account_quota_limit: U256,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            block_number: 0,
            coin_base: Address::default(),
            timestamp: 0,
            difficulty: U256::default(),
            block_quota_limit: U256::from(u64::max_value()),
            last_hashes: Arc::new(vec![]),
            quota_used: U256::default(),
            account_quota_limit: U256::default(),
        }
    }
}

impl From<EVMContext> for Context {
    fn from(evm_context: EVMContext) -> Context {
        Context {
            block_quota_limit: U256::from(evm_context.gas_limit),
            coin_base: evm_context.coinbase,
            block_number: evm_context.number.as_u64(),
            timestamp: evm_context.timestamp,
            difficulty: evm_context.difficulty,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Context;
    use cita_types::U256;

    #[test]
    fn test_default() {
        let context = Context::default();
        assert_eq!(context.quota_used, U256::zero());
        assert_eq!(context.block_quota_limit, U256::from(u64::max_value()));
    }
}
