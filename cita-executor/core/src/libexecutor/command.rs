// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::economical_model::EconomicalModel;
use super::executor::CitaTrieDB;
use super::executor::{make_consensus_config, Executor};
use super::sys_config::GlobalSysConfig;
use crate::cita_executive::{CitaExecutive, ExecutedResult as CitaExecuted};
use crate::contracts::solc::{
    sys_config::ChainId, PermissionManagement, SysConfig, VersionManager,
};
use crate::libexecutor::block::EVMBlockDataProvider;
pub use crate::libexecutor::block::*;
use crate::libexecutor::call_request::CallRequest;
use crate::trie_db::TrieDB;
use crate::types::block_number::{BlockTag, Tag};
use crate::types::context::Context;
use crate::types::errors::CallError;
use crate::types::errors::ExecutionError;
use crate::types::transaction::{Action, SignedTransaction, Transaction};
pub use byteorder::{BigEndian, ByteOrder};
use cita_database::RocksDB;
use cita_types::traits::LowerHex;
use cita_types::{Address, H256, U256};
use cita_vm::state::{State as CitaState, StateObjectInfo};
use crossbeam_channel::{Receiver, Sender};
use jsonrpc_types::rpc_types::{
    BlockNumber as RpcBlockNumber, BlockTag as RpcBlockTag, EconomicalModel as RpcEconomicalModel,
    MetaData,
};
use libproto::ExecutedResult;
use serde_json;
use std::cell::RefCell;
use std::convert::{From, Into};
use std::fmt;
use std::sync::Arc;
use types::Bytes;
use util::RwLock;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum Command {
    StateAt(BlockTag),
    GenState(H256, H256),
    CodeAt(Address, BlockTag),
    ABIAt(Address, BlockTag),
    BalanceAt(Address, BlockTag),
    NonceAt(Address, BlockTag),
    ETHCall(CallRequest, BlockTag),
    EstimateQuota(CallRequest, BlockTag),
    SignCall(CallRequest),
    Call(SignedTransaction, BlockTag),
    ChainID,
    Metadata(String),
    EconomicalModel,
    LoadExecutedResult(u64),
    Grow(ClosedBlock),
    Exit(BlockTag),
    CloneExecutorReader,
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum CommandResp {
    StateAt(Option<CitaState<CitaTrieDB>>),
    GenState(Option<CitaState<CitaTrieDB>>),
    CodeAt(Option<Bytes>),
    ABIAt(Option<Bytes>),
    BalanceAt(Option<Bytes>),
    NonceAt(Option<U256>),
    ETHCall(Result<Bytes, String>),
    EstimateQuota(Result<Bytes, String>),
    SignCall(SignedTransaction),
    Call(Result<CitaExecuted, CallError>),
    ChainID(Option<ChainId>),
    Metadata(Result<MetaData, String>),
    EconomicalModel(EconomicalModel),
    LoadExecutedResult(ExecutedResult),
    Grow(ExecutedResult),
    Exit,
    CloneExecutorReader(Executor),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::StateAt(_) => write!(f, "Command::StateAt"),
            Command::GenState(_, _) => write!(f, "Command::GenState"),
            Command::CodeAt(_, _) => write!(f, "Command::CodeAt"),
            Command::ABIAt(_, _) => write!(f, "Command::ABIAt"),
            Command::BalanceAt(_, _) => write!(f, "Command::BalanceAt"),
            Command::NonceAt(_, _) => write!(f, "Command::NonceAt"),
            Command::ETHCall(_, _) => write!(f, "Command::ETHCall"),
            Command::EstimateQuota(_, _) => write!(f, "Command::EstimateQuota"),
            Command::SignCall(_) => write!(f, "Command::SignCall"),
            Command::Call(_, _) => write!(f, "Command::Call"),
            Command::ChainID => write!(f, "Command::ChainID "),
            Command::Metadata(_) => write!(f, "Command::Metadata"),
            Command::EconomicalModel => write!(f, "Command::EconomicalModel"),
            Command::LoadExecutedResult(_) => write!(f, "Command::LoadExecutedResult"),
            Command::Grow(_) => write!(f, "Command::Grow"),
            Command::Exit(_) => write!(f, "Command::Exit"),
            Command::CloneExecutorReader => write!(f, "Command::CloneExecutorReader"),
        }
    }
}

