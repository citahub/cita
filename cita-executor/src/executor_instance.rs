use core::contracts::node_manager::NodeManager;
use core::contracts::sys_config::SysConfig;
use core::db;
use core::libexecutor::Genesis;
use core::libexecutor::ServiceMap;
use core::libexecutor::block::{Block, ClosedBlock};
use core::libexecutor::call_request::CallRequest;
use core::libexecutor::executor::{BlockInQueue, Config, Executor, Stage};
use error::ErrorCode;
use jsonrpc_types::rpctypes::{BlockNumber, BlockTag, CountOrCode, MetaData};
use libproto::{request, response, Message, SyncResponse};
use libproto::blockchain::{BlockWithProof, Proof, ProofType};
use libproto::consensus::SignedProposal;
use libproto::request::Request_oneof_req as Request;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
use proof::TendermintProof;
use serde_json;
use std::cell::RefCell;
use std::convert::{Into, TryFrom, TryInto};
use std::fs::File;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use types::ids::BlockId;
use util::Address;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};

use core::snapshot;
use core::snapshot::Progress;
use core::snapshot::io::{PackedReader, PackedWriter};
use core::snapshot::service::{Service as SnapshotService, ServiceParams as SnapServiceParams};
use core::state::backend::Backend;
use std::path::Path;

#[derive(Clone)]
pub struct ExecutorInstance {
    ctx_pub: Sender<(String, Vec<u8>)>,
    write_sender: Sender<u64>,
    pub ext: Arc<Executor>,
    pub grpc_port: u16,
    closed_block: RefCell<Option<ClosedBlock>>,
}

impl ExecutorInstance {
    pub fn new(
        ctx_pub: Sender<(String, Vec<u8>)>,
        write_sender: Sender<u64>,
        config_path: &str,
        genesis_path: &str,
        service_map: Arc<ServiceMap>,
    ) -> Self {
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let nosql_path = DataPath::root_node_path() + "/statedb";
        let db = Database::open(&config, &nosql_path).unwrap();

        let mut genesis = Genesis::init(genesis_path);

        let executor_config = Config::new(config_path);
        let grpc_port = executor_config.grpc_port;
        let mut executor = Executor::init_executor(Arc::new(db), genesis, executor_config);
        executor.set_service_map(service_map);
        let executor = Arc::new(executor);
        executor.set_gas_and_nodes();
        executor.send_executed_info_to_chain(&ctx_pub);
        ExecutorInstance {
            ctx_pub: ctx_pub,
            write_sender: write_sender,
            ext: executor,
            grpc_port: grpc_port,
            closed_block: RefCell::new(None),
        }
    }

