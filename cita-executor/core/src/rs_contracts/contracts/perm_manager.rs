use super::check;
use super::utils::{extract_to_u32, get_latest_key};

use cita_types::{Address, H256, U256};
use cita_vm::evm::{InterpreterParams, InterpreterResult, Log};
use common_types::context::Context;
use common_types::errors::ContractError;
use serde::{Deserialize, Serialize};

use super::contract::Contract;
use crate::rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::storage::db_trait::DataBase;
use crate::rs_contracts::storage::db_trait::DataCategory;
use crate::rs_contracts::contracts::build_in_perm;
use crate::rs_contracts::contracts::perm::Permission;

use std::collections::BTreeMap;
use std::sync::Arc;
use tiny_keccak::keccak256;
use std::cell::RefCell;
use cita_vm::state::State;
use cita_trie::DB;
use std::collections::HashSet;
use ethabi::encode;
use ethabi::Token;
use ethabi::param_type::ParamType;

use crate::cita_executive::create_address_from_address_and_nonce;
use common_types::reserved_addresses;

pub type FuncSig = [u8; 4];

#[derive(Serialize, Deserialize, Debug)]
pub struct PermStore {
    // key -> height, value -> json(PermissionManager)
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for PermStore {
    fn default() -> PermStore {
        PermStore {
            contracts: BTreeMap::new(),
        }
    }
}

impl PermStore {

