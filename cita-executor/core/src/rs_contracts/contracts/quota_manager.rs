use super::check;
use super::utils::{extract_to_u32, get_latest_key, h256_to_bool};

use cita_types::{Address, H256, U256};
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
use ethabi::Token;
use num_traits::checked_pow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use tiny_keccak::keccak256;

const BQL_VALUE: u64 = 1_073_741_824;
const AQL_VALUE: u64 = 268_435_456;
pub const AUTO_EXEC_QL_VALUE: u64 = 1_048_576;
const MAX_LIMIT: u64 = 9_223_372_036_854_775_808;   // 2 ** 63
const MIN_LIMIT: u64 = 4_194_304;   // 2 ** 22

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
        trace!("System contract quota {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(
        &self,
        current_height: u64,
        contracts_db: Arc<ContractsDB>,
    ) -> (Option<QuotaContract>, Option<QuotaManager>) {
        if let Some(emerg_map) = contracts_db
            .get(DataCategory::Contracts, b"quota-contract".to_vec())
            .expect("get emerg error")
        {
            let s = String::from_utf8(emerg_map).expect("from vec to string error");
            let contract_map: QuotaContract = serde_json::from_str(&s).unwrap();
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

            let latest_item: QuotaManager = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest quota {:?}", latest_item);
            return (Some(contract_map), Some(latest_item));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for QuotaContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - enter execute");
        let (contract_map, latest_item) =
            self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_item) {
            (Some(mut contract_map), Some(mut latest_item)) => {
                trace!("System contracts - quota - params input {:?}", params.input);
                let mut updated = false;
                let result =
                    extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                        0xb107ea12 => latest_item.set_default_aql(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0x499a1bcd => {
                            latest_item.set_aql(params, &mut updated, context, contracts_db.clone())
                        }
                        0x931cd0cc => {
                            latest_item.set_bql(params, &mut updated, context, contracts_db.clone())
                        }
                        0x8a48ac03 => latest_item.get_accounts(params),
                        0xcdbcff6d => latest_item.get_quotas(params),
                        0x0bc8982f => latest_item.get_bql(params),
                        0xbd9fbe7b => latest_item.get_default_aql(params),
                        0x942a8ad3 => latest_item.get_aql(params),
                        0x9cd981bb => latest_item.get_auto_exec_ql(params),
                        _ => panic!("Invalid function signature".to_owned()),
                    });

                if result.is_ok() & updated {
                    let new_item = latest_item;
                    let str = serde_json::to_string(&new_item).unwrap();
                    contract_map
                        .contracts
                        .insert(context.block_number, Some(str));
                    let str = serde_json::to_string(&contract_map).unwrap();
                    let updated_hash = keccak256(&str.as_bytes().to_vec());
                    let _ = contracts_db.insert(
                        DataCategory::Contracts,
                        b"quota-contract".to_vec(),
                        str.as_bytes().to_vec(),
                    );

                    // debug information, can be ommited
                    let bin_map = contracts_db
                        .get(DataCategory::Contracts, b"quota-contract".to_vec())
                        .unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: QuotaContract = serde_json::from_str(&str).unwrap();
                    trace!("System contract quota {:?} after update.", contracts);

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
pub struct QuotaManager {
    account_quota: HashMap<Address, U256>,
    default_block_quota_limit: U256,
    default_account_quota_limit: U256,
}

impl QuotaManager {
    pub fn new(admin: Address) -> Self {
        let mut account_quota = HashMap::new();
        account_quota.insert(admin, U256::from(BQL_VALUE));

        QuotaManager {
            account_quota,
            default_account_quota_limit: U256::from(AQL_VALUE),
            default_block_quota_limit: U256::from(BQL_VALUE),
        }
    }

    pub fn set_default_aql(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - set_defaul_aql");
        let param_default_aql = U256::from(&params.input[4..]);
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
            && self.check_base_limit(param_default_aql)
        {
            self.default_account_quota_limit = param_default_aql;
            *changed = true;
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

    pub fn set_aql(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!(
            "System contract - quota - set_aql, input {:?}",
            params.input
        );
        let param_address = Address::from(&params.input[16..36]);
        let param_aql = U256::from(&params.input[36..]);
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
            && self.check_base_limit(param_aql)
        {
            trace!("param address is {:?}", param_address);
            trace!("param param_aql is {:?}", param_aql);
            trace!("account quota before {:?}", self.account_quota);
            self.account_quota.entry(param_address).or_insert(param_aql);
            trace!("account quota after {:?}", self.account_quota);

            *changed = true;
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

    pub fn set_bql(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - set_defaul_bql");
        let param_default_bql = U256::from(&params.input[4..]);
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
            && self.check_block_limit(param_default_bql)
            && self.check_base_limit(param_default_bql)
        {
            self.default_block_quota_limit = param_default_bql;
            *changed = true;
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

    pub fn get_accounts(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - get_accounts");
        let mut accounts = Vec::new();
        for key in self.account_quota.keys() {
            let tmp = Token::Address(key.0);
            accounts.push(tmp);
        }

        let mut tokens = Vec::new();
        tokens.push(Token::Array(accounts));
        return Ok(InterpreterResult::Normal(
            ethabi::encode(&tokens),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_quotas(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - get_quotas");
        let mut quotas = Vec::new();
        for v in self.account_quota.values() {
            let tmp = Token::Uint(H256::from(v).0);
            quotas.push(tmp);
        }

        let mut tokens = Vec::new();
        tokens.push(Token::Array(quotas));
        return Ok(InterpreterResult::Normal(
            ethabi::encode(&tokens),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_bql(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - get_bql");
        return Ok(InterpreterResult::Normal(
            H256::from(self.default_block_quota_limit).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_default_aql(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - get_default_aql");
        return Ok(InterpreterResult::Normal(
            H256::from(self.default_account_quota_limit).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_aql(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        trace!(
            "System contract - quota - get_aql, input {:?}",
            params.input
        );
        let param_address = Address::from_slice(&params.input[16..36]);
        if let Some(quota) = self.account_quota.get(&param_address) {
            if *quota == U256::zero() {
                return Ok(InterpreterResult::Normal(
                    H256::from(self.default_account_quota_limit).0.to_vec(),
                    params.gas_limit,
                    vec![],
                ));
            } else {
                return Ok(InterpreterResult::Normal(
                    H256::from(quota).0.to_vec(),
                    params.gas_limit,
                    vec![],
                ));
            }
        }
        return Ok(InterpreterResult::Normal(
            H256::from(self.default_account_quota_limit).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_auto_exec_ql(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - quota - get_auto_exec_ql");

        return Ok(InterpreterResult::Normal(
            H256::from(U256::from(AUTO_EXEC_QL_VALUE)).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn check_base_limit(&self, param_limit: U256) -> bool {
        if param_limit < U256::from(MAX_LIMIT) && param_limit > U256::from(MIN_LIMIT) {
            return true;
        }
        false
    }

    pub fn check_block_limit(&self, param_limit: U256) -> bool {
        if param_limit >= self.default_account_quota_limit {
            return true;
        }
        false
    }
}
