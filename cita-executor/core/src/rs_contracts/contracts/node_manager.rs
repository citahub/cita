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
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use tiny_keccak::keccak256;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeStore {
    contracts: BTreeMap<u64, Option<String>>,
}

impl Default for NodeStore {
    fn default() -> Self {
        NodeStore {
            contracts: BTreeMap::new(),
        }
    }
}

impl NodeStore {
    pub fn init(&self, str: String, contracts_db: Arc<ContractsDB>) -> [u8; 32] {
        let mut a = NodeStore::default();
        a.contracts.insert(0, Some(str));
        let s = serde_json::to_string(&a).unwrap();
        let _ = contracts_db.insert(
            DataCategory::Contracts,
            b"nodes-contract".to_vec(),
            s.as_bytes().to_vec(),
        );

        // debug info
        let bin_map = contracts_db
            .get(DataCategory::Contracts, b"nodes-contract".to_vec())
            .unwrap();
        let str = String::from_utf8(bin_map.unwrap()).unwrap();
        let contracts: NodeStore = serde_json::from_str(&str).unwrap();
        trace!("System contract nodes {:?} after init.", contracts);

        keccak256(&s.as_bytes().to_vec())
    }

    pub fn get_latest_item(
        &self,
        current_height: u64,
        contracts_db: Arc<ContractsDB>,
    ) -> (Option<NodeStore>, Option<NodeManager>) {
        if let Some(nodes_map) = contracts_db
            .get(DataCategory::Contracts, b"nodes-contract".to_vec())
            .expect("get nodes error")
        {
            let s = String::from_utf8(nodes_map).expect("from vec to string error");
            let contract_map: NodeStore = serde_json::from_str(&s).unwrap();
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

            let latest_item: NodeManager = serde_json::from_str(&(*bin).clone().unwrap()).unwrap();
            trace!("System contracts latest nodes {:?}", latest_item);
            return (Some(contract_map), Some(latest_item));
        }
        (None, None)
    }
}

impl<B: DB> Contract<B> for NodeStore {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("System contract - nodes - enter execute");
        let (contract_map, latest_item) =
            self.get_latest_item(context.block_number, contracts_db.clone());
        match (contract_map, latest_item) {
            (Some(mut contract_map), Some(mut latest_item)) => {
                trace!("System contracts - nodes - params input {:?}", params.input);
                let mut updated = false;
                let result =
                    extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                        0x51222d50 => latest_item.set_stake(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0xdd4c97a0 => latest_item.approve_node(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0x2d4ede93 => latest_item.delete_node(
                            params,
                            &mut updated,
                            context,
                            contracts_db.clone(),
                        ),
                        0x609df32f => latest_item.list_nodes(params),
                        0x6ed3876d => latest_item.list_stake(params),
                        0x30ccebb5 => latest_item.get_status(params),
                        0x0c829315 => latest_item.stake_permillage(params),
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
                        b"nodes-contract".to_vec(),
                        str.as_bytes().to_vec(),
                    );

                    // debug information, can be ommited
                    let bin_map = contracts_db
                        .get(DataCategory::Contracts, b"nodes-contract".to_vec())
                        .unwrap();
                    let str = String::from_utf8(bin_map.unwrap()).unwrap();
                    let contracts: NodeStore = serde_json::from_str(&str).unwrap();
                    trace!("System contract nodes {:?} after update.", contracts);

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
pub struct NodeManager {
    status: HashMap<Address, bool>, // false -> closed, false -> open
    nodes: Vec<Address>,
    stakes: HashMap<Address, U256>,
}

impl NodeManager {
    pub fn new(nodes: Vec<Address>, stakes: Vec<U256>) -> Self {
        let mut stakes_map = HashMap::new();
        let mut status_map = HashMap::new();
        for i in 0..nodes.len() {
            stakes_map.insert(nodes[i], stakes[i]);
            status_map.insert(nodes[i], true);
        }
        NodeManager {
            status: status_map,
            nodes: nodes,
            stakes: stakes_map,
        }
    }

    pub fn set_stake(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("Node contract set_stake, params {:?}", params.input);
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            let param_address = Address::from_slice(&params.input[16..36]);
            let param_stake = U256::from(20);
            trace!("param address decoded is {:?}", param_address);
            trace!("param stake decoded is {:?}", param_stake);
            if let Some(stake) = self.stakes.get_mut(&param_address) {
                *stake = param_stake;
                *changed = true;

                return Ok(InterpreterResult::Normal(
                    H256::from(1).0.to_vec(),
                    params.gas_limit,
                    vec![],
                ));
            } else {
                warn!("the address not in nodes list.");
                return Err(ContractError::Internal("Only admin can do".to_owned()));
            }
        }
        Err(ContractError::Internal("Only admin can do".to_owned()))
    }

    pub fn approve_node(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("Node contract approve_node, params {:?}", params.input);
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            let param_address = Address::from_slice(&params.input[16..36]);
            if !*self.status.get(&param_address).unwrap_or(&false) {
                self.status.insert(param_address, true);
                self.nodes.push(param_address);
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

    pub fn delete_node(
        &mut self,
        params: &InterpreterParams,
        changed: &mut bool,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("Node contract delete_node, params {:?}", params.input);
        if check::only_admin(params, context, contracts_db.clone())
            .expect("only admin can invoke price setting")
        {
            let param_address = Address::from_slice(&params.input[16..36]);
            if *self.status.get(&param_address).unwrap_or(&false) {
                self.nodes.retain(|&n| n != param_address);
                if let Some(s) = self.status.get_mut(&param_address) {
                    *s = false;
                }
                if let Some(s) = self.stakes.get_mut(&param_address) {
                    *s = U256::zero();
                }
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

    pub fn list_nodes(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("Node contract list_nodes, params {:?}", params.input);
        let nodes = self
            .nodes
            .iter()
            .map(|i| Token::Address(i.0))
            .collect::<Vec<ethabi::Token>>();
        let mut tokens = Vec::new();
        tokens.push(ethabi::Token::Array(nodes));
        return Ok(InterpreterResult::Normal(
            ethabi::encode(&tokens),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn list_stake(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("Node contract list_stake, params {:?}", params.input);
        let mut tokens = Vec::new();
        let mut stakes = Vec::new();
        for (_key, value) in self.stakes.iter() {
            stakes.push(Token::Uint(H256::from(value).0));
        }
        tokens.push(ethabi::Token::Array(stakes));
        return Ok(InterpreterResult::Normal(
            ethabi::encode(&tokens),
            params.gas_limit,
            vec![],
        ));
    }

    pub fn get_status(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        trace!("Node contract get_status, params {:?}", params.input);
        let param_address = Address::from_slice(&params.input[16..36]);
        if *self.status.get(&param_address).unwrap_or(&false) {
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

    pub fn stake_permillage(
        &self,
        params: &InterpreterParams,
    ) -> Result<InterpreterResult, ContractError> {
        // only in charge mode
        trace!("Node contract stake_permillage, params {:?}", params.input);
        let param_address = Address::from_slice(&params.input[16..36]);
        let node_stakes = self.stakes.get(&param_address).unwrap();

        let total = U256::zero();
        for i in self.stakes.values() {
            total.overflowing_add(*i);
        }

        if total == U256::zero() {
            return Ok(InterpreterResult::Normal(
                H256::from(0).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        } else {
            let extend_stake = node_stakes.overflowing_mul(U256::from(1000)).0;
            let res = extend_stake.checked_div(total).unwrap();
            return Ok(InterpreterResult::Normal(
                H256::from(res).to_vec(),
                params.gas_limit,
                vec![],
            ));
        }
    }
}
