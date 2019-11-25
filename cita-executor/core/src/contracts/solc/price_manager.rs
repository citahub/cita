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

//! Quota Price Management

use super::ContractCallExt;
use std::str::FromStr;

use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;

use cita_types::{Address, U256};

lazy_static! {
    static ref GET_QUOTA_PRICE: Vec<u8> = method_tools::encode_to_vec(b"getQuotaPrice()");
    static ref CONTRACT_ADDRESS: Address = Address::from_str(reserved_addresses::PRICE_MANAGEMENT).unwrap();
}

/// Configuration items from system contract
pub struct PriceManagement<'a> {
    executor: &'a Executor,
}

impl<'a> PriceManagement<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        PriceManagement { executor }
    }

    /// Set quota price
    pub fn quota_price(&self, block_tag: BlockTag) -> Option<U256> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*GET_QUOTA_PRICE.as_slice(), None, block_tag)
            .ok()
            .and_then(|output| decode_tools::to_u256(&output))
    }

    pub fn default_quota_price() -> U256 {
        info!("Use default quota price");
        U256::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::PriceManagement;
    use crate::tests::helpers::init_executor;
    use crate::types::block_number::{BlockTag, Tag};
    use cita_types::U256;

    #[test]
    fn test_quota_price() {
        let executor = init_executor();
        let price_management = PriceManagement::new(&executor);
        let price = price_management.quota_price(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(price, U256::from(100_0000));
    }
}
