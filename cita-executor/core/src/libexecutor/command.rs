// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub use byteorder::{BigEndian, ByteOrder};
use call_analytics::CallAnalytics;
use contracts::solc::{sys_config::ChainId, PermissionManagement, SysConfig, VersionManager};
use error::CallError;
use evm::env_info::EnvInfo;
use executive::{Executed, Executive, TransactOptions};
pub use libexecutor::block::*;
use libexecutor::call_request::CallRequest;
use serde_json;
use std::fmt;

use jsonrpc_types::rpctypes::{
    BlockNumber, BlockTag, EconomicalModel as RpcEconomicalModel, MetaData,
};

use super::economical_model::EconomicalModel;
use super::executor::{make_consensus_config, Executor};
use super::sys_config::GlobalSysConfig;
use cita_types::traits::LowerHex;
use cita_types::{Address, H256, U256};
use crossbeam_channel::{Receiver, Sender};
use libproto::ExecutedResult;
use state::State;
use state_db::StateDB;
use std::convert::{From, Into};
use types::ids::BlockId;
use types::transaction::{Action, SignedTransaction, Transaction};
use util::Bytes;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum Command {
    StateAt(BlockId),
    GenState(H256, H256),
    CodeAt(Address, BlockId),
    ABIAt(Address, BlockId),
    BalanceAt(Address, BlockId),
    NonceAt(Address, BlockId),
    ETHCall(CallRequest, BlockId),
    SignCall(CallRequest),
    Call(SignedTransaction, BlockId, CallAnalytics),
    ChainID,
    Metadata(String),
    EconomicalModel,
    LoadExecutedResult(u64),
    Grow(ClosedBlock),
    Exit(BlockId),
}

#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum CommandResp {
    StateAt(Option<State<StateDB>>),
    GenState(Option<State<StateDB>>),
    CodeAt(Option<Bytes>),
    ABIAt(Option<Bytes>),
    BalanceAt(Option<Bytes>),
    NonceAt(Option<U256>),
    ETHCall(Result<Bytes, String>),
    SignCall(SignedTransaction),
    Call(Result<Executed, CallError>),
    ChainID(Option<ChainId>),
    Metadata(Result<MetaData, String>),
    EconomicalModel(EconomicalModel),
    LoadExecutedResult(ExecutedResult),
    Grow(ExecutedResult),
    Exit,
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
            Command::SignCall(_) => write!(f, "Command::SignCall"),
            Command::Call(_, _, _) => write!(f, "Command::Call"),
            Command::ChainID => write!(f, "Command::ChainID "),
            Command::Metadata(_) => write!(f, "Command::Metadata"),
            Command::EconomicalModel => write!(f, "Command::EconomicalModel"),
            Command::LoadExecutedResult(_) => write!(f, "Command::LoadExecutedResult"),
            Command::Grow(_) => write!(f, "Command::Grow"),
            Command::Exit(_) => write!(f, "Command::Exit"),
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
            CommandResp::SignCall(_) => write!(f, "CommandResp::SignCall"),
            CommandResp::Call(_) => write!(f, "CommandResp::Call"),
            CommandResp::ChainID(_) => write!(f, "CommandResp::ChainID "),
            CommandResp::Metadata(_) => write!(f, "CommandResp::Metadata"),
            CommandResp::EconomicalModel(_) => write!(f, "CommandResp::EconomicalModel"),
            CommandResp::LoadExecutedResult(_) => write!(f, "CommandResp::LoadExecutedResult"),
            CommandResp::Grow(_) => write!(f, "CommandResp::Grow"),
            CommandResp::Exit => write!(f, "CommandResp::Exit"),
        }
    }
}

