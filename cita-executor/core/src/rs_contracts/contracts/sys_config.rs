use super::check;
use super::utils::{extract_to_u32, get_latest_key};

use cita_types::{Address, H256, U256};
use cita_vm::evm::{InterpreterParams, InterpreterResult, Log};
use common_types::context::Context;
use common_types::errors::ContractError;

use super::contract::Contract;
use crate::rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::storage::db_trait::DataBase;
use crate::rs_contracts::storage::db_trait::DataCategory;

use crate::libexecutor::economical_model::EconomicalModel;
use crate::rs_contracts::contracts::utils::{clean_0x, encode_string};
use cita_trie::DB;
use cita_vm::state::State;
use ethabi::param_type::ParamType;
use ethabi::token::LenientTokenizer;
use ethabi::token::Tokenizer;
use ethabi::Token;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Arc;
use tiny_keccak::keccak256;

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemContract {
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for SystemContract {
    fn default() -> Self {
        SystemContract {
            contracts: BTreeMap::new(),
        }
    }
}

impl SystemContract {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = SystemContract::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(
            DataCategory::Contracts,
            b"system-contract".to_vec(),
            s.as_bytes().to_vec(),
        );

        // debug info
        let bin_map = contracts_db
            .get(DataCategory::Contracts, b"system-contract".to_vec())
            .unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: SystemContract = serde_json::from_str(&str).unwrap();
        trace!("System contract system {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(
        &self,
        current_height: u64,
        contracts_db: Arc<ContractsDB>,
    ) -> (Option<SystemContract>, Option<Sysconfig>) {
        if let Some(price_map) = contracts_db
            .get(DataCategory::Contracts, b"system-contract".to_vec())
            .expect("get contract map error")
        {
            let s = String::from_utf8(price_map).expect("from vec to string error");
            let contract_map: SystemContract = serde_json::from_str(&s).unwrap();
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

            let latest_item: Sysconfig = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest system {:?}", latest_item);
            return (Some(contract_map), Some(latest_item));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for SystemContract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - enter execute");
        let (contract_map, latest_item) =
            self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_item) {
            (Some(mut contract_map), Some(mut latest_item)) => {
                trace!(
                    "System contracts - system - params input {:?}",
                    params.input
                );
                let mut updated = false;
                let result =
                    extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                        0x6fbf656a => latest_item.set_operator(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0xf87f44b9 => latest_item.set_website(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0xc0c41f22 => latest_item.set_chain_name(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0xde7aa05d => latest_item.set_block_interval(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        // 0x28ccf1fd => latest_item.update_to_chain_id_v1(
                        //     params,
                        //     &mut updated,
                        //     context,
                        //     contracts_db.clone(),
                        // ),
                        0x8ec1aaed => latest_item.get_delay_block_number(params),
                        0xdd12b51f => latest_item.get_permission_check(params),
                        0x3fd24419 => latest_item.get_send_tx_permission_check(params),
                        0x6f25ac3f => latest_item.get_create_contract_permission_check(params),
                        0xb4a0e24c => latest_item.get_quota_check(params),
                        0x984dc34b => latest_item.get_fee_back_platform_check(params),
                        0xe19709e0 => latest_item.get_chain_owner(params),
                        0xd722b0bc => latest_item.get_chain_name(params),
                        0x3408e470 => latest_item.get_chain_id(params),
                        0x60952274 => latest_item.get_chain_id_v1(params),
                        0xe7f43c68 => latest_item.get_operator(params),
                        0xdf51aa49 => latest_item.get_website(params),
                        0xd5cd402c => latest_item.get_block_interval(params),
                        0x63ffec6e => latest_item.get_economical_model(params),
                        0xabb1dc44 => latest_item.get_token_info(params),
                        0xf337a9d6 => latest_item.get_auto_exec(params),
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
                        b"system-contract".to_vec(),
                        str.as_bytes().to_vec(),
                    );

                    // debug information, can be ommited
                    let bin_map = contracts_db
                        .get(DataCategory::Contracts, b"system-contract".to_vec())
                        .unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: SystemContract = serde_json::from_str(&str).unwrap();
                    trace!("System contract system {:?} after update.", contracts);

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Sysconfig {
    delay_block_number: u64,
    check_permission: bool,
    check_send_tx_permission: bool,
    check_create_contract_permission: bool,
    check_quota: bool,
    check_feeback_platform: bool,
    chain_owner: Address,
    chain_name: String,
    chain_id: u64,
    operator: String,
    website: String,
    block_interval: u64,
    economical_model: u64,
    name: String,
    symbol: String,
    avatar: String,
    auto_exec: bool,
}

impl Sysconfig {
    pub fn new(
        delay_block_number: u64,
        check_permission: bool,
        check_send_tx_permission: bool,
        check_create_contract_permission: bool,
        check_quota: bool,
        check_feeback_platform: bool,
        chain_owner: Address,
        chain_name: String,
        chain_id: u64,
        operator: String,
        website: String,
        block_interval: u64,
        economical_model: u64,
        name: String,
        symbol: String,
        avatar: String,
        auto_exec: bool,
    ) -> Self {
        Sysconfig {
            delay_block_number,
            check_permission,
            check_send_tx_permission,
            check_create_contract_permission,
            check_quota,
            check_feeback_platform,
            chain_owner,
            chain_name,
            chain_id,
            operator,
            website,
            block_interval,
            economical_model,
            name,
            symbol,
            avatar,
            auto_exec,
        }
    }

    pub fn set_chain_name(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!(
            "System contract system set_chain_name, params {:?}",
            params.input
        );
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            if let Ok(param) = ethabi::decode(&[ParamType::String], &params.input[4..]) {
                let param_chain_name = match &param[0] {
                    Token::String(s) => s,
                    _ => unreachable!(),
                };
                self.chain_name = encode_string(&param_chain_name);
                *changed = true;
                return Ok(InterpreterResult::Normal(
                    H256::from(1).0.to_vec(),
                    params.gas_limit,
                    vec![],
                ));
            }
        }

        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    pub fn set_operator(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!(
            "System contract - system - set_operator, params {:?}",
            params.input
        );
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            if let Ok(param) = ethabi::decode(&[ParamType::String], &params.input[4..]) {
                let param_operator = match &param[0] {
                    Token::String(s) => s,
                    _ => unreachable!(),
                };
                self.operator = encode_string(&param_operator);
                *changed = true;
                return Ok(InterpreterResult::Normal(
                    H256::from(1).0.to_vec(),
                    params.gas_limit,
                    vec![],
                ));
            }
        }
        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    pub fn set_website(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!(
            "System contract - system - set_website, params {:?}",
            params.input
        );
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            if let Ok(param) = ethabi::decode(&[ParamType::String], &params.input[4..]) {
                let param_website = match &param[0] {
                    Token::String(s) => s,
                    _ => unreachable!(),
                };
                self.chain_name = encode_string(&param_website);
                *changed = true;
                return Ok(InterpreterResult::Normal(
                    H256::from(1).0.to_vec(),
                    params.gas_limit,
                    vec![],
                ));
            }
        }
        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    pub fn set_block_interval(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!(
            "System contract - system - set_block_interval, params {:?}",
            params.input
        );
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            let param = U256::from(&params.input[4..]);
            self.block_interval = param.as_u64();
            *changed = true;
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }
        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    // pub fn update_to_chain_id_v1(
    //     &mut self,
    //     params: &InterpreterParams,
    //     changed: &mut bool,
    //     context: &Context,
    //     contracts_db: Arc<ContractsDB>,
    // ) -> Result<InterpreterResult, ContractError> {
    //     trace!(
    //         "System contract - system - update_to_chain_id_v1, params {:?}",
    //         params.input
    //     );
    //     Err(ContractError::Internal("Only admin can do".to_owned()))
    // }

    pub fn get_permission_check(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_permission_check");
        if self.check_permission {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }

        Ok(InterpreterResult::Normal(
            H256::from(0).0.to_vec(),
            params.gas_limit,
            vec![],
        ))
    }

    pub fn get_send_tx_permission_check(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_permission_check");
        if self.check_send_tx_permission {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }

        Ok(InterpreterResult::Normal(
            H256::from(0).0.to_vec(),
            params.gas_limit,
            vec![],
        ))
    }

    pub fn get_create_contract_permission_check(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_permission_check");
        if self.check_create_contract_permission {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }

        Ok(InterpreterResult::Normal(
            H256::from(0).0.to_vec(),
            params.gas_limit,
            vec![],
        ))
    }

    pub fn get_quota_check(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_quota_check");
        if self.check_quota {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }

        Ok(InterpreterResult::Normal(
            H256::from(0).0.to_vec(),
            params.gas_limit,
            vec![],
        ))
    }

    pub fn get_fee_back_platform_check(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_fee_back_platform_check");
        if self.check_feeback_platform {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }

        Ok(InterpreterResult::Normal(
            H256::from(0).0.to_vec(),
            params.gas_limit,
            vec![],
        ))
    }

    pub fn get_chain_owner(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_chain_owner");
        return Ok(InterpreterResult::Normal(
            H256::from(self.chain_owner).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_chain_name(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_chain_name");
        trace!("chain owner is {:?}", self.chain_owner);
        let bin = hex::decode(self.chain_name.clone()).unwrap();
        trace!("bin is {:?}", bin);
        return Ok(InterpreterResult::Normal(bin, params.gas_limit, vec![]));
    }

    pub fn get_chain_id(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_chain_id");
        let res = U256::from(self.chain_id);
        return Ok(InterpreterResult::Normal(
            H256::from(res).to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_chain_id_v1(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_chain_id_v1");
        let res = U256::from(self.chain_id);
        return Ok(InterpreterResult::Normal(
            H256::from(res).to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_operator(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_operator");
        trace!("chain owner is {:?}", self.operator);
        let bin = hex::decode(self.operator.clone()).unwrap();
        trace!("bin is {:?}", bin);
        return Ok(InterpreterResult::Normal(bin, params.gas_limit, vec![]));
    }

    pub fn get_website(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_website");
        trace!("chain owner is {:?}", self.website);
        let bin = hex::decode(self.website.clone()).unwrap();
        trace!("bin is {:?}", bin);
        return Ok(InterpreterResult::Normal(bin, params.gas_limit, vec![]));
    }

    pub fn get_block_interval(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_block_interval");
        let res = U256::from(self.block_interval);
        return Ok(InterpreterResult::Normal(
            H256::from(res).to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_economical_model(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_economical_model");
        let res = U256::from(self.economical_model);
        return Ok(InterpreterResult::Normal(
            H256::from(res).to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_token_info(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - ");
        let mut tokens = Vec::new();
        tokens.push(Token::String(self.name.clone()));
        tokens.push(Token::String(self.symbol.clone()));
        tokens.push(Token::String(self.avatar.clone()));
        let res = ethabi::encode(&tokens);

        return Ok(InterpreterResult::Normal(
            res.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_auto_exec(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_auto_exec");
        if self.auto_exec {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }

        Ok(InterpreterResult::Normal(
            H256::from(0).0.to_vec(),
            params.gas_limit,
            vec![],
        ))
    }

    pub fn get_delay_block_number(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - system - get_delay_block_number");
        let res = U256::from(self.delay_block_number);
        return Ok(InterpreterResult::Normal(
            H256::from(res).to_vec(),
            params.gas_limit,
            vec![],
        ));
    }
}
