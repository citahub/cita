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

use std::collections::BTreeMap;
use std::fs::File;
use std::str::FromStr;

use cita_types::{clean_0x, Address, U256};
use ethabi::Token;
use serde::{Deserialize, Serialize};

pub trait GetParams {
    fn as_params(&self) -> Vec<Token>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InitData {
    #[serde(rename = "Contracts")]
    pub contracts: Contracts,
}

impl InitData {
    pub fn load_contract_args(path: &str) -> InitData {
        let f = File::open(path).expect("failed to open file");
        serde_yaml::from_reader(f).unwrap()
    }

    pub fn get_params(&self) -> BTreeMap<String, Vec<Token>> {
        let mut params = BTreeMap::new();
        params.insert(
            "SysConfig".to_string(),
            self.contracts.sys_config.as_params(),
        );
        params.insert(
            "QuotaManager".to_string(),
            self.contracts.quota_manager.as_params(),
        );
        params.insert(
            "NodeManager".to_string(),
            self.contracts.node_manager.as_params(),
        );
        params.insert(
            "ChainManager".to_string(),
            self.contracts.chain_manager.as_params(),
        );
        params.insert(
            "Authorization".to_string(),
            self.contracts.authorization.as_params(),
        );
        params.insert("Group".to_string(), self.contracts.group.as_params());
        params.insert("Admin".to_string(), self.contracts.admin.as_params());
        params.insert(
            "VersionManager".to_string(),
            self.contracts.version_manager.as_params(),
        );
        params.insert(
            "PriceManager".to_string(),
            self.contracts.price_manager.as_params(),
        );
        params
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Contracts {
    #[serde(rename = "SysConfig")]
    pub sys_config: SysConfig,

    #[serde(rename = "QuotaManager")]
    pub quota_manager: QuotaManager,

    #[serde(rename = "NodeManager")]
    pub node_manager: NodeManager,

    #[serde(rename = "ChainManager")]
    pub chain_manager: ChainManager,

    #[serde(rename = "Authorization")]
    pub authorization: Authorization,

    #[serde(rename = "Group")]
    pub group: Group,

    #[serde(rename = "Admin")]
    pub admin: Admin,

    #[serde(rename = "VersionManager")]
    pub version_manager: VersionManager,

    #[serde(rename = "PriceManager")]
    pub price_manager: PriceManager,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SysConfig {
    #[serde(rename = "delayBlockNumber")]
    pub delay_block_number: String,

    #[serde(rename = "checkCallPermission")]
    pub check_call_permission: bool,

    #[serde(rename = "checkSendTxPermission")]
    pub check_send_tx_permission: bool,

    #[serde(rename = "checkCreateContractPermission")]
    pub check_create_contract_permission: bool,

    #[serde(rename = "checkQuota")]
    pub check_quota: bool,

    #[serde(rename = "checkFeeBackPlatform")]
    pub check_fee_back_platform: bool,

    #[serde(rename = "chainOwner")]
    pub chain_owner: String,

    #[serde(rename = "chainName")]
    pub chain_name: String,

    #[serde(rename = "chainId")]
    pub chain_id: String,

    #[serde(rename = "operator")]
    pub operator: String,

    #[serde(rename = "website")]
    pub website: String,

    #[serde(rename = "blockInterval")]
    pub block_interval: String,

    #[serde(rename = "economicalModel")]
    pub economical_model: String,

    pub name: String,
    pub symbol: String,
    pub avatar: String,

    #[serde(rename = "autoExec")]
    pub auto_exec: bool,
}

impl GetParams for SysConfig {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Uint(
            U256::from_str(&self.delay_block_number).unwrap(),
        ));
        tokens.push(Token::Address(
            Address::from_str(clean_0x(&self.chain_owner)).unwrap(),
        ));
        tokens.push(Token::String(self.chain_name.clone()));
        tokens.push(Token::Uint(U256::from_str(&self.chain_id).unwrap()));
        tokens.push(Token::String(self.operator.clone()));
        tokens.push(Token::String(self.website.clone()));
        tokens.push(Token::Uint(
            U256::from_dec_str(&self.block_interval).unwrap(),
        ));
        tokens.push(Token::Uint(U256::from_str(&self.economical_model).unwrap()));
        tokens.push(Token::String(self.name.clone()));
        tokens.push(Token::String(self.symbol.clone()));
        tokens.push(Token::String(self.avatar.clone()));

        let mut flags = Vec::new();
        flags.push(Token::Bool(self.check_call_permission));
        flags.push(Token::Bool(self.check_send_tx_permission));
        flags.push(Token::Bool(self.check_create_contract_permission));
        flags.push(Token::Bool(self.check_quota));
        flags.push(Token::Bool(self.check_fee_back_platform));
        flags.push(Token::Bool(self.auto_exec));

        tokens.push(Token::Array(flags));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaManager {
    pub admin: String,
}

impl GetParams for QuotaManager {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Address(
            Address::from_str(clean_0x(&self.admin)).unwrap(),
        ));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeManager {
    pub nodes: Vec<String>,
    pub stakes: Vec<String>,
}

impl GetParams for NodeManager {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut nodes = Vec::new();
        for n in self.nodes.iter() {
            let addr = Address::from_str(clean_0x(&n)).unwrap();
            nodes.push(Token::Address(addr));
        }
        let mut stakes = Vec::new();
        for s in self.stakes.iter() {
            stakes.push(Token::Uint(U256::from_str(&s).unwrap()));
        }

        tokens.push(Token::Array(nodes));
        tokens.push(Token::Array(stakes));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChainManager {
    #[serde(rename = "parentChainId")]
    pub parent_chain_id: String,

    #[serde(rename = "parentChainAuthorities")]
    pub parent_chain_authorities: Vec<String>,
}

impl GetParams for ChainManager {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Uint(U256::from_str(&self.parent_chain_id).unwrap()));

        let mut authorities = Vec::new();
        for a in self.parent_chain_authorities.iter() {
            let addr = Address::from_str(clean_0x(a)).unwrap();
            authorities.push(Token::Address(addr));
        }
        tokens.push(Token::Array(authorities));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(rename = "superAdmin")]
    pub super_admin: String,
}

impl GetParams for Authorization {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Address(
            Address::from_str(clean_0x(&self.super_admin)).unwrap(),
        ));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub parent: String,
    pub name: String,
    pub accounts: Vec<String>,
}

impl GetParams for Group {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Address(
            Address::from_str(clean_0x(&self.parent)).unwrap(),
        ));
        tokens.push(Token::FixedBytes(self.name.clone().into_bytes()));