pub trait Commander {
    fn operate(&mut self, command: Command) -> CommandResp;
    fn state_at(&self, block_id: BlockId) -> Option<State<StateDB>>;
    fn gen_state(&self, root: H256, parent_hash: H256) -> Option<State<StateDB>>;
    fn code_at(&self, address: &Address, block_id: BlockId) -> Option<Bytes>;
    fn abi_at(&self, address: &Address, block_id: BlockId) -> Option<Bytes>;
    fn balance_at(&self, address: &Address, block_id: BlockId) -> Option<Bytes>;
    fn nonce_at(&self, address: &Address, block_id: BlockId) -> Option<U256>;
    fn eth_call(&self, request: CallRequest, block_id: BlockId) -> Result<Bytes, String>;
    fn sign_call(&self, request: CallRequest) -> SignedTransaction;
    fn call(
        &self,
        t: &SignedTransaction,
        block_id: BlockId,
        analytics: CallAnalytics,
    ) -> Result<Executed, CallError>;
    fn chain_id(&self) -> Option<ChainId>;
    fn metadata(&self, data: String) -> Result<MetaData, String>;
    fn economical_model(&self) -> EconomicalModel;
    fn load_executed_result(&self, height: u64) -> ExecutedResult;
    fn grow(&mut self, closed_block: ClosedBlock) -> ExecutedResult;
    fn exit(&mut self, rollback_id: BlockId);
}

impl Commander for Executor {
    fn operate(&mut self, command: Command) -> CommandResp {
        match command {
            Command::StateAt(block_id) => CommandResp::StateAt(self.state_at(block_id)),
            Command::GenState(root, parent_hash) => {
                CommandResp::GenState(self.gen_state(root, parent_hash))
            }
            Command::CodeAt(address, block_id) => {
                CommandResp::CodeAt(self.code_at(&address, block_id))
            }
            Command::ABIAt(address, block_id) => {
                CommandResp::ABIAt(self.abi_at(&address, block_id))
            }
            Command::BalanceAt(address, block_id) => {
                CommandResp::BalanceAt(self.balance_at(&address, block_id))
            }
            Command::NonceAt(address, block_id) => {
                CommandResp::NonceAt(self.nonce_at(&address, block_id))
            }
            Command::ETHCall(call_request, block_id) => {
                CommandResp::ETHCall(self.eth_call(call_request, block_id))
            }
            Command::SignCall(call_request) => CommandResp::SignCall(self.sign_call(call_request)),
            Command::Call(signed_transaction, block_id, call_analytics) => {
                CommandResp::Call(self.call(&signed_transaction, block_id, call_analytics))
            }
            Command::ChainID => CommandResp::ChainID(self.chain_id()),
            Command::Metadata(data) => CommandResp::Metadata(self.metadata(data)),
            Command::EconomicalModel => CommandResp::EconomicalModel(self.economical_model()),
            Command::LoadExecutedResult(height) => {
                CommandResp::LoadExecutedResult(self.load_executed_result(height))
            }
            Command::Grow(closed_block) => CommandResp::Grow(self.grow(closed_block)),
            Command::Exit(rollback_id) => {
                self.exit(rollback_id);
                CommandResp::Exit
            }
        }
    }

    /// Attempt to get a copy of a specific block's final state.
    fn state_at(&self, id: BlockId) -> Option<State<StateDB>> {
        self.block_header(id)
            .and_then(|h| self.gen_state(*h.state_root(), *h.parent_hash()))
    }

    /// Generate block's final state.
    fn gen_state(&self, root: H256, parent_hash: H256) -> Option<State<StateDB>> {
        let db = self.state_db.read().boxed_clone_canon(&parent_hash);
        State::from_existing(db, root, U256::from(0), self.factories.clone()).ok()
    }

    /// Get code by address
    fn code_at(&self, address: &Address, id: BlockId) -> Option<Bytes> {
        self.state_at(id)
            .and_then(|s| s.code(address).ok())
            .and_then(|c| c.map(|c| (&*c).clone()))
    }

    /// Get abi by address
    fn abi_at(&self, address: &Address, id: BlockId) -> Option<Bytes> {
        self.state_at(id)
            .and_then(|s| s.abi(address).ok())
            .and_then(|c| c.map(|c| (&*c).clone()))
    }

