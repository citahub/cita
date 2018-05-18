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

use cita_types::{Address, H256};
use types::ids::BlockId;

use super::encode_contract_name;
use super::ContractCallExt;
use libexecutor::executor::{EconomicalModel, Executor};
use num::FromPrimitive;

lazy_static! {
    static ref DELAY_BLOCK_NUMBER: Vec<u8> = encode_contract_name(b"getDelayBlockNumber()");
    static ref PERMISSION_CHECK: Vec<u8> = encode_contract_name(b"getPermissionCheck()");
    static ref QUOTA_CHECK: Vec<u8> = encode_contract_name(b"getQuotaCheck()");
    static ref CHAIN_NAME: Vec<u8> = encode_contract_name(b"getChainName()");
    static ref CHAIN_ID: Vec<u8> = encode_contract_name(b"getChainId()");
    static ref OPERATOR: Vec<u8> = encode_contract_name(b"getOperator()");
    static ref WEBSITE: Vec<u8> = encode_contract_name(b"getWebsite()");
    static ref BLOCK_INTERVAL: Vec<u8> = encode_contract_name(b"getBlockInterval()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str("0000000000000000000000000000000031415926").unwrap();
    static ref ECONOMICAL_MODEL: Vec<u8> = encode_contract_name(b"getEconomicalModel()");
}

/// Configuration items from system contract
pub struct SysConfig<'a> {
    executor: &'a Executor,
}

impl<'a> SysConfig<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        SysConfig { executor }
    }

    fn get_value(
        &self,
        param_types: &[ParamType],
        method: &[u8],
        block_id: Option<BlockId>,
    ) -> Vec<Token> {
        let address = &*CONTRACT_ADDRESS;
        let block_id = block_id.unwrap_or(BlockId::Latest);
        let output = self.executor.call_method(address, method, None, block_id);
        trace!("sys_config value output: {:?}", output);
        decode(param_types, &output).expect("decode value error")
    }

    /// Delay block number before validate
    pub fn delay_block_number(&self) -> u64 {
        let value =
            self.get_value(
                &[ParamType::Uint(256)],
                DELAY_BLOCK_NUMBER.as_slice(),
                Some(BlockId::Latest),
            ).remove(0)
                .to_uint()
                .expect("decode delay number");
        let number = H256::from(value).low_u64();
        debug!("delay block number: {:?}", number);
        number
    }

    /// Whether check permission or not
    pub fn permission_check(&self) -> bool {
        let check =
            self.get_value(
                &[ParamType::Bool],
                PERMISSION_CHECK.as_slice(),
                Some(BlockId::Latest),
            ).remove(0)
                .to_bool()
                .expect("decode check permission");
        debug!("check permission: {:?}", check);
        check
    }

    /// Whether check quota or not
    pub fn quota_check(&self) -> bool {
        let check =
            self.get_value(
                &[ParamType::Bool],
                QUOTA_CHECK.as_slice(),
                Some(BlockId::Latest),
            ).remove(0)
                .to_bool()
                .expect("decode check quota");
        debug!("check quota: {:?}", check);
        check
    }

    /// The name of current chain
    pub fn chain_name(&self, block_id: Option<BlockId>) -> String {
        let chain_name = self
            .get_value(&[ParamType::String], CHAIN_NAME.as_slice(), block_id)
            .remove(0)
            .to_string()
            .expect("decode chain name");
        debug!("current chain name: {:?}", chain_name);
        chain_name
    }

    /// The id of current chain
    pub fn chain_id(&self) -> u32 {
        let value =
            self.get_value(
                &[ParamType::Uint(64)],
                CHAIN_ID.as_slice(),
                Some(BlockId::Latest),
            ).remove(0)
                .to_uint()
                .expect("decode chain id");
        let chain_id = H256::from(value).low_u64() as u32;
        debug!("current chain id: {:?}", chain_id);
        chain_id
    }

    /// The operator of current chain
    pub fn operator(&self, block_id: Option<BlockId>) -> String {
        let operator = self
            .get_value(&[ParamType::String], OPERATOR.as_slice(), block_id)
            .remove(0)
            .to_string()
            .expect("decode operator");
        debug!("current operator: {:?}", operator);
        operator
    }

    /// Current operator's website URL
    pub fn website(&self, block_id: Option<BlockId>) -> String {
        let website = self
            .get_value(&[ParamType::String], WEBSITE.as_slice(), block_id)
            .remove(0)
            .to_string()
            .expect("decode website URL");
        debug!("website: {:?}", website);
        website
    }

    /// The interval time for creating a block (milliseconds)
    pub fn block_interval(&self) -> u64 {
        let value =
            self.get_value(
                &[ParamType::Uint(64)],
                BLOCK_INTERVAL.as_slice(),
                Some(BlockId::Latest),
            ).remove(0)
                .to_uint()
                .expect("decode block interval");
        let interval = H256::from(value).low_u64();
        debug!("block interval: {:?}", interval);
        interval
    }

    /// enum EconomicalModel { Quota, Charge }
    /// Quota: Default config is quota
    /// Charge: Charging by gas * gasPrice and reward for proposer
    pub fn economical_model(&self) -> EconomicalModel {
        let value =
            self.get_value(
                &[ParamType::Uint(64)],
                ECONOMICAL_MODEL.as_slice(),
                Some(BlockId::Latest),
            ).remove(0)
                .to_uint()
                .expect("decode economical model");
        let t = H256::from(value).low_u64() as u8;
        debug!("economical model: {:?}", t);
        EconomicalModel::from_u8(t).expect("unknown economical model")
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::{EconomicalModel, SysConfig};
    use tests::helpers::init_executor;

    #[test]
    fn test_delay_block_number() {
        let executor = init_executor();
        let number = SysConfig::new(&executor).delay_block_number();
        assert_eq!(number, 1);
    }

    #[test]
    fn test_permission_check() {
        let executor = init_executor();
        let check_permission = SysConfig::new(&executor).permission_check();
        // Is true in the test module.
        assert_eq!(check_permission, true);
    }

    #[test]
    fn test_quota_check() {
        let executor = init_executor();
        let check_quota = SysConfig::new(&executor).quota_check();
        // Is true in the test module.
        assert_eq!(check_quota, true);
    }

    #[test]
    fn test_chain_name() {
        let executor = init_executor();
        let value = SysConfig::new(&executor).chain_name(None);
        assert_eq!(value, "test-chain");
    }

    #[test]
    fn test_chain_id() {
        let executor = init_executor();
        let value = SysConfig::new(&executor).chain_id();
        assert_eq!(value, 123);
    }

    #[test]
    fn test_operator() {
        let executor = init_executor();
        let value = SysConfig::new(&executor).operator(None);
        assert_eq!(value, "test-operator");
    }

    #[test]
    fn test_website() {
        let executor = init_executor();
        let value = SysConfig::new(&executor).website(None);
        assert_eq!(value, "https://www.cryptape.com");
    }

    #[test]
    fn test_block_interval() {
        let executor = init_executor();
        let value = SysConfig::new(&executor).block_interval();
        assert_eq!(value, 3000);
    }

    #[test]
    fn test_economical_model() {
        let executor = init_executor();
        let value = SysConfig::new(&executor).economical_model();
        assert_eq!(value, EconomicalModel::Quota);
    }
}