        let mut accounts = Vec::new();
        for a in self.accounts.iter() {
            accounts.push(Token::Address(Address::from_str(clean_0x(&a)).unwrap()))
        }
        tokens.push(Token::Array(accounts));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin {
    pub admin: String,
}

impl GetParams for Admin {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Address(
            Address::from_str(clean_0x(&self.admin)).unwrap(),
        ));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionManager {
    pub version: String,
}

impl GetParams for VersionManager {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Uint(U256::from_str(&self.version).unwrap()));
        tokens
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PriceManager {
    #[serde(rename = "quotaPrice")]
    pub quota_price: String,
}

impl GetParams for PriceManager {
    fn as_params(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        tokens.push(Token::Uint(U256::from_dec_str(&self.quota_price).unwrap()));
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::InitData;

    #[test]
    pub fn test_params() {
        let params_string = r#"
    Contracts:
        Admin:
            admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
        Authorization:
            superAdmin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
        ChainManager:
            parentChainAuthorities: []
            parentChainId: 0
        Group:
            accounts:
                - '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
            name: rootGroup
            parent: '0x0000000000000000000000000000000000000000'
        NodeManager:
            nodes:
                - '0xdcfa10bf99d3618c5a9d08ec41b448585e45e0ee'
            stakes:
                - 0
        PriceManager:
            quotaPrice: 1000000
        QuotaManager:
            admin: '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523'
        SysConfig:
            autoExec: false
            avatar: https://cdn.cryptape.com/icon_cita.png
            blockInterval: 3000
            chainId: 1
            chainName: test-chain
            chainOwner: '0x0000000000000000000000000000000000000000'
            checkCallPermission: false
            checkCreateContractPermission: false
            checkFeeBackPlatform: false
            checkQuota: false
            checkSendTxPermission: false
            delayBlockNumber: 1
            economicalModel: 0
            name: CITA Test Token
            operator: test-operator
            symbol: CTT
            website: https://www.example.com
        VersionManager:
            version: 2"#;

        let config: InitData = serde_yaml::from_str(&params_string).unwrap();
        assert_eq!(
            config.contracts.sys_config.delay_block_number,
            "1".to_string()
        );
        assert_eq!(config.contracts.sys_config.check_call_permission, false);
        assert_eq!(
            config.contracts.sys_config.avatar,
            "https://cdn.cryptape.com/icon_cita.png".to_string()
        );
        assert_eq!(
            config.contracts.quota_manager.admin,
            String::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")
        );
        assert_eq!(
            *config.contracts.node_manager.nodes.get(0).unwrap(),
            String::from("0xdcfa10bf99d3618c5a9d08ec41b448585e45e0ee")
        );

        assert_eq!(
            config.contracts.group.parent,
            String::from("0x0000000000000000000000000000000000000000")
        );
        assert_eq!(config.contracts.group.name, String::from("rootGroup"));
        assert_eq!(config.contracts.version_manager.version, String::from("2"));
        assert_eq!(
            config.contracts.price_manager.quota_price,
            String::from("1000000")
        );
    }
}
