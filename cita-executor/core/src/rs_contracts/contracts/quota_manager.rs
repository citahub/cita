use super::check;
use super::utils::{extract_to_u32, get_latest_key, h256_to_bool};

use cita_types::H256;
use cita_vm::evm::{InterpreterParams, InterpreterResult};
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

const BQL_VALUE: u64 = 1_073_741_824;
const AQL_VALUE: u64 = 268_435_456;
pub const AUTO_EXEC_QL_VALUE: u64 = 1_048_576;


#[derive(Serialize, Deserialize, Debug)]
pub struct QuotaContract {
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for QuotaContract {
    fn default() -> Self {
        QuotaContract {
            contracts: BTreeMap::new(),
        }
    }
}

impl QuotaContract {

    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = QuotaContract::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(
            DataCategory::Contracts,
            b"quota-contract".to_vec(),
            s.as_bytes().to_vec(),
        );

        // debug info
        let bin_map = contracts_db
            .get(DataCategory::Contracts, b"quota-contract".to_vec())
            .unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: QuotaContract = serde_json::from_str(&str).unwrap();
        trace!(
            "System contract quota {:?} after init.",
            contracts
        );

        keccak256(&s.as_bytes().to_vec())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaManager {
    account_quota: HashMap<Address, U256>,

}
