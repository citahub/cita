// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

#![allow(unused_must_use)]

use cita_types::H256;
use core::filters::eth_filter::EthFilter;
use core::libchain::block::Block;
use core::libchain::chain::{BlockInQueue, Chain};
use error::ErrorCode;
use jsonrpc_types::rpctypes::{
    BlockNumber as RpcBlockNumber, BlockParamsByHash, BlockParamsByNumber, Filter as RpcFilter,
    Log as RpcLog, Receipt as RpcReceipt, RpcBlock,
};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
use libproto::{
    request, response, Block as ProtobufBlock, BlockTxHashes, BlockTxHashesReq, BlockWithProof,
    ExecutedResult, Message, OperateType, Proof, ProofType, Request_oneof_req as Request,
    SyncRequest, SyncResponse,
};
use proof::BftProof;
use serde_json;
use std::convert::{Into, TryFrom, TryInto};
use std::mem;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use types::filter::Filter;
use types::ids::BlockId;

use core::db;
use std::fs::File;
use std::path::Path;
use util::datapath::DataPath;
use util::kvdb::DatabaseConfig;

use core::snapshot;
use core::snapshot::io::{PackedReader, PackedWriter, SnapshotReader};
use core::snapshot::service::{Service as SnapshotService, ServiceParams as SnapServiceParams};
use core::snapshot::Progress;

/// Message forwarding and query data
#[derive(Clone)]
pub struct Forward {
    write_sender: Sender<ExecutedResult>,
    chain: Arc<Chain>,
    ctx_pub: Sender<(String, Vec<u8>)>,
}

// TODO: Add future client to support forward
impl Forward {
    pub fn new(
        chain: Arc<Chain>,
        ctx_pub: Sender<(String, Vec<u8>)>,
        write_sender: Sender<ExecutedResult>,
    ) -> Forward {
        Forward {
            chain,
            ctx_pub,
            write_sender,
        }
    }

    // 注意: 划分函数处理流程
    pub fn dispatch_msg(&self, key: &str, msg_bytes: &[u8]) {
        let mut msg = Message::try_from(msg_bytes).unwrap();
        let origin = msg.get_origin();
        match RoutingKey::from(key) {
            routing_key!(Jsonrpc >> Request) => {
                let req = msg.take_request().unwrap();
                self.reply_request(req, msg_bytes.to_vec());
            }

            //send to block_processor to operate
            routing_key!(Executor >> ExecutedResult) => {
                let info = msg.take_executed_result().unwrap();
                self.write_sender.send(info).unwrap();
            }

            routing_key!(Executor >> StateSignal) => {
                if let Some(state_signal) = msg.take_state_signal() {
                    let specified_height = state_signal.get_height();
                    let missing_blocks =
                        (specified_height + 1..=self.chain.get_current_height()).collect();
                    self.reply_local_syn_req(missing_blocks);
                }
            }

            routing_key!(Consensus >> BlockWithProof) => {
                let proof_blk = msg.take_block_with_proof().unwrap();
                self.consensus_block_enqueue(proof_blk);
            }

            routing_key!(Net >> SyncRequest) => {
                let sync_req = msg.take_sync_request().unwrap();
                self.reply_syn_req(sync_req, origin);
            }

            routing_key!(Net >> SyncResponse) => {
                let sync_res = msg.take_sync_response().unwrap();
                self.deal_sync_blocks(sync_res);
            }

            routing_key!(Auth >> BlockTxHashesReq) => {
                let block_tx_hashes_req = msg.take_block_tx_hashes_req().unwrap();
                self.deal_block_tx_req(&block_tx_hashes_req);
            }

            routing_key!(Snapshot >> SnapshotReq) => {
                let snapshot_req = msg.take_snapshot_req().unwrap();
                self.deal_snapshot_req(&snapshot_req);
            }

            _ => {
                error!("forward dispatch msg found error key {}!!!!", key);
            }
        }
    }

