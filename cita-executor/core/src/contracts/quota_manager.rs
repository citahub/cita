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
use super::{encode_contract_name, to_address_vec, to_u256, to_u256_vec};
use cita_types::traits::LowerHex;
use cita_types::{Address, H160};
use libexecutor::executor::Executor;
use libproto::blockchain::AccountGasLimit as ProtoAccountGasLimit;
use std::collections::HashMap;
use std::str::FromStr;
use types::reserved_addresses;

const QUOTAS: &'static [u8] = &*b"getQuotas()";
const ACCOUNTS: &'static [u8] = &*b"getAccounts()";
const BQL: &'static [u8] = &*b"getBQL()";
const DEFAULT_AQL: &'static [u8] = &*b"getDefaultAQL()";

lazy_static! {
    static ref QUOTAS_HASH: Vec<u8> = encode_contract_name(QUOTAS);
    static ref ACCOUNTS_HASH: Vec<u8> = encode_contract_name(ACCOUNTS);
    static ref BQL_HASH: Vec<u8> = encode_contract_name(BQL);
    static ref DEFAULT_AQL_HASH: Vec<u8> = encode_contract_name(DEFAULT_AQL);
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
        r.common_gas_limit = self.common_gas_limit;
        let specific_gas_limit: HashMap<String, u64> = self
            .get_specific_gas_limit()
            .iter()
            .map(|(k, v)| (k.lower_hex(), *v))
            .collect();
        r.set_specific_gas_limit(specific_gas_limit);
        r
    }
}

pub struct QuotaManager;

impl QuotaManager {
    /// Special account gas limit
    pub fn specific(executor: &Executor) -> HashMap<Address, u64> {
        let users = QuotaManager::users(executor);
        let quota = QuotaManager::quota(executor);
        let mut specific = HashMap::new();
        for (k, v) in users.iter().zip(quota.iter()) {
            specific.insert(*k, *v);
        }
        specific
    }

    /// Quota array
    pub fn quota(executor: &Executor) -> Vec<u64> {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*QUOTAS_HASH.as_slice());
        trace!("quota output: {:?}", output);

        to_u256_vec(&output).iter().map(|i| i.low_u64()).collect()
    }

    /// Account array
    pub fn users(executor: &Executor) -> Vec<Address> {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*ACCOUNTS_HASH.as_slice());
        trace!("users output: {:?}", output);

        to_address_vec(&output)
    }

    /// Global gas limit
    pub fn block_gas_limit(executor: &Executor) -> u64 {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*BQL_HASH.as_slice());
        trace!("block_gas_limit output: {:?}", output);

        to_u256(&output).low_u64()
    }

    /// Global account gas limit
    pub fn account_gas_limit(executor: &Executor) -> u64 {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*DEFAULT_AQL_HASH.as_slice());
        trace!("account_gas_limit output: {:?}", output);

        to_u256(&output).low_u64()
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;
    use super::QuotaManager;
    use cita_types::H160;
    use std::str::FromStr;
    use tests::helpers::init_executor;

    #[test]
    fn test_users() {
        let executor = init_executor(vec![
            ((
                "QuotaManager.admin",
                "0xd3f1a71d1d8f073f4e725f57bbe14d67da22f888",
            )),
        ]);
        println!("init executor finish");
        let users = QuotaManager::users(&executor);
        assert_eq!(
            users,
            vec![H160::from_str("d3f1a71d1d8f073f4e725f57bbe14d67da22f888").unwrap()]
        );
    }

    #[test]
    fn test_quota() {
        let executor = init_executor(vec![]);
        println!("init executor finish");
        let quota = QuotaManager::quota(&executor);
        assert_eq!(quota, vec![1073741824]);
    }

    #[test]
    fn test_block_gas_limit() {
        let executor = init_executor(vec![]);
        println!("init executor finish");
        let block_gas_limit = QuotaManager::block_gas_limit(&executor);
        assert_eq!(block_gas_limit, 1073741824);
    }

    #[test]
    fn test_account_gas_limit() {
        let executor = init_executor(vec![]);
        println!("init executor finish");
        let account_gas_limit = QuotaManager::account_gas_limit(&executor);
        assert_eq!(account_gas_limit, 268435456);
    }
}
