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

//! Quota manager.

use super::ContractCallExt;
use super::encode_contract_name;
use ethabi::{decode, ParamType};
use libexecutor::executor::Executor;
use libproto::blockchain::AccountGasLimit as ProtoAccountGasLimit;
use std::collections::HashMap;
use std::str::FromStr;
use util::*;

const QUOTA: &'static [u8] = &*b"getUsersQuota()";
const USERS_METHOD_NAME: &'static [u8] = &*b"getSpecialUsers()";
const BLOCK_GAS_LIMIT: &'static [u8] = &*b"getblockGasLimit()";
const ACCOUNT_GAS_LIMIT: &'static [u8] = &*b"getAccountGasLimit()";

lazy_static! {
    static ref QUOTA_ENCODED: Vec<u8> = encode_contract_name(QUOTA);
    static ref USERS_METHOD_HASH: Vec<u8> = encode_contract_name(USERS_METHOD_NAME);
    static ref BLOCK_GAS_LIMIT_HASH: Vec<u8> = encode_contract_name(BLOCK_GAS_LIMIT);
    static ref ACCOUNT_GAS_LIMIT_HASH: Vec<u8> = encode_contract_name(ACCOUNT_GAS_LIMIT);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("00000000000000000000000000000000013241a3").unwrap();
}

#[derive(PartialEq, Clone, Default, Debug)]
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
        let specific_gas_limit: HashMap<String, u64> = self.get_specific_gas_limit()
            .iter()
            .map(|(k, v)| (k.hex(), *v))
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
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*QUOTA_ENCODED.as_slice());
        trace!("quota output: {:?}", output);

        let mut decoded = decode(&[ParamType::Array(Box::new(ParamType::Uint(256)))], &output).unwrap();
        let quotas = decoded.remove(0).to_array().unwrap();
        let quotas = quotas
            .into_iter()
            .map(|quota| {
                let quota = quota.to_uint();
                let h256 = H256::from(quota.expect("decode quota"));
                h256.low_u64()
            })
            .collect();
        debug!("quotas: {:?}", quotas);
        quotas
    }

    /// Account array
    pub fn users(executor: &Executor) -> Vec<Address> {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*USERS_METHOD_HASH.as_slice());
        trace!("users output: {:?}", output);

        let mut decoded = decode(&[ParamType::Array(Box::new(ParamType::Address))], &output).unwrap();
        let users = decoded.remove(0).to_array().unwrap();
        let users = users
            .into_iter()
            .map(|de| Address::from(de.to_address().expect("decode quota users")))
            .collect();
        debug!("quota users: {:?}", users);
        users
    }

    /// Global gas limit
    pub fn block_gas_limit(executor: &Executor) -> u64 {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*BLOCK_GAS_LIMIT_HASH.as_slice());
        trace!("block_gas_limit output: {:?}", output);

        let mut decoded = decode(&[ParamType::Uint(256)], &output).expect("decode quota");
        let block_gas_limit = decoded.remove(0);
        let block_gas_limit = block_gas_limit.to_uint();

        let h256 = H256::from(block_gas_limit.expect("decode block gas limit"));
        debug!("block gas limit: {:?}", h256.low_u64());
        h256.low_u64()
    }

    /// Global account gas limit
    pub fn account_gas_limit(executor: &Executor) -> u64 {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*ACCOUNT_GAS_LIMIT_HASH.as_slice());
        trace!("account_gas_limit output: {:?}", output);

        let mut decoded = decode(&[ParamType::Uint(256)], &output).expect("decode quota");
        let account_gas_limit = decoded.remove(0);
        let account_gas_limit = account_gas_limit.to_uint();

        let h256 = H256::from(account_gas_limit.expect("decode block gas limit"));
        debug!("account gas limit: {:?}", h256.low_u64());
        h256.low_u64()
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;
    use super::*;
    use cita_crypto::{PrivKey, SIGNATURE_NAME};
    use tests::helpers::init_executor;

    #[test]
    fn test_users() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8c0e\
                 304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcfd9786\
                 61983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let executor = init_executor();
        println!("init executor finish");

        let users = QuotaManager::users(&executor);

        assert_eq!(
            users,
            vec![
                H160::from_str("d3f1a71d1d8f073f4e725f57bbe14d67da22f888").unwrap(),
            ]
        );
    }

    #[test]
    fn test_quota() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8\
                 c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcf\
                 d978661983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let executor = init_executor();
        println!("init executor finish");

        let quota = QuotaManager::quota(&executor);

        assert_eq!(quota, vec![1073741824]);
    }

    #[test]
    fn test_block_gas_limit() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c9\
                 ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed\
                 2bcbcfd978661983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let executor = init_executor();
        println!("init executor finish");

        let block_gas_limit = QuotaManager::block_gas_limit(&executor);

        assert_eq!(block_gas_limit, 1073741824);
    }

    #[test]
    fn test_account_gas_limit() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from(
                "fc8937b92a38faf0196bdac328723c52da0e810f78d257c\
                 9ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223\
                 ed2bcbcfd978661983b8af4bc115d2d66",
            )
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let executor = init_executor();
        println!("init executor finish");

        let account_gas_limit = QuotaManager::account_gas_limit(&executor);

        assert_eq!(account_gas_limit, 268435456);
    }
}