    fn reply_request(&self, mut req: request::Request, imsg: Vec<u8>) {
        let mut response = response::Response::new();
        response.set_request_id(req.take_request_id());
        match req.req.unwrap() {
            // TODO: should check the result, parse it first!
            Request::block_number(_) => {
                response.set_block_number(self.chain.get_current_height());
            }

            Request::block_by_hash(rpc) => {
                //let rpc: BlockParamsByHash = serde_json::from_str(&rpc);
                match serde_json::from_str::<BlockParamsByHash>(&rpc) {
                    Ok(param) => {
                        let hash = param.hash;
                        let include_txs = param.include_txs;
                        match self.chain.block_by_hash(H256::from(hash.as_slice())) {
                            Some(block) => {
                                let rpc_block = RpcBlock::new(
                                    hash,
                                    include_txs,
                                    block.protobuf().try_into().unwrap(),
                                );
                                serde_json::to_string(&rpc_block)
                                    .map(|data| response.set_block(data))
                                    .map_err(|err| {
                                        response.set_code(ErrorCode::query_error());
                                        response.set_error_msg(format!("{:?}", err));
                                    });
                            }
                            None => response.set_none(true),
                        }
                    }
                    Err(err) => {
                        response.set_block(format!("{:?}", err));
                        response.set_code(ErrorCode::query_error());
                    }
                };
            }

            Request::block_by_height(block_height) => {
                let block_height: BlockParamsByNumber =
                    serde_json::from_str(&block_height).expect("Invalid param");
                let include_txs = block_height.include_txs;
                match self.chain.block(block_height.block_id.into()) {
                    Some(block) => {
                        let rpc_block = RpcBlock::new(
                            block.hash().to_vec(),
                            include_txs,
                            block.protobuf().try_into().unwrap(),
                        );
                        serde_json::to_string(&rpc_block)
                            .map(|data| response.set_block(data))
                            .map_err(|err| {
                                response.set_code(ErrorCode::query_error());
                                response.set_error_msg(format!("{:?}", err));
                            });
                    }
                    None => {
                        response.set_none(true);
                    }
                }
            }

            Request::transaction(hash) => {
                match self.chain.full_transaction(H256::from_slice(&hash)) {
                    Some(ts) => {
                        response.set_ts(ts);
                    }
                    None => {
                        response.set_none(true);
                    }
                }
            }

            Request::transaction_proof(hash) => {
                match self.chain.get_transaction_proof(H256::from_slice(&hash)) {
                    Some(proof) => {
                        response.set_transaction_proof(proof);
                    }
                    None => {
                        response.set_none(true);
                    }
                };
            }

            Request::transaction_receipt(hash) => {
                let tx_hash = H256::from_slice(&hash);
                let receipt = self.chain.localized_receipt(tx_hash);
                if let Some(receipt) = receipt {
                    let rpc_receipt: RpcReceipt = receipt.into();
                    let serialized = serde_json::to_string(&rpc_receipt).unwrap();
                    response.set_receipt(serialized);
                } else {
                    response.set_none(true);
                }
            }

            Request::filter(encoded) => {
                trace!("filter: {:?}", encoded);
                if let Ok(rpc_filter) = serde_json::from_str::<RpcFilter>(&encoded).map_err(|err| {
                    response.set_code(ErrorCode::query_error());
                    response.set_error_msg(format!("{:?}", err));
                }) {
                    let filter: Filter = rpc_filter.into();
                    let logs = self.chain.get_logs(&filter);
                    let rpc_logs: Vec<RpcLog> = logs.into_iter().map(|x| x.into()).collect();
                    response.set_logs(serde_json::to_string(&rpc_logs).unwrap());
                };
            }

            Request::call(call) => {
                trace!("Chainvm Call {:?}", call);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::transaction_count(tx_count) => {
                trace!("transaction count request from jsonrpc {:?}", tx_count);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::meta_data(number) => {
                trace!("metadata request from jsonrpc {:?}", number);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::code(code_content) => {
                trace!("code request from josnrpc  {:?}", code_content);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::abi(abi_content) => {
                trace!("abi request from josnrpc  {:?}", abi_content);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::balance(balance_content) => {
                trace!("balance request from josnrpc  {:?}", balance_content);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::new_filter(new_filter) => {
                trace!("new_filter {:?}", new_filter);
                let new_filter: RpcFilter =
                    serde_json::from_str(&new_filter).expect("Invalid param");
                trace!("new_filter {:?}", new_filter);
                response.set_filter_id(self.chain.new_filter(new_filter) as u64);
            }

            Request::new_block_filter(_) => {
                let block_filter = self.chain.new_block_filter();
                response.set_filter_id(block_filter as u64);
            }

            Request::uninstall_filter(filter_id) => {
                trace!("uninstall_filter's id is {:?}", filter_id);
                let b = self.chain.uninstall_filter(filter_id as usize);
                response.set_uninstall_filter(b);
            }

            Request::filter_changes(filter_id) => {
                trace!("filter_changes's id is {:?}", filter_id);
                let log = self.chain.filter_changes(filter_id as usize).unwrap();
                trace!("Log is: {:?}", log);
                response.set_filter_changes(serde_json::to_string(&log).unwrap());
            }

            Request::filter_logs(filter_id) => {
                trace!("filter_log's id is {:?}", filter_id);
                let log = self
                    .chain
                    .filter_logs(filter_id as usize)
                    .unwrap_or_default();
                trace!("Log is: {:?}", log);
                response.set_filter_logs(serde_json::to_string(&log).unwrap());
            }

            Request::state_proof(state_info) => {
                trace!("state_proof info is {:?}", state_info);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }

            Request::block_header_height(block_height) => {
                let block_height: RpcBlockNumber =
                    serde_json::from_str(&block_height).expect("Invalid param");
                match self.chain.get_block_header_bytes(block_height.into()) {
                    Some(block_header_bytes) => {
                        response.set_block_header(block_header_bytes);
                    }
                    None => {
                        response.set_none(true);
                    }
                }
            }

            Request::height(_) | Request::batch_req(_) => {
                error!("Get messages which should not handle by this function!");
            }

            Request::peercount(_) | Request::un_tx(_) => {
                error!("Get messages which should sent to other micro services!");
            }
            Request::storage_key(skey) => {
                trace!("storage key info is {:?}", skey);
                self.ctx_pub
                    .send((routing_key!(Chain >> Request).into(), imsg))
                    .unwrap();
                return;
            }
        };
        let msg: Message = response.into();
        self.ctx_pub
            .send((
                routing_key!(Chain >> Response).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    // Consensus block enqueue
    fn consensus_block_enqueue(&self, proof_blk: BlockWithProof) {
        let current_height = self.chain.get_current_height() as usize;
        let mut proof_blk = proof_blk;
        let proto_block = proof_blk.take_blk();
        let proof = proof_blk.take_proof();
        let blk_height = proto_block.get_header().get_height() as usize;

        trace!(
            "Received consensus block: block_number:{:?} current_height: {:?} ",
            blk_height,
            current_height
        );

        let block = Block::from(proto_block);
        debug!(
            "consensus block {} {:?} tx hash  {:?} len {} version {}",
            block.number(),
            block.hash(),
            block.transactions_root(),
            block.body().transactions().len(),
            block.header.version()
        );
        if blk_height == (current_height + 1) {
            {
                self.chain.block_map.write().insert(
                    blk_height as u64,
                    BlockInQueue::ConsensusBlock(block.clone(), proof.clone()),
                );
            };
            self.chain.set_proof_with_height(blk_height as u64, &proof);
            self.chain.save_current_block_poof(&proof);
            self.chain.set_block_body(blk_height as u64, &block);
            self.chain.set_max_store_height(blk_height as u64);
        }
    }

    fn reply_syn_req(&self, sync_req: SyncRequest, origin: u32) {
        let mut sync_req = sync_req;
        let heights = sync_req.take_heights();
        debug!(
            "sync: receive sync from node {:?}, height lists = {:?}",
            origin, heights
        );

        let res_vec = self.sync_response(heights);

        debug!(
            "sync: reply node = {}, response blocks len = {}",
            origin,
            res_vec.get_blocks().len()
        );
        if !res_vec.get_blocks().is_empty() {
            let msg = Message::init(OperateType::Single, origin, res_vec.into());
            trace!(
                "sync: origin {:?}, chain.blk: OperateType {:?}",
                origin,
                OperateType::Single
            );
            self.ctx_pub
                .send((
                    routing_key!(Chain >> SyncResponse).into(),
                    msg.try_into().unwrap(),
                ))
                .unwrap();
        }
    }

    fn reply_local_syn_req(&self, heights: Vec<u64>) {
        let res_vec = self.sync_response(heights);
        if !res_vec.get_blocks().is_empty() {
            let msg = Message::init(OperateType::Single, 0, res_vec.into());
            trace!(
                "local_sync: chain.blk: OperateType {:?}",
                OperateType::Single
            );
            self.ctx_pub
                .send((
                    routing_key!(Chain >> LocalSync).into(),
                    msg.try_into().unwrap(),
                ))
                .unwrap();
        }
    }

    #[inline]
    fn sync_response(&self, heights: Vec<u64>) -> SyncResponse {
        let mut res_vec = SyncResponse::new();
        for height in heights
            .into_iter()
            .filter(|height| *height <= self.chain.get_current_height())
        {
            if let Some(block) = self.chain.block(BlockId::Number(height)) {
                res_vec.mut_blocks().push(block.protobuf());
                //push double
                if height == self.chain.get_current_height() {
                    if let Some(pproof) = self.chain.get_proof_with_height(height) {
                        let mut proof_block = ProtobufBlock::new();
                        proof_block.mut_header().set_proof(pproof);
                        proof_block.mut_header().set_height(::std::u64::MAX);
                        res_vec.mut_blocks().push(proof_block);
                        trace!(
                            "sync: max height {:?}, chain.blk: OperateType {:?}",
                            height,
                            OperateType::Single
                        );
                    }
                }
            }
        }
        res_vec
    }

    fn deal_sync_blocks(&self, mut sync_res: SyncResponse) {
        debug!("sync: current height = {}", self.chain.get_current_height());
        for block in sync_res.take_blocks().into_iter() {
            let blk_height = block.get_header().get_height();

            // return if the block existed
            if blk_height < self.chain.get_current_height() {
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

            self.add_sync_block(Block::from(block));
        }
    }

    // Check block group from remote and enqueue
    fn add_sync_block(&self, block: Block) {
        let block_proof_type = block.proof_type();
        let chain_proof_type = self.chain.get_chain_prooftype();
        let blk_height = block.number() as usize;
        let chain_max_height = self.chain.get_current_height();
        let chain_max_store_height = self.chain.get_max_store_height();
        //check sync_block's proof type, it must be consistent with chain
        if chain_proof_type != block_proof_type {
            error!(
                "sync: block_proof_type {:?} mismatch with chain_proof_type {:?}",
                block_proof_type, chain_proof_type
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
                    proof_height, blk_height, chain_max_height
                );

                let height = block.number();
                if blk_height != ::std::usize::MAX {
                    if proof_height == chain_max_height || proof_height == chain_max_store_height {
                        // Set proof of prev sync block
                        if let Some(prev_block_in_queue) =
                            self.chain.block_map.write().get_mut(&proof_height)
                        {
                            if let BlockInQueue::SyncBlock(ref mut value) = *prev_block_in_queue {
                                if value.1.is_none() {
                                    debug!("sync: set prev sync block proof {}", value.0.number());
                                    mem::swap(&mut value.1, &mut Some(block.proof().clone()));
                                }
                            }
                        }
                        self.chain.set_block_body(height, &block);
                        self.chain.set_max_store_height(height);

                        /*when syncing blocks,not deliver blockhashs when recieving block,
                        just deliver blockhashs when executed block*/

                        let saved_proof = self.chain.get_proof_with_height(height);
                        debug!(
                            "sync: insert block-{} proof {:?} in map",
                            height, saved_proof
                        );
                        self.chain
                            .block_map
                            .write()
                            .insert(height, BlockInQueue::SyncBlock((block, saved_proof)));
                    } else {
                        warn!(
                            "sync: insert block-{} is not continious proof height {}",
                            block.number(),
                            proof_height
                        );
                    }
                } else if proof_height > self.chain.get_current_height() {
                    if let Some(block_in_queue) =
                        self.chain.block_map.write().get_mut(&proof_height)
                    {
                        if let BlockInQueue::SyncBlock(ref mut value) = *block_in_queue {
                            if value.1.is_none() {
                                debug!("sync: insert block proof {} in map", proof_height);
                                mem::swap(&mut value.1, &mut Some(block.proof().clone()));
                            }
                        }
                    }
                    self.chain
                        .set_proof_with_height(proof_height, block.proof());
                    self.chain.save_current_block_poof(block.proof());
                }
            }
            // TODO: Handle Raft and POA
            _ => {
                unimplemented!();
            }
        }
    }

    fn deal_block_tx_req(&self, block_tx_hashes_req: &BlockTxHashesReq) {
        let block_height = block_tx_hashes_req.get_height();

        if let Some(tx_hashes) = self.chain.transaction_hashes(BlockId::Number(block_height)) {
            //prepare and send the block tx hashes to auth
            let mut block_tx_hashes = BlockTxHashes::new();
            block_tx_hashes.set_height(block_height);
            let mut tx_hashes_in_u8 = Vec::new();
            for tx_hash_in_h256 in &tx_hashes {
                tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());
            }
            block_tx_hashes.set_tx_hashes(tx_hashes_in_u8.into());
            block_tx_hashes
                .set_block_quota_limit(self.chain.block_quota_limit.load(Ordering::SeqCst) as u64);
            block_tx_hashes.set_account_quota_limit(self.chain.account_quota_limit.read().clone());
            let msg: Message = block_tx_hashes.into();
            self.ctx_pub
                .send((
                    routing_key!(Chain >> BlockTxHashes).into(),
                    msg.try_into().unwrap(),
                ))
                .unwrap();

            trace!("response block's tx hashes for height:{}", block_height);
        } else {
            warn!("get block's tx hashes for height:{} error", block_height);
        }
    }

    fn deal_snapshot_req(&self, snapshot_req: &SnapshotReq) {
        let mut resp = SnapshotResp::new();
        match snapshot_req.cmd {
            Cmd::Snapshot => {
                info!("[snapshot] receive {:?}", snapshot_req);
                let chain = self.chain.clone();
                let ctx_pub = self.ctx_pub.clone();
                let snapshot_req = snapshot_req.clone();
                let snapshot_builder = thread::Builder::new().name("snapshot_chain".into());
                snapshot_builder.spawn(move || {
                    take_snapshot(&chain, &snapshot_req);

                    info!("Taking snapshot finished!!!");

                    //resp SnapshotAck to snapshot_tool
                    resp.set_resp(Resp::SnapshotAck);
                    resp.set_flag(true);
                    let msg: Message = resp.into();
                    ctx_pub
                        .send((
                            routing_key!(Chain >> SnapshotResp).into(),
                            msg.try_into().unwrap(),
                        ))
                        .unwrap();
                });
            }
            Cmd::Begin => {
                info!("[snapshot] receive cmd: Begin");
                let mut is_snapshot = self.chain.is_snapshot.write();
                *is_snapshot = true;
            }
            Cmd::Restore => {
                info!("[snapshot] receive {:?}", snapshot_req);
                match restore_snapshot(&self.chain.clone(), snapshot_req) {
                    Ok(proof) => {
                        resp.set_proof(proof);
                        resp.set_height(self.chain.get_current_height());
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
                        routing_key!(Chain >> SnapshotResp).into(),
                        msg.try_into().unwrap(),
                    ))
                    .unwrap();
            }
            Cmd::Clear => {
                info!("[snapshot] receive cmd: Clear");
            }
            Cmd::End => {
                info!("[snapshot] receive {:?}", snapshot_req);
                let chain = self.chain.clone();
                let ctx_pub = self.ctx_pub.clone();
                thread::spawn(move || {
                    thread::sleep(Duration::new(3, 0));
                    // broadcast status and rich-status.
                    chain.broadcast_current_status(&ctx_pub);

                    let mut is_snapshot = chain.is_snapshot.write();
                    *is_snapshot = false;
                });
            }
        }
    }
}

fn take_snapshot(chain: &Arc<Chain>, snapshot_req: &SnapshotReq) {
    // use given path
    let file_name = snapshot_req.file.clone() + "_chain.rlp";
    let writer = PackedWriter {
        file: File::create(file_name).unwrap(), //TODO:use given path
        block_hashes: Vec::new(),
        cur_len: 0,
    };

    // use given height: ancient block, or latest
    let mut block_at = snapshot_req.get_end_height();
    let current_height = chain.get_current_height();
    if block_at == 0 || block_at > current_height {
        warn!(
            "block height is equal to 0 or bigger than current height, \
             and be set to current height!"
        );
        block_at = current_height;
    }

    let progress = Arc::new(Progress::default());

    snapshot::take_snapshot(chain, block_at, writer, &*progress).unwrap();
}

fn restore_snapshot(chain: &Arc<Chain>, snapshot_req: &SnapshotReq) -> Result<Proof, String> {
    let file_name = snapshot_req.file.clone() + "_chain.rlp";
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
    let snap_path = DataPath::root_node_path() + "/snapshot_chain";
    let snapshot_params = SnapServiceParams {
        db_config: db_config.clone(),
        snapshot_root: snap_path.into(),
        db_restore: chain.clone(),
        chain: chain.clone(),
    };

    let snapshot = SnapshotService::new(snapshot_params).unwrap();
    let snapshot = Arc::new(snapshot);
    match snapshot::restore_using(&snapshot, &reader, true) {
        Ok(_) => {
            // return proof
            let proof = reader.manifest().last_proof.clone();
            Ok(proof)
        }
        Err(e) => {
            warn!("restore_using failed: {:?}", e);
            Err(e)
        }
    }
}
