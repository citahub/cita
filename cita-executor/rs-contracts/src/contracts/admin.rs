use super::contract::Contract;
use super::object::VmExecParams;
use super::utils::{extract_to_u32, get_latest_key};

use cita_types::{Address, H256, U256};
use cita_vm::evm::{InterpreterParams, InterpreterResult};
// use cita_vm::evm::InterpreterResult;
use common_types::context::Context;
use common_types::errors::ContractError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::storage::db_contracts::ContractsDB;
use crate::storage::db_trait::DataBase;
use crate::storage::db_trait::DataCategory;

use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminContract {
    admin_contract: BTreeMap<u64, Option<String>>,
}

impl Default for AdminContract {
    fn default() -> Self {
        AdminContract {
            admin_contract: BTreeMap::new(),
        }
    }
}

impl AdminContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) {
        let mut a = AdminContract::default();
        a.admin_contract.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(
            DataCategory::Contracts,
            b"admin-contract".to_vec(),
            s.as_bytes().to_vec(),
        );

        // debug information
        let bin_map = contracts_db
            .get(DataCategory::Contracts, b"admin-contract".to_vec())
            .unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts = AdminContract::deserialize(str);
        trace!("System contract admin {:?} after init.", contracts);
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn deserialize(str: String) -> AdminContract {
        let a: AdminContract = serde_json::from_str(&str).unwrap();
        a
    }
}

impl Contract for AdminContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Admin - enter execute");
        let current_height = context.block_number;
        trace!("===> lala current height {:?}", current_height);
        trace!("===> lala context {:?}", context);
        let mut latest_admin = Admin::default();
        let mut contract_map = AdminContract::default();

        if let Some(admin_map) = contracts_db
            .get(DataCategory::Contracts, b"admin-contract".to_vec())
            .expect("get admin map error")
        {
            let s = String::from_utf8(admin_map).expect("from vec to string error");
            contract_map = AdminContract::deserialize(s);
            trace!("==> lala contract map {:?}", contract_map);
            let map_len = contract_map.admin_contract.len();
            trace!("==> lala contract map length {:?}", map_len);
            let keys: Vec<_> = contract_map.admin_contract.keys().collect();
            let latest_key = get_latest_key(current_height, keys);
            trace!("==> lala contract latest key {:?}", latest_key);

            let bin = contract_map
                .admin_contract
                .get(&(current_height as u64))
                .or(contract_map.admin_contract.get(&latest_key))
                .expect("get contract according to height error");

            // let bin = if contract_map.admin_contract.get(&(current_height as u64)).or()
            latest_admin = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest admin {:?}", latest_admin);
        }

        trace!(
            "System contracts - admin - params {:?}, input {:?}",
            params.read_only,
            params.input
        );
        let signature = extract_to_u32(&params.input[..]);
        trace!("signature is {:?}", signature);

        let mut updated = false;
        let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
            0xf851a440u32 => latest_admin.get_admin(),
            0x24d7806cu32 => latest_admin.is_admin(params),
            0x1c1b8772u32 => latest_admin.update(params, &mut updated),
            _ => panic!("Invalid function signature".to_owned()),
        });

        // update contract db
        if result.is_ok() && updated {
            let new_admin = latest_admin;
            let str = serde_json::to_string(&new_admin).unwrap();
            contract_map
                .admin_contract
                .insert(context.block_number, Some(str));
            let str = serde_json::to_string(&contract_map).unwrap();
            let _ = contracts_db.insert(
                DataCategory::Contracts,
                b"admin-contract".to_vec(),
                str.as_bytes().to_vec(),
            );

            // debug information
            let bin_map = contracts_db
                .get(DataCategory::Contracts, b"admin-contract".to_vec())
                .unwrap();
            let str = String::from_utf8(bin_map.unwrap()).unwrap();
            let contracts: AdminContract = serde_json::from_str(&str).unwrap();
            trace!("System contract admin {:?} after update.", contracts);
        }
        return result;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    admin: Address,
}

impl Default for Admin {
    fn default() -> Self {
        Admin {
            admin: Address::default(),
        }
    }
}

impl Admin {
    pub fn init(admin: Address) -> Self {
        Admin { admin: admin }
    }

    fn get_admin(&self) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Admin - get_admin");
        return Ok(InterpreterResult::Normal(
            H256::from(self.admin).0.to_vec(),
            0,
            vec![],
        ));
    }

    fn update(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Admin - update");
        // TODO, only admin can invoke
        let param_address = Address::from_slice(&params.input[16..36]);
        self.admin = param_address;
        *changed = true;
        return Ok(InterpreterResult::Normal(
            H256::from(1).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    fn is_admin(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Admin - is_admin");
        let param_address = Address::from_slice(&params.input[16..36]);
        if param_address == self.admin {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        } else {
            return Ok(InterpreterResult::Normal(
                H256::from(0).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use super::Admin;
    use cita_types::Address;

    #[test]
    fn test_admin_seralization() {
        let admin_contract =
            Admin::init(Address::from("0x17142e6484cb72d1f1e6dca02eedf877a90e49d9"));
        let serialized = serde_json::to_string(&admin_contract).unwrap();

        let admin_deserialized: Admin = serde_json::from_str(&serialized).unwrap();
        assert_eq!(admin_contract.admin, admin_deserialized.admin);
    }
}
