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

use cita_types::traits::LowerHex;
use cita_types::{Address, H256, U256};
use core::contracts::solc::sys_config::SysConfig;
use core::contracts::solc::VersionManager;
use core::db;
use core::libexecutor::block::{Block, ClosedBlock};
use core::libexecutor::call_request::CallRequest;
use core::libexecutor::executor::{BlockInQueue, Config, Executor, Stage};
use core::libexecutor::Genesis;
use error::ErrorCode;
use jsonrpc_types::rpctypes::{BlockNumber, BlockTag, CountOrCode, EconomicalModel, MetaData};
use libproto::auth::Miscellaneous;
use libproto::blockchain::{BlockWithProof, Proof, ProofType, RichStatus, StateSignal};
use libproto::consensus::SignedProposal;
use libproto::request::Request_oneof_req as Request;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
use libproto::{request, response, Message, SyncResponse};
use proof::BftProof;
use serde_json;
use std::cell::RefCell;
use std::convert::{Into, TryFrom, TryInto};
use std::fs::File;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::{mem, u8};
use types::ids::BlockId;
use util::datapath::DataPath;
use util::journaldb::Algorithm;
use util::kvdb::{Database, DatabaseConfig};

use core::snapshot;
use core::snapshot::io::{PackedReader, PackedWriter};
use core::snapshot::service::{Service as SnapshotService, ServiceParams as SnapServiceParams};
use core::snapshot::Progress;
use core::state::backend::Backend;
use std::path::Path;

#[derive(Clone)]
pub struct ExecutorInstance {
    ctx_pub: Sender<(String, Vec<u8>)>,
    write_sender: Sender<u64>,
    pub ext: Arc<Executor>,
    pub grpc_port: u16,
    closed_block: RefCell<Option<ClosedBlock>>,
    pub is_snapshot: bool,
}

impl ExecutorInstance {
    pub fn new(
        ctx_pub: Sender<(String, Vec<u8>)>,
        write_sender: Sender<u64>,
        config_path: &str,
        genesis_path: &str,
    ) -> Self {
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let nosql_path = DataPath::root_node_path() + "/statedb";
        let db = Database::open(&config, &nosql_path).unwrap();

        let genesis = Genesis::init(genesis_path);

        let executor_config = Config::new(config_path);
        let grpc_port = executor_config.grpc_port;
        let executor = Executor::init_executor(Arc::new(db), genesis, &executor_config);
        let executor = Arc::new(executor);
        // send init executed info only have ConsensusConfig
        executor.send_executed_info_to_chain(executor.get_max_height(), &ctx_pub);
        ExecutorInstance {
            ctx_pub,
            write_sender,
            ext: executor,
            grpc_port,
            closed_block: RefCell::new(None),
            is_snapshot: false,
        }
    }

