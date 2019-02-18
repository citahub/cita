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
use libproto::blockchain::AccountGasLimit as ProtoAccountQuotaLimit;
use std::collections::HashMap;
use std::str::FromStr;
use types::ids::BlockId;
use types::reserved_addresses;

const QUOTAS: &[u8] = &*b"getQuotas()";
const ACCOUNTS: &[u8] = &*b"getAccounts()";
const BQL: &[u8] = &*b"getBQL()";
const DEFAULT_AQL: &[u8] = &*b"getDefaultAQL()";
// Quota limit of autoExec
const AUTO_EXEC_QL: &[u8] = &*b"getAutoExecQL()";
const BQL_VALUE: u64 = 1_073_741_824;
const AQL_VALUE: u64 = 268_435_456;
pub const AUTO_EXEC_QL_VALUE: u64 = 1_048_576;

lazy_static! {
    static ref QUOTAS_HASH: Vec<u8> = method_tools::encode_to_vec(QUOTAS);
    static ref ACCOUNTS_HASH: Vec<u8> = method_tools::encode_to_vec(ACCOUNTS);
    static ref BQL_HASH: Vec<u8> = method_tools::encode_to_vec(BQL);
    static ref DEFAULT_AQL_HASH: Vec<u8> = method_tools::encode_to_vec(DEFAULT_AQL);
    static ref AUTO_EXEC_QL_HASH: Vec<u8> = method_tools::encode_to_vec(AUTO_EXEC_QL);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(reserved_addresses::QUOTA_MANAGER).unwrap();
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
pub struct AccountQuotaLimit {
    pub common_quota_limit: u64,
    pub specific_quota_limit: HashMap<Address, u64>,
}

impl AccountQuotaLimit {
    pub fn new() -> Self {
        AccountQuotaLimit {
            common_quota_limit: 4_294_967_296,
            specific_quota_limit: HashMap::new(),
        }
    }

    pub fn set_common_quota_limit(&mut self, v: u64) {
        self.common_quota_limit = v;
    }

    pub fn get_common_quota_limit(&self) -> u64 {
        self.common_quota_limit
    }

    pub fn set_specific_quota_limit(&mut self, v: HashMap<Address, u64>) {
        self.specific_quota_limit = v;
    }

    pub fn get_specific_quota_limit(&self) -> &HashMap<Address, u64> {
        &self.specific_quota_limit
    }
}

impl Into<ProtoAccountQuotaLimit> for AccountQuotaLimit {
    fn into(self) -> ProtoAccountQuotaLimit {
        let mut r = ProtoAccountQuotaLimit::new();
        r.common_quota_limit = self.common_quota_limit;
        let specific_quota_limit: HashMap<String, u64> = self
            .get_specific_quota_limit()
            .iter()
            .map(|(k, v)| (k.lower_hex(), *v))
            .collect();
        r.set_specific_quota_limit(specific_quota_limit);
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

    /// Special account quota limit
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

    /// Global quota limit
    pub fn block_quota_limit(&self, block_id: BlockId) -> Option<u64> {
        self.executor
            .call_method(&*CONTRACT_ADDRESS, &*BQL_HASH.as_slice(), None, block_id)
            .ok()
            .and_then(|output| decode_tools::to_u64(&output))
    }

    pub fn default_block_quota_limit() -> u64 {
        error!("Use default block quota limit.");
        BQL_VALUE
    }

    /// Global account quota limit
    pub fn account_quota_limit(&self, block_id: BlockId) -> Option<u64> {
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

    pub fn default_account_quota_limit() -> u64 {
        error!("Use default account quota limit.");
        AQL_VALUE
    }

    /// Auto exec quota limit
    pub fn auto_exec_quota_limit(&self, block_id: BlockId) -> Option<u64> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*AUTO_EXEC_QL_HASH.as_slice(),
                None,
                block_id,
            )
            .ok()
            .and_then(|output| decode_tools::to_u64(&output))
    }

    pub fn default_auto_exec_quota_limit() -> u64 {
        error!("Use default auto exec quota limit.");
        AUTO_EXEC_QL_VALUE
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;

    use super::{QuotaManager, AQL_VALUE, AUTO_EXEC_QL_VALUE, BQL_VALUE};
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

        // Test quota
        let quota = quota_management.quota(BlockId::Pending).unwrap();
        assert_eq!(quota, vec![BQL_VALUE]);

        // Test block quota limit
        let block_quota_limit = quota_management
            .block_quota_limit(BlockId::Pending)
            .unwrap();
        assert_eq!(block_quota_limit, BQL_VALUE);

        // Test account quota limit
        let account_quota_limit = quota_management
            .account_quota_limit(BlockId::Pending)
            .unwrap();
        assert_eq!(account_quota_limit, AQL_VALUE);

        // Test auto exec quota limit
        let auto_exec_quota_limit = quota_management
            .auto_exec_quota_limit(BlockId::Pending)
            .unwrap();
        assert_eq!(auto_exec_quota_limit, AUTO_EXEC_QL_VALUE);
    }
}