impl fmt::Display for CommandResp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandResp::StateAt(_) => write!(f, "CommandResp::StateAt"),
            CommandResp::GenState(_) => write!(f, "CommandResp::GenState"),
            CommandResp::CodeAt(_) => write!(f, "CommandResp::CodeAt"),
            CommandResp::ABIAt(_) => write!(f, "CommandResp::ABIAt"),
            CommandResp::BalanceAt(_) => write!(f, "CommandResp::BalanceAt"),
            CommandResp::NonceAt(_) => write!(f, "CommandResp::NonceAt"),
            CommandResp::ETHCall(_) => write!(f, "CommandResp::ETHCall"),
            CommandResp::EstimateQuota(_) => write!(f, "CommandResp::EstimateQuota"),
            CommandResp::SignCall(_) => write!(f, "CommandResp::SignCall"),
            CommandResp::Call(_) => write!(f, "CommandResp::Call"),
            CommandResp::ChainID(_) => write!(f, "CommandResp::ChainID "),
            CommandResp::Metadata(_) => write!(f, "CommandResp::Metadata"),
            CommandResp::EconomicalModel(_) => write!(f, "CommandResp::EconomicalModel"),
            CommandResp::LoadExecutedResult(_) => write!(f, "CommandResp::LoadExecutedResult"),
            CommandResp::Grow(_) => write!(f, "CommandResp::Grow"),
            CommandResp::Exit => write!(f, "CommandResp::Exit"),
            CommandResp::CloneExecutorReader(_) => write!(f, "CommandResp::CloneExecurorReader"),
        }
    }
}

pub trait Commander {
    fn operate(&mut self, command: Command) -> CommandResp;
    fn state_at(&self, block_tag: BlockTag) -> Option<CitaState<CitaTrieDB>>;
    fn gen_state(&self, root: H256, parent_hash: H256) -> Option<CitaState<CitaTrieDB>>;
    fn code_at(&self, address: &Address, block_tag: BlockTag) -> Option<Bytes>;
    fn abi_at(&self, address: &Address, block_tag: BlockTag) -> Option<Bytes>;
    fn balance_at(&self, address: &Address, block_tag: BlockTag) -> Option<Bytes>;
    fn nonce_at(&self, address: &Address, block_tag: BlockTag) -> Option<U256>;
    fn eth_call(&self, request: CallRequest, block_tag: BlockTag) -> Result<Bytes, String>;
    fn estimate_quota(&self, request: CallRequest, block_tag: BlockTag) -> Result<Bytes, String>;
    fn sign_call(&self, request: CallRequest) -> SignedTransaction;
    fn call(&self, t: &SignedTransaction, block_tag: BlockTag) -> Result<CitaExecuted, CallError>;
    fn chain_id(&self) -> Option<ChainId>;
    fn metadata(&self, data: String) -> Result<MetaData, String>;
    fn economical_model(&self) -> EconomicalModel;
    fn load_executed_result(&self, height: u64) -> ExecutedResult;
    fn grow(&mut self, closed_block: &ClosedBlock) -> ExecutedResult;
    fn exit(&mut self, rollback_id: BlockTag);
    fn clone_executor_reader(&mut self) -> Self;
}