    pub fn distribute_msg(&mut self, key: &str, msg_vec: &[u8]) {
        let mut msg = Message::try_from(msg_vec).unwrap();
        let origin = msg.get_origin();
        trace!("distribute_msg call key = {}, origin = {}", key, origin);
        match RoutingKey::from(key) {
            routing_key!(Auth >> MiscellaneousReq) => {
                self.get_auth_miscellaneous();
            }

            routing_key!(Chain >> Request) => {
                let req = msg.take_request().unwrap();
                self.reply_request(req);
            }

            routing_key!(Chain >> RichStatus) => {
                if let Some(status) = msg.take_rich_status() {
                    self.execute_chain_status(&status);
                };
            }

            routing_key!(Chain >> StateSignal) => {
                if let Some(state_signal) = msg.take_state_signal() {
                    let specified_height = state_signal.get_height();
                    if specified_height < self.ext.get_current_height() {
                        self.ext
                            .send_executed_info_to_chain(specified_height + 1, &self.ctx_pub);
                        let executed_result = {
                            let executed_result = self.ext.executed_result.read();
                            executed_result.clone()
                        };
                        for height in executed_result.keys() {
                            if *height > specified_height + 1 {
                                self.ext.send_executed_info_to_chain(*height, &self.ctx_pub);
                            }
                        }
                    } else if specified_height > self.ext.get_current_height() {
                        self.signal_to_chain(&self.ctx_pub);
                    }
                }
            }

            routing_key!(Consensus >> BlockWithProof) => {
                let proof_blk = msg.take_block_with_proof().unwrap();
                self.consensus_block_enqueue(proof_blk);
            }

            routing_key!(Net >> SyncResponse) | routing_key!(Chain >> LocalSync) => {
                let sync_res = msg.take_sync_response().unwrap();
                self.deal_sync_blocks(sync_res);
            }

            routing_key!(Consensus >> SignedProposal) | routing_key!(Net >> SignedProposal) => {
                if !self.ext.is_sync.load(Ordering::SeqCst) {
                    let signed_proposal = msg.take_signed_proposal().unwrap();
                    self.proposal_enqueue(signed_proposal);
                } else {
                    debug!("receive proposal while sync");
                }
            }

            routing_key!(Snapshot >> SnapshotReq) => {
                let snapshot_req = msg.take_snapshot_req().unwrap();
                self.deal_snapshot_req(&snapshot_req);
            }

            _ => {
                error!("dispatch msg found error key {}!!!!", key);
            }
        }
    }

    fn is_dup_block(&self, inum: u64) -> bool {
        inum <= self.ext.get_current_height()
    }

    /// TODO: Move to a separated file
    /// execute block transaction
    pub fn execute_block(&self, number: u64) {
        let block_in_queue = {
            let block_map = self.ext.block_map.read();
            block_map.get(&number).cloned()
        };

        let stage = { self.ext.stage.read().clone() };
        let mut need_clean_map = false;

        match block_in_queue {
            Some(BlockInQueue::ConsensusBlock(block, _)) => {
                if self.ext.validate_height(block.number())
                    && self.ext.validate_hash(block.parent_hash())
                {
                    // Not Match before proposal
                    // TODO: check proposal transaction root is eq block transaction root
                    if self.ext.is_interrupted.load(Ordering::SeqCst) {
                        self.ext.is_interrupted.store(false, Ordering::SeqCst);
                        {
                            *self.ext.stage.write() = Stage::ExecutingBlock;
                        }
                        self.ext.execute_block(block, &self.ctx_pub);
                    } else {
                        match stage {
                            // Match before proposal
                            Stage::WaitFinalized => {
                                {
                                    *self.ext.stage.write() = Stage::ExecutingBlock;
                                }
                                match self.closed_block.replace(None).and_then(|closed_block| {
                                    if closed_block.is_equivalent(&block) {
                                        Some(closed_block)
                                    } else {
                                        None
                                    }
                                }) {
                                    Some(closed_block) => {
                                        self.ext.finalize_proposal(
                                            closed_block,
                                            &block,
                                            &self.ctx_pub,
                                        );
                                    }
                                    _ => {
                                        self.ext.execute_block(block, &self.ctx_pub);
                                    }
                                }
                            }
                            // Not receive proposal
                            Stage::Idle => {
                                {
                                    *self.ext.stage.write() = Stage::ExecutingBlock;
                                }
                                self.ext.execute_block(block, &self.ctx_pub);
                            }
                            _ => {
                                // Maybe never reach
                                warn!(
                                    "something wrong, comming consensus block, but wrong stage {:?}",
                                    stage
                                );
                            }
                        }
                    }
                    {
                        *self.ext.stage.write() = Stage::Idle;
                    }
                    info!("execute consensus block [height {}] finish !", number);
                    need_clean_map = true;
                }
            }
            Some(BlockInQueue::SyncBlock((_, Some(_)))) => {
                if number == self.ext.get_current_height() + 1 {
                    {
                        *self.ext.stage.write() = Stage::ExecutingBlock;
                    }
                    self.sync_blocks(number);
                    {
                        *self.ext.stage.write() = Stage::Idle;
                    }
                    need_clean_map = true;
                };
            }
            // State must be Idle or WaitFinalized after executed proposal
            Some(BlockInQueue::Proposal(proposal)) => {
                // Interrupt pre proposal
                if self.ext.is_interrupted.load(Ordering::SeqCst) {
                    self.ext.is_interrupted.store(false, Ordering::SeqCst);
                }
                {
                    *self.ext.stage.write() = Stage::ExecutingProposal;
                }
                if let Some(closed_block) = self.ext.execute_proposal(proposal) {
                    // Interrupted by latest proposal/consensus block
                    if self.ext.is_interrupted.load(Ordering::SeqCst) {
                        self.ext.is_interrupted.store(false, Ordering::SeqCst);
                        *self.ext.stage.write() = Stage::Idle;
                        return;
                    }
                    // After execute proposal, check whether block-in-map is changed
                    let in_queue = {
                        let block_map = self.ext.block_map.read();
                        block_map.get(&number).cloned()
                    };
                    match in_queue {
                        Some(BlockInQueue::ConsensusBlock(coming, _)) => {
                            if coming.is_equivalent(&closed_block) {
                                self.ext
                                    .finalize_proposal(closed_block, &coming, &self.ctx_pub);
                                info!("execute proposal block [height {}] finish !", number);
                            } else {
                                // Maybe never reach
                                warn!("something is wrong when execute proposal block")
                            }
                            {
                                *self.ext.stage.write() = Stage::Idle;
                            }
                        }
                        Some(BlockInQueue::Proposal(_)) => {
                            let mut cb = self.closed_block.borrow_mut();
                            *cb = Some(closed_block);
                            *self.ext.stage.write() = Stage::WaitFinalized;
                            debug!("wait finalized");
                        }
                        _ => {
                            // Maybe never reach
                            warn!("Block in queue is wrong, go into no-man's-land");
                        }
                    }
                } else {
                    {
                        *self.ext.stage.write() = Stage::Idle;
                    }
                    warn!("executing proposal is interrupted.");
                }
            }
            _ => {
                warn!("block-{} in queue is without proof", number);
            }
        }

        if need_clean_map {
            let mut guard = self.ext.block_map.write();
            let new_map = guard.split_off(&self.ext.get_current_height());
            *guard = new_map;
        }
    }