    pub fn distribute_msg(&self, key: &str, msg_vec: &[u8]) {
        let mut msg = Message::try_from(msg_vec).unwrap();
        let origin = msg.get_origin();
        trace!("distribute_msg call key = {}, origin = {}", key, origin);
        match RoutingKey::from(key) {
            routing_key!(Chain >> Request) => {
                let req = msg.take_request().unwrap();
                self.reply_request(req);
            }

            routing_key!(Consensus >> BlockWithProof) => {
                let proof_blk = msg.take_block_with_proof().unwrap();
                self.consensus_block_enqueue(proof_blk);
            }

            routing_key!(Chain >> LocalSync) | routing_key!(Net >> SyncResponse) => {
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

            routing_key!(Consensus >> RawBytes) | routing_key!(Net >> RawBytes) => {
                trace!("Receive other message content.");
            }

            routing_key!(Snapshot >> SnapshotReq) => {
                let req = msg.take_snapshot_req().unwrap();
                let mut resp = SnapshotResp::new();
                match req.cmd {
                    Cmd::Snapshot => {
                        info!("executor receive snapshot cmd: {:?}", req);
                        self.take_snapshot(&req);
                        info!("executor snapshot creation complete");

                        //resp SnapshotAck to snapshot_tool
                        //let mut resp = SnapshotResp::new();
                        resp.set_resp(Resp::SnapshotAck);
                        let msg: Message = resp.into();
                        self.ctx_pub
                            .send((
                                routing_key!(Executor >> SnapshotResp).into(),
                                msg.try_into().unwrap(),
                            ))
                            .unwrap();
                    }
                    Cmd::Restore => {
                        info!("executor receive restore cmd: {:?}", req);
                        self.restore(&req);
                        info!("executor snapshot restore complete");

                        //resp RestoreAck to snapshot_tool
                        //let mut resp = SnapshotResp::new();
                        resp.set_resp(Resp::RestoreAck);
                        let msg: Message = resp.into();
                        self.ctx_pub
                            .send((
                                routing_key!(Executor >> SnapshotResp).into(),
                                msg.try_into().unwrap(),
                            ))
                            .unwrap();
                    }
                    _ => {
                        trace!("executor receive other snapshot message");
                    }
                }
            }

            _ => {
                error!("error key {}!!!!", key);
            }
        }
    }

    pub fn is_dup_block(&self, inum: u64) -> bool {
        inum <= self.ext.get_current_height()
    }

    ///执行block交易
    pub fn execute_block(&self, number: u64) {
        let block_in_queue = {
            let block_map = self.ext.block_map.read();
            block_map.get(&number).cloned()
        };

        let stage = { self.ext.stage.read().clone() };
        let mut need_clean_map = false;

        match block_in_queue {
            Some(BlockInQueue::ConsensusBlock(block, _)) => {
                if self.ext.validate_height(block.number()) && self.ext.validate_hash(block.parent_hash()) {
                    // Not Match before proposal
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
                                if let Some(closed_block) = self.closed_block.replace(None) {
                                    {
                                        *self.ext.stage.write() = Stage::ExecutingBlock;
                                    }
                                    self.ext
                                        .finalize_proposal(closed_block, block, &self.ctx_pub);
                                } else {
                                    // Maybe never reach
                                    warn!("at WaitFinalized, but no closed block found!");
                                    {
                                        *self.ext.stage.write() = Stage::ExecutingBlock;
                                    }
                                    self.ext.execute_block(block, &self.ctx_pub);
                                };
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
                    debug!("execute consensus block [height {}] finish !", number);
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
            Some(BlockInQueue::Proposal(proposal)) => {
                // Interrupte pre proposal
                if self.ext.is_interrupted.load(Ordering::SeqCst) {
                    self.ext.is_interrupted.store(false, Ordering::SeqCst);
                }
                {
                    *self.ext.stage.write() = Stage::ExecutingProposal;
                }
                if let Some(closed_block) = self.ext.execute_proposal(proposal) {
                    // Interruppted by laster proposal/consensus block
                    if self.ext.is_interrupted.load(Ordering::SeqCst) {
                        self.ext.is_interrupted.store(false, Ordering::SeqCst);
                        return;
                    }
                    // After execute proposal, check whether block-in-map is changed
                    let in_queue = {
                        let block_map = self.ext.block_map.read();
                        block_map.get(&number).cloned()
                    };
                    match in_queue {
                        Some(BlockInQueue::ConsensusBlock(comming, _)) => {
                            if comming.header().transactions_root() == closed_block.header().transactions_root() {
                                self.ext
                                    .finalize_proposal(closed_block, comming, &self.ctx_pub);
                                {
                                    *self.ext.stage.write() = Stage::Idle;
                                }
                                info!("execute proposal block [height {}] finish !", number);
                            } else {
                                // Maybe never reach
                                warn!("something is wrong, go into no-man's-land")
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
                    warn!("executing proposal is interrupted.");
                }
            }
            _ => {
                info!("block-{} in queue is without proof", number);
            }
        }

        if need_clean_map {
            let mut guard = self.ext.block_map.write();
            let new_map = guard.split_off(&self.ext.get_current_height());
            *guard = new_map;
        }
    }

    fn reply_request(&self, mut req: request::Request) {
        let mut response = response::Response::new();
        response.set_request_id(req.take_request_id());

        match req.req.unwrap() {
            Request::call(call) => {
                trace!("Chainvm Call {:?}", call);
                serde_json::from_str::<BlockNumber>(&call.height)
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
                serde_json::from_str::<CountOrCode>(&tx_count)
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
                trace!("code request from josnrpc  {:?}", code_content);
                serde_json::from_str::<CountOrCode>(&code_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|code_content| {
                        let address = Address::from_slice(code_content.address.as_ref());
                        match self.ext.code_at(&address, code_content.block_id.into()) {
                            Some(code) => match code {
                                Some(code) => {
                                    response.set_contract_code(code);
                                }
                                None => {
                                    response.set_contract_code(vec![]);
                                }
                            },
                            None => {
                                response.set_contract_code(vec![]);
                            }
                        };
                    });
            }

            Request::abi(abi_content) => {
                trace!("abi request from josnrpc  {:?}", abi_content);
                serde_json::from_str::<CountOrCode>(&abi_content)
                    .map_err(|err| {
                        response.set_code(ErrorCode::query_error());
                        response.set_error_msg(format!("{:?}", err));
                    })
                    .map(|abi_content| {
                        let address = Address::from_slice(abi_content.address.as_ref());
                        match self.ext.abi_at(&address, abi_content.block_id.into()) {
                            Some(abi) => match abi {
                                Some(abi) => {
                                    response.set_contract_abi(abi);
                                }
                                None => {
                                    response.set_contract_abi(vec![]);
                                }
                            },
                            None => {
                                response.set_contract_abi(vec![]);
                            }
                        };
                    });
            }

            Request::meta_data(data) => {
                trace!("metadata request from josnrpc {:?}", data);
                match serde_json::from_str::<BlockNumber>(&data)
                    .map_err(|err| (ErrorCode::query_error(), format!("{:?}", err)))
                    .and_then(|number: BlockNumber| {
                        let current_height = self.ext.get_current_height();
                        let number = match number {
                            BlockNumber::Tag(BlockTag::Earliest) => 0,
                            BlockNumber::Height(n) => n,
                            BlockNumber::Tag(BlockTag::Latest) => current_height,
                        };
                        if number > current_height {
                            Err((
                                ErrorCode::query_error(),
                                format!("Block number overflow: {} > {}", number, current_height),
                            ))
                        } else {
                            Ok(number)
                        }
                    })
                    .map(|number: u64| {
                        // TODO: get chain_name by current block number
                        let block_id = BlockId::Number(number);
                        let sys_config = SysConfig::new(&self.ext);
                        let genesis_timestamp = self.ext
                            .block_header(BlockId::Earliest)
                            .unwrap()
                            .timestamp();
                        MetaData {
                            genesis_timestamp,
                            chain_id: sys_config.chain_id(),
                            chain_name: sys_config.chain_name(Some(block_id)),
                            operator: sys_config.operator(Some(block_id)),
                            website: sys_config.website(Some(block_id)),
                            validators: NodeManager::nodes(&self.ext),
                            block_interval: sys_config.block_interval(),
                        }
                    }) {
                    Ok(metadata) => response.set_meta_data(serde_json::to_string(&metadata).unwrap()),
                    Err((code, error_msg)) => {
                        response.set_code(code);
                        response.set_error_msg(error_msg);
                    }
                }
            }

            _ => {
                error!("mtach error Request_oneof_req msg!!!!");
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
            "consensus block {} {:?} tx hash  {:?} len {}",
            block.number(),
            block.hash(),
            block.transactions_root(),
            block.body().transactions().len()
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
                        if value.header().transactions_root() != block.transactions_root() {
                            if !self.ext.is_interrupted.load(Ordering::SeqCst) {
                                self.ext.is_interrupted.store(true, Ordering::SeqCst);
                            }
                        }
                        self.send_block(blk_height, block, proof);
                    }
                }
                Stage::WaitFinalized => {
                    if let Some(BlockInQueue::Proposal(value)) = block_in_queue {
                        // Not interrupt but to notify chain to execute the block
                        if value.header().transactions_root() != block.transactions_root()
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
                "execute_block rblock {} {:?}  tx hash {:?} len {}",
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
            self.write_sender.send(number);
        }
    }

    // Check block group from remote and enqueue
    #[cfg_attr(feature = "clippy", allow(single_match))]
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
            Some(ProofType::Tendermint) => {
                let proof = TendermintProof::from(block.proof().clone());
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

                        self.ext
                            .max_height
                            .store(block.number() as usize, Ordering::SeqCst);
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
                        if value.header().transactions_root() != block.transactions_root() {
                            if !self.ext.is_interrupted.load(Ordering::SeqCst) {
                                self.ext.is_interrupted.store(true, Ordering::SeqCst);
                            }
                            self.send_proposal(blk_height, block);
                        }
                    }
                }
                Stage::WaitFinalized => {
                    if let Some(BlockInQueue::Proposal(value)) = block_in_queue {
                        if value.header().transactions_root() != block.transactions_root() {
                            self.send_proposal(blk_height, block);
                        }
                    }
                }
                Stage::Idle => {
                    self.send_proposal(blk_height, block);
                }
                Stage::ExecutingBlock => {
                    warn!("Something wrong! Coming proposal while executing consensus block");
                }
            }
        }
    }

    fn set_sync_block(&self, block: Block, proto_proof: Proof) -> bool {
        let number = block.number();
        info!("set sync block-{}", number);
        let proof = TendermintProof::from(proto_proof);
        let proof_height = if proof.height == ::std::usize::MAX {
            0
        } else {
            proof.height as u64
        };
        let conf = self.ext.get_sys_config(number);
        let authorities = conf.nodes.clone();

        //fixbug when conf have changed such as adding consensus node
        let prev_conf = self.ext.get_sys_config(number - 1);
        let prev_authorities = prev_conf.nodes.clone();
        if self.ext.validate_height(number) && self.ext.validate_hash(block.parent_hash())
            && (proof.check(proof_height as usize, &authorities)
                || proof.check(proof_height as usize, &prev_authorities))
        {
            self.ext.execute_block(block, &self.ctx_pub);
            info!("set sync block-{} is finished", number);
            true
        } else {
            info!("sync block-{} is invalid", number);
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
                        info!("set sync block end to {} as invalid block", number - 1);
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
        self.ext
            .max_height
            .store(blk_height as usize, Ordering::SeqCst);
        self.write_sender.send(blk_height);
    }

    fn send_proposal(&self, blk_height: u64, block: Block) {
        {
            self.ext
                .block_map
                .write()
                .insert(blk_height, BlockInQueue::Proposal(block));
        };
        self.write_sender.send(blk_height);
    }

    fn take_snapshot(&self, _snap_shot: &SnapshotReq) {
        // executor snapshot entry
        let writer = PackedWriter {
            file: File::create("snap.rlp").unwrap(), //TODO:use given path
            state_hashes: Vec::new(),
            cur_len: 0,
        };

        let progress = Arc::new(Progress::default());
        //let block_at = snap_shot.get_start_height(); //ancient block,latest?
        //let start_hash = self.ext.block_hash(block_at).unwrap();

        info!(
            "snapshot: current height = {}",
            self.ext.get_current_height()
        );
        let start_hash = self.ext.get_current_hash();
        //let db = self.ext.state_db.journal_db().boxed_clone();
        let db = self.ext.state_db.boxed_clone();
        info!("take_snapshot start_hash: {:?}", start_hash);
        snapshot::take_snapshot(&self.ext, start_hash, db.as_hashdb(), writer, &*progress).unwrap();
    }

    fn restore(&self, _snap_shot: &SnapshotReq) -> Result<(), String> {
        let file = "snap-executor.rlp";
        let reader = PackedReader::new(Path::new(&file))
            .map_err(|e| format!("Couldn't open snapshot file: {}", e))
            .and_then(|x| x.ok_or_else(|| "Snapshot file has invalid format.".into()));
        let reader = reader?;

        let mut db_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let snap_path = DataPath::root_node_path() + "/snapshot";
        let snapshot_params = SnapServiceParams {
            db_config: db_config.clone(),
            //pruning: pruning,
            //snapshot_root: DataPath::root_node_path().into(),
            snapshot_root: snap_path.into(),
            db_restore: self.ext.clone(),
        };
        //TODO:get manifest from snap_shot for restore
        let snapshot = SnapshotService::new(snapshot_params).unwrap();
        let snapshot = Arc::new(snapshot);
        snapshot::restore_using(Arc::clone(&snapshot), &reader, true);
        Ok(())
    }
}
