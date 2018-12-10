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

use super::executor::Executor;
use cita_types::{Address, U256};
use contracts::solc::{
    AccountQuotaLimit, EmergencyBrake, NodeManager, PermissionManagement, PriceManagement,
    QuotaManager, Resource, SysConfig, UserManagement, VersionManager, AUTO_EXEC_QL_VALUE,
};
use libexecutor::economical_model::EconomicalModel;
use std::collections::HashMap;
use types::ids::BlockId;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GlobalSysConfig {
    pub nodes: Vec<Address>,
    pub validators: Vec<Address>,
    pub block_quota_limit: usize,
    pub delay_active_interval: usize,
    pub changed_height: usize,
    /// Interval time for creating a block (milliseconds)
    pub block_interval: u64,
    pub emergency_brake: bool,
    pub chain_version: u32,
    pub block_sys_config: BlockSysConfig,
}

impl Default for GlobalSysConfig {
    fn default() -> Self {
        GlobalSysConfig {
            nodes: Vec::new(),
            validators: Vec::new(),
            block_quota_limit: 18_446_744_073_709_551_615,
            delay_active_interval: 1,
            changed_height: 0,
            block_interval: 3000,
            emergency_brake: false,
            chain_version: 0,
            block_sys_config: BlockSysConfig::default(),
        }
    }
}

impl GlobalSysConfig {
    // TODO We have to update all default value when they was changed in .sol files.
    // Is there any better solution?
    pub fn load(executor: &Executor, block_id: BlockId) -> Self {
        let mut conf = GlobalSysConfig::default();
        conf.nodes = executor
            .node_manager()
            .shuffled_stake_nodes(block_id)
            .unwrap_or_else(NodeManager::default_shuffled_stake_nodes);

        conf.validators = executor
            .node_manager()
            .nodes(block_id)
            .unwrap_or_else(NodeManager::default_shuffled_stake_nodes);

        let quota_manager = QuotaManager::new(executor);
        conf.block_quota_limit = quota_manager
            .block_quota_limit(block_id)
            .unwrap_or_else(QuotaManager::default_block_quota_limit)
            as usize;
        conf.block_sys_config.auto_exec_quota_limit = quota_manager
            .auto_exec_quota_limit(block_id)
            .unwrap_or_else(QuotaManager::default_auto_exec_quota_limit);
        let sys_config = SysConfig::new(executor);
        conf.delay_active_interval = sys_config
            .delay_block_number(block_id)
            .unwrap_or_else(SysConfig::default_delay_block_number)
            as usize;
        conf.block_sys_config.check_options.call_permission = sys_config
            .call_permission_check(block_id)
            .unwrap_or_else(SysConfig::default_call_permission_check);
        conf.block_sys_config.check_options.send_tx_permission = sys_config
            .send_tx_permission_check(block_id)
            .unwrap_or_else(SysConfig::default_send_tx_permission_check);
        conf.block_sys_config
            .check_options
            .create_contract_permission = sys_config
            .create_contract_permission_check(block_id)
            .unwrap_or_else(SysConfig::default_create_contract_permission_check);
        conf.block_sys_config.check_options.quota = sys_config
            .quota_check(block_id)
            .unwrap_or_else(SysConfig::default_quota_check);
        conf.block_sys_config.check_options.fee_back_platform = sys_config
            .fee_back_platform_check(block_id)
            .unwrap_or_else(SysConfig::default_fee_back_platform_check);
        conf.block_sys_config.chain_owner = sys_config
            .chain_owner(block_id)
            .unwrap_or_else(SysConfig::default_chain_owner);
        conf.block_interval = sys_config
            .block_interval(block_id)
            .unwrap_or_else(SysConfig::default_block_interval);
        conf.block_sys_config.auto_exec = sys_config
            .auto_exec(block_id)
            .unwrap_or_else(SysConfig::default_auto_exec);

        let permission_manager = PermissionManagement::new(executor);
        conf.block_sys_config.account_permissions =
            permission_manager.load_account_permissions(block_id);
        conf.block_sys_config.super_admin_account =
            permission_manager.get_super_admin_account(block_id);

        let user_manager = UserManagement::new(executor);
        conf.block_sys_config.group_accounts = user_manager.load_group_accounts(block_id);
        conf.block_sys_config.economical_model = sys_config
            .economical_model(block_id)
            .unwrap_or_else(SysConfig::default_economical_model);

        let common_quota_limit = quota_manager
            .account_quota_limit(block_id)
            .unwrap_or_else(QuotaManager::default_account_quota_limit);
        let specific = quota_manager.specific(block_id);

        conf.block_sys_config
            .account_quota_limit
            .set_common_quota_limit(common_quota_limit);
        conf.block_sys_config
            .account_quota_limit
            .set_specific_quota_limit(specific);
        conf.changed_height = executor.get_current_height() as usize;

        let emergency_manager = EmergencyBrake::new(executor);
        conf.emergency_brake = emergency_manager
            .state(block_id)
            .unwrap_or_else(EmergencyBrake::default_state);

        let version_manager = VersionManager::new(executor);
        conf.chain_version = version_manager
            .get_version(block_id)
            .unwrap_or_else(VersionManager::default_version);

        let price_management = PriceManagement::new(executor);
        conf.block_sys_config.quota_price = price_management
            .quota_price(block_id)
            .unwrap_or_else(PriceManagement::default_quota_price);

        conf
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Copy)]
pub struct CheckOptions {
    pub call_permission: bool,
    pub quota: bool,
    pub fee_back_platform: bool,
    pub send_tx_permission: bool,
    pub create_contract_permission: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlockSysConfig {
    pub account_quota_limit: AccountQuotaLimit,
    pub chain_owner: Address,
    pub auto_exec_quota_limit: u64,
    pub auto_exec: bool,
    pub quota_price: U256,
    pub super_admin_account: Option<Address>,
    pub account_permissions: HashMap<Address, Vec<Resource>>,
    pub group_accounts: HashMap<Address, Vec<Address>>,
    pub check_options: CheckOptions,
    pub economical_model: EconomicalModel,
}

impl Default for BlockSysConfig {
    fn default() -> Self {
        BlockSysConfig {
            account_quota_limit: AccountQuotaLimit::new(),
            chain_owner: Address::from(0),
            auto_exec_quota_limit: AUTO_EXEC_QL_VALUE,
            auto_exec: false,
            quota_price: PriceManagement::default_quota_price(),
            super_admin_account: None,
            account_permissions: HashMap::new(),
            group_accounts: HashMap::new(),
            check_options: CheckOptions::default(),
            economical_model: EconomicalModel::Quota,
        }
    }
}

impl BlockSysConfig {
    pub fn exempt_checking(&mut self) {
        self.check_options = CheckOptions::default();
    }
}
