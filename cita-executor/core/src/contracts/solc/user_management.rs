// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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
        info!("Use default all groups.");
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
        info!("Use default accounts.");
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_logger as logger;

    use super::UserManagement;
    use cita_types::{Address, H160};
    use std::str::FromStr;
    use tests::helpers::init_executor;
    use types::ids::BlockId;
    use types::reserved_addresses;

    #[test]
    fn test_all_groups() {
        let executor = init_executor();

        let user_management = UserManagement::new(&executor);
        let all_groups: Vec<Address> = user_management.all_groups(BlockId::Pending).unwrap();

        assert_eq!(
            all_groups,
            vec![H160::from_str(reserved_addresses::GROUP).unwrap()]
        );
    }
}
