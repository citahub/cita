// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

//! Quota Price Management

use super::ContractCallExt;
use std::str::FromStr;

use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::libexecutor::executor::Executor;
use crate::types::block_tag::BlockTag;
use crate::types::reserved_addresses;

use cita_types::{Address, U256};

lazy_static! {
    static ref GET_QUOTA_PRICE: Vec<u8> = method_tools::encode_to_vec(b"getQuotaPrice()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::PRICE_MANAGEMENT).unwrap();
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
            .call_method(
                &*CONTRACT_ADDRESS,
                &*GET_QUOTA_PRICE.as_slice(),
                None,
                block_tag,
            )
            .ok()
            .and_then(|output| decode_tools::to_u256(&output))
    }

    pub fn default_quota_price() -> U256 {
        info!("Use default quota price");
        U256::from(1)
    }
}

//#[cfg(test)]
//mod tests {
//    use super::PriceManagement;
//    use crate::tests::helpers::init_executor;
//    use crate::types::block_tag::BlockTag;
//    use cita_types::U256;
//
//    #[test]
//    fn test_quota_price() {
//        let executor = init_executor();
//        let price_management = PriceManagement::new(&executor);
//        let price = price_management.quota_price(BlockTag::Pending).unwrap();
//        assert_eq!(price, U256::from(100_0000));
//    }
//}
