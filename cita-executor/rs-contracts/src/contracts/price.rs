use super::contract::Contract;
use super::check;
use super::utils::{extract_to_u32, get_latest_key};

use cita_types::{Address, H256, U256};
use cita_vm::evm::{InterpreterParams, InterpreterResult, Log};
use common_types::context::Context;
use common_types::errors::ContractError;
use serde::{Deserialize, Serialize};

use crate::storage::db_contracts::ContractsDB;
use crate::storage::db_trait::DataBase;
use crate::storage::db_trait::DataCategory;

use std::collections::BTreeMap;
use std::sync::Arc;
use tiny_keccak::keccak256;

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceContract {
    contracts: BTreeMap<u64, Option<String>>,
}


impl Default for PriceContract {
    fn default() -> Self {
        PriceContract {
            contracts: BTreeMap::new(),
        }
    }
}

impl PriceContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) {
        let mut a = PriceContract::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(DataCategory::Contracts, b"price-contract".to_vec(), s.as_bytes().to_vec());

        // debug info
        let bin_map = contracts_db.get(DataCategory::Contracts, b"price-contract".to_vec()).unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: PriceContract = serde_json::from_str(&str).unwrap();
        trace!("System contract price {:?} after init.", contracts);
    }

    pub fn get_latest_item(&self, current_height: u64, contracts_db: Arc<ContractsDB>) -> (Option<PriceContract>, Option<Price>) {
        let mut latest_price = Price::default();
        let mut contract_map = PriceContract::default();

        if let Some(price_map) = contracts_db.get(DataCategory::Contracts, b"price-contract".to_vec()).expect("get price error") {
            let s = String::from_utf8(price_map).expect("from vec to string error");
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

            latest_price = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest price {:?}", latest_price);
            return (Some(contract_map), Some(latest_price));
        }
        (None, None)
    }
}

impl Contract for PriceContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - price - enter execute");
        let (contract_map, latest_price) = self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_price) {
            (Some(mut contract_map), Some(mut latest_price)) => {
                trace!("System contracts - price - params input {:?}", params.input);
                let mut updated = false;
                let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                    0x6bacc53fu32 => latest_price.get_quota_price(),
                    0x52da800au32 => latest_price.set_quota_price(params, &mut updated, context, contracts_db.clone()),
                    _ => panic!("Invalid function signature".to_owned()),
                });

                if result.is_ok() & updated {
                    let new_price = latest_price;
                    let str = serde_json::to_string(&new_price).unwrap();
                    contract_map.contracts.insert(context.block_number, Some(str));
                    let str = serde_json::to_string(&contract_map).unwrap();
                    let _ = contracts_db.insert(DataCategory::Contracts, b"price-contract".to_vec(), str.as_bytes().to_vec());

                    // debug information
                    let bin_map = contracts_db.get(DataCategory::Contracts, b"price-contract".to_vec()).unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: PriceContract = serde_json::from_str(&str).unwrap();
                    trace!("System contract price {:?} after update.", contracts);
                }
                return result;
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Price {
    quota_price: U256,
}

impl Price {
    pub fn new(quota_price: U256) -> Self {
        Price { quota_price }
    }

    pub fn set_quota_price(&mut self,
            params: &InterpreterParams, changed: &mut bool,
            context: &Context, contracts_db: Arc<ContractsDB>)-> Result<InterpreterResult, ContractError> {
        trace!("System contract - Price - set_quota_price");
        let param_quota_price = U256::from(&params.input[16..36]);
        // Note: Only admin can change quota price
        if check::only_admin(params, context, contracts_db.clone()).expect("only admin can invoke price setting") && param_quota_price > U256::zero() {
            self.quota_price = param_quota_price;
            *changed = true;

            let mut logs = Vec::new();
            let mut topics = Vec::new();
            let signature = "SetQuotaPrice(uint256)".as_bytes();
            topics.push(H256::from(keccak256(signature)));
            topics.push(H256::from(self.quota_price));
            topics.push(H256::default());
            topics.push(H256::default());
            let log = Log(params.contract.code_address, topics, vec![]);
            logs.push(log);

            return Ok(InterpreterResult::Normal(H256::from(1).0.to_vec(), params.gas_limit, logs));
        }

        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    pub fn get_quota_price(&self) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - Price - get_quota_price");
        return Ok(InterpreterResult::Normal(H256::from(self.quota_price).to_vec(), 0, vec![]));
    }
}

