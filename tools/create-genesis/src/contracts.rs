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

use cita_types::{clean_0x, Address};
use ethabi::Token;
use serde::{Deserialize, Serialize};
use tiny_keccak::keccak256;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractsData {
    #[serde(rename = "NormalContracts")]
    pub normal_contracts: NormalContracts,

    #[serde(rename = "PermissionContracts")]
    pub permission_contracts: PermissionContracts,
}

impl ContractsData {
    pub fn load_contract_list(path: &str) -> ContractsData {
        let f = File::open(path).expect("failed to open file");
        serde_yaml::from_reader(f).unwrap()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NormalContracts {
    #[serde(rename = "SysConfig")]
    pub sys_config: Info,

    #[serde(rename = "NodeManager")]
    pub node_manager: Info,

    #[serde(rename = "ChainManager")]
    pub chain_manager: Info,

    #[serde(rename = "QuotaManager")]
    pub quota_manager: Info,

    #[serde(rename = "PermissionManagement")]
    pub permission_management: Info,

    #[serde(rename = "PermissionCreator")]
    pub permission_creator: Info,

    #[serde(rename = "Authorization")]
    pub authorization: Info,

    #[serde(rename = "RoleManagement")]
    pub role_management: Info,

    #[serde(rename = "RoleCreator")]
    pub role_creator: Info,

    #[serde(rename = "Group")]
    pub group: Info,

    #[serde(rename = "GroupManagement")]
    pub group_management: Info,

    #[serde(rename = "GroupCreator")]
    pub group_creator: Info,

    #[serde(rename = "Admin")]
    pub admin: Info,

    #[serde(rename = "RoleAuth")]
    pub role_auth: Info,

    #[serde(rename = "BatchTx")]
    pub batch_tx: Info,

    #[serde(rename = "EmergencyIntervention")]
    pub emergency_intervention: Info,

    #[serde(rename = "PriceManager")]
    pub price_manager: Info,

    #[serde(rename = "VersionManager")]
    pub version_manager: Info,

    #[serde(rename = "AllGroups")]
    pub all_groups: Info,

    #[serde(rename = "AutoExec")]
    pub auto_exec: Info,
}

impl NormalContracts {
    pub fn list(&self) -> BTreeMap<&'static str, Info> {
        let mut normal_contracts = BTreeMap::new();
        normal_contracts.insert("SysConfig", self.sys_config.clone());
        normal_contracts.insert("NodeManager", self.node_manager.clone());
        normal_contracts.insert("ChainManager", self.chain_manager.clone());
        normal_contracts.insert("QuotaManager", self.quota_manager.clone());
        normal_contracts.insert("PermissionManagement", self.permission_management.clone());
        normal_contracts.insert("PermissionCreator", self.permission_creator.clone());
        normal_contracts.insert("Authorization", self.authorization.clone());
        normal_contracts.insert("RoleManagement", self.role_management.clone());
        normal_contracts.insert("RoleCreator", self.role_creator.clone());
        normal_contracts.insert("Group", self.group.clone());
        normal_contracts.insert("GroupManagement", self.group_management.clone());
        normal_contracts.insert("GroupCreator", self.group_creator.clone());
        normal_contracts.insert("Admin", self.admin.clone());
        normal_contracts.insert("RoleAuth", self.role_auth.clone());
        normal_contracts.insert("BatchTx", self.batch_tx.clone());
        normal_contracts.insert("EmergencyIntervention", self.emergency_intervention.clone());
        normal_contracts.insert("PriceManager", self.price_manager.clone());
        normal_contracts.insert("VersionManager", self.version_manager.clone());
        normal_contracts.insert("AllGroups", self.all_groups.clone());
        normal_contracts.insert("AutoExec", self.auto_exec.clone());
        normal_contracts
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Info {
    pub address: String,
    pub file: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionContracts {
    pub file: String,
    pub basic: Basic,
    pub contracts: Contracts,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Basic {
    #[serde(rename = "sendTx")]
    pub send_tx: BasicInfo,

    #[serde(rename = "createContract")]
    pub create_contract: BasicInfo,
}

impl Basic {
    pub fn list(&self) -> BTreeMap<&'static str, BasicInfo> {
        let mut basic = BTreeMap::new();
        basic.insert("sendTx", self.send_tx.clone());
        basic.insert("createContract", self.create_contract.clone());
        basic
    }

    pub fn as_params(&self, name: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        if let Some(info) = self.list().get(name) {
            tokens.push(Token::FixedBytes(String::from(name).into_bytes()));
            let mut conts = Vec::new();
            let addr = Address::from_str(clean_0x(&info.address)).unwrap();
            conts.push(Token::Address(addr));
            let mut funcs = Vec::new();
            funcs.push(Token::FixedBytes(vec![0, 0, 0, 0]));

            tokens.push(Token::Array(conts));
            tokens.push(Token::Array(funcs));
        }
        tokens
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicInfo {
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contracts {
    #[serde(rename = "newPermission")]
    pub new_permission: ContractsInfo,

    #[serde(rename = "deletePermission")]
    pub delete_permission: ContractsInfo,

    #[serde(rename = "updatePermission")]
    pub update_permission: ContractsInfo,

    #[serde(rename = "setAuth")]
    pub set_auth: ContractsInfo,

    #[serde(rename = "cancelAuth")]
    pub cancel_auth: ContractsInfo,

    #[serde(rename = "newRole")]
    pub new_role: ContractsInfo,

    #[serde(rename = "deleteRole")]
    pub delete_role: ContractsInfo,

    #[serde(rename = "updateRole")]
    pub update_role: ContractsInfo,

    #[serde(rename = "setRole")]
    pub set_role: ContractsInfo,

    #[serde(rename = "cancelRole")]
    pub cancel_role: ContractsInfo,

    #[serde(rename = "newGroup")]
    pub new_group: ContractsInfo,

    #[serde(rename = "deleteGroup")]
    pub delete_group: ContractsInfo,

    #[serde(rename = "updateGroup")]
    pub update_group: ContractsInfo,

    #[serde(rename = "newNode")]
    pub new_node: ContractsInfo,

    #[serde(rename = "deleteNode")]
    pub delete_node: ContractsInfo,

    #[serde(rename = "updateNode")]
    pub update_node: ContractsInfo,

    #[serde(rename = "accountQuota")]
    pub account_quota: ContractsInfo,

    #[serde(rename = "blockQuota")]
    pub block_quota: ContractsInfo,

    #[serde(rename = "batchTx")]
    pub batch_tx: ContractsInfo,

    #[serde(rename = "emergencyIntervention")]
    pub emergency_intervention: ContractsInfo,

    #[serde(rename = "quotaPrice")]
    pub quota_price: ContractsInfo,

    #[serde(rename = "version")]
    pub version: ContractsInfo,
}

impl Contracts {
    pub fn list(&self) -> BTreeMap<&'static str, ContractsInfo> {
        let mut contracts = BTreeMap::new();
        contracts.insert("newPermission", self.new_permission.clone());
        contracts.insert("deletePermission", self.delete_permission.clone());
        contracts.insert("updatePermission", self.update_permission.clone());
        contracts.insert("setAuth", self.set_auth.clone());
        contracts.insert("cancelAuth", self.cancel_auth.clone());
        contracts.insert("newRole", self.new_role.clone());
        contracts.insert("deleteRole", self.delete_role.clone());
        contracts.insert("updateRole", self.update_role.clone());
        contracts.insert("setRole", self.set_role.clone());
        contracts.insert("cancelRole", self.cancel_role.clone());
        contracts.insert("newGroup", self.new_group.clone());
        contracts.insert("deleteGroup", self.delete_group.clone());
        contracts.insert("updateGroup", self.update_group.clone());
        contracts.insert("newNode", self.new_node.clone());
        contracts.insert("deleteNode", self.delete_node.clone());
        contracts.insert("updateNode", self.update_node.clone());
        contracts.insert("accountQuota", self.account_quota.clone());
        contracts.insert("blockQuota", self.block_quota.clone());
        contracts.insert("batchTx", self.batch_tx.clone());
        contracts.insert("emergencyIntervention", self.emergency_intervention.clone());
        contracts.insert("quotaPrice", self.quota_price.clone());
        contracts.insert("version", self.version.clone());
        contracts
    }

    pub fn as_params(
        &self,
        normal_contracts: &NormalContracts,
        name: &str,
    ) -> BTreeMap<String, String> {
        let mut params = BTreeMap::new();
        if let Some(info) = self.list().get(name) {
            let (conts, funcs) = info.get_contract_info(normal_contracts);
            println!("===> conts {:?} funcs {:?}", conts, funcs);
            for i in 0..conts.len() {
                params.insert(
                    funcs.get(i).unwrap().to_string(),
                    conts.get(i).unwrap().to_string(),
                );
            }
            params.insert("perm_name".to_string(), name.to_string());
        }
        params
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractsInfo {
    pub address: String,
    pub contracts: Vec<String>,
    pub functions: Vec<String>,
}

impl ContractsInfo {
    pub fn get_contract_info(
        &self,
        normal_contracts: &NormalContracts,
    ) -> (Vec<String>, Vec<String>) {
        let mut conts = Vec::new();
        let mut funcs = Vec::new();
        let reference = normal_contracts.list();
        for i in 0..self.contracts.len() {
            let contract_info = &reference[self.contracts.get(i).unwrap().as_str()];
            let address = clean_0x(&contract_info.address);

            conts.push(address.to_string());
            funcs.push(self.functions.get(i).unwrap().to_string());
        }

        // for f in self.functions.iter() {
        //     let func = keccak256(f.as_bytes()).to_vec();
        //     funcs.push(Token::FixedBytes(func[0..4].to_vec()));
        // }
        (conts, funcs)
    }
}
#[cfg(test)]
mod tests {
    use super::ContractsData;

    #[test]
    pub fn test_contracts() {
        let contracts_string = r#"
        NormalContracts:
            SysConfig:
                address: '0xffffffffffffffffffffffffffffffffff020000'
                file: system/SysConfig.sol
            NodeManager:
                address: '0xffffffffffffffffffffffffffffffffff020001'
                file: system/NodeManager.sol
            ChainManager:
                address: '0xffffffffffffffffffffffffffffffffff020002'
                file: system/ChainManager.sol
            QuotaManager:
                address: '0xffffffffffffffffffffffffffffffffff020003'
                file: system/QuotaManager.sol
            PermissionManagement:
                address: '0xffffffffffffffffffffffffffffffffff020004'
                file: permission_management/PermissionManagement.sol
            PermissionCreator:
                address: '0xffffffffffffffffffffffffffffffffff020005'
                file: permission_management/PermissionCreator.sol
            Authorization:
                address: '0xffffffffffffffffffffffffffffffffff020006'
                file: permission_management/Authorization.sol
            RoleManagement:
                address: '0xffffffffffffffffffffffffffffffffff020007'
                file: role_management/RoleManagement.sol
            RoleCreator:
                address: '0xffffffffffffffffffffffffffffffffff020008'
                file: role_management/RoleCreator.sol
            Group:
                address: '0xffffffffffffffffffffffffffffffffff020009'
                file: user_management/Group.sol
            GroupManagement:
                address: '0xffffffffffffffffffffffffffffffffff02000a'
                file: user_management/GroupManagement.sol
            GroupCreator:
                address: '0xffffffffffffffffffffffffffffffffff02000b'
                file: user_management/GroupCreator.sol
            Admin:
                address: '0xffffffffffffffffffffffffffffffffff02000c'
                file: common/Admin.sol
            RoleAuth:
                address: '0xffffffffffffffffffffffffffffffffff02000d'
                file: role_management/RoleAuth.sol
            BatchTx:
                address: '0xffffffffffffffffffffffffffffffffff02000e'
                file: system/BatchTx.sol
            EmergencyIntervention:
                address: '0xffffffffffffffffffffffffffffffffff02000f'
                file: system/EmergencyIntervention.sol
            PriceManager:
                address: '0xffffffffffffffffffffffffffffffffff020010'
                file: system/PriceManager.sol
            VersionManager:
                address: '0xffffffffffffffffffffffffffffffffff020011'
                file: system/VersionManager.sol
            AllGroups:
                address: '0xffffffffffffffffffffffffffffffffff020012'
                file: user_management/AllGroups.sol
            AutoExec:
                address: '0xffffffffffffffffffffffffffffffffff020013'
                file: system/AutoExec.sol

        PermissionContracts:
            file: permission_management/Permission.sol
            basic:
                sendTx:
                    address: '0xffffffffffffffffffffffffffffffffff021000'
                createContract:
                    address: '0xffffffffffffffffffffffffffffffffff021001'
            contracts:
                newPermission:
                    address: '0xffffffffffffffffffffffffffffffffff021010'
                    contracts:
                        - PermissionManagement
                    functions:
                        - 'newPermission(bytes32,address[],bytes4[])'
                deletePermission:
                    address: '0xffffffffffffffffffffffffffffffffff021011'
                    contracts:
                        - PermissionManagement
                    functions:
                        - 'deletePermission(address)'
                updatePermission:
                    address: '0xffffffffffffffffffffffffffffffffff021012'
                    contracts:
                        - PermissionManagement
                        - PermissionManagement
                        - PermissionManagement
                    functions:
                        - 'addResources(address,address[],bytes4[])'
                        - 'deleteResources(address,address[],bytes4[])'
                        - 'updatePermissionName(address,bytes32)'
                setAuth:
                    address: '0xffffffffffffffffffffffffffffffffff021013'
                    contracts:
                        - PermissionManagement
                        - PermissionManagement
                    functions:
                        - 'setAuthorization(address,address)'
                        - 'setAuthorizations(address,address[])'
                cancelAuth:
                    address: '0xffffffffffffffffffffffffffffffffff021014'
                    contracts:
                        - PermissionManagement
                        - PermissionManagement
                        - PermissionManagement
                    functions:
                       - 'cancelAuthorization(address,address)'
                       - 'clearAuthorization(address)'
                       - 'cancelAuthorizations(address,address[])'
                newRole:
                    address: '0xffffffffffffffffffffffffffffffffff021015'
                    contracts:
                        - RoleManagement
                    functions:
                        - 'newRole(bytes32,address[])'
                deleteRole:
                    address: '0xffffffffffffffffffffffffffffffffff021016'
                    contracts:
                        - RoleManagement
                    functions:
                        - 'deleteRole(address)'
                updateRole:
                    address: '0xffffffffffffffffffffffffffffffffff021017'
                    contracts:
                        - RoleManagement
                        - RoleManagement
                        - RoleManagement
                    functions:
                        - 'addPermissions(address,address[])'
                        - 'deletePermissions(address,address[])'
                        - 'updateRoleName(address,bytes32)'
                setRole:
                    address: '0xffffffffffffffffffffffffffffffffff021018'
                    contracts:
                        - RoleManagement
                    functions:
                        - 'setRole(address,address)'
                cancelRole:
                    address: '0xffffffffffffffffffffffffffffffffff021019'
                    contracts:
                        - RoleManagement
                        - RoleManagement
                    functions:
                        - 'cancelRole(address,address)'
                        - 'clearRole(address)'
                newGroup:
                    address: '0xffffffffffffffffffffffffffffffffff02101a'
                    contracts:
                        - GroupManagement
                    functions:
                        - 'newGroup(address,bytes32,address[])'
                deleteGroup:
                    address: '0xffffffffffffffffffffffffffffffffff02101b'
                    contracts:
                        - GroupManagement
                    functions:
                        - 'deleteGroup(address,address)'
                updateGroup:
                    address: '0xffffffffffffffffffffffffffffffffff02101c'
                    contracts:
                        - GroupManagement
                        - GroupManagement
                        - GroupManagement
                    functions:
                        - 'addAccounts(address,address,address[])'
                        - 'deleteAccounts(address,address,address[])'
                        - 'updateGroupName(address,address,bytes32)'
                newNode:
                    address: '0xffffffffffffffffffffffffffffffffff021020'
                    contracts:
                        - NodeManager
                    functions:
                        - 'approveNode(address)'
                deleteNode:
                    address: '0xffffffffffffffffffffffffffffffffff021021'
                    contracts:
                        - NodeManager
                    functions:
                        - 'deleteNode(address)'
                updateNode:
                    address: '0xffffffffffffffffffffffffffffffffff021022'
                    contracts:
                        - NodeManager
                    functions:
                        - 'setStake(address,uint64)'
                accountQuota:
                    address: '0xffffffffffffffffffffffffffffffffff021023'
                    contracts:
                        - QuotaManager
                        - QuotaManager
                    functions:
                        - 'setDefaultAQL(uint256)'
                        - 'setAQL(address,uint256)'
                blockQuota:
                    address: '0xffffffffffffffffffffffffffffffffff021024'
                    contracts:
                        - QuotaManager
                    functions:
                        - 'setBQL(uint256)'
                batchTx:
                    address: '0xffffffffffffffffffffffffffffffffff021025'
                    contracts:
                        - BatchTx
                    functions:
                        - 'multiTxs(bytes)'
                emergencyIntervention:
                    address: '0xffffffffffffffffffffffffffffffffff021026'
                    contracts:
                        - EmergencyIntervention
                    functions:
                        - 'setState(bool)'
                quotaPrice:
                    address: '0xffffffffffffffffffffffffffffffffff021027'
                    contracts:
                        - PriceManager
                    functions:
                        - 'setQuotaPrice(uint256)'
                version:
                    address: '0xffffffffffffffffffffffffffffffffff021028'
                    contracts:
                        - VersionManager
                    functions:
                        - 'setVersion(uint32)'
                  "#;
        let contracts: ContractsData = serde_yaml::from_str(&contracts_string).unwrap();
        assert_eq!(
            contracts.normal_contracts.sys_config.address,
            String::from("0xffffffffffffffffffffffffffffffffff020000")
        );
        assert_eq!(
            contracts.normal_contracts.sys_config.file,
            String::from("system/SysConfig.sol")
        );
        assert_eq!(
            contracts.normal_contracts.auto_exec.address,
            String::from("0xffffffffffffffffffffffffffffffffff020013")
        );
        assert_eq!(
            contracts.normal_contracts.auto_exec.file,
            String::from("system/AutoExec.sol")
        );

        assert_eq!(
            contracts.permission_contracts.file,
            String::from("permission_management/Permission.sol")
        );
        assert_eq!(
            contracts.permission_contracts.basic.send_tx.address,
            String::from("0xffffffffffffffffffffffffffffffffff021000")
        );

        assert_eq!(
            contracts
                .permission_contracts
                .contracts
                .new_permission
                .address,
            String::from("0xffffffffffffffffffffffffffffffffff021010")
        );
    }
}
