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

impl PriceContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = PriceContract::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(
            DataCategory::Contracts,
            b"price-contract".to_vec(),
            s.as_bytes().to_vec(),
        );

        // debug info
        let bin_map = contracts_db
            .get(DataCategory::Contracts, b"price-contract".to_vec())
            .unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: PriceContract = serde_json::from_str(&str).unwrap();
        trace!("System contract price {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmergencyIntervention {
    state: bool,
}

impl EmergencyIntervention {
    pub fn new(state: bool) -> Self {
        EmergencyIntervention { state }
    }

    
}