    fn get_auth_miscellaneous(&self) {
        let version_manager = VersionManager::new(&self.ext);

        let sys_config = SysConfig::new(&self.ext);
        let mut miscellaneous = Miscellaneous::new();

        if let Some(chain_id) = sys_config.deal_chain_id_version(&version_manager) {
            miscellaneous.set_chain_id(chain_id.id_v0);
            miscellaneous.set_chain_id_v1(<[u8; 32]>::from(chain_id.id_v1).to_vec());
            trace!(
                "miscellaneous msg, chain_id: id_v0 = {}, id_v1 = {}",
                chain_id.id_v0,
                chain_id.id_v1
            );
        }

        let msg: Message = miscellaneous.into();
        self.ctx_pub
            .send((
                routing_key!(Executor >> Miscellaneous).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn reply_request(&self, mut req: request::Request) {
        let mut response = response::Response::new();
        response.set_request_id(req.take_request_id());

        match req.req.unwrap() {
            Request::call(call) => {
                trace!("Chainvm Call {:?}", call);
                let _ = serde_json::from_str::<BlockNumber>(&call.height)
                    .map(|block_id| {
                        let call_request = CallRequest::from(call);
                        self.ext
                            .eth_call(call_request, block_id.into())
                            .map(|ok| {
                                response.set_call_result(ok);
                            })
                            .map_err(|err| {
                                response.set_code(ErrorCode::query_error());
                                response.set_error_msg(err);
                            })
                    })
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    });
            }

            Request::transaction_count(tx_count) => {
                trace!("transaction count request from jsonrpc {:?}", tx_count);
                let _ = serde_json::from_str::<CountOrCode>(&tx_count)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|tx_count| {
                        let address = Address::from_slice(tx_count.address.as_ref());
                        match self.ext.nonce(&address, tx_count.block_id.into()) {
                            Some(nonce) => {
                                response.set_transaction_count(u64::from(nonce));
                            }
                            None => {
                                response.set_transaction_count(0);
                            }
                        };
                    });
            }

            Request::code(code_content) => {
                trace!("code request from jsonrpc  {:?}", code_content);
                let _ = serde_json::from_str::<CountOrCode>(&code_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|code_content| {
                        let address = Address::from_slice(code_content.address.as_ref());
                        if let Some(code) = self.ext.code_at(&address, code_content.block_id.into())
                        {
                            response.set_contract_code(code);
                        } else {
                            response.set_contract_code(vec![]);
                        };
                    });
            }

            Request::abi(abi_content) => {
                trace!("abi request from jsonrpc  {:?}", abi_content);
                let _ = serde_json::from_str::<CountOrCode>(&abi_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|abi_content| {
                        let address = Address::from_slice(abi_content.address.as_ref());
                        if let Some(abi) = self.ext.abi_at(&address, abi_content.block_id.into()) {
                            response.set_contract_abi(abi);
                        } else {
                            response.set_contract_abi(vec![]);
                        };
                    });
            }

            Request::balance(balance_content) => {
                trace!("balance request from jsonrpc  {:?}", balance_content);
                let _ = serde_json::from_str::<CountOrCode>(&balance_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|balance_content| {
                        let address = Address::from_slice(balance_content.address.as_ref());
                        if let Some(balance) = self
                            .ext
                            .balance_at(&address, balance_content.block_id.into())
                        {
                            response.set_balance(balance);
                        } else {
                            response.set_balance(vec![]);
                        };
                    });
            }

            Request::meta_data(data) => {
                trace!("metadata request from jsonrpc {:?}", data);
                let economical_model: EconomicalModel = (*self.ext.economical_model.read()).into();
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
                        let current_height = self.ext.get_current_height();
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
                        let sys_config = SysConfig::new(&self.ext);
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
                        self.ext
                            .block_header(BlockId::Earliest)
                            .map(|header| metadata.genesis_timestamp = header.timestamp())
                            .ok_or_else(|| "Query genesis_timestamp failed".to_owned())?;
                        self.ext
                            .node_manager()
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

                        let version_manager = VersionManager::new(&*self.ext);
                        metadata.version = version_manager
                            .get_version(block_id)
                            .unwrap_or_else(VersionManager::default_version);

                        sys_config
                            .deal_chain_id_version(&version_manager)
                            .map(|chain_id| {
                                metadata.chain_id = chain_id.id_v0;
                                metadata.chain_id_v1 = chain_id.id_v1.lower_hex()
                            })
                            .ok_or_else(|| "Query chain id failed".to_owned())?;
                        Ok(())
                    });
                match result {
                    Ok(_) => response.set_meta_data(serde_json::to_string(&metadata).unwrap()),
                    Err(error_msg) => {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(error_msg);
                    }
                }
            }

