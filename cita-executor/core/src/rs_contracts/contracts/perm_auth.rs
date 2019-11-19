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

use std::collections::BTreeMap;
use std::sync::Arc;
use tiny_keccak::keccak256;
use std::cell::RefCell;
use cita_vm::state::State;
use cita_trie::DB;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizationContract {
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for AuthorizationContract {
    fn default() -> AuthorizationContract {
        AuthorizationContract {
            contracts: BTreeMap::new(),
        }
    }
}

impl AuthorizationContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = AuthorizationContract::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(DataCategory::Contracts, b"authorization-contract".to_vec(), s.as_bytes().to_vec());

        // debug info
        let bin_map = contracts_db.get(DataCategory::Contracts, b"authorization-contract".to_vec()).unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: AuthorizationContract = serde_json::from_str(&str).unwrap();
        trace!("System contract authorization {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(&self, current_height: u64, contracts_db: Arc<ContractsDB>) -> (Option<AuthorizationContract>, Option<Authorization>) {
        let mut latest_auth = Authorization::default();
        let mut contract_map = AuthorizationContract::default();

        if let Some(auth_map) = contracts_db.get(DataCategory::Contracts, b"authorization-contract".to_vec()).expect("get authorizations error") {
            let s = String::from_utf8(auth_map).expect("from vec to string error");
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

            latest_auth = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest auth {:?}", latest_auth);
            return (Some(contract_map), Some(latest_auth));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for AuthorizationContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - authorization - enter execute");
        let (contract_map, latest_auth) = self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_auth) {
            (Some(mut contract_map), Some(mut latest_auth)) => {
                trace!("System contracts - auth - params input {:?}", params.input);
                let mut updated = false;
                let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                    0xf10a7798u32 => latest_auth.set_auth(params, &mut updated, context, contracts_db.clone()),
                    0x0c0a5c55u32 => latest_auth.cancel_auth(params, &mut updated, context, contracts_db.clone()),
                    0xb4026ed5u32 => latest_auth.clear_auth(params, &mut updated, context, contracts_db.clone()),
                    0x945a2555u32 => latest_auth.query_perms(params, context, contracts_db.clone()),
                    _ => panic!("Invalid function signature".to_owned()),
                });

                if result.is_ok() & updated {
                    let new_auth = latest_auth;
                    let str = serde_json::to_string(&new_auth).unwrap();
                    contract_map.contracts.insert(context.block_number, Some(str));
                    let str = serde_json::to_string(&contract_map).unwrap();
                    let updated_hash = keccak256(&str.as_bytes().to_vec());
                    let _ = contracts_db.insert(DataCategory::Contracts, b"authorization-contract".to_vec(), str.as_bytes().to_vec());

                    // debug information, can be ommited
                    let bin_map = contracts_db.get(DataCategory::Contracts, b"authorization-contract".to_vec()).unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: AuthorizationContract = serde_json::from_str(&str).unwrap();
                    trace!("System contract auth {:?} after update.", contracts);

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
pub struct Authorization {
    account_own_perms: BTreeMap<Address, HashSet<Address>>,
    // perm_own_accounts: BTreeMap<Address, Vec<Address>>,
}

impl Authorization {
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

        Authorization {
            account_own_perms,
        }
    }


    pub fn set_auth(&mut self, params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        // TODO: only PermissionManagement
        trace!("System contract - Auth - set auth");
        let account = Address::default();
        let permission = Address::default();

        if let Some(perms) = self.account_own_perms.get_mut(&account) {
            (*perms).insert(permission);
        } else {
            let mut perms = HashSet::new();
            perms.insert(permission);
            self.account_own_perms.insert(account, perms);
        }

        *changed = true;
        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn cancel_auth(&mut self, params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Auth - cancel auth");
        // TODO: only PermissionManagement
        // TODO: account can not be super admin
        let account = Address::default();
        let permission = Address::default();

        if let Some(perms) = self.account_own_perms.get_mut(&account) {
            (*perms).remove(&permission);
        } else {
            let mut perms = HashSet::new();
            perms.insert(permission);
            self.account_own_perms.insert(account, perms);
        }

        *changed = true;
        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn clear_auth(&mut self, params: &InterpreterParams, changed: &mut bool,
        _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Auth - clear auth");
        // TODO: only PermissionManagement
        // TODO: account can not be super admin
        let account = Address::default();

        if let Some(perms) = self.account_own_perms.get_mut(&account) {
            (*perms).clear();
        }

        *changed = true;
        return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
    }

    pub fn query_perms(&self, params: &InterpreterParams, _context: &Context, _contracts_db: Arc<ContractsDB>) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Auth - query_perms");
        let account = Address::from_slice(&params.input[16..36]);
        let permissions = self.account_own_perms.get(&account).unwrap_or(&HashSet::new());

        // 把这个 vec 返回去

        return Ok(InterpreterResult::Normal(vec![], 0, vec![]));
    }
}


