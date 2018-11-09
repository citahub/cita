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

//! User management.

use super::ContractCallExt;
use cita_types::{Address, H160};
use contracts::tools::{decode as decode_tools, method as method_tools};
use libexecutor::executor::Executor;
use std::collections::HashMap;
use std::str::FromStr;
use types::ids::BlockId;
use types::reserved_addresses;

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

    pub fn load_group_accounts(&self, block_id: BlockId) -> HashMap<Address, Vec<Address>> {
        let mut group_accounts = HashMap::new();
        let groups = self
            .all_groups(block_id)
            .unwrap_or_else(Self::default_all_groups);

        trace!("ALl groups: {:?}", groups);
        for group in groups {
            let accounts = self
                .accounts(&group, block_id)
                .unwrap_or_else(Self::default_accounts);
            trace!("ALl accounts for group {}: {:?}", group, accounts);
            group_accounts.insert(group, accounts);
        }

        group_accounts
    }

    /// Group array
    pub fn all_groups(&self, block_id: BlockId) -> Option<Vec<Address>> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*ALLGROUPS_HASH.as_slice(),
                None,
                block_id,
            )
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }

    pub fn default_all_groups() -> Vec<Address> {
        error!("Use default all groups.");
        Vec::new()
    }

    /// Accounts array
    pub fn accounts(&self, address: &Address, block_id: BlockId) -> Option<Vec<Address>> {
        self.executor
            .call_method(address, &ACCOUNTS_HASH.as_slice(), None, block_id)
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }

    pub fn default_accounts() -> Vec<Address> {
        error!("Use default accounts.");
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;

    use super::UserManagement;
    use cita_types::{Address, H160};
    use std::str::FromStr;
    use tests::helpers::init_executor;
    use types::ids::BlockId;
    use types::reserved_addresses;

    #[test]
    fn test_all_groups() {
        let executor = init_executor(vec![]);

        let user_management = UserManagement::new(&executor);
        let all_groups: Vec<Address> = user_management.all_groups(BlockId::Pending).unwrap();

        assert_eq!(
            all_groups,
            vec![H160::from_str(reserved_addresses::GROUP).unwrap()]
        );
    }

    #[test]
    fn test_accounts() {
        let executor = init_executor(vec![(
            "Group.accounts",
            concat!(
                "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523,",
                "0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888,",
                "0x9dcd6b234e2772c5451fd4ccf7582f4283140697"
            ),
        )]);

        let user_management = UserManagement::new(&executor);
        let accounts: Vec<Address> = user_management
            .accounts(
                &H160::from_str("ffffffffffffffffffffffffffffffffff020009").unwrap(),
                BlockId::Pending,
            )
            .unwrap();

        assert_eq!(
            accounts,
            vec![
                Address::from_str("4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523").unwrap(),
                Address::from_str("d3f1a71d1d8f073f4e725f57bbe14d67da22f888").unwrap(),
                Address::from_str("9dcd6b234e2772c5451fd4ccf7582f4283140697").unwrap(),
            ]
        );
    }

    #[test]
    fn test_load_group_accounts() {
        let executor = init_executor(vec![(
            "Group.accounts",
            concat!(
                "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523,",
                "0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888,",
                "0x9dcd6b234e2772c5451fd4ccf7582f4283140697"
            ),
        )]);
        let root = H160::from_str(reserved_addresses::GROUP).unwrap();

        let user_management = UserManagement::new(&executor);
        let group_accounts = user_management.load_group_accounts(BlockId::Pending);
        assert_eq!(group_accounts.contains_key(&root), true);
        assert_eq!(
            *group_accounts.get(&root).unwrap(),
            vec![
                Address::from_str("4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523").unwrap(),
                Address::from_str("d3f1a71d1d8f073f4e725f57bbe14d67da22f888").unwrap(),
                Address::from_str("9dcd6b234e2772c5451fd4ccf7582f4283140697").unwrap(),
            ]
        );
    }
}
