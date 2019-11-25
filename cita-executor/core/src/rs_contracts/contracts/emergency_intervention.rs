use super::check;
use super::utils::{extract_to_u32, get_latest_key, u256_to_bool};

use cita_types::{Address, H256, U256};
use cita_vm::evm::{InterpreterParams, InterpreterResult, Log};
use common_types::context::Context;
use common_types::errors::ContractError;
use serde::{Deserialize, Serialize};

use super::contract::Contract;
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
pub struct EmergContract {
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for EmergContract {
    fn default() -> Self {
        EmergContract {
            contracts: BTreeMap::new(),
        }
    }
}

impl EmergContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = EmergContract::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(DataCategory::Contracts, b"emerg-contract".to_vec(), s.as_bytes().to_vec());

        // debug info
        let bin_map = contracts_db.get(DataCategory::Contracts, b"emerg-contract".to_vec()).unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: EmergContract = serde_json::from_str(&str).unwrap();
        trace!("System contract emergency intervention {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(
        &self,
        current_height: u64,
        contracts_db: Arc<ContractsDB>,
    ) -> (Option<EmergContract>, Option<EmergencyIntervention>) {
        let mut latest_item = EmergencyIntervention::default();
        let mut contract_map = EmergContract::default();

        if let Some(emerg_map) = contracts_db
            .get(DataCategory::Contracts, b"emerg-contract".to_vec())
            .expect("get emerg error")
        {
            let s = String::from_utf8(emerg_map).expect("from vec to string error");
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

            latest_item = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest emerg {:?}", latest_item);
            return (Some(contract_map), Some(latest_item));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for EmergContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - emerg - enter execute");
        let (contract_map, latest_item) = self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_item) {
            (Some(mut contract_map), Some(mut latest_item)) => {
                trace!("System contracts - emerg - params input {:?}", params.input);
                let mut updated = false;
                let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                    0xc19d93fb => latest_item.get_state(),
                    0xac9f0222 => latest_item.set_state(params, &mut updated, context, contracts_db.clone()),
                    _ => panic!("Invalid function signature".to_owned()),
                });

                if result.is_ok() & updated {
                    let new_item = latest_item;
                    let str = serde_json::to_string(&new_item).unwrap();
                    contract_map.contracts.insert(context.block_number, Some(str));
                    let str = serde_json::to_string(&contract_map).unwrap();
                    let updated_hash = keccak256(&str.as_bytes().to_vec());
                    let _ =
                        contracts_db.insert(DataCategory::Contracts, b"emerg-contract".to_vec(), str.as_bytes().to_vec());

                    // debug information, can be ommited
                    let bin_map = contracts_db.get(DataCategory::Contracts, b"emerg-contract".to_vec()).unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: EmergContract = serde_json::from_str(&str).unwrap();
                    trace!("System contract emerg {:?} after update.", contracts);

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
pub struct EmergencyIntervention {
    state: bool,
}

impl Default for EmergencyIntervention {
    fn default() -> Self {
        EmergencyIntervention { state: false }
    }
}

impl EmergencyIntervention {
    pub fn new(state: bool) -> Self {
        EmergencyIntervention { state }
    }

    pub fn set_state(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - emerg - set_state");
        let param_state = U256::from(&params.input[16..36]);
        // Note: Only admin can change quota price
        if check::only_admin(params, context, contracts_db.clone()).expect("only admin can invoke price setting") {
            self.state = u256_to_bool(param_state);
            *changed = true;
            return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, vec![]));
        }
        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    pub fn get_state(&self) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - emerg - get_state");
        if self.state {
            return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), 0, vec![]));
        } else {
            return Ok(InterpreterResult::Normal(H256::from(0).0.to_vec(), 0, vec![]));
        }
    }
}