            Request::state_proof(state_info) => {
                trace!("state_proof info is {:?}", state_info);
                let _ = serde_json::from_str::<BlockNumber>(&state_info.height)
                    .map(|block_id| {
                        match self.ext.state_at(block_id.into()).and_then(|state| {
                            state.get_state_proof(
                                &Address::from(state_info.get_address()),
                                &H256::from(state_info.get_position()),
                            )
                        }) {
                            Some(state_proof_bs) => {
                                response.set_state_proof(state_proof_bs);
                            }
                            None => {
                                response.set_code(ErrorCode::query_error());
                                response.set_error_msg("get state proof failed".to_string());
                            }
                        }
                    })
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    });
            }
            Request::storage_key(skey) => {
                trace!("storage key info is {:?}", skey);
                let _ = serde_json::from_str::<BlockNumber>(&skey.height)
                    .map(|block_id| {
                        match self.ext.state_at(block_id.into()).and_then(|state| {
                            state
                                .storage_at(
                                    &Address::from(skey.get_address()),
                                    &H256::from(skey.get_position()),
                                )
                                .ok()
                        }) {
                            Some(storage_val) => {
                                response.set_storage_value(storage_val.to_vec());
                            }
                            None => {
                                response.set_code(ErrorCode::query_error());
                                response
                                    .set_error_msg("get storage at something failed".to_string());
                            }
                        }
                    })
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    });
            }

            _ => {
                error!("bad request msg!!!!");
            }
        };
        let msg: Message = response.into();
        self.ctx_pub
            .send((
                routing_key!(Executor >> Response).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn consensus_block_enqueue(&self, proof_blk: BlockWithProof) {
        let current_height = self.ext.get_current_height();
        let mut proof_blk = proof_blk;
        let proto_block = proof_blk.take_blk();
        let proof = proof_blk.take_proof();
        let blk_height = proto_block.get_header().get_height();
        let block = Block::from(proto_block);

        debug!(
            "consensus block {} {:?} tx hash  {:?} len {} version {}",
            block.number(),
            block.hash(),
            block.transactions_root(),
            block.body().transactions().len(),
            block.header.version()
        );
        if self.is_dup_block(block.number()) {
            return;
        }

        let block_in_queue = {
            let block_map = self.ext.block_map.read();
            block_map.get(&blk_height).cloned()
        };
        let stage = { self.ext.stage.read().clone() };

        debug!(
            "Received consensus block, block_number: {:?} current_height: {:?}, stage: {:?}",
            blk_height, current_height, stage
        );

        if self.ext.validate_height(block.number()) && self.ext.validate_hash(block.parent_hash()) {
            match stage {
                Stage::ExecutingProposal => {
                    if let Some(BlockInQueue::Proposal(value)) = block_in_queue {
                        if !value.is_equivalent(&block)
                            && !self.ext.is_interrupted.load(Ordering::SeqCst)
                        {
                            self.ext.is_interrupted.store(true, Ordering::SeqCst);
                        }
                        self.send_block(blk_height, block, proof);
                    }
                }
                Stage::WaitFinalized => {
                    if let Some(BlockInQueue::Proposal(value)) = block_in_queue {
                        // Not interrupt but to notify chain to execute the block
                        if !value.is_equivalent(&block)
                            && !self.ext.is_interrupted.load(Ordering::SeqCst)
                        {
                            self.ext.is_interrupted.store(true, Ordering::SeqCst);
                        }
                        self.send_block(blk_height, block, proof);
                    }
                }
                Stage::Idle => {
                    self.send_block(blk_height, block, proof);
                }
                Stage::ExecutingBlock => {
                    warn!("Something is wrong! Coming consensus block while executing consensus block");
                }
            }
        } else {
            warn!("something is wrong! Coming consensus is not valid");
        }
    }

    fn deal_sync_blocks(&self, mut sync_res: SyncResponse) {
        debug!("sync: current height = {}", self.ext.get_current_height());
        for block in sync_res.take_blocks().into_iter() {
            let blk_height = block.get_header().get_height();

            // return if the block existed
            if blk_height < self.ext.get_max_height() {
                continue;
            };

            // Check transaction root
            if blk_height != ::std::u64::MAX && !block.check_hash() {
                warn!(
                    "sync: transactions root isn't correct, height is {}",
                    blk_height
                );
                break;
            }

            let rblock = Block::from(block);

            trace!(
                "sync: Received block {} {:?}  tx hash {:?} len {}",
                rblock.number(),
                rblock.hash(),
                rblock.transactions_root(),
                rblock.body().transactions().len()
            );
            if self.is_dup_block(rblock.number()) {
                return;
            }

            self.add_sync_block(rblock);
        }

        if !self.ext.is_sync.load(Ordering::SeqCst) {
            self.closed_block.replace(None);
            let number = self.ext.get_current_height() + 1;
            debug!("sync block number is {}", number);
            let _ = self.write_sender.send(number);
        }
    }

    // Check block group from remote and enqueue
    fn add_sync_block(&self, block: Block) {
        let block_proof_type = block.proof_type();
        let ext_proof_type = self.ext.get_prooftype();
        //check sync_block's proof type, it must be consistent with chain
        if ext_proof_type != block_proof_type {
            error!(
                "sync: block_proof_type {:?} mismatch with ext_proof_type {:?}",
                block_proof_type, ext_proof_type
            );
            return;
        }
        match block_proof_type {
            Some(ProofType::Bft) => {
                let proof = BftProof::from(block.proof().clone());
                let proof_height = if proof.height == ::std::usize::MAX {
                    0
                } else {
                    proof.height as u64
                };

                debug!(
                    "sync: add_sync_block: proof_height = {}, block height = {} max_height = {}",
                    proof_height,
                    block.number(),
                    self.ext.get_max_height()
                );

                let mut blocks = self.ext.block_map.write();
                if (block.number() as usize) != ::std::usize::MAX {
                    if proof_height == self.ext.get_max_height() {
                        // Set proof of prev sync block
                        if let Some(prev_block_in_queue) = blocks.get_mut(&proof_height) {
                            if let BlockInQueue::SyncBlock(ref mut value) = *prev_block_in_queue {
                                if value.1.is_none() {
                                    debug!("sync: set prev sync block proof {}", value.0.number());
                                    mem::swap(&mut value.1, &mut Some(block.proof().clone()));
                                }
                            }
                        }

                        self.ext.set_max_height(block.number() as usize);

                        debug!("sync: insert block-{} in map", block.number());
                        blocks.insert(block.number(), BlockInQueue::SyncBlock((block, None)));
                    }
                } else if proof_height > self.ext.get_current_height() {
                    if let Some(block_in_queue) = blocks.get_mut(&proof_height) {
                        if let BlockInQueue::SyncBlock(ref mut value) = *block_in_queue {
                            if value.1.is_none() {
                                debug!("sync: insert block proof {} in map", proof_height);
                                mem::swap(&mut value.1, &mut Some(block.proof().clone()));
                            }
                        }
                    }
                }
            }
            // TODO: Handle Raft and POA
            _ => {
                unimplemented!();
            }
        }
    }

    fn proposal_enqueue(&self, mut signed_proposal: SignedProposal) {
        let proposal = signed_proposal.take_proposal().take_block();

        let current_height = self.ext.get_current_height();
        let blk_height = proposal.get_header().get_height();
        let block = Block::from(proposal);

        let block_in_queue = {
            let block_map = self.ext.block_map.read();
            block_map.get(&blk_height).cloned()
        };

        let stage = { self.ext.stage.read().clone() };
        debug!(
            "received proposal, block_number: {:?} current_height: {:?}, stage: {:?}",
            blk_height, current_height, stage
        );

        if self.ext.validate_height(blk_height) && self.ext.validate_hash(block.parent_hash()) {
            match stage {
                Stage::ExecutingProposal => {
                    if let Some(BlockInQueue::Proposal(value)) = block_in_queue {
                        if !value.is_equivalent(&block) {
                            if !self.ext.is_interrupted.load(Ordering::SeqCst) {
                                self.ext.is_interrupted.store(true, Ordering::SeqCst);
                            }
                            self.send_proposal(blk_height, block);
                        }
                    }
                }
                Stage::WaitFinalized => {
                    if let Some(BlockInQueue::Proposal(value)) = block_in_queue {
                        if !value.is_equivalent(&block) {
                            self.send_proposal(blk_height, block);
                        }
                    }
                }
                Stage::Idle => match block_in_queue {
                    Some(BlockInQueue::ConsensusBlock(_, _)) | Some(BlockInQueue::SyncBlock(_)) => {
                    }
                    _ => self.send_proposal(blk_height, block),
                },
                Stage::ExecutingBlock => {
                    warn!("Something wrong! Coming proposal while executing consensus block");
                }
            }
        }
    }

    fn set_sync_block(&self, block: Block, proto_proof: Proof) -> bool {
        let number = block.number();
        trace!("set sync block-{}", number);
        let proof = BftProof::from(proto_proof);
        let proof_height = if proof.height == ::std::usize::MAX {
            0
        } else {
            proof.height as u64
        };
        let conf = self.ext.global_config.read().clone();
        let authorities = conf.nodes.clone();

        // fixbug when conf have changed such as adding consensus node
        let prev_conf = self.ext.load_config(BlockId::Number(number - 1));
        let prev_authorities = prev_conf.nodes.clone();

        if self.ext.validate_height(number)
            && self.ext.validate_hash(block.parent_hash())
            && (proof.check(proof_height as usize, &authorities)
                || proof.check(proof_height as usize, &prev_authorities))
        {
            self.ext.execute_block(block, &self.ctx_pub);
            trace!("set sync block-{} is finished", number);
            true
        } else {
            warn!("The proof is {:?}", proof);
            warn!(
                "The authorities is {:?}, prev_authorities is {:?}",
                authorities, prev_authorities
            );
            trace!("sync block-{} is invalid", number);
            false
        }
    }

    fn sync_blocks(&self, mut number: u64) {
        self.ext.is_sync.store(true, Ordering::SeqCst);
        info!("set sync block start from {}", number);
        let mut invalid_block_in_queue = false;
        let mut block_map = {
            let guard = self.ext.block_map.read();
            guard.clone()
        };
        loop {
            let block_in_queue = block_map.remove(&number);
            match block_in_queue {
                Some(BlockInQueue::SyncBlock((block, Some(proof)))) => {
                    if self.set_sync_block(block, proof) {
                        number += 1;
                    } else {
                        invalid_block_in_queue = true;
                        // Reach here only in byzantine condition
                        warn!("set sync block end to {} as invalid block", number - 1);
                        break;
                    }
                }
                _ => {
                    info!("set sync block end to {}", number - 1);
                    break;
                }
            }
        }

        if invalid_block_in_queue {
            let mut guard = self.ext.block_map.write();
            guard.clear();
            self.ext
                .set_max_height(self.ext.get_current_height() as usize);
        }

        self.ext.is_sync.store(false, Ordering::SeqCst);
    }

    fn send_block(&self, blk_height: u64, block: Block, proof: Proof) {
        {
            self.ext
                .block_map
                .write()
                .insert(blk_height, BlockInQueue::ConsensusBlock(block, proof));
        };
        self.ext.set_max_height(blk_height as usize);
        let _ = self.write_sender.send(blk_height);
    }

    fn send_proposal(&self, blk_height: u64, block: Block) {
        {
            self.ext
                .block_map
                .write()
                .insert(blk_height, BlockInQueue::Proposal(block));
        };
        let _ = self.write_sender.send(blk_height);
    }

    pub fn signal_to_chain(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let mut state_signal = StateSignal::new();
        state_signal.set_height(self.ext.get_current_height());
        let msg: Message = state_signal.into();
        ctx_pub
            .send((
                routing_key!(Executor >> StateSignal).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn deal_snapshot_req(&mut self, snapshot_req: &SnapshotReq) {
        let mut resp = SnapshotResp::new();
        match snapshot_req.cmd {
            Cmd::Snapshot => {
                info!("[snapshot] receive {:?}", snapshot_req);
                let ext = self.ext.clone();
                let snapshot_req = snapshot_req.clone();
                let ctx_pub = self.ctx_pub.clone();
                let snapshot_builder = thread::Builder::new().name("snapshot_executor".into());
                let _ = snapshot_builder.spawn(move || {
                    take_snapshot(&ext, &snapshot_req);

                    info!("Taking snapshot finished!!!");

                    //resp SnapshotAck to snapshot_tool
                    resp.set_resp(Resp::SnapshotAck);
                    resp.set_flag(true);
                    let msg: Message = resp.into();
                    ctx_pub
                        .send((
                            routing_key!(Executor >> SnapshotResp).into(),
                            msg.try_into().unwrap(),
                        ))
                        .unwrap();
                });
            }
            Cmd::Begin => {
                info!("[snapshot] receive cmd: Begin");
                self.is_snapshot = true;
            }
            Cmd::Restore => {
                info!("[snapshot] receive {:?}", snapshot_req);
                match restore_snapshot(&self.ext, snapshot_req) {
                    Ok(_) => {
                        resp.set_flag(true);
                    }
                    Err(e) => {
                        warn!("restore_snapshot failed: {:?}", e);
                        resp.set_flag(false);
                    }
                }

                //resp RestoreAck to snapshot_tool
                resp.set_resp(Resp::RestoreAck);
                let msg: Message = resp.into();
                self.ctx_pub
                    .send((
                        routing_key!(Executor >> SnapshotResp).into(),
                        msg.try_into().unwrap(),
                    ))
                    .unwrap();
            }
            Cmd::Clear => {
                info!("[snapshot] receive cmd: Clear");
            }
            Cmd::End => {
                info!("[snapshot] receive cmd: End");
                self.is_snapshot = false;
            }
        }
    }

    /// The processing logic here is the same as the network pruned/re-transmitted information based on
    /// the state of the chain, but here is pruned/re-transmitted `ExecutedResult`.
    #[inline]
    fn execute_chain_status(&mut self, status: &RichStatus) {
        self.ext.prune_execute_result_cache(status);
    }
}

fn take_snapshot(ext: &Arc<Executor>, snapshot_req: &SnapshotReq) {
    // use given path
    let file_name = snapshot_req.file.clone() + "_executor.rlp";
    let writer = PackedWriter {
        file: File::create(file_name).unwrap(),
        state_hashes: Vec::new(),
        block_hashes: Vec::new(),
        cur_len: 0,
    };

    // use given height: ancient block, or latest
    let mut block_at = snapshot_req.get_end_height();
    let current_height = ext.get_current_height();
    if block_at == 0 || block_at > current_height {
        warn!(
            "block height is equal to 0 or bigger than current height, \
             and be set to current height!"
        );
        block_at = current_height;
    }
    let start_hash = ext.block_hash(block_at).unwrap();

    let db = ext.state_db.read().boxed_clone();

    let progress = Arc::new(Progress::default());

    snapshot::take_snapshot(&ext, start_hash, db.as_hashdb(), writer, &*progress).unwrap();
}

fn restore_snapshot(ext: &Arc<Executor>, snapshot_req: &SnapshotReq) -> Result<(), String> {
    let file_name = snapshot_req.file.clone() + "_executor.rlp";
    let reader = PackedReader::new(Path::new(&file_name))
        .map_err(|e| format!("Couldn't open snapshot file: {}", e))
        .and_then(|x| x.ok_or_else(|| "Snapshot file has invalid format.".into()));
    let reader = match reader {
        Ok(r) => r,
        Err(e) => {
            warn!("get reader failed: {:?}", e);
            return Err(e);
        }
    };

    let db_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let snap_path = DataPath::root_node_path() + "/snapshot_executor";
    let snapshot_params = SnapServiceParams {
        db_config: db_config.clone(),
        pruning: Algorithm::Archive,
        snapshot_root: snap_path.into(),
        db_restore: ext.clone(),
        executor: ext.clone(),
    };

    let snapshot = SnapshotService::new(snapshot_params).unwrap();
    let snapshot = Arc::new(snapshot);
    match snapshot::restore_using(&snapshot, &reader, true) {
        Ok(_) => Ok(()),
        Err(e) => {
            warn!("restore_using failed: {:?}", e);
            Err(e)
        }
    }
}