impl Commander for Executor {
    fn operate(&mut self, command: Command) -> CommandResp {
        match command {
            Command::StateAt(block_tag) => CommandResp::StateAt(self.state_at(block_tag)),
            Command::GenState(root, parent_hash) => {
                CommandResp::GenState(self.gen_state(root, parent_hash))
            }
            Command::CodeAt(address, block_tag) => {
                CommandResp::CodeAt(self.code_at(&address, block_tag))
            }
            Command::ABIAt(address, block_tag) => {
                CommandResp::ABIAt(self.abi_at(&address, block_tag))
            }
            Command::BalanceAt(address, block_tag) => {
                CommandResp::BalanceAt(self.balance_at(&address, block_tag))
            }
            Command::NonceAt(address, block_tag) => {
                CommandResp::NonceAt(self.nonce_at(&address, block_tag))
            }
            Command::ETHCall(call_request, block_tag) => {
                CommandResp::ETHCall(self.eth_call(call_request, block_tag))
            }
            Command::EstimateQuota(call_request, block_tag) => {
                CommandResp::EstimateQuota(self.estimate_quota(call_request, block_tag))
            }
            Command::SignCall(call_request) => CommandResp::SignCall(self.sign_call(call_request)),
            Command::Call(signed_transaction, block_tag) => {
                CommandResp::Call(self.call(&signed_transaction, block_tag))
            }
            Command::ChainID => CommandResp::ChainID(self.chain_id()),
            Command::Metadata(data) => CommandResp::Metadata(self.metadata(data)),
            Command::EconomicalModel => CommandResp::EconomicalModel(self.economical_model()),
            Command::LoadExecutedResult(height) => {
                CommandResp::LoadExecutedResult(self.load_executed_result(height))
            }
            Command::Grow(mut closed_block) => {
                let r = self.grow(&closed_block);
                closed_block.clear_cache();
                CommandResp::Grow(r)
            }
            Command::Exit(rollback_id) => {
                self.exit(rollback_id);
                CommandResp::Exit
            }
            Command::CloneExecutorReader => {
                CommandResp::CloneExecutorReader(self.clone_executor_reader())
            }
        }
    }

    /// Attempt to get a copy of a specific block's final state.
    fn state_at(&self, id: BlockTag) -> Option<CitaState<CitaTrieDB>> {
        self.block_header(id)
            .and_then(|h| self.gen_state(*h.state_root(), *h.parent_hash()))
    }

    /// Generate block's final state.
    fn gen_state(&self, root: H256, _parent_hash: H256) -> Option<CitaState<CitaTrieDB>> {
        // FIXME: There is a RWLock for clone a db, is it ok for using Arc::clone?
        CitaState::from_existing(Arc::<CitaTrieDB>::clone(&self.state_db), root).ok()
    }

    /// Get code by address
    fn code_at(&self, address: &Address, id: BlockTag) -> Option<Bytes> {
        self.state_at(id).and_then(|mut s| s.code(address).ok())
    }

    /// Get abi by address
    fn abi_at(&self, address: &Address, id: BlockTag) -> Option<Bytes> {
        self.state_at(id).and_then(|mut s| s.abi(address).ok())
    }

    /// Get balance by address
    fn balance_at(&self, address: &Address, id: BlockTag) -> Option<Bytes> {
        self.state_at(id)
            .and_then(|mut s| s.balance(address).ok())
            .map(|c| {
                let balance = &mut [0u8; 32];
                c.to_big_endian(balance);
                balance.to_vec()
            })
    }

    fn nonce_at(&self, address: &Address, id: BlockTag) -> Option<U256> {
        self.state_at(id).and_then(|mut s| s.nonce(address).ok())
    }

    fn eth_call(&self, request: CallRequest, id: BlockTag) -> Result<Bytes, String> {
        let signed = self.sign_call(request);
        let result = self.call(&signed, id);
        result
            .map(|b| b.output)
            .or_else(|e| Err(format!("Call Error {}", e)))
    }