    /// Get balance by address
    fn balance_at(&self, address: &Address, id: BlockId) -> Option<Bytes> {
        self.state_at(id)
            .and_then(|s| s.balance(address).ok())
            .map(|c| {
                let mut bytes = [0u8; 32];
                c.to_big_endian(&mut bytes);
                bytes.to_vec()
            })
    }

    fn nonce_at(&self, address: &Address, id: BlockId) -> Option<U256> {
        self.state_at(id).and_then(|s| s.nonce(address).ok())
    }

    fn eth_call(&self, request: CallRequest, id: BlockId) -> Result<Bytes, String> {
        let signed = self.sign_call(request);
        let result = self.call(&signed, id, Default::default());
        result
            .map(|b| b.output)
            .or_else(|e| Err(format!("Call Error {}", e)))
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
            // TODO: Should Fixed?
            chain_id: U256::default(),
            version: 0u32,
        }
        .fake_sign(from)
    }

    fn call(
        &self,
        t: &SignedTransaction,
        block_id: BlockId,
        analytics: CallAnalytics,
    ) -> Result<Executed, CallError> {
        let header = self.block_header(block_id).ok_or(CallError::StatePruned)?;
        let last_hashes = self.build_last_hashes(Some(header.hash().unwrap()), header.number());
        let env_info = EnvInfo {
            number: header.number(),
            author: *header.proposer(),
            timestamp: header.timestamp(),
            difficulty: U256::default(),
            last_hashes: ::std::sync::Arc::new(last_hashes),
            gas_used: *header.quota_used(),
            gas_limit: *header.quota_limit(),
            account_gas_limit: u64::max_value().into(),
        };
        // that's just a copy of the state.
        let mut state = self.state_at(block_id).ok_or(CallError::StatePruned)?;

        let options = TransactOptions {
            tracing: analytics.transaction_tracing,
            vm_tracing: analytics.vm_tracing,
        };

        // Never check permission and quota
        let mut conf = self.sys_config.block_sys_config.clone();
        conf.exempt_checking();

        Executive::new(
            &mut state,
            &env_info,
            &*self.engine,
            &self.factories.vm,
            &self.factories.native,
            false,
            EconomicalModel::Quota,
        )
        .transact(t, options, &conf)
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
            chain_id_v1: U256::from(0).lower_hex(),
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
        let result = serde_json::from_str::<BlockNumber>(&data)
            .map_err(|err| format!("{:?}", err))
            .and_then(|number: BlockNumber| {
                let current_height = self.get_current_height();
                let number = match number {
                    BlockNumber::Tag(BlockTag::Earliest) => 0,
                    BlockNumber::Height(n) => n.into(),
                    BlockNumber::Tag(BlockTag::Latest) => current_height.saturating_sub(1),
                    BlockNumber::Tag(BlockTag::Pending) => current_height,
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
                let block_id = BlockId::Number(number);
                sys_config
                    .chain_name(block_id)
                    .map(|chain_name| metadata.chain_name = chain_name)
                    .ok_or_else(|| "Query chain name failed".to_owned())?;
                sys_config
                    .operator(block_id)
                    .map(|operator| metadata.operator = operator)
                    .ok_or_else(|| "Query operator failed".to_owned())?;
                sys_config
                    .website(block_id)
                    .map(|website| metadata.website = website)
                    .ok_or_else(|| "Query website failed".to_owned())?;
                self.block_header(BlockId::Earliest)
                    .map(|header| metadata.genesis_timestamp = header.timestamp())
                    .ok_or_else(|| "Query genesis_timestamp failed".to_owned())?;
                self.node_manager()
                    .shuffled_stake_nodes(block_id)
                    .map(|validators| metadata.validators = validators)
                    .ok_or_else(|| "Query validators failed".to_owned())?;
                sys_config
                    .block_interval(block_id)
                    .map(|block_interval| metadata.block_interval = block_interval)
                    .ok_or_else(|| "Query block_interval failed".to_owned())?;
                sys_config
                    .token_info(block_id)
                    .map(|token_info| {
                        metadata.token_name = token_info.name;
                        metadata.token_avatar = token_info.avatar;
                        metadata.token_symbol = token_info.symbol;
                    })
                    .ok_or_else(|| "Query token info failed".to_owned())?;

                let version_manager = VersionManager::new(&self);
                metadata.version = version_manager
                    .get_version(block_id)
                    .unwrap_or_else(VersionManager::default_version);

                sys_config
                    .deal_chain_id_version(&version_manager)
                    .map(|chain_id| match chain_id {
                        ChainId::V0(v0) => metadata.chain_id = v0,
                        ChainId::V1(v1) => metadata.chain_id_v1 = v1.lower_hex(),
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

    fn grow(&mut self, closed_block: ClosedBlock) -> ExecutedResult {
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
            let cache = closed_block.state.cache();
            let permission_management = PermissionManagement::new(self);
            let permissions = permission_management.permission_addresses(BlockId::Pending);
            cache.iter().any(|(address, ref _a)| {
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
            self.sys_config = GlobalSysConfig::load(&self, BlockId::Pending);
        }
        let mut executed_result = ExecutedResult::new();
        let consensus_config = make_consensus_config(self.sys_config.clone());
        executed_result.set_config(consensus_config);
        executed_result.set_executed_info(executed_info);
        executed_result
    }

    fn exit(&mut self, rollback_id: BlockId) {
        self.rollback_current_height(rollback_id);
        self.close();
    }
}

// TODO hope someone refactor these public function via macro

pub fn state_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    block_id: BlockId,
) -> Option<State<StateDB>> {
    command_req_sender.send(Command::StateAt(block_id));
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
) -> Option<State<StateDB>> {
    command_req_sender.send(Command::GenState(root, parent_hash));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::GenState(r) => r,
        _ => unimplemented!(),
    }
}

pub fn code_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_id: BlockId,
) -> Option<Bytes> {
    command_req_sender.send(Command::CodeAt(address, block_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::CodeAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn abi_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_id: BlockId,
) -> Option<Bytes> {
    command_req_sender.send(Command::ABIAt(address, block_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::ABIAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn balance_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_id: BlockId,
) -> Option<Bytes> {
    command_req_sender.send(Command::BalanceAt(address, block_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::BalanceAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn nonce_at(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    address: Address,
    block_id: BlockId,
) -> Option<U256> {
    command_req_sender.send(Command::NonceAt(address, block_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::NonceAt(r) => r,
        _ => unimplemented!(),
    }
}

pub fn eth_call(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    call_request: CallRequest,
    block_id: BlockId,
) -> Result<Bytes, String> {
    command_req_sender.send(Command::ETHCall(call_request, block_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::ETHCall(r) => r,
        _ => unimplemented!(),
    }
}

pub fn sign_call(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    call_request: CallRequest,
) -> SignedTransaction {
    command_req_sender.send(Command::SignCall(call_request));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::SignCall(r) => r,
        _ => unimplemented!(),
    }
}

pub fn call(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    signed_transaction: SignedTransaction,
    block_id: BlockId,
    call_analytics: CallAnalytics,
) -> Result<Executed, CallError> {
    command_req_sender.send(Command::Call(signed_transaction, block_id, call_analytics));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Call(r) => r,
        _ => unimplemented!(),
    }
}

pub fn chain_id(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
) -> Option<ChainId> {
    command_req_sender.send(Command::ChainID);
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
    command_req_sender.send(Command::Metadata(data));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Metadata(r) => r,
        _ => unimplemented!(),
    }
}

pub fn economical_model(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
) -> EconomicalModel {
    command_req_sender.send(Command::EconomicalModel);
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
    command_req_sender.send(Command::LoadExecutedResult(height));
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
    command_req_sender.send(Command::Grow(closed_block));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Grow(r) => r,
        _ => unimplemented!(),
    }
}

pub fn exit(
    command_req_sender: &Sender<Command>,
    command_resp_receiver: &Receiver<CommandResp>,
    rollback_id: BlockId,
) -> () {
    command_req_sender.send(Command::Exit(rollback_id));
    match command_resp_receiver.recv().unwrap() {
        CommandResp::Exit => (),
        _ => unimplemented!(),
    }
}
