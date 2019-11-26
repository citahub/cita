use super::contract::Contract;
use super::utils::{extract_to_u32, get_latest_key};

use cita_types::{Address, H256};
use cita_vm::evm::{InterpreterParams, InterpreterResult, Log};
use common_types::context::Context;
use common_types::errors::ContractError;
use serde::{Deserialize, Serialize};

use crate::rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::storage::db_trait::DataBase;
use crate::rs_contracts::storage::db_trait::DataCategory;

use cita_trie::DB;
use cita_vm::state::State;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;
use tiny_keccak::keccak256;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminContract {
    pub contracts: BTreeMap<u64, Option<String>>,
}

impl Default for AdminContract {
    fn default() -> Self {
        AdminContract {
            contracts: BTreeMap::new(),
        }
    }
}

impl AdminContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = AdminContract::default();
        a.contracts.insert(0, Some(str));
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
        let contracts: AdminContract = serde_json::from_str(&str).unwrap();
        trace!("System contract admin {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(
        &self,
        current_height: u64,
        contracts_db: Arc<ContractsDB>,
    ) -> (Option<AdminContract>, Option<Admin>) {
        let mut latest_admin = Admin::default();
        let mut contract_map = AdminContract::default();

        if let Some(admin_map) = contracts_db
            .get(DataCategory::Contracts, b"admin-contract".to_vec())
            .expect("get admin map error")
        {
            let s = String::from_utf8(admin_map).expect("from vec to string error");
            contract_map = serde_json::from_str(&s).unwrap();
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

            latest_admin = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest admin {:?}", latest_admin);
            return (Some(contract_map), Some(latest_admin));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for AdminContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Admin - enter execute");
        let (contract_map, latest_admin) =
            self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_admin) {
            (Some(mut contract_map), Some(mut latest_admin)) => {
                trace!(
                    "System contracts - admin - params {:?}, input {:?}",
                    params.read_only,
                    params.input
                );

                let mut updated = false;
                let result =
                    extract_to_u32(&params.input[..]).and_then(|signature| match signature {
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
                        .contracts
                        .insert(context.block_number, Some(str));
                    let str = serde_json::to_string(&contract_map).unwrap();
                    let updated_hash = keccak256(&str.as_bytes().to_vec());
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

                    // update state
                    let _ = state
                        .borrow_mut()
                        .set_storage(
                            &params.contract.code_address,
                            H256::from(context.block_number),
                            H256::from(updated_hash),
                        )
                        .expect("state set storage error");
                }
                return result;
            }
            _ => unreachable!(),
        }
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
        let param_address = Address::from_slice(&params.input[16..36]);
        // only admin can invoke
        if self.only_admin(params.sender) {
            self.admin = param_address;
            *changed = true;

            let mut logs = Vec::new();
            let mut topics = Vec::new();
            let signature = "AdminUpdated(address,address,address)".as_bytes();
            topics.push(H256::from(keccak256(signature)));
            topics.push(H256::from(param_address));
            topics.push(H256::from(self.admin));
            topics.push(H256::from(params.sender));
            let log = Log(param_address, topics, vec![]);
            logs.push(log);

            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                logs,
            ));
        }

        Err(ContractError::Internal("Only admin can do".to_owned()))
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

    pub fn only_admin(&self, sender: Address) -> bool {
        if sender.to_vec() == self.admin.to_vec() {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_admin_seralization() {
        let admin_contract =
            Admin::init(Address::from("0x17142e6484cb72d1f1e6dca02eedf877a90e49d9"));
        let serialized = serde_json::to_string(&admin_contract).unwrap();

        let admin_deserialized: Admin = serde_json::from_str(&serialized).unwrap();
        assert_eq!(admin_contract.admin, admin_deserialized.admin);
    }

    #[test]
    fn test_get_admin() {
        let db = Arc::new(ContractsDB::new("rocksdb/test_get_admin").unwrap());
        let a = Admin::init(Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"));
        let a_json = serde_json::to_string(&a).unwrap();

        let admin_contract = AdminContract::default();
        admin_contract.init(a_json, db.clone());

        // TODO: change to more readable format, signature of "getadmin()"
        let get_admin_input = vec![248, 81, 164, 64];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = get_admin_input;
        context.block_number = 0;

        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let expected_output = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 90, 228, 86, 122, 213, 217, 251, 146,
                    188, 154, 253, 106, 101, 126, 111, 161, 58, 37, 35,
                ];
                assert_eq!(data, expected_output);
            }
            _ => unimplemented!(),
        }
    }

    #[test]
    fn test_is_admin_return_true() {
        let db = Arc::new(ContractsDB::new("rocksdb/test_is_admin_return_true").unwrap());
        let a = Admin::init(Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"));
        let a_json = serde_json::to_string(&a).unwrap();

        let admin_contract = AdminContract::default();
        admin_contract.init(a_json, db.clone());

        // "isadmin("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523")"
        let is_admin_input = vec![
            36, 215, 128, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 90, 228, 86, 122, 213, 217,
            251, 146, 188, 154, 253, 106, 101, 126, 111, 161, 58, 37, 35,
        ];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = is_admin_input;
        context.block_number = 0;

        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let output_true = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ];
                assert_eq!(data, output_true);
            }
            _ => unimplemented!(),
        }
    }

    #[test]
    fn test_is_admin_return_false() {
        let db = Arc::new(ContractsDB::new("rocksdb/test_is_admin_return_false").unwrap());
        let a = Admin::init(Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"));
        let a_json = serde_json::to_string(&a).unwrap();

        let admin_contract = AdminContract::default();
        admin_contract.init(a_json, db.clone());

        // "isadmin("0x17142e6484cb72d1f1e6dca02eedf877a90e49d9")"
        let is_admin_input = vec![
            36, 215, 128, 108, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 20, 46, 100, 132, 203, 114,
            209, 241, 230, 220, 160, 46, 237, 248, 119, 169, 14, 73, 217,
        ];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = is_admin_input;
        context.block_number = 0;

        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let output_false = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ];
                assert_eq!(data, output_false);
            }
            _ => unimplemented!(),
        }
    }

    #[test]
    fn test_update_admin() {
        let db = Arc::new(ContractsDB::new("rocksdb/test_udpate_admin").unwrap());
        let a = Admin::init(Address::from("0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523"));
        let a_json = serde_json::to_string(&a).unwrap();

        let admin_contract = AdminContract::default();
        admin_contract.init(a_json, db.clone());

        // "upateAdmin("0x17142e6484cb72d1f1e6dca02eedf877a90e49d9")"
        let update_admin_input = vec![
            28, 27, 135, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 20, 46, 100, 132, 203, 114,
            209, 241, 230, 220, 160, 46, 237, 248, 119, 169, 14, 73, 217,
        ];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = update_admin_input;
        context.block_number = 4;
        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let expected_output = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ];
                assert_eq!(data, expected_output);
            }
            _ => unimplemented!(),
        }

        // test getAdmin at height 0
        let get_admin_input = vec![248, 81, 164, 64];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = get_admin_input;
        context.block_number = 0;

        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let output = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 90, 228, 86, 122, 213, 217, 251, 146,
                    188, 154, 253, 106, 101, 126, 111, 161, 58, 37, 35,
                ];
                assert_eq!(data, output);
            }
            _ => unimplemented!(),
        }

        // test getAdmin at height 2
        let get_admin_input = vec![248, 81, 164, 64];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = get_admin_input;
        context.block_number = 2;
        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let output = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 90, 228, 86, 122, 213, 217, 251, 146,
                    188, 154, 253, 106, 101, 126, 111, 161, 58, 37, 35,
                ];
                assert_eq!(data, output);
            }
            _ => unimplemented!(),
        }

        // test getAdmin at height 4
        let get_admin_input = vec![248, 81, 164, 64];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = get_admin_input;
        context.block_number = 4;
        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let expected_output = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 20, 46, 100, 132, 203, 114, 209, 241,
                    230, 220, 160, 46, 237, 248, 119, 169, 14, 73, 217,
                ];
                assert_eq!(data, expected_output);
            }
            _ => unimplemented!(),
        }

        // test getAdmin at height 50
        let get_admin_input = vec![248, 81, 164, 64];
        let mut params = InterpreterParams::default();
        let mut context = Context::default();
        params.contract.code_address = Address::from("0xffffffffffffffffffffffffffffffffff02000c");
        params.input = get_admin_input;
        context.block_number = 50;
        let result = admin_contract
            .execute(&params, &context, db.clone())
            .unwrap();
        match result {
            InterpreterResult::Normal(data, _, _) => {
                let expected_output = vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 20, 46, 100, 132, 203, 114, 209, 241,
                    230, 220, 160, 46, 237, 248, 119, 169, 14, 73, 217,
                ];
                assert_eq!(data, expected_output);
            }
            _ => unimplemented!(),
        }
    }
}
