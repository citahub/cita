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

//! Quota manager.

use super::ContractCallExt;
use cita_types::traits::LowerHex;
use cita_types::{Address, H160};
use contracts::tools::{decode as decode_tools, method as method_tools};
use libexecutor::executor::Executor;
use libproto::blockchain::AccountGasLimit as ProtoAccountGasLimit;
use std::collections::HashMap;
use std::str::FromStr;
use types::ids::BlockId;
use types::reserved_addresses;

const QUOTAS: &[u8] = &*b"getQuotas()";
const ACCOUNTS: &[u8] = &*b"getAccounts()";
const BQL: &[u8] = &*b"getBQL()";
const DEFAULT_AQL: &[u8] = &*b"getDefaultAQL()";

lazy_static! {
    static ref QUOTAS_HASH: Vec<u8> = method_tools::encode_to_vec(QUOTAS);
    static ref ACCOUNTS_HASH: Vec<u8> = method_tools::encode_to_vec(ACCOUNTS);
    static ref BQL_HASH: Vec<u8> = method_tools::encode_to_vec(BQL);
    static ref DEFAULT_AQL_HASH: Vec<u8> = method_tools::encode_to_vec(DEFAULT_AQL);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(reserved_addresses::QUOTA_MANAGER).unwrap();
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
pub struct AccountGasLimit {
    pub common_gas_limit: u64,
    pub specific_gas_limit: HashMap<Address, u64>,
}

impl AccountGasLimit {
    pub fn new() -> Self {
        AccountGasLimit {
            common_gas_limit: 4_294_967_296,
            specific_gas_limit: HashMap::new(),
        }
    }

    pub fn set_common_gas_limit(&mut self, v: u64) {
        self.common_gas_limit = v;
    }

    pub fn get_common_gas_limit(&self) -> u64 {
        self.common_gas_limit
    }

    pub fn set_specific_gas_limit(&mut self, v: HashMap<Address, u64>) {
        self.specific_gas_limit = v;
    }

    pub fn get_specific_gas_limit(&self) -> &HashMap<Address, u64> {
        &self.specific_gas_limit
    }
}

impl Into<ProtoAccountGasLimit> for AccountGasLimit {
    fn into(self) -> ProtoAccountGasLimit {
        let mut r = ProtoAccountGasLimit::new();
        r.common_quota_limit = self.common_gas_limit;
        let specific_gas_limit: HashMap<String, u64> = self
            .get_specific_gas_limit()
            .iter()
            .map(|(k, v)| (k.lower_hex(), *v))
            .collect();
        r.set_specific_quota_limit(specific_gas_limit);
        r
    }
}

pub struct QuotaManager<'a> {
    executor: &'a Executor,
}

impl<'a> QuotaManager<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        QuotaManager { executor }
    }

    /// Special account gas limit
    pub fn specific(&self, block_id: BlockId) -> HashMap<Address, u64> {
        let users = self.users(block_id).unwrap_or_else(Self::default_users);
        let quota = self.quota(block_id).unwrap_or_else(Self::default_quota);
        let mut specific = HashMap::new();
        for (k, v) in users.iter().zip(quota.iter()) {
            specific.insert(*k, *v);
        }
        specific
    }

    /// Quota array
    pub fn quota(&self, block_id: BlockId) -> Option<Vec<u64>> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*QUOTAS_HASH.as_slice(), None, block_id)
            .ok()
            .and_then(|output| decode_tools::to_u64_vec(&output))
    }

    pub fn default_quota() -> Vec<u64> {
        error!("Use default quota.");
        Vec::new()
    }

    /// Account array
    pub fn users(&self, block_id: BlockId) -> Option<Vec<Address>> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*ACCOUNTS_HASH.as_slice(),
                None,
                block_id,
            )
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }

    pub fn default_users() -> Vec<Address> {
        error!("Use default users.");
        Vec::new()
    }

    /// Global gas limit
    pub fn block_gas_limit(&self, block_id: BlockId) -> Option<u64> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*BQL_HASH.as_slice(), None, block_id)
            .ok()
            .and_then(|output| decode_tools::to_u64(&output))
    }

    pub fn default_block_gas_limit() -> u64 {
        error!("Use default block gas limit.");
        1_073_741_824
    }

    /// Global account gas limit
    pub fn account_gas_limit(&self, block_id: BlockId) -> Option<u64> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*DEFAULT_AQL_HASH.as_slice(),
                None,
                block_id,
            )
            .ok()
            .and_then(|output| decode_tools::to_u64(&output))
    }

    pub fn default_account_gas_limit() -> u64 {
        error!("Use default account gas limit.");
        268_435_456
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;

    use super::QuotaManager;
    use cita_types::H160;
    use std::str::FromStr;
    use tests::helpers::init_executor;
    use types::ids::BlockId;

    #[test]
    fn test_users() {
        let executor = init_executor(vec![
            ((
                "QuotaManager.admin",
                "0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888",
            )),
        ]);
        println!("init executor finish");

        let quota_management = QuotaManager::new(&executor);
        let users = quota_management.users(BlockId::Pending).unwrap();
        assert_eq!(
            users,
            vec![H160::from_str("d3f1a71d1d8f073f4e725f57bbe14d67da22f888").unwrap()]
        );
    }

    #[test]
    fn test_quota() {
        let executor = init_executor(vec![]);
        println!("init executor finish");

        let quota_management = QuotaManager::new(&executor);
        let quota = quota_management.quota(BlockId::Pending).unwrap();
        assert_eq!(quota, vec![1073741824]);
    }

    #[test]
    fn test_block_gas_limit() {
        let executor = init_executor(vec![]);
        println!("init executor finish");

        let quota_management = QuotaManager::new(&executor);
        let block_gas_limit = quota_management.block_gas_limit(BlockId::Pending).unwrap();
        assert_eq!(block_gas_limit, 1073741824);
    }

    #[test]
    fn test_account_gas_limit() {
        let executor = init_executor(vec![]);
        println!("init executor finish");

        let quota_management = QuotaManager::new(&executor);
        let account_gas_limit = quota_management
            .account_gas_limit(BlockId::Pending)
            .unwrap();
        assert_eq!(account_gas_limit, 268435456);
    }
}
