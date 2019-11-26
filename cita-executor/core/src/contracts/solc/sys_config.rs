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

//! System Config

use super::ContractCallExt;
use std::str::FromStr;

use crate::contracts::solc::version_management::VersionManager;
use crate::contracts::tools::method as method_tools;
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::executor::Executor;
use crate::types::block_number::{BlockTag, Tag};
use crate::types::reserved_addresses;

use cita_types::{Address, H256, U256};
use ethabi::{decode, ParamType, Token};
use num::FromPrimitive;

lazy_static! {
    static ref DELAY_BLOCK_NUMBER: Vec<u8> = method_tools::encode_to_vec(b"getDelayBlockNumber()");
    static ref CALL_PERMISSION_CHECK: Vec<u8> =
        method_tools::encode_to_vec(b"getPermissionCheck()");
    static ref PERMISSION_SEND_TX_CHECK: Vec<u8> =
        method_tools::encode_to_vec(b"getSendTxPermissionCheck()");
    static ref PERMISSION_CREATE_CONTRACT_CHECK: Vec<u8> =
        method_tools::encode_to_vec(b"getCreateContractPermissionCheck()");
    static ref QUOTA_CHECK: Vec<u8> = method_tools::encode_to_vec(b"getQuotaCheck()");
    static ref FEE_BACK_PLATFORM_CHECK: Vec<u8> =
        method_tools::encode_to_vec(b"getFeeBackPlatformCheck()");
    static ref CHAIN_OWNER: Vec<u8> = method_tools::encode_to_vec(b"getChainOwner()");
    static ref CHAIN_NAME: Vec<u8> = method_tools::encode_to_vec(b"getChainName()");
    static ref CHAIN_ID: Vec<u8> = method_tools::encode_to_vec(b"getChainId()");
    static ref CHAIN_ID_V1: Vec<u8> = method_tools::encode_to_vec(b"getChainIdV1()");
    static ref OPERATOR: Vec<u8> = method_tools::encode_to_vec(b"getOperator()");
    static ref WEBSITE: Vec<u8> = method_tools::encode_to_vec(b"getWebsite()");
    static ref BLOCK_INTERVAL: Vec<u8> = method_tools::encode_to_vec(b"getBlockInterval()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::SYS_CONFIG).unwrap();
    static ref ECONOMICAL_MODEL: Vec<u8> = method_tools::encode_to_vec(b"getEconomicalModel()");
    static ref GET_TOKEN_INFO: Vec<u8> = method_tools::encode_to_vec(b"getTokenInfo()");
    static ref AUTO_EXEC: Vec<u8> = method_tools::encode_to_vec(b"getAutoExec()");
}

#[derive(PartialEq, Debug)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub avatar: String,
}