    pub fn init(&self, admin: Address,  perm_contracts: BTreeMap<Address, String>, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut perm_store = PermStore::default();

        let mut perm_manager = PermManager::default();
        for (addr, contract) in perm_contracts.iter() {
            let p: Permission = serde_json::from_str(&contract).unwrap();
            perm_manager.perm_collection.insert(*addr, p);
        }

        let mut account_own_perms = BTreeMap::new();
        let mut super_admin_perms = HashSet::new();
        for p in build_in_perm::BUILD_IN_PERMS.iter() {
            super_admin_perms.insert(Address::from(*p));
        }
        account_own_perms.insert(admin, super_admin_perms);
        let mut group_perms = HashSet::new();
        group_perms.insert(Address::from(build_in_perm::SEND_TX_ADDRESS));
        group_perms.insert(Address::from(build_in_perm::CREATE_CONTRACT_ADDRESS));
        account_own_perms.insert(Address::from(build_in_perm::ROOT_GROUP_ADDRESS), group_perms);
        perm_manager.account_own_perms = account_own_perms;


        let str = serde_json::to_string(&perm_manager).unwrap();
        perm_store.contracts.insert(0, Some(str));

        let s = serde_json::to_string(&perm_store).unwrap();
        let _ = contracts_db.insert(DataCategory::Contracts, b"permission-contract".to_vec(), s.as_bytes().to_vec());

        // debug info
        let perm_store_bin = contracts_db.get(DataCategory::Contracts, b"permission-contract".to_vec()).unwrap();
        let perm_store_str = String::from_utf8(perm_store_bin.unwrap()).unwrap();
        let perm_store: PermStore = serde_json::from_str(&perm_store_str).unwrap();

        // let perm_manager_str = perm_store.contracts.get(&0).unwrap();
        // let perm_manager: PermManager = serde_json::from_str(&*perm_manager_str.unwrap()).unwrap();
        trace!("System contract permission {:?} after init.", perm_store);
        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(&self, current_height: u64, contracts_db: Arc<ContractsDB>) -> (Option<PermStore>, Option<PermManager>) {
        let mut latest_perm_manager = PermManager::default();
        let mut contract_map = PermStore::default();

        if let Some(perm_store) = contracts_db.get(DataCategory::Contracts, b"permission-contract".to_vec()).expect("get permission error") {
            let s = String::from_utf8(perm_store).expect("from vec to string error");
            contract_map = serde_json::from_str(&s).unwrap();
            trace!("==> lala contract map {:?}", contract_map);

            let map_len = contract_map.contracts.len();
            trace!("==> lala contract map length {:?}", map_len);
            let keys: Vec<_> = contract_map.contracts.keys().collect();
            let latest_key = get_latest_key(current_height, keys);
            trace!("==> lala contract latest key {:?}", latest_key);

            let bin = contract_map.contracts
                .get(&(current_height as u64))
                .or(contract_map.contracts.get(&latest_key))
                .expect("get contract according to height error");

            latest_perm_manager = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest permission {:?}", latest_perm_manager);
            return (Some(contract_map), Some(latest_perm_manager));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for PermStore {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission - enter execute");
        let (contract_map, latest_permission_manager) = self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_permission_manager) {
            (Some(mut contract_map), Some(mut latest_permission_manager)) => {
                trace!("System contracts - permission - params input {:?}", params.input);
                let mut updated = false;
                let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                    0xfc4a089c => latest_permission_manager.new_permission(params, &mut updated, context, contracts_db.clone()),
                    0x98a05bb1 => latest_permission_manager.del_permission(params, &mut updated, context, contracts_db.clone()),
                    0x537bf9a3 => latest_permission_manager.update_permission_name(params, &mut updated, context, contracts_db.clone()),
                    0xf036ed56 => latest_permission_manager.add_permission_resources(params, &mut updated, context, contracts_db.clone()),
                    0x6446ebd8 => latest_permission_manager.del_permission_resources(params, &mut updated, context, contracts_db.clone()),
                    0x52c5b4cc => latest_permission_manager.set_authorizations(params, &mut updated, context, contracts_db.clone()),
                    0x0f5aa9f3 => latest_permission_manager.set_authorization(params, &mut updated, context, contracts_db.clone()),
                    0xba00ab60 => latest_permission_manager.cancel_authorizations(params, &mut updated, context, contracts_db.clone()),
                    0x3482e0c9 => latest_permission_manager.cancel_authorization(params, &mut updated, context, contracts_db.clone()),
                    0xa5925b5b => latest_permission_manager.clear_authorization(params, &mut updated, context, contracts_db.clone()),
                    0x1a160fe9 => latest_permission_manager.check_permission(params),
                    0xde6afd60 => latest_permission_manager.check_resource(params),
                    0x945a2555 => latest_permission_manager.query_permssions(params),
                    0xd28d4e0c => latest_permission_manager.query_all_accounts(params),
                    0xe286599b => latest_permission_manager.query_resource(params),
                    0xdff7eafe => latest_permission_manager.query_name(params),
                    0x9795e7e0 => latest_permission_manager.in_permission(params),
                    _ => panic!("Invalid function signature {} ", signature),
                });

                if result.is_ok() & updated {
                    let new_perm_manager = latest_permission_manager;
                    let str = serde_json::to_string(&new_perm_manager).unwrap();
                    contract_map.contracts.insert(context.block_number, Some(str));
                    let str = serde_json::to_string(&contract_map).unwrap();
                    let updated_hash = keccak256(&str.as_bytes().to_vec());
                    let _ = contracts_db.insert(DataCategory::Contracts, b"permission-contract".to_vec(), str.as_bytes().to_vec());

                    // debug information, can be ommited
                    let bin_map = contracts_db.get(DataCategory::Contracts, b"permission-contract".to_vec()).unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: PermStore = serde_json::from_str(&str).unwrap();
                    trace!("System contract permission {:?} after update.", contracts);

                    // update state
                    let _ = state.borrow_mut().set_storage(&params.contract.code_address,
                        H256::from(context.block_number), H256::from(updated_hash)).expect("state set storage error");
                }
                return result;
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PermManager {
    pub perm_collection: BTreeMap<Address, Permission>,
    pub account_own_perms: BTreeMap<Address, HashSet<Address>>,
}

impl PermManager {

    pub fn new(super_admin: Address) -> Self {
        let mut account_own_perms = BTreeMap::new();

        let mut super_admin_perms = HashSet::new();
        for p in build_in_perm::BUILD_IN_PERMS.iter() {
            super_admin_perms.insert(Address::from(*p));
        }
        account_own_perms.insert(super_admin, super_admin_perms);

        let mut group_perms = HashSet::new();
        group_perms.insert(Address::from(build_in_perm::SEND_TX_ADDRESS));
        group_perms.insert(Address::from(build_in_perm::CREATE_CONTRACT_ADDRESS));
        account_own_perms.insert(Address::from(build_in_perm::ROOT_GROUP_ADDRESS), group_perms);

        PermManager {
            account_own_perms,
            perm_collection: BTreeMap::new(),
        }
    }

    pub fn new_permission(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - new permission");

        let tokens = vec![
            ParamType::FixedBytes(32),
            ParamType::Array(Box::new(ParamType::Address)),
            ParamType::Array(Box::new(ParamType::FixedBytes(4)))
        ];
        if let Ok(params) = ethabi::decode(&tokens, &params.input[4..]) {
            match (&params[0], &params[1], &params[2]) {
                (Token::FixedBytes(name), Token::Array(addrs), Token::Array(funcs)) => {
                    let perm_name = H256::from_slice(name);
                    let perm_addrs = addrs.iter().map(|i| match i {
                        Token::Address(x) => Address::from_slice(x),
                        _ => unreachable!(),
                    }).collect::<Vec<Address>>();
                    let perm_funcs = funcs.iter().map(|i| match i {
                        Token::FixedBytes(x) => x.clone(),
                        _ => unreachable!(),
                    }).collect::<Vec<_>>();
                    trace!("perm_name {:?}", perm_name);
                    trace!("perm_addrs {:?}", perm_addrs);
                    trace!("perm_funcs {:?}", perm_funcs);

                    let perm = Permission::new(perm_name.to_string(), perm_addrs, perm_funcs);
                    let perm_address = create_address_from_address_and_nonce(
                        &Address::from(reserved_addresses::PERMISSION_CREATOR), &U256::zero());
                    trace!("perm address created in new_permission {:?}", perm_address);
                    self.perm_collection.insert(perm_address, perm);

                    let mut logs = Vec::new();
                    let mut topics = Vec::new();
                    let signature = "newPermission(bytes32,address[],bytes4[])".as_bytes();
                    topics.push(H256::from(keccak256(signature)));
                    topics.push(H256::from(perm_address));
                    let log = Log(perm_address, topics, vec![]);
                    logs.push(log);

                    *changed = true;
                    return Ok(InterpreterResult::Normal(H256::from(perm_address).0.to_vec(), 0, logs));
                }
                _ => unimplemented!(),
            }
        }
        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn del_permission(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
            trace!("System contract - permission  - del permission");
            let perm_address = Address::from(&params.input[16..36]);
            self.perm_collection.remove(&perm_address);

            // update accounts permissions
            for (_account, perms) in self.account_own_perms.iter_mut() {
                perms.retain(|&k| k != perm_address);
            }

            *changed = true;
         return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn update_permission_name(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - update permission name");
        // 解析两个参数
        let perm_address = Address::from(&params.input[16..36]);
        let perm_name = String::from("bbb");
        if let Some(perm) = self.perm_collection.get_mut(&perm_address) {
            perm.update_name(&perm_name);
        }

        *changed = true;
        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn add_permission_resources(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - add_permission_resource");
        // 解析一个address, 两个数组
        let perm_address = Address::from(&params.input[16..36]);
        let perm_conts = Vec::new();
        let perm_funcs = Vec::new();

        if let Some(p) = self.perm_collection.get_mut(&perm_address) {
            p.add_resources(perm_conts, perm_funcs);
        }

        *changed = true;
        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn del_permission_resources(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - del_permission_resource");
        let perm_address = Address::from(&params.input[16..36]);
        let perm_conts = Vec::new();
        let perm_funcs = Vec::new();
         // 解析一个address, 两个数组

        if let Some(p) = self.perm_collection.get_mut(&perm_address) {
            p.delete_resources(perm_conts, perm_funcs);
        }

        *changed = true;

        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn set_authorizations(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - set_authorizations");
        let account = Address::from(&params.input[16..36]);
        let permissions = Vec::new();
        // 解析一个address, 一个数组

        for p in permissions.iter() {
            if let Some(perms) = self.account_own_perms.get_mut(&account) {
                perms.insert(*p);
            }
        }
        *changed = true;
        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn set_authorization(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - set_authorization");
        let account = Address::from(&params.input[16..36]);
        let permission = Address::from(&params.input[48..68]);

        if let Some(perms) = self.account_own_perms.get_mut(&account) {
            perms.insert(permission);
        }
        *changed = true;

        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn cancel_authorizations(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - cancel_authorizations");
        let account = Address::from(&params.input[16..36]);
        let permissions = Vec::new();
         // 解析一个address, 一个数组

        if let Some(perms) = self.account_own_perms.get_mut(&account) {
            for p in permissions {
                perms.remove(p);
            }
            *changed = true;
            return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
        }

        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn cancel_authorization(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - cancel_authorization");
        let account = Address::from(&params.input[16..36]);
        let permission = Address::from(&params.input[48..68]);

        if let Some(perms) = self.account_own_perms.get_mut(&account) {
            perms.remove(&permission);
            *changed = true;
            return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
        }
        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn clear_authorization(&mut self,
        params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - clear_authorization");
        let account = Address::from(&params.input[16..36]);
        if let Some(perms) = self.account_own_perms.get_mut(&account) {
           perms.clear();
           *changed = true;
           return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
        }
        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn query_permssions(&mut self,
        params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
            trace!("System contract - permission  - query_permssions");
            let param_address = Address::from_slice(&params.input[16..36]);
            if let Some(permissions) = self.account_own_perms.get(&param_address) {
                let mut perms = Vec::new();
                for p in permissions.iter() {
                    perms.push(Token::Address(p.0));
                }

                let mut tokens = Vec::new();
                tokens.push(Token::Array(perms));
                return Ok(InterpreterResult::Normal(ethabi::encode(&tokens), params.gas_limit, vec![]));
            }

            return Ok(InterpreterResult::Normal(vec![], params.gas_limit, vec![]));
    }

    pub fn query_all_accounts(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - query_permssions");
        let mut accounts = Vec::new();
        for k in self.account_own_perms.keys() {
            accounts.push(Token::Address(k.0));
        }

        let mut tokens = Vec::new();
        tokens.push(Token::Array(accounts));

        return Ok(InterpreterResult::Normal(ethabi::encode(&tokens), params.gas_limit, vec![]));
    }

    pub fn query_resource(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - query_resource");
        let perm_address = Address::from(&params.input[16..36]);
        if let Some(p) = self.perm_collection.get(&perm_address) {
            let mut tokens = Vec::new();
            let (conts, funcs) = p.query_resource();

            let mut conts_return = Vec::new();
            let mut funcs_return = Vec::new();
            for i in 0..conts.len() {
                conts_return.push(Token::Address(conts[i].0));
                funcs_return.push(Token::FixedBytes(funcs[i].clone()));
            }

            tokens.push(Token::Array(conts_return));
            tokens.push(Token::Array(funcs_return));
            return Ok(InterpreterResult::Normal(ethabi::encode(&tokens), params.gas_limit, vec![]));
        }
        return Ok(InterpreterResult::Normal(vec![], params.gas_limit, vec![]));
    }

    pub fn query_name(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - query_name");
        let perm_address = Address::from(&params.input[16..36]);
        if let Some(p) = self.perm_collection.get(&perm_address) {
            let name = p.query_name();
            let mut res = H256::from(0);
            res.clone_from_slice(&name.as_bytes());
            return Ok(InterpreterResult::Normal(res.to_vec(), params.gas_limit, vec![]));
        }
        return Ok(InterpreterResult::Normal(vec![], params.gas_limit, vec![]));
    }

    pub fn in_permission(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - in_permission");
        let perm_address = Address::from(&params.input[16..36]);
        let resource_cont = Address::from(&params.input[48..68]);
        let resource_func = &params.input[68..72];
        trace!("Check_resource, perm_address: {:?}, resource_cont: {:?}, resource_func {:?}", perm_address, resource_cont, resource_func);

        if let Some(perm) = self.perm_collection.get(&perm_address) {
            if perm.in_permission(resource_cont, resource_func.to_vec()) {
                return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
            }
        }

        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn check_resource(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - check_resource");
        let account = Address::from(&params.input[16..36]);
        let resource_cont = Address::from(&params.input[48..68]);
        let resource_func = &params.input[68..72];
        trace!("Check_resource, account: {:?}, resource_cont: {:?}, resource_func {:?}", account, resource_cont, resource_func);

        if let Some(perms_address) = self.account_own_perms.get(&account) {
            for p in perms_address.iter() {
                if let Some(permission) = self.perm_collection.get(p) {
                    if permission.in_permission(resource_cont, resource_func.to_vec()) {
                        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
                    }
                }
            }
        }

        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn check_permission(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - permission  - check_permission");
        let account = Address::from(&params.input[16..36]);
        let permission = Address::from(&params.input[48..68]);

        trace!("Check_permission, account: {:?}, permission: {:?}", account, permission);
        if let Some(perms) = self.account_own_perms.get(&account) {
            if perms.contains(&permission) {
                return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
            }
        }
        return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), params.gas_limit, vec![]));
    }
}
