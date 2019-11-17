// use super::cons_error::ContractError;
use byteorder::{BigEndian, ByteOrder};
use cita_types::{Address, H256};
use common_types::errors::ContractError;

use crate::storage::db_contracts::ContractsDB;
use crate::storage::db_trait::DataBase;
use crate::storage::db_trait::DataCategory;

use tiny_keccak::keccak256;
use cita_vm::evm::InterpreterParams;
use common_types::context::Context;
use std::sync::Arc;

use crate::contracts::admin::{AdminContract, Admin};

pub fn extract_to_u32(data: &[u8]) -> Result<u32, ContractError> {
    if let Some(ref bytes4) = data.get(0..4) {
        // trace!("")
        Ok(BigEndian::read_u32(bytes4))
    // let encode = hex::encode(bytes4.to_vec());
    // Ok(encode)
    } else {
        Err(ContractError::Internal("out of gas".to_string()))
    }
}

pub fn encode_to_u32(name: &[u8]) -> u32 {
    BigEndian::read_u32(&keccak256(name)[..])
}

pub fn encode_to_vec(name: &[u8]) -> Vec<u8> {
    keccak256(name)[0..4].to_vec()
}

// keys: ordered list
pub fn get_latest_key(target: u64, keys: Vec<&u64>) -> u64 {
    if target == 0 {
        return 0;
    }

    for i in 0..keys.len() {
        if *keys[i] >= target {
            return *keys[i - 1];
        } else if i == keys.len() - 1 {
            return *keys[i];
        }
        continue;
    }
    0
}

pub fn only_admin(params: &InterpreterParams, context: &Context, contracts_db: Arc<ContractsDB>) -> Result<bool, ContractError> {
    let mut latest_admin = Admin::default();
    let mut contract_map = AdminContract::default();
    let current_height = context.block_number;

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

            latest_admin = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest admin {:?}", latest_admin);
            return Ok(latest_admin.only_admin(params.sender));
        }
    Ok(false)
}