    fn estimate_quota(&self, request: CallRequest, id: BlockTag) -> Result<Bytes, String> {
        // The estimated transaction cost cannot exceed BQL
        let max_quota = U256::from(self.sys_config.block_quota_limit);
        let precision = U256::from(1024);

        let signed = self.sign_call(request);
        let header = self
            .block_header(id)
            .ok_or_else(|| "Estimate Error CallError::StatePruned".to_owned())?;
        let last_hashes = self.build_last_hashes(Some(header.hash().unwrap()), header.number());

        let context = Context {
            block_number: header.number(),
            coin_base: *header.proposer(),
            timestamp: if self.eth_compatibility {
                header.timestamp() / 1000
            } else {
                header.timestamp()
            },
            difficulty: U256::default(),
            last_hashes: ::std::sync::Arc::new(last_hashes),
            quota_used: *header.quota_used(),
            block_quota_limit: max_quota,
            account_quota_limit: u64::max_value().into(),
        };
        let block_data_provider = Arc::new(EVMBlockDataProvider::new(context.clone()));

        let mut conf = self.sys_config.block_sys_config.clone();
        conf.exempt_checking();
        let sender = *signed.sender();

        // Try different quota to run tx.
        let exec_tx = |quota| {
            let mut tx = signed.as_unsigned().clone();
            tx.gas = quota;
            let tx = tx.fake_sign(sender);

            // The same transaction will get different result in different state.
            // And the estimate action will change the state, so it should take the most primitive
            // state for each estimate.
            let state = self.state_at(id).ok_or_else(|| {
                ExecutionError::Internal("Estimate Error CallError::StatePruned".to_owned())
            })?;
            let state = Arc::new(RefCell::new(state));

            let clone_conf = conf.clone();
            CitaExecutive::new(
                block_data_provider.clone(),
                state,
                self.contracts_db.clone(),
                &context.clone(),
                clone_conf.economical_model,
            )
            .exec(&tx, &clone_conf)
        };
        let check_quota = |quota| {
            exec_tx(quota).ok().map_or((false, U256::from(0)), |r| {
                (r.exception.is_none(), r.quota_used)
            })
        };

        // Try block_quota_limit first
        let (run_ok, quota_used) = check_quota(max_quota);
        let lower = if !run_ok {
            trace!("estimate_gas failed with {}.", max_quota);
            return Err(
                format!("Requires quota higher than upper limit: {}", max_quota).to_owned(),
            );
        } else {
            quota_used
        };

        //Try lower (quota_used)
        let estimate_quota = &mut [0u8; 32];
        let (run_ok, quota_used) = check_quota(lower);
        if run_ok {
            quota_used.to_big_endian(estimate_quota);
            return Ok(estimate_quota.to_vec());
        }

        // Binary search the point between `lower` and `upper`, with the precision which means
        // the estimate quota deviation　less than precision.
        fn binary_search<F>(
            mut lower: U256,
            mut upper: U256,
            mut check_quota: F,
            precision: U256,
        ) -> U256
        where
            F: FnMut(U256) -> (bool, U256),
        {
            while upper - lower > precision {
                let mid = (lower + upper) / 2;
                trace!(
                    "estimate_gas : lower {} .. mid {} .. upper {}",
                    lower,
                    mid,
                    upper
                );
                let (c, _) = check_quota(mid);
                if c {
                    upper = mid;
                } else {
                    lower = mid;
                }
            }
            upper
        }

        let quota_used = binary_search(lower, max_quota, check_quota, precision);
        quota_used.to_big_endian(estimate_quota);
        Ok(estimate_quota.to_vec())
    }

    fn sign_call(&self, request: CallRequest) -> SignedTransaction {
        let from = request.from.unwrap_or_else(Address::zero);
        Transaction {
            nonce: "".to_string(),
            action: Action::Call(request.to),
            gas: U256::from(50_000_000),
            gas_price: U256::zero(),
            value: U256::zero(),
            data: request.data.map_or_else(Vec::new, |d| d.to_vec()),
            block_limit: u64::max_value(),
            chain_id: U256::default(),
            version: 0u32,
        }
        .fake_sign(from)
    }

