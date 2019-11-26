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

//! User management.

use super::ContractCallExt;
use std::collections::HashMap;
use std::str::FromStr;

use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;

use cita_types::{Address, H160};

const ALLGROUPS: &[u8] = &*b"queryGroups()";
const ACCOUNTS: &[u8] = &*b"queryAccounts()";

lazy_static! {
    static ref ACCOUNTS_HASH: Vec<u8> = method_tools::encode_to_vec(ACCOUNTS);
    static ref ALLGROUPS_HASH: Vec<u8> = method_tools::encode_to_vec(ALLGROUPS);
    static ref CONTRACT_ADDRESS: H160 =
        H160::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap();
}

pub struct UserManagement<'a> {
    executor: &'a Executor,
}

impl<'a> UserManagement<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        UserManagement { executor }
    }

    pub fn load_group_accounts(&self, block_tag: BlockTag) -> HashMap<Address, Vec<Address>> {
        let mut group_accounts = HashMap::new();
        let groups = self
            .all_groups(block_tag)
            .unwrap_or_else(Self::default_all_groups);

        trace!("ALl groups: {:?}", groups);
        for group in groups {
            let accounts = self
                .accounts(&group, block_tag)
                .unwrap_or_else(Self::default_accounts);
            trace!("ALl accounts for group {}: {:?}", group, accounts);
            group_accounts.insert(group, accounts);
        }

        group_accounts
    }

    /// Group array
    pub fn all_groups(&self, block_tag: BlockTag) -> Option<Vec<Address>> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*ALLGROUPS_HASH.as_slice(),
                None,
                block_tag,
            )
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }

    pub fn default_all_groups() -> Vec<Address> {
        info!("Use default all groups.");
        Vec::new()
    }

    /// Accounts array
    pub fn accounts(&self, address: &Address, block_tag: BlockTag) -> Option<Vec<Address>> {
        self.executor
            .call_method(address, &ACCOUNTS_HASH.as_slice(), None, block_tag)
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }

    pub fn default_accounts() -> Vec<Address> {
        info!("Use default accounts.");
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_logger as logger;

    use super::UserManagement;
    use crate::tests::helpers::init_executor;
    use crate::types::block_number::{BlockTag, Tag};
    use crate::types::reserved_addresses;
    use cita_types::{Address, H160};
    use std::str::FromStr;

    #[test]
    fn test_all_groups() {
        let executor = init_executor();

        let user_management = UserManagement::new(&executor);
        let all_groups: Vec<Address> = user_management
            .all_groups(BlockTag::Tag(Tag::Pending))
            .unwrap();

        assert_eq!(
            all_groups,
            vec![H160::from_str(reserved_addresses::GROUP).unwrap()]
        );
    }
}
