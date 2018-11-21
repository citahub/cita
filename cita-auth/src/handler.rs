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

use block_txn::{BlockTxnMessage, BlockTxnReq};
use cita_types::traits::LowerHex;
use cita_types::{clean_0x, Address, H256, U256};
use crypto::{pubkey_to_address, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use dispatcher::Dispatcher;
use error::ErrorCode;
use history::HistoryHeights;
use jsonrpc_types::rpctypes::TxResponse;
use libproto::auth::{Miscellaneous, MiscellaneousReq};
use libproto::blockchain::{AccountGasLimit, SignedTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
use libproto::{
    BlackList, BlockTxHashes, BlockTxHashesReq, BlockTxn, Crypto, GetBlockTxn, Message,
    OperateType, Origin, Request, Response, Ret, UnverifiedTransaction, VerifyBlockReq,
    VerifyBlockResp, VerifyTxReq,
};
use lru::LruCache;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::convert::{Into, TryFrom, TryInto};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use util::BLOCKLIMIT;

// verify signature
pub fn verify_tx_sig(crypto: Crypto, hash: &H256, sig_bytes: &[u8]) -> Result<Vec<u8>, ()> {
    if sig_bytes.len() != SIGNATURE_BYTES_LEN {
        return Err(());
    }

    let sig = Signature::from(sig_bytes);
    match crypto {
        Crypto::SECP => sig
            .recover(&hash)
            .map(|pubkey| pubkey.to_vec())
            .map_err(|_| ()),
        _ => {
            warn!("Unexpected crypto");
            Err(())
        }
    }
}

pub struct SysConfigInfo {
    pub block_quota_limit: u64,
    pub account_quota_limit: AccountGasLimit,
    pub check_quota: bool,
    pub admin_address: Option<Address>,
    pub version: Option<u32>,
}

#[derive(Debug, PartialEq)]
enum ChainId {
    V0(u32),
    V1(U256),
}

pub struct MsgHandler {
    rx_sub: Receiver<(String, Vec<u8>)>,
    tx_pub: Sender<(String, Vec<u8>)>,
    // only cache verify sig
    cache: LruCache<H256, Option<Vec<u8>>>,
    chain_id: Option<ChainId>,
    history_heights: HistoryHeights,
    cache_block_req: Option<VerifyBlockReq>,
    history_hashes: HashMap<u64, HashSet<H256>>,
    dispatcher: Dispatcher,
    tx_request: Sender<Request>,
    tx_pool_limit: usize,
    is_snapshot: bool,
    black_list_cache: HashMap<Address, i8>,
    is_need_proposal_new_block: bool,
    config_info: SysConfigInfo,
    block_txn_req: Option<(BlockTxnReq)>,
}

impl MsgHandler {
    pub fn new(
        rx_sub: Receiver<(String, Vec<u8>)>,
        tx_pub: Sender<(String, Vec<u8>)>,
        dispatcher: Dispatcher,
        tx_request: Sender<Request>,
        tx_pool_limit: usize,
        tx_verify_thread_num: usize,
        tx_verify_cache_size: usize,
    ) -> Self {
        ThreadPoolBuilder::new()
            .num_threads(tx_verify_thread_num)
            .build_global()
            .unwrap();
        MsgHandler {
            rx_sub,
            tx_pub,
            cache: LruCache::new(tx_verify_cache_size),
            chain_id: None,
            history_heights: HistoryHeights::new(),
            cache_block_req: None,
            history_hashes: HashMap::with_capacity(BLOCKLIMIT as usize),
            dispatcher,
            tx_request,
            tx_pool_limit,
            is_snapshot: false,
            black_list_cache: HashMap::new(),
            is_need_proposal_new_block: false,
            config_info: SysConfigInfo {
                block_quota_limit: 0,
                account_quota_limit: AccountGasLimit::new(),
                check_quota: false,
                admin_address: None,
                version: None,
            },
            block_txn_req: None,
        }
    }

    fn is_ready(&self) -> bool {
        self.history_heights.is_init()
            && self.chain_id.is_some()
            && !self.is_snapshot
            && self.config_info.version.is_some()
    }

    fn is_flow_control(&self, tx_count: usize) -> bool {
        self.tx_pool_limit != 0 && tx_count + self.dispatcher.tx_pool_len() > self.tx_pool_limit
    }

    fn cache_block_request_id(&self) -> Option<u64> {
        self.cache_block_req
            .as_ref()
            .map(|cache_block_req| cache_block_req.get_id())
    }

    // max(new_request_id, next_request_id, cache_request_id):
    // new_request_id -> replace the cache
    // next_request_id -> clean the cache
    // cache_request_id -> keep the cache
    fn update_cache_block_req(&mut self, blk_req: VerifyBlockReq) {
        let new_request_id = blk_req.get_id();
        let next_height = self.history_heights.next_height();
        let next_request_id = next_height << 16;
        match self.cache_block_request_id() {
            Some(cache_request_id) => {
                if new_request_id > cache_request_id && new_request_id >= next_request_id {
                    self.cache_block_req = Some(blk_req);
                } else if next_request_id > cache_request_id {
                    self.cache_block_req = None;
                }
            }
            None => {
                if new_request_id > next_request_id {
                    self.cache_block_req = Some(blk_req);
                }
            }
        }
    }

    #[allow(unknown_lints, clippy::option_option)] // TODO clippy
    fn get_ret_from_cache(&self, tx_hash: &H256) -> Option<Option<Vec<u8>>> {
        self.cache.peek(tx_hash).cloned()
    }

    fn save_ret_to_cache(&mut self, tx_hash: H256, option_pubkey: Option<Vec<u8>>) {
        self.cache.put(tx_hash, option_pubkey);
    }

    pub fn verify_block_quota(&self, blkreq: &VerifyBlockReq) -> bool {
        let reqs = blkreq.get_reqs();
        let quota_limit = self
            .config_info
            .account_quota_limit
            .get_common_quota_limit();
        let mut specific_quota_limit = self
            .config_info
            .account_quota_limit
            .get_specific_quota_limit()
            .clone();
        let mut account_gas_used: HashMap<Address, u64> = HashMap::new();
        let mut n = self.config_info.block_quota_limit;
        for req in reqs {
            let quota = req.get_quota();
            let signer = pubkey_to_address(&PubKey::from(req.get_signer()));

            if n < quota {
                return false;
            }

            if self.config_info.check_quota {
                let value = account_gas_used.entry(signer).or_insert_with(|| {
                    if let Some(value) = specific_quota_limit.remove(&signer.lower_hex()) {
                        value
                    } else {
                        quota_limit
                    }
                });
                if *value < quota {
                    return false;
                } else {
                    *value -= quota;
                }
            }
            n -= quota;
        }
        true
    }

    pub fn verify_tx_quota(&self, quota: u64, signer: &[u8]) -> bool {
        if quota > self.config_info.block_quota_limit {
            return false;
        }
        if self.config_info.check_quota {
            let addr = pubkey_to_address(&PubKey::from(signer));
            let mut quota_limit = self
                .config_info
                .account_quota_limit
                .get_common_quota_limit();
            if let Some(value) = self
                .config_info
                .account_quota_limit
                .get_specific_quota_limit()
                .get(&addr.lower_hex())
            {
                quota_limit = *value;
            }
            if quota > quota_limit {
                return false;
            }
        }
        true
    }

    fn process_block_verify(&mut self, blk_req: VerifyBlockReq) {
        let tx_cnt = blk_req.get_reqs().len();
        let request_id = blk_req.get_id();
        let height = request_id >> 16;

        if self.history_heights.next_height() != height {
            trace!(
                "Not current block verify request with request_id: {}",
                request_id
            );
            self.update_cache_block_req(blk_req);
            return;
        }

        info!(
            "Process block verify request with request_id: {}",
            request_id
        );

        let mut reqs_no_cache = Vec::new();
        for req in blk_req.get_reqs() {
            let tx_hash = H256::from_slice(req.get_tx_hash());
            if let Some(option_pubkey) = self.get_ret_from_cache(&tx_hash) {
                if let Some(pubkey) = option_pubkey {
                    let req_signer = req.get_signer();
                    if req_signer != pubkey.to_vec().as_slice() {
                        self.publish_block_verification_result(request_id, Ret::BadSig);
                        return;
                    }
                } else {
                    // cached result is bad
                    self.publish_block_verification_result(request_id, Ret::BadSig);
                    return;
                }
            } else {
                reqs_no_cache.push(req);
            }
        }

        info!(
            "block verify request with {} tx not hit cache {}",
            tx_cnt,
            reqs_no_cache.len()
        );

        // parallel verify tx and collect results
        let reqs_no_cache_count = reqs_no_cache.len();
        let results: Vec<(H256, Vec<u8>)> = reqs_no_cache
            .into_par_iter()
            .map(|req| {
                let tx_hash = H256::from_slice(req.get_tx_hash());
                let result = verify_tx_sig(
                    req.get_crypto(),
                    &H256::from(req.get_hash()),
                    &req.get_signature(),
                );
                match result {
                    Ok(pubkey) => {
                        let req_signer = req.get_signer();
                        if req_signer != pubkey.as_slice() {
                            None
                        } else {
                            Some((tx_hash, pubkey))
                        }
                    }
                    Err(_) => None,
                }
            })
            .while_some()
            .collect();

        let results_len = results.len();
        for (tx_hash, pubkey) in results {
            self.save_ret_to_cache(tx_hash, Some(pubkey));
        }

        if results_len != reqs_no_cache_count {
            self.publish_block_verification_result(request_id, Ret::BadSig);
            return;
        }

        // check valid_until_block and history block dup
        for req in blk_req.get_reqs() {
            let ret = self.verify_tx_req(req);
            if ret != Ret::OK {
                self.publish_block_verification_result(request_id, ret);
                return;
            }
        }

        if !self.verify_block_quota(&blk_req) {
            self.publish_block_verification_result(request_id, Ret::QuotaNotEnough);
            return;
        }

        self.publish_block_verification_result(request_id, Ret::OK);
    }

    // verify to and version
    fn verify_request(&self, req: &Request) -> Ret {
        let un_tx = req.get_un_tx();
        let tx = un_tx.get_transaction();
        let tx_version = tx.get_version();
        if tx_version != self.config_info.version.unwrap() {
            return Ret::InvalidVersion;
        }
        if tx_version == 0 {
            // new to must be empty
            if !tx.get_to_v1().is_empty() {
                return Ret::InvalidValue;
            }
            let to = clean_0x(tx.get_to());
            if !to.is_empty() && Address::from_str(to).is_err() {
                return Ret::InvalidValue;
            }
        } else if tx_version == 1 {
            // old to must be empty
            if !tx.get_to().is_empty() {
                return Ret::InvalidValue;
            }
            // check to_v1
            let to = tx.get_to_v1();
            if !to.is_empty() && to.len() != 20 {
                return Ret::InvalidValue;
            }
        } else {
            error!("unexpected version {}!", tx_version);
            return Ret::InvalidValue;
        }

        Ret::OK
    }

    /// Verify black list
    fn verify_black_list(&self, req: &VerifyTxReq) -> Ret {
        if let Some(credit) = self
            .black_list_cache
            .get(&pubkey_to_address(&PubKey::from_slice(req.get_signer())))
        {
            if *credit < 0 {
                Ret::Forbidden
            } else {
                Ret::OK
            }
        } else {
            Ret::OK
        }
    }

    fn verify_tx_req_chain_id(&self, req: &VerifyTxReq) -> Ret {
        let version = self.config_info.version.unwrap();

        let chain_id = match version {
            0 => {
                // new chain id must be empty
                if !req.get_chain_id_v1().is_empty() {
                    None
                } else {
                    let chain_id = req.get_chain_id();
                    Some(ChainId::V0(chain_id))
                }
            }
            1 => {
                // old chain id must be empty
                if req.get_chain_id() != 0 || req.get_chain_id_v1().len() != 32 {
                    None
                } else {
                    let chain_id = U256::from(req.get_chain_id_v1());
                    Some(ChainId::V1(chain_id))
                }
            }
            _ => {
                error!("unexpected version {}!", version);
                None
            }
        };

        if chain_id != self.chain_id {
            trace!(
                "tx chain_id {:?}, self.chain_id {:?}",
                chain_id.unwrap(),
                self.chain_id
            );
            return Ret::BadChainId;
        }

        Ret::OK
    }

    // verify chain id, nonce, valid_until_block, dup, quota and black list
    fn verify_tx_req(&self, req: &VerifyTxReq) -> Ret {
        let ret = self.verify_tx_req_chain_id(req);
        if ret != Ret::OK {
            return ret;
        }

        if req.get_nonce().len() > 128 {
            return Ret::InvalidNonce;
        }

        if req.get_value().len() != 32 {
            return Ret::InvalidValue;
        }

        if self
            .config_info
            .admin_address
            .map(|admin| pubkey_to_address(&PubKey::from_slice(req.get_signer())) != admin)
            .unwrap_or_else(|| false)
        {
            return Ret::Forbidden;
        }

        let valid_until_block = req.get_valid_until_block();
        let next_height = self.history_heights.next_height();
        if valid_until_block < next_height || valid_until_block >= (next_height + BLOCKLIMIT) {
            return Ret::InvalidUntilBlock;
        }

        let tx_hash = H256::from_slice(req.get_tx_hash());
        for (height, hashes) in &self.history_hashes {
            if hashes.contains(&tx_hash) {
                trace!(
                    "Tx with hash {:?} has already existed in height:{}",
                    tx_hash,
                    height
                );
                return Ret::Dup;
            }
        }

        if !self.verify_tx_quota(req.get_quota(), req.get_signer()) {
            return Ret::QuotaNotEnough;
        }

        Ret::OK
    }

    fn publish_block_verification_result(&self, request_id: u64, ret: Ret) {
        let mut blkresp = VerifyBlockResp::new();
        blkresp.set_id(request_id);
        blkresp.set_ret(ret);

        let msg: Message = blkresp.into();
        self.tx_pub
            .send((
                routing_key!(Auth >> VerifyBlockResp).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn publish_tx_failed_result(&self, request_id: Vec<u8>, ret: Ret) {
        let result = format!("{:?}", ret);
        let mut response = Response::new();
        response.set_request_id(request_id);
        response.set_code(ErrorCode::tx_auth_error());
        response.set_error_msg(result);

        trace!("response new tx {:?}", response);
        let msg: Message = response.into();
        self.tx_pub
            .send((
                routing_key!(Auth >> Response).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn publish_tx_success_result(&self, request_id: Vec<u8>, tx_hash: H256) {
        let mut response = Response::new();
        response.set_request_id(request_id);

        let result = format!("{:?}", Ret::OK);
        let tx_response = TxResponse::new(tx_hash, result.clone());
        let tx_state = serde_json::to_string(&tx_response).unwrap();
        response.set_tx_state(tx_state);

        let msg: Message = response.into();
        self.tx_pub
            .send((
                routing_key!(Auth >> Response).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn forward_request(&self, tx_req: Request) {
        let _ = self.tx_request.send(tx_req);
    }

    fn send_single_block_tx_hashes_req(&mut self, height: u64) {
        let mut req = BlockTxHashesReq::new();
        req.set_height(height);
        let msg: Message = req.into();
        self.tx_pub
            .send((
                routing_key!(Auth >> BlockTxHashesReq).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    fn send_block_tx_hashes_req(&mut self, check: bool) {
        // we will send req for all height
        // so don't too frequent
        if check && self.history_heights.is_too_frequent() {
            warn!("Too frequent to send request!");
            return;
        }
        trace!(
            "send block tx hashes request {} to {}",
            self.history_heights.min_height(),
            self.history_heights.max_height()
        );
        for i in self.history_heights.min_height()..self.history_heights.max_height() {
            if !self.history_hashes.contains_key(&i) {
                self.send_single_block_tx_hashes_req(i);
            }
        }

        self.history_heights.update_time_stamp();
    }

    pub fn handle_remote_msg(&mut self) {
        loop {
            // send request to get chain id if we have not got it
            // chain id need version
            // so get chain id after get version
            if self.chain_id.is_none() && self.config_info.version.is_some() {
                trace!("chain id is not ready");
                let msg: Message = MiscellaneousReq::new().into();
                self.tx_pub
                    .send((
                        routing_key!(Auth >> MiscellaneousReq).into(),
                        msg.try_into().unwrap(),
                    ))
                    .unwrap();
            }

            // block hashes of some height we not have
            // we will send request for these height
            if !self.history_heights.is_init() {
                trace!("history block hashes is not ready");
                self.send_block_tx_hashes_req(true);
            }

            // Daily tasks
            {
                if self.is_ready() {
                    // process block verify if we have cached block request
                    if let Some(cache_request_id) = self.cache_block_request_id() {
                        let cache_height = cache_request_id >> 16;
                        if cache_height == self.history_heights.next_height() {
                            let cache_block_req = self.cache_block_req.take().unwrap();
                            self.process_block_verify(cache_block_req);
                        }
                    }
                }

                if self.is_need_proposal_new_block && self.is_ready() {
                    self.dispatcher.proposal_tx_list(
                        (self.history_heights.next_height() - 1) as usize, // todo fix bft
                        &self.tx_pub,
                        &self.config_info,
                    );
                    // after proposal new block clear flag
                    self.is_need_proposal_new_block = false;
                }
            }

            // process message from MQ
            if let Ok((key, payload)) = self.rx_sub.recv_timeout(Duration::new(3, 0)) {
                let mut msg = Message::try_from(&payload).unwrap();
                let rounting_key = RoutingKey::from(&key);
                trace!("process message key = {}", key);
                match rounting_key {
                    // we got this message when chain reach new height or response the BlockTxHashesReq
                    routing_key!(Chain >> BlockTxHashes) => {
                        let block_tx_hashes = msg.take_block_tx_hashes().unwrap();
                        self.deal_block_tx_hashes(&block_tx_hashes)
                    }
                    routing_key!(Executor >> BlackList) => {
                        let black_list = msg.take_black_list().unwrap();
                        self.deal_black_list(&black_list);
                    }
                    routing_key!(Consensus >> VerifyBlockReq) => {
                        let blk_req = msg.take_verify_block_req().unwrap();
                        self.deal_verify_block_req(blk_req);
                    }
                    routing_key!(Net >> Request) | routing_key!(Jsonrpc >> RequestNewTxBatch) => {
                        let is_local = rounting_key.is_sub_module(SubModules::Jsonrpc);
                        let newtx_req = msg.take_request().unwrap();
                        self.deal_request(is_local, newtx_req);
                    }
                    routing_key!(Executor >> Miscellaneous) => {
                        let miscellaneous = msg.take_miscellaneous().unwrap();
                        self.deal_miscellaneous(&miscellaneous);
                    }
                    routing_key!(Snapshot >> SnapshotReq) => {
                        let snapshot_req = msg.take_snapshot_req().unwrap();
                        self.deal_snapshot(&snapshot_req);
                    }
                    routing_key!(Net >> GetBlockTxn) => {
                        let mut get_block_txn = msg.take_get_block_txn().unwrap();
                        let origin = msg.get_origin();
                        self.deal_get_block_txn(&mut get_block_txn, origin);
                    }
                    routing_key!(Net >> BlockTxn) => {
                        let block_txn = msg.take_block_txn().unwrap();
                        let origin = msg.get_origin();
                        let mut block_txn_message = BlockTxnMessage { origin, block_txn };
                        self.deal_block_txn(block_txn_message);
                        // TODO: notify network to add the origin to blacklist if BadTxSignature
                        // TODO: Verify block quota limit
                        // TODO: send block transactions to consensus
                    }
                    _ => {
                        error!("receive unexpected message key {}", key);
                    }
                }
            }
        }
    }

    fn deal_block_tx_hashes(&mut self, block_tx_hashes: &BlockTxHashes) {
        let height = block_tx_hashes.get_height();
        info!("get block tx hashes for height {:?}", height);
        let tx_hashes = block_tx_hashes.get_tx_hashes();

        // because next height init value is 1
        // the empty chain first msg height is 0 with quota info
        if height >= self.history_heights.next_height()
            || (self.history_heights.next_height() == 1 && height == 0)
        {
            // get latest quota info from chain
            let block_quota_limit = block_tx_hashes.get_block_quota_limit();
            let account_quota_limit = block_tx_hashes.get_account_quota_limit().clone();
            let check_quota = block_tx_hashes.get_check_quota();
            self.config_info.check_quota = check_quota;
            self.config_info.block_quota_limit = block_quota_limit;
            self.config_info.account_quota_limit = account_quota_limit.clone();
            self.config_info.admin_address = if block_tx_hashes.get_admin_address().is_empty() {
                None
            } else {
                Some(Address::from(block_tx_hashes.get_admin_address()))
            };
            if block_tx_hashes.get_version() == 1 && self.config_info.version == Some(0) {
                trace!("Fetch new chain id");
                let msg: Message = MiscellaneousReq::new().into();
                self.tx_pub
                    .send((
                        routing_key!(Auth >> MiscellaneousReq).into(),
                        msg.try_into().unwrap(),
                    ))
                    .unwrap();
            }
            self.config_info.version = Some(block_tx_hashes.get_version());

            // need to proposal new block
            self.is_need_proposal_new_block = true;
        }

        // update history_heights
        let old_min_height = self.history_heights.min_height();
        self.history_heights.update_height(height);

        // update tx pool
        let mut tx_hashes_h256 = HashSet::with_capacity(tx_hashes.len());
        for data in tx_hashes.iter() {
            let hash = H256::from_slice(data);
            tx_hashes_h256.insert(hash);
        }
        self.dispatcher.del_txs_from_pool_with_hash(&tx_hashes_h256);

        // update history_hashes
        for i in old_min_height..self.history_heights.min_height() {
            self.history_hashes.remove(&i);
        }
        self.history_hashes.entry(height).or_insert(tx_hashes_h256);
    }

    fn deal_black_list(&mut self, black_list: &BlackList) {
        black_list
            .get_clear_list()
            .into_iter()
            .for_each(|clear_list: &Vec<u8>| {
                self.black_list_cache
                    .remove(&Address::from_slice(clear_list.as_slice()));
            });

        black_list
            .get_black_list()
            .into_iter()
            .for_each(|blacklist: &Vec<u8>| {
                self.black_list_cache
                    .entry(Address::from_slice(blacklist.as_slice()))
                    .and_modify(|e| {
                        if *e >= 0 {
                            *e -= 1;
                        }
                    })
                    .or_insert(3);
                debug!("Current black list is {:?}", self.black_list_cache);
            });
    }

    fn deal_verify_block_req(&mut self, blk_req: VerifyBlockReq) {
        let tx_cnt = blk_req.get_reqs().len();
        info!("get block verify request with {:?} request", tx_cnt);

        if tx_cnt == 0 {
            error!(
                "Wrong block verify request with 0 tx request_id: {}",
                blk_req.get_id()
            );
            return;
        }

        if !self.is_ready() {
            trace!("consensus >> verifyblock: auth is not ready");
            self.update_cache_block_req(blk_req);
            return;
        }

        self.process_block_verify(blk_req);
    }

    #[allow(unknown_lints, clippy::cyclomatic_complexity)] // TODO clippy
    fn deal_request(&mut self, is_local: bool, newtx_req: Request) {
        if newtx_req.has_batch_req() {
            let batch_new_tx = newtx_req.get_batch_req().get_new_tx_requests();
            trace!(
                "get batch new tx request has {} tx, is local? {}",
                batch_new_tx.len(),
                is_local
            );
            if !self.is_ready() {
                if is_local {
                    for tx_req in batch_new_tx.iter() {
                        let request_id = tx_req.get_request_id().to_vec();
                        self.publish_tx_failed_result(request_id, Ret::NotReady);
                    }
                }
                return;
            }

            if self.is_flow_control(batch_new_tx.len()) {
                trace!("flow control ...");
                if is_local {
                    for tx_req in batch_new_tx.iter() {
                        let request_id = tx_req.get_request_id().to_vec();
                        self.publish_tx_failed_result(request_id, Ret::Busy);
                    }
                }
                return;
            }

            let mut requests = HashMap::new();
            let mut requests_no_cached = HashMap::new();
            for tx_req in batch_new_tx {
                let req = tx_req.get_un_tx().tx_verify_req_msg();
                let tx_hash = H256::from_slice(req.get_tx_hash());
                if let Some(option_pubkey) = self.get_ret_from_cache(&tx_hash) {
                    if option_pubkey.is_none() {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, Ret::BadSig);
                        }
                        continue;
                    }
                    let mut new_req = req.clone();
                    new_req.set_signer(option_pubkey.unwrap());
                    requests.insert(tx_hash, (new_req, tx_req, true));
                } else {
                    requests_no_cached.insert(tx_hash, req.clone());
                    requests.insert(tx_hash, (req, tx_req, true));
                }
            }

            let results: Vec<(H256, Option<Vec<u8>>)> = requests_no_cached
                .into_par_iter()
                .map(|(tx_hash, ref req)| {
                    let result = verify_tx_sig(
                        req.get_crypto(),
                        &H256::from(req.get_hash()),
                        &req.get_signature(),
                    );
                    match result {
                        Ok(pubkey) => (tx_hash, Some(pubkey)),
                        Err(_) => (tx_hash, None),
                    }
                })
                .collect();

            for (tx_hash, option_pubkey) in results {
                self.save_ret_to_cache(tx_hash, option_pubkey.clone());
                if let Some(pubkey) = option_pubkey {
                    if let Some(ref mut v) = requests.get_mut(&tx_hash) {
                        v.0.set_signer(pubkey);
                    }
                } else if let Some(ref mut v) = requests.get_mut(&tx_hash) {
                    if is_local {
                        let request_id = v.1.get_request_id().to_vec();
                        self.publish_tx_failed_result(request_id, Ret::BadSig);
                    }
                    v.2 = false;
                }
            }

            // other verify
            requests
                .into_iter()
                .filter(|(_tx_hash, (_req, _tx_req, flag))| *flag)
                .filter(|(_tx_hash, (ref req, ref tx_req, _flag))| {
                    let ret = self.verify_black_list(&req);
                    if ret != Ret::OK {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, ret);
                        }
                        false
                    } else {
                        true
                    }
                })
                .filter(|(_tx_hash, (ref _req, ref tx_req, _flag))| {
                    let ret = self.verify_request(tx_req);
                    if ret != Ret::OK {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, ret);
                        }
                        false
                    } else {
                        true
                    }
                })
                .filter(|(_tx_hash, (ref req, ref tx_req, _flag))| {
                    let ret = self.verify_tx_req(&req);
                    if ret != Ret::OK {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, ret);
                        }
                        false
                    } else {
                        true
                    }
                })
                .for_each(|(tx_hash, (req, tx_req, _flag))| {
                    let mut signed_tx = SignedTransaction::new();
                    signed_tx.set_transaction_with_sig(tx_req.get_un_tx().clone());
                    signed_tx.set_signer(req.get_signer().to_vec());
                    signed_tx.set_tx_hash(tx_hash.to_vec());
                    let request_id = tx_req.get_request_id().to_vec();
                    if self.dispatcher.add_tx_to_pool(&signed_tx) {
                        if is_local {
                            self.publish_tx_success_result(request_id, tx_hash);
                        }
                        // new tx need forward to other nodes
                        self.forward_request(tx_req.clone());
                    } else if is_local {
                        // dup with transaction in tx pool
                        self.publish_tx_failed_result(request_id, Ret::Dup);
                    }
                });
        } else if newtx_req.has_un_tx() {
            trace!("get single new tx request from Jsonrpc");
            let request_id = newtx_req.get_request_id().to_vec();
            if !self.is_ready() {
                trace!("net || jsonrpc: auth is not ready");
                if is_local {
                    self.publish_tx_failed_result(request_id, Ret::NotReady);
                }
                return;
            }
            if self.is_flow_control(1) {
                trace!("flow control ...");
                if is_local {
                    self.publish_tx_failed_result(request_id, Ret::Busy);
                }
                return;
            }
            let mut req = newtx_req.get_un_tx().tx_verify_req_msg();
            // verify with cache
            let tx_hash = H256::from_slice(req.get_tx_hash());
            if let Some(option_pubkey) = self.get_ret_from_cache(&tx_hash) {
                if option_pubkey.is_none() {
                    self.publish_tx_failed_result(request_id, Ret::BadSig);
                    return;
                }
                req.set_signer(option_pubkey.unwrap());
            } else {
                let result = verify_tx_sig(
                    req.get_crypto(),
                    &H256::from(req.get_hash()),
                    &req.get_signature(),
                );
                self.save_ret_to_cache(tx_hash, result.clone().ok());
                match result {
                    Ok(pubkey) => {
                        req.set_signer(pubkey);
                    }
                    Err(_) => {
                        if is_local {
                            self.publish_tx_failed_result(request_id, Ret::BadSig);
                        }
                        return;
                    }
                }
            }

            // black verify
            let ret = self.verify_black_list(&req);
            if ret != Ret::OK {
                if is_local {
                    self.publish_tx_failed_result(request_id, ret);
                }
                return;
            }

            let ret = self.verify_request(&newtx_req);
            if ret != Ret::OK {
                if is_local {
                    self.publish_tx_failed_result(request_id, ret);
                }
                return;
            }

            // other verify
            let ret = self.verify_tx_req(&req);
            if ret != Ret::OK {
                if is_local {
                    self.publish_tx_failed_result(request_id, ret);
                }
                return;
            }

            // add tx pool
            let mut signed_tx = SignedTransaction::new();
            signed_tx.set_transaction_with_sig(newtx_req.get_un_tx().clone());
            signed_tx.set_signer(req.get_signer().to_vec());
            signed_tx.set_tx_hash(tx_hash.to_vec());
            if self.dispatcher.add_tx_to_pool(&signed_tx) {
                if is_local {
                    self.publish_tx_success_result(request_id, tx_hash);
                }
                // new tx need forward to other nodes
                self.forward_request(newtx_req);
            } else if is_local {
                // dup with transaction in tx pool
                self.publish_tx_failed_result(request_id, Ret::Dup);
            }
        }
    }

    fn deal_snapshot(&mut self, snapshot_req: &SnapshotReq) {
        let mut resp = SnapshotResp::new();
        let mut send = false;
        match snapshot_req.cmd {
            Cmd::Snapshot => {
                info!("[snapshot] receive cmd: Snapshot");
            }
            Cmd::Begin => {
                info!("[snapshot] receive cmd: Begin");
                self.is_snapshot = true;

                resp.set_resp(Resp::BeginAck);
                resp.set_flag(true);
                send = true;
            }
            Cmd::Restore => {
                info!("[snapshot] receive cmd: Restore");
            }
            Cmd::Clear => {
                info!("[snapshot] receive cmd: Clear");

                self.dispatcher.clear_txs_pool(0);
                self.cache.clear();
                self.history_heights.reset();
                self.cache_block_req = None;
                self.history_hashes.clear();
                self.black_list_cache.clear();

                resp.set_resp(Resp::ClearAck);
                resp.set_flag(true);
                send = true;
            }
            Cmd::End => {
                info!(
                    "[snapshot] receive cmd: End, height = {}",
                    snapshot_req.end_height
                );
                self.send_single_block_tx_hashes_req(snapshot_req.end_height);
                self.is_snapshot = false;

                resp.set_resp(Resp::EndAck);
                resp.set_flag(true);
                send = true;
            }
        }

        if send {
            let msg: Message = resp.into();
            self.tx_pub
                .send((
                    routing_key!(Auth >> SnapshotResp).into(),
                    (&msg).try_into().unwrap(),
                ))
                .unwrap();
        }
    }

    fn deal_miscellaneous(&mut self, miscellaneous: &Miscellaneous) {
        if let Some(version) = self.config_info.version {
            self.chain_id = if version == 0 {
                Some(ChainId::V0(miscellaneous.chain_id))
            } else if version == 1 {
                if miscellaneous.chain_id_v1.len() == 32 {
                    Some(ChainId::V1(U256::from(
                        miscellaneous.chain_id_v1.as_slice(),
                    )))
                } else {
                    None
                }
            } else {
                error!("unexpected version {}!", version);
                None
            };
            info!("Get chain_id({:?}) from executor", self.chain_id);
        }
    }

    fn deal_get_block_txn(&mut self, get_block_txn: &mut GetBlockTxn, origin: Origin) {
        let short_ids: Vec<H256> = get_block_txn
            .get_short_ids()
            .into_iter()
            .map(|id| H256::from(U256::from(id.as_slice())))
            .collect();
        let txs: Vec<UnverifiedTransaction> = self
            .dispatcher
            .get_txs(&short_ids)
            .into_iter()
            .map(|mut tx| tx.take_transaction_with_sig())
            .collect();

        let mut block_txn = BlockTxn::new();
        block_txn.set_block_hash(get_block_txn.take_block_hash());
        block_txn.set_transactions(txs.into());
        let msg = Message::init(OperateType::Single, origin, block_txn.into());

        self.tx_pub
            .send((
                routing_key!(Auth >> BlockTxn).into(),
                (&msg).try_into().unwrap(),
            ))
            .unwrap();
    }

    fn deal_block_txn(&mut self, mut block_txn: BlockTxnMessage) {
        // TODO: Need NLL to avoid clone
        if let Some(ref block_txn_req) = self.block_txn_req.clone() {
            let result = block_txn.validate(block_txn_req);
            match result {
                Ok(pubkey_and_hashes) => {
                    let signed_txn: Vec<SignedTransaction> = pubkey_and_hashes
                        .into_iter()
                        .zip(block_txn.take_transactions().into_iter())
                        .map(|((pubkey, hash), tx)| {
                            self.save_ret_to_cache(hash, Some(pubkey.clone()));

                            let mut signed_tx = SignedTransaction::new();
                            signed_tx.set_transaction_with_sig(tx);
                            signed_tx.set_signer(pubkey);
                            signed_tx.set_tx_hash(hash.to_vec());
                            signed_tx
                        })
                        .collect();
                    self.dispatcher.add_txs_to_pool(signed_txn);
                }
                Err(error) => {
                    info!("Validate BlockTxn error: {}", error);
                }
            }
        }
    }
}