    fn call(&self, t: &SignedTransaction, block_tag: BlockTag) -> Result<CitaExecuted, CallError> {
        let header = self.block_header(block_tag).ok_or(CallError::StatePruned)?;
        let last_hashes = self.build_last_hashes(Some(header.hash().unwrap()), header.number());
        let mut context = Context {
            block_number: header.number(),
            coin_base: *header.proposer(),
            timestamp: if self.eth_compatibility {
                header.timestamp() / 1000
            } else {
                header.timestamp()
            },
            difficulty: U256::default(),
            last_hashes: ::std::sync::Arc::new(last_hashes),
            quota_used: *header.quota_used(),
            block_quota_limit: *header.quota_limit(),
            account_quota_limit: u64::max_value().into(),
        };
        context.block_quota_limit = U256::from(self.sys_config.block_quota_limit);

        // FIXME: Need to implement state_at
        // that's just a copy of the state.
        //        let mut state = self.state_at(block_tag).ok_or(CallError::StatePruned)?;

        // Never check permission and quota
        let mut conf = self.sys_config.block_sys_config.clone();
        conf.exempt_checking();

        let block_data_provider = EVMBlockDataProvider::new(context.clone());

        let state_root = if let Some(h) = self.block_header(block_tag) {
            (*h.state_root())
        } else {
            error!("Can not get state root from trie db!");
            return Err(CallError::StatePruned);
        };

        let state = match CitaState::from_existing(
            Arc::<TrieDB<RocksDB>>::clone(&self.state_db),
            state_root,
        ) {
            Ok(state_db) => state_db,
            Err(e) => {
                error!("Can not get state from trie db! error: {:?}", e);
                return Err(CallError::StatePruned);
            }
        };

        let state = Arc::new(RefCell::new(state));
        CitaExecutive::new(
            Arc::new(block_data_provider),
            state,
            self.contracts_db.clone(),
            &context,
            conf.economical_model,
        )
        .exec(t, &conf)
        .map_err(Into::into)
    }

    fn chain_id(&self) -> Option<ChainId> {
        let version_manager = VersionManager::new(&self);
        let system_config = SysConfig::new(&self);
        system_config.deal_chain_id_version(&version_manager)
    }

    fn metadata(&self, data: String) -> Result<MetaData, String> {
        trace!("metadata request from jsonrpc {:?}", data);
        let economical_model: RpcEconomicalModel =
            (self.sys_config.block_sys_config.economical_model).into();
        let mut metadata = MetaData {
            chain_id: 0,
            chain_id_v1: U256::from(0).into(),
            chain_name: "".to_owned(),
            operator: "".to_owned(),
            website: "".to_owned(),
            genesis_timestamp: 0,
            validators: Vec::new(),
            block_interval: 0,
            token_name: "".to_owned(),
            token_symbol: "".to_owned(),
            token_avatar: "".to_owned(),
            version: 0,
            economical_model,
        };
        let result = serde_json::from_str::<RpcBlockNumber>(&data)
            .map_err(|err| format!("{:?}", err))
            .and_then(|number: RpcBlockNumber| {
                let current_height = self.get_current_height();
                let number = match number {
                    RpcBlockNumber::Tag(RpcBlockTag::Earliest) => 0,
                    RpcBlockNumber::Height(n) => n.into(),
                    RpcBlockNumber::Tag(RpcBlockTag::Latest) => current_height.saturating_sub(1),
                    RpcBlockNumber::Tag(RpcBlockTag::Pending) => current_height,
                };
                if number > current_height {
                    Err(format!(
                        "Block number overflow: {} > {}",
                        number, current_height
                    ))
                } else {
                    Ok(number)
                }
            })
            .and_then(|number| {
                let sys_config = SysConfig::new(&self);
                let block_tag = BlockTag::Height(number);
                sys_config
                    .chain_name(block_tag)
                    .map(|chain_name| metadata.chain_name = chain_name)
                    .ok_or_else(|| "Query chain name failed".to_owned())?;
                sys_config
                    .operator(block_tag)
                    .map(|operator| metadata.operator = operator)
                    .ok_or_else(|| "Query operator failed".to_owned())?;
                sys_config
                    .website(block_tag)
                    .map(|website| metadata.website = website)
                    .ok_or_else(|| "Query website failed".to_owned())?;
                self.block_header(BlockTag::Tag(Tag::Earliest))
                    .map(|header| metadata.genesis_timestamp = header.timestamp())
                    .ok_or_else(|| "Query genesis_timestamp failed".to_owned())?;
                self.node_manager()
                    .shuffled_stake_nodes(block_tag)
                    .map(|validators| {
                        metadata.validators =
                            validators.into_iter().map(Into::into).collect::<Vec<_>>()
                    })
                    .ok_or_else(|| "Query validators failed".to_owned())?;
                sys_config
                    .block_interval(block_tag)
                    .map(|block_interval| metadata.block_interval = block_interval)
                    .ok_or_else(|| "Query block_interval failed".to_owned())?;
                sys_config
                    .token_info(block_tag)
                    .map(|token_info| {
                        metadata.token_name = token_info.name;
                        metadata.token_avatar = token_info.avatar;
                        metadata.token_symbol = token_info.symbol;
                    })
                    .ok_or_else(|| "Query token info failed".to_owned())?;

                let version_manager = VersionManager::new(&self);
                metadata.version = version_manager
                    .get_version(block_tag)
                    .unwrap_or_else(VersionManager::default_version);

                sys_config
                    .deal_chain_id_version(&version_manager)
                    .map(|chain_id| match chain_id {
                        ChainId::V0(v0) => metadata.chain_id = v0,
                        ChainId::V1(v1) => metadata.chain_id_v1 = v1.into(),
                    })
                    .ok_or_else(|| "Query chain id failed".to_owned())?;
                Ok(())
            });
        match result {
            Ok(()) => Ok(metadata),
            Err(err) => Err(err),
        }
    }

