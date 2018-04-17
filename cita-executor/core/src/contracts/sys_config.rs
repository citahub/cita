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

//! System Config

use std::str::FromStr;

use ethabi::{decode, ParamType, Token};

use util::{Address, H160, H256};

use super::ContractCallExt;
use super::encode_contract_name;
use libexecutor::executor::Executor;

lazy_static! {
    static ref DELAY_BLOCK_NUMBER: Vec<u8> = encode_contract_name(b"getDelayBlockNumber()");
    static ref PERMISSION_CHECK: Vec<u8> = encode_contract_name(b"getPermissionCheck()");
    static ref QUOTA_CHECK: Vec<u8> = encode_contract_name(b"getQuotaCheck()");
    static ref CHAIN_NAME: Vec<u8> = encode_contract_name(b"getChainName()");
    static ref CHAIN_ID: Vec<u8> = encode_contract_name(b"getChainId()");
    static ref OPERATOR: Vec<u8> = encode_contract_name(b"getOperator()");
    static ref WEBSITE: Vec<u8> = encode_contract_name(b"getWebsite()");
    static ref BLOCK_INTERVAL: Vec<u8> = encode_contract_name(b"getBlockInterval()");
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("0000000000000000000000000000000031415926").unwrap();
}

/// Configuration items from system contract
pub struct SysConfig;

impl SysConfig {
    fn get_value(executor: &Executor, param_types: &[ParamType], address: &Address, method: &[u8]) -> Vec<Token> {
        let output = executor.call_contract_method(address, method);
        trace!("sys_config value output: {:?}", output);
        decode(param_types, &output).expect("decode value error")
    }

    /// Delay block number before validate
    pub fn delay_block_number(executor: &Executor) -> u64 {
        let value = SysConfig::get_value(
            executor,
            &[ParamType::Uint(256)],
            &*CONTRACT_ADDRESS,
            DELAY_BLOCK_NUMBER.as_slice(),
        ).remove(0)
            .to_uint()
            .expect("decode delay number");
        let number = H256::from(value).low_u64();
        debug!("delay block number: {:?}", number);
        number
    }

    /// Whether check permission or not
    pub fn permission_check(executor: &Executor) -> bool {
        let check = SysConfig::get_value(
            executor,
            &[ParamType::Bool],
            &*CONTRACT_ADDRESS,
            PERMISSION_CHECK.as_slice(),
        ).remove(0)
            .to_bool()
            .expect("decode check permission");
        debug!("check permission: {:?}", check);
        check
    }

    /// Whether check quota or not
    pub fn quota_check(executor: &Executor) -> bool {
        let check = SysConfig::get_value(
            executor,
            &[ParamType::Bool],
            &*CONTRACT_ADDRESS,
            QUOTA_CHECK.as_slice(),
        ).remove(0)
            .to_bool()
            .expect("decode check quota");
        debug!("check quota: {:?}", check);
        check
    }

    /// The name of current chain
    pub fn chain_name(executor: &Executor) -> String {
        let chain_name = SysConfig::get_value(
            executor,
            &[ParamType::String],
            &*CONTRACT_ADDRESS,
            CHAIN_NAME.as_slice(),
        ).remove(0)
            .to_string()
            .expect("decode chain name");
        debug!("current chain name: {:?}", chain_name);
        chain_name
    }

    /// The id of current chain
    pub fn chain_id(executor: &Executor) -> u32 {
        let value = SysConfig::get_value(
            executor,
            &[ParamType::Uint(64)],
            &*CONTRACT_ADDRESS,
            CHAIN_ID.as_slice(),
        ).remove(0)
            .to_uint()
            .expect("decode chain id");
        let chain_id = H256::from(value).low_u64() as u32;
        debug!("current chain id: {:?}", chain_id);
        chain_id
    }

    /// The operator of current chain
    pub fn operator(executor: &Executor) -> String {
        let operator = SysConfig::get_value(
            executor,
            &[ParamType::String],
            &*CONTRACT_ADDRESS,
            OPERATOR.as_slice(),
        ).remove(0)
            .to_string()
            .expect("decode operator");
        debug!("current operator: {:?}", operator);
        operator
    }

    /// Current operator's website URL
    pub fn website(executor: &Executor) -> String {
        let website = SysConfig::get_value(
            executor,
            &[ParamType::String],
            &*CONTRACT_ADDRESS,
            WEBSITE.as_slice(),
        ).remove(0)
            .to_string()
            .expect("decode website URL");
        debug!("website: {:?}", website);
        website
    }

    /// The interval time for creating a block (milliseconds)
    pub fn block_interval(executor: &Executor) -> u64 {
        let value = SysConfig::get_value(
            executor,
            &[ParamType::Uint(64)],
            &*CONTRACT_ADDRESS,
            BLOCK_INTERVAL.as_slice(),
        ).remove(0)
            .to_uint()
            .expect("decode block interval");
        let interval = H256::from(value).low_u64();
        debug!("block interval: {:?}", interval);
        interval
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::SysConfig;
    use tests::helpers::init_executor;

    #[test]
    fn test_delay_block_number() {
        let executor = init_executor();
        let number = SysConfig::delay_block_number(&executor);
        assert_eq!(number, 1);
    }

    #[test]
    fn test_permission_check() {
        let executor = init_executor();
        let check_permission = SysConfig::permission_check(&executor);
        // Is true in the test module.
        assert_eq!(check_permission, true);
    }

    #[test]
    fn test_quota_check() {
        let executor = init_executor();
        let check_quota = SysConfig::quota_check(&executor);
        // Is true in the test module.
        assert_eq!(check_quota, true);
    }

    #[test]
    fn test_chain_name() {
        let executor = init_executor();
        let value = SysConfig::chain_name(&executor);
        assert_eq!(value, "test-chain");
    }

    #[test]
    fn test_chain_id() {
        let executor = init_executor();
        let value = SysConfig::chain_id(&executor);
        assert_eq!(value, 123);
    }

    #[test]
    fn test_operator() {
        let executor = init_executor();
        let value = SysConfig::operator(&executor);
        assert_eq!(value, "test-operator");
    }

    #[test]
    fn test_website() {
        let executor = init_executor();
        let value = SysConfig::website(&executor);
        assert_eq!(value, "https://www.cryptape.com");
    }

    #[test]
    fn test_block_interval() {
        let executor = init_executor();
        let value = SysConfig::block_interval(&executor);
        assert_eq!(value, 3000);
    }
}
