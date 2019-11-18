use cita_vm::evm::InterpreterParams;
use common_types::context::Context;
use common_types::errors::ContractError;
use std::sync::Arc;

use crate::rs_contracts::contracts::admin::{AdminContract, Admin};
use crate::rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::storage::db_trait::DataBase;
use crate::rs_contracts::storage::db_trait::DataCategory;
use crate::rs_contracts::contracts::utils::get_latest_key;

pub fn only_admin(params: &InterpreterParams, context: &Context, contracts_db: Arc<ContractsDB>) -> Result<bool, ContractError> {
    let current_height = context.block_number;

    if let Some(admin_map) = contracts_db
            .get(DataCategory::Contracts, b"admin-contract".to_vec())
            .expect("get admin map error")
        {
            let s = String::from_utf8(admin_map).expect("from vec to string error");
            let contract_map: AdminContract = serde_json::from_str(&s).unwrap();
            trace!("==> lala contract map {:?}", contract_map);
            let map_len = contract_map.contracts.len();
            trace!("==> lala contract map length {:?}", map_len);
            let keys: Vec<_> = contract_map.contracts.keys().collect();
            let latest_key = get_latest_key(current_height, keys);
            trace!("==> lala contract latest key {:?}", latest_key);

            let bin = contract_map
                .contracts
                .get(&(current_height as u64))
                .or(contract_map.contracts.get(&latest_key))
                .expect("get contract according to height error");

            let latest_admin: Admin = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest admin {:?}", latest_admin);
            return Ok(latest_admin.only_admin(params.sender));
        }
    Ok(false)
}