    fn economical_model(&self) -> EconomicalModel {
        self.sys_config.block_sys_config.economical_model
    }

    fn load_executed_result(&self, height: u64) -> ExecutedResult {
        self.executed_result_by_height(height)
    }

    fn grow(&mut self, closed_block: &ClosedBlock) -> ExecutedResult {
        info!(
            "executor grow according to ClosedBlock(height: {}, hash: {:?}, parent_hash: {:?}, \
             timestamp: {}, state_root: {:?}, transaction_root: {:?}, proposer: {:?})",
            closed_block.number(),
            closed_block.hash().unwrap(),
            closed_block.parent_hash(),
            closed_block.timestamp(),
            closed_block.state_root(),
            closed_block.transactions_root(),
            closed_block.proposer(),
        );
        let are_permissions_changed = {
            let cache = closed_block.state.cache.clone();
            let permission_management = PermissionManagement::new(self);
            let permissions =
                permission_management.permission_addresses(BlockTag::Tag(Tag::Pending));
            cache.into_inner().iter().any(|(address, ref _a)| {
                &address.lower_hex()[..34] == "ffffffffffffffffffffffffffffffffff"
                    || permissions.contains(&address)
            })
        };

        {
            *self.current_header.write() = closed_block.header().clone();
        }

        let executed_info = closed_block.protobuf();

        // Must make sure write into database before load_sys_config
        self.write_batch(closed_block);

        if are_permissions_changed {
            trace!("Permissions changed, reload global sys config.");
            self.sys_config = GlobalSysConfig::load(&self, BlockTag::Tag(Tag::Pending));
        }
        let mut executed_result = ExecutedResult::new();
        let consensus_config = make_consensus_config(self.sys_config.clone());
        executed_result.set_config(consensus_config);
        executed_result.set_executed_info(executed_info);
        executed_result
    }

    fn exit(&mut self, rollback_id: BlockTag) {
        self.rollback_current_height(rollback_id);
        self.close();
    }

    fn clone_executor_reader(&mut self) -> Self {
        let current_header = self.current_header.read().clone();
        let state_db = self.state_db.clone();
        let db = self.db.clone();
        let contracts_db = self.contracts_db.clone();
        // let fake_parent_hash: H256 = Default::default();
        let sys_config = self.sys_config.clone();
        let fsm_req_receiver = self.fsm_req_receiver.clone();
        let fsm_resp_sender = self.fsm_resp_sender.clone();
        let command_req_receiver = self.command_req_receiver.clone();
        let command_resp_sender = self.command_resp_sender.clone();
        let eth_compatibility = self.eth_compatibility;
        Executor {
            current_header: RwLock::new(current_header),
            state_db,
            db,
            contracts_db,
            sys_config,
            fsm_req_receiver,
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
            eth_compatibility,
        }
    }
}