#[derive(Debug, PartialEq)]
pub enum ChainId {
    V0(u32),
    V1(U256),
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
        block_tag: BlockTag,
    ) -> Result<Vec<Token>, String> {
        let address = &*CONTRACT_ADDRESS;
        let output = self
            .executor
            .call_method(address, method, None, block_tag)?;
        trace!("sys_config value output: {:?}", output);
        decode(param_types, &output).map_err(|_| "decode value error".to_string())
    }

    /// Delay block number before validate
    pub fn delay_block_number(&self, block_tag: BlockTag) -> Option<u64> {
        self.get_value(
            &[ParamType::Uint(256)],
            DELAY_BLOCK_NUMBER.as_slice(),
            block_tag,
        )
        .ok()
        .and_then(|mut x| x.remove(0).to_uint())
        .map(|x| H256::from(x).low_u64())
    }

    pub fn default_delay_block_number() -> u64 {
        info!("Use default delay block number.");
        1
    }

    /// Whether check call permission or not
    pub fn call_permission_check(&self, block_tag: BlockTag) -> Option<bool> {
        self.get_value(
            &[ParamType::Bool],
            CALL_PERMISSION_CHECK.as_slice(),
            block_tag,
        )
        .ok()
        .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_call_permission_check() -> bool {
        info!("Use default permission check.");
        false
    }

    pub fn send_tx_permission_check(&self, block_tag: BlockTag) -> Option<bool> {
        self.get_value(
            &[ParamType::Bool],
            PERMISSION_SEND_TX_CHECK.as_slice(),
            block_tag,
        )
        .ok()
        .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_send_tx_permission_check() -> bool {
        info!("Use default send tx permission check.");
        false
    }

    pub fn create_contract_permission_check(&self, block_tag: BlockTag) -> Option<bool> {
        self.get_value(
            &[ParamType::Bool],
            PERMISSION_CREATE_CONTRACT_CHECK.as_slice(),
            block_tag,
        )
        .ok()
        .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_create_contract_permission_check() -> bool {
        info!("Use default create contract permission check.");
        false
    }

    /// Whether check quota or not
    pub fn quota_check(&self, block_tag: BlockTag) -> Option<bool> {
        self.get_value(&[ParamType::Bool], QUOTA_CHECK.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_quota_check() -> bool {
        info!("Use default quota check.");
        false
    }

    /// Check fee back to platform or node
    pub fn fee_back_platform_check(&self, block_tag: BlockTag) -> Option<bool> {
        self.get_value(
            &[ParamType::Bool],
            FEE_BACK_PLATFORM_CHECK.as_slice(),
            block_tag,
        )
        .ok()
        .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_fee_back_platform_check() -> bool {
        info!("Use default fee back platform check.");
        false
    }

    /// The owner of current chain
    pub fn chain_owner(&self, block_tag: BlockTag) -> Option<Address> {
        self.get_value(&[ParamType::Address], CHAIN_OWNER.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_address())
            .map(Address::from)
    }

    pub fn default_chain_owner() -> Address {
        info!("Use default chain owner.");
        Address::from([0u8; 20])
    }

    /// The name of current chain
    pub fn chain_name(&self, block_tag: BlockTag) -> Option<String> {
        self.get_value(&[ParamType::String], CHAIN_NAME.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_string())
    }

    /// The id of current chain
    pub fn chain_id(&self, block_tag: BlockTag) -> Option<u32> {
        self.get_value(&[ParamType::Uint(64)], CHAIN_ID.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_uint())
            .map(|x| H256::from(x).low_u64() as u32)
    }

    pub fn default_chain_id() -> u32 {
        error!("Use default chain id.");
        1
    }

    /// The id v1 of current chain
    pub fn chain_id_v1(&self, block_tag: BlockTag) -> Option<U256> {
        self.get_value(&[ParamType::Uint(256)], CHAIN_ID_V1.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_uint())
            .map(U256::from)
    }

    pub fn default_chain_id_v1() -> U256 {
        error!("Use default chain id v1.");
        U256::from(1)
    }

    /// The operator of current chain
    pub fn operator(&self, block_tag: BlockTag) -> Option<String> {
        self.get_value(&[ParamType::String], OPERATOR.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_string())
    }

    /// Current operator's website URL
    pub fn website(&self, block_tag: BlockTag) -> Option<String> {
        self.get_value(&[ParamType::String], WEBSITE.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_string())
    }

    /// The interval time for creating a block (milliseconds)
    pub fn block_interval(&self, block_tag: BlockTag) -> Option<u64> {
        self.get_value(&[ParamType::Uint(64)], BLOCK_INTERVAL.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_uint())
            .map(|x| H256::from(x).low_u64())
    }

    pub fn default_block_interval() -> u64 {
        error!("Use default block interval.");
        3000
    }

    /// enum EconomicalModel { Quota, Charge }
    /// Quota: Default config is quota
    /// Charge: Charging by gas * gasPrice and reward for proposer
    pub fn economical_model(&self, block_tag: BlockTag) -> Option<EconomicalModel> {
        self.get_value(
            &[ParamType::Uint(64)],
            ECONOMICAL_MODEL.as_slice(),
            block_tag,
        )
        .ok()
        .and_then(|mut x| x.remove(0).to_uint())
        .map(|x| H256::from(x).low_u64() as u8)
        .and_then(EconomicalModel::from_u8)
    }

    pub fn default_economical_model() -> EconomicalModel {
        error!("Use default economical model.");
        EconomicalModel::Quota
    }

    pub fn token_info(&self, block_tag: BlockTag) -> Option<TokenInfo> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                GET_TOKEN_INFO.as_slice(),
                None,
                block_tag,
            )
            .ok()
            .and_then(|output| {
                decode(
                    &[ParamType::String, ParamType::String, ParamType::String],
                    &output,
                )
                .ok()
            })
            .and_then(|mut token_info| {
                if token_info.len() < 3 {
                    return None;
                }
                Some((
                    token_info.remove(0),
                    token_info.remove(0),
                    token_info.remove(0),
                ))
            })
            .and_then(|(n, s, a)| n.to_string().map(|n| (n, s, a)))
            .and_then(|(n, s, a)| s.to_string().map(|s| (n, s, a)))
            .and_then(|(n, s, a)| a.to_string().map(|a| (n, s, a)))
            .map(|(name, symbol, avatar)| TokenInfo {
                name,
                symbol,
                avatar,
            })
    }

    pub fn deal_chain_id_version(&self, version_manager: &VersionManager) -> Option<ChainId> {
        let version = version_manager
            .get_version(BlockTag::Tag(Tag::Pending))
            .unwrap_or_else(VersionManager::default_version);

        if version == 0 {
            let id_v0 = self
                .chain_id(BlockTag::Tag(Tag::Pending))
                .unwrap_or_else(SysConfig::default_chain_id);

            Some(ChainId::V0(id_v0))
        } else if version < 3 {
            let id_v1 = self
                .chain_id_v1(BlockTag::Tag(Tag::Pending))
                .unwrap_or_else(SysConfig::default_chain_id_v1);

            Some(ChainId::V1(id_v1))
        } else {
            error!("unexpected version {}!", version);
            None
        }
    }

    /// Get the flag of autoExec
    pub fn auto_exec(&self, block_tag: BlockTag) -> Option<bool> {
        self.get_value(&[ParamType::Bool], AUTO_EXEC.as_slice(), block_tag)
            .ok()
            .and_then(|mut x| x.remove(0).to_bool())
    }

    pub fn default_auto_exec() -> bool {
        info!("Use the default autoEXEC.");
        false
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_logger as logger;

    use super::{EconomicalModel, SysConfig, TokenInfo};
    use crate::tests::helpers::init_executor;
    use crate::types::block_number::{BlockTag, Tag};
    use cita_types::Address;
    use std::str::FromStr;

    #[test]
    fn test_delay_block_number() {
        let executor = init_executor();

        let config = SysConfig::new(&executor);

        // Test delay block number
        let number = config
            .delay_block_number(BlockTag::Tag(Tag::Pending))
            .unwrap();
        assert_eq!(number, 1);

        // Test call permission_check
        let check_call_permission = config
            .call_permission_check(BlockTag::Tag(Tag::Pending))
            .unwrap();
        assert_eq!(check_call_permission, false);

        // Test send_tx_permission_check
        let check_send_tx_permission = config
            .send_tx_permission_check(BlockTag::Tag(Tag::Pending))
            .unwrap();
        assert_eq!(check_send_tx_permission, false);

        // Test create_contract_permission_check
        let check_create_contract_permission = config
            .create_contract_permission_check(BlockTag::Tag(Tag::Pending))
            .unwrap();
        assert_eq!(check_create_contract_permission, false);

        // Test quota_check
        let check_quota = config.quota_check(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(check_quota, false);

        // Test fee_back_platform_check
        let check_fee_back_platform = config
            .fee_back_platform_check(BlockTag::Tag(Tag::Pending))
            .unwrap();
        assert_eq!(check_fee_back_platform, false);

        // Test chain_owner
        let value = config.chain_owner(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(
            value,
            Address::from_str("0000000000000000000000000000000000000000").unwrap()
        );

        // Test chain_name
        let value = config.chain_name(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(value, "test-chain");

        // Test chain_id
        let value = config.chain_id(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(value, 1);

        // Test operator
        let value = config.operator(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(value, "test-operator");

        // Test website
        let value = config.website(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(value, "https://www.example.com");

        // Test block_interval
        let value = config.block_interval(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(value, 3000);

        // Test economical_model
        let value = config
            .economical_model(BlockTag::Tag(Tag::Pending))
            .unwrap();
        assert_eq!(value, EconomicalModel::Quota);

        // Test token info
        let value = config.token_info(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(
            value,
            TokenInfo {
                name: "CITA Test Token".to_owned(),
                symbol: "CTT".to_owned(),
                avatar: "https://cdn.cryptape.com/icon_cita.png".to_owned()
            }
        );

        // Test auto_exec
        let auto_exec = config.auto_exec(BlockTag::Tag(Tag::Pending)).unwrap();
        assert_eq!(auto_exec, false);
    }
}