// TODO hope someone refactor these public function via macro

pub fn state_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    block_tag: BlockTag,
) -> Option<CitaState<CitaTrieDB>> {
    let _ = command_req_sender.send(Command::StateAt(block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::StateAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn gen_state(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    root: H256,
    parent_hash: H256,
) -> Option<CitaState<CitaTrieDB>> {
    let _ = command_req_sender.send(Command::GenState(root, parent_hash));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::GenState(r) => r,
        _ => unimplemented!(),
    }
}

pub fn code_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_tag: BlockTag,
) -> Option<Bytes> {
    let _ = command_req_sender.send(Command::CodeAt(address, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::CodeAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn abi_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_tag: BlockTag,
) -> Option<Bytes> {
    let _ = command_req_sender.send(Command::ABIAt(address, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::ABIAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn balance_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_tag: BlockTag,
) -> Option<Bytes> {
    let _ = command_req_sender.send(Command::BalanceAt(address, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::BalanceAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn nonce_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_tag: BlockTag,
) -> Option<U256> {
    let _ = command_req_sender.send(Command::NonceAt(address, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::NonceAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn eth_call(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    call_request: CallRequest,
    block_tag: BlockTag,
) -> Result<Bytes, String> {
    let _ = command_req_sender.send(Command::ETHCall(call_request, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::ETHCall(r) => r,
        _ => unimplemented!(),
    }
}

pub fn estimate_quota(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    call_request: CallRequest,
    block_tag: BlockTag,
) -> Result<Bytes, String> {
    let _ = command_req_sender.send(Command::EstimateQuota(call_request, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::EstimateQuota(r) => r,
        _ => unimplemented!(),
    }
}

pub fn sign_call(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    call_request: CallRequest,
) -> SignedTransaction {
    let _ = command_req_sender.send(Command::SignCall(call_request));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::SignCall(r) => r,
        _ => unimplemented!(),
    }
}

pub fn call(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    signed_transaction: SignedTransaction,

    block_tag: BlockTag,
) -> Result<CitaExecuted, CallError> {
    let _ = command_req_sender.send(Command::Call(signed_transaction, block_tag));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Call(r) => r,
        _ => unimplemented!(),
    }
}

pub fn chain_id(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
) -> Option<ChainId> {
    let _ = command_req_sender.send(Command::ChainID);
    match command_resp_receiver.recv().unwrap() {
        CommandResp::ChainID(r) => r,
        _ => unimplemented!(),
    }
}

pub fn metadata(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    data: String,
) -> Result<MetaData, String> {
    let _ = command_req_sender.send(Command::Metadata(data));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Metadata(r) => r,
        _ => unimplemented!(),
    }
}

pub fn economical_model(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
) -> EconomicalModel {
    let _ = command_req_sender.send(Command::EconomicalModel);
    match command_resp_receiver.recv().unwrap() {
        CommandResp::EconomicalModel(r) => r,
        _ => unimplemented!(),
    }
}

pub fn load_executed_result(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    height: u64,
) -> ExecutedResult {
    let _ = command_req_sender.send(Command::LoadExecutedResult(height));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::LoadExecutedResult(r) => r,
        _ => unimplemented!(),
    }
}

pub fn grow(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    closed_block: ClosedBlock,
) -> ExecutedResult {
    let _ = command_req_sender.send(Command::Grow(closed_block));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Grow(r) => r,
        _ => unimplemented!(),
    }
}

pub fn exit(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    rollback_id: BlockTag,
) {
    let _ = command_req_sender.send(Command::Exit(rollback_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Exit => {}
        _ => unimplemented!(),
    }
}

pub fn clone_executor_reader(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
) -> Executor {
    let _ = command_req_sender.send(Command::CloneExecutorReader);
    match command_resp_receiver.recv().unwrap() {
        CommandResp::CloneExecutorReader(r) => r,
        _ => unimplemented!(),
    }
}
