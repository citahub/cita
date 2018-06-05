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

use cita_types::traits::LowerHex;
use cita_types::{Address, H256};
use crypto::{pubkey_to_address, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use dispatcher::Dispatcher;
use error::ErrorCode;
use jsonrpc_types::rpctypes::TxResponse;
use libproto::auth::MiscellaneousReq;
use libproto::blockchain::{AccountGasLimit, SignedTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotResp};
use libproto::{
    BlockTxHashesReq, Crypto, Message, Request, Response, Ret, VerifyBlockReq, VerifyBlockResp,
    VerifyTxReq,
};
use lru::LruCache;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use serde_json;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::convert::{Into, TryFrom, TryInto};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use util::instrument::{unix_now, AsMillis};
use util::BLOCKLIMIT;

struct HistoryHeights {
    heights: HashSet<u64>,
    max_height: u64,
    min_height: u64,
    is_init: bool,
    latest_update_timestamp: u64,
}

impl HistoryHeights {
    pub fn new() -> Self {
        HistoryHeights {
            heights: HashSet::new(),
            max_height: 0,
            min_height: 0,
            is_init: false,
            //init value is 0 mean first time must not too frequent
            latest_update_timestamp: 0,
        }
    }

    fn update(&mut self, height: u64) {
        let old_min_height = self.min_height;
        self.max_height = height;
        self.min_height = if height > BLOCKLIMIT {
            height - BLOCKLIMIT + 1
        } else {
            0
        };
        for i in old_min_height..self.min_height {
            self.heights.remove(&i);
        }
        let mut is_init = true;
        for i in self.min_height..self.max_height {
            if !self.heights.contains(&i) {
                is_init = false;
                break;
            }
        }
        self.is_init = is_init;
        self.latest_update_timestamp = unix_now().as_millis();
    }

    pub fn add_height(&mut self, height: u64) {
        if height >= self.min_height {
            self.heights.insert(height);
            let max_height = self.max_height;
            self.update(max(height, max_height));
        }
    }

    pub fn next_height(&self) -> u64 {
        self.max_height + 1
    }

    pub fn is_init(&self) -> bool {
        self.is_init
    }

    pub fn max_height(&self) -> u64 {
        self.max_height
    }

    pub fn min_height(&self) -> u64 {
        self.min_height
    }

    // at least wait 3s from latest update
    pub fn is_too_frequent(&self) -> bool {
        unix_now().as_millis() < self.latest_update_timestamp + 3000
    }
}

#[cfg(test)]
mod history_heights_tests {
    use super::HistoryHeights;

    #[test]
    fn basic() {
        let mut h = HistoryHeights::new();
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 1);

        h.add_height(60);
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 61);

        for i in 0..60 {
            h.add_height(i);
        }
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 61);

        h.add_height(70);
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 71);

        for i in 0..70 {
            h.add_height(i);
        }
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 71);

        h.add_height(99);
        assert_eq!(h.is_init(), false);
        assert_eq!(h.next_height(), 100);

        for i in 0..99 {
            h.add_height(i);
        }
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 100);

        h.add_height(100);
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 101);

        h.add_height(101);
        assert_eq!(h.is_init(), true);
        assert_eq!(h.next_height(), 102);
    }
}

// verify chain id and signature
pub fn verify_tx_sig(req: &VerifyTxReq) -> Result<Vec<u8>, ()> {
    let hash = H256::from(req.get_hash());
    let sig_bytes = req.get_signature();
    if sig_bytes.len() != SIGNATURE_BYTES_LEN {
        return Err(());
    }

    let sig = Signature::from(sig_bytes);
    match req.get_crypto() {
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

pub struct MsgHandler {
    rx_sub: Receiver<(String, Vec<u8>)>,
    tx_pub: Sender<(String, Vec<u8>)>,
    // only cache verify sig
    cache: LruCache<H256, Option<Vec<u8>>>,
    chain_id: Option<u32>,
    history_heights: HistoryHeights,
    cache_block_req: Option<VerifyBlockReq>,
    history_hashes: HashMap<u64, HashSet<H256>>,
    dispatcher: Dispatcher,
    check_quota: bool,
    block_gas_limit: u64,
    account_gas_limit: AccountGasLimit,
    tx_request: Sender<Request>,
    tx_pool_limit: usize,
    is_recovery_mod: bool,
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
            check_quota: false,
            block_gas_limit: 0,
            account_gas_limit: AccountGasLimit::new(),
            tx_request,
            tx_pool_limit,
            is_recovery_mod: false,
        }
    }

    fn is_ready(&self) -> bool {
        self.history_heights.is_init() && self.chain_id.is_some() && !self.is_recovery_mod
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

    fn get_ret_from_cache(&self, tx_hash: &H256) -> Option<Option<Vec<u8>>> {
        self.cache
            .peek(tx_hash)
            .map(|option_pubkey| option_pubkey.clone())
    }

    fn save_ret_to_cache(&mut self, tx_hash: H256, option_pubkey: Option<Vec<u8>>) {
        self.cache.put(tx_hash, option_pubkey);
    }

    pub fn verify_block_quota(&self, blkreq: &VerifyBlockReq) -> bool {
        let reqs = blkreq.get_reqs();
        let mut gas_limit = self.account_gas_limit.get_common_gas_limit();
        let mut specific_gas_limit = self.account_gas_limit.get_specific_gas_limit().clone();
        let mut account_gas_used: HashMap<Address, u64> = HashMap::new();
        let mut n = self.block_gas_limit;
        for req in reqs {
            let quota = req.get_quota();
            let signer = pubkey_to_address(&PubKey::from(req.get_signer()));

            if n < quota {
                return false;
            }

            if self.check_quota {
                if account_gas_used.contains_key(&signer) {
                    if let Some(value) = account_gas_used.get_mut(&signer) {
                        if *value < quota {
                            return false;
                        } else {
                            *value = *value - quota;
                        }
                    }
                } else {
                    if let Some(value) = specific_gas_limit.remove(&signer.lower_hex()) {
                        gas_limit = value;
                    }
                    let mut _remainder = 0;
                    if quota < gas_limit {
                        _remainder = gas_limit - quota;
                    } else {
                        return false;
                    }
                    account_gas_used.insert(Address::from(signer), _remainder);
                }
            }
            n = n - quota;
        }
        true
    }

    pub fn verify_tx_quota(&self, quota: u64, signer: &[u8]) -> bool {
        if quota > self.block_gas_limit {
            return false;
        }
        if self.check_quota {
            let addr = pubkey_to_address(&PubKey::from(signer));
            let mut gas_limit = self.account_gas_limit.get_common_gas_limit();
            let mut specific_gas_limit = self.account_gas_limit.get_specific_gas_limit().clone();
            if let Some(value) = specific_gas_limit.remove(&addr.lower_hex()) {
                gas_limit = value;
            }
            if quota > gas_limit {
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

        // for block verify, req must include signer
        for req in blk_req.get_reqs() {
            let req_signer = req.get_signer();
            if req_signer.is_empty() {
                let tx_hash = H256::from_slice(req.get_tx_hash());
                self.publish_block_verification_result(request_id, Ret::BadSig);
                self.save_ret_to_cache(tx_hash, None);
                return;
            }
        }

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
                let result = verify_tx_sig(&req);
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

    // verify chain id, nonce, valid_until_block, dup and quota
    fn verify_tx_req(&self, req: &VerifyTxReq) -> Ret {
        let chain_id = req.get_chain_id();
        if chain_id != self.chain_id.unwrap() {
            return Ret::BadChainId;
        }

        if req.get_nonce().len() > 128 {
            return Ret::InvalidNonce;
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

        return Ret::OK;
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

    fn publish_tx_success_result(&self, request_id: Vec<u8>, ret: Ret, tx_hash: H256) {
        let mut response = Response::new();
        response.set_request_id(request_id);

        let result = format!("{:?}", ret);
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

    fn send_block_tx_hashes_req(&self) {
        // we will send req for all height
        // so don't too frequent
        if self.history_heights.is_too_frequent() {
            return;
        }
        for i in self.history_heights.min_height()..self.history_heights.max_height() {
            if !self.history_hashes.contains_key(&i) {
                let mut req = BlockTxHashesReq::new();
                req.set_height(i);
                let msg: Message = req.into();
                self.tx_pub
                    .send((
                        routing_key!(Auth >> BlockTxHashesReq).into(),
                        msg.try_into().unwrap(),
                    ))
                    .unwrap();
            }
        }
    }

    pub fn handle_remote_msg(&mut self) {
        loop {
            // send request to get chain id if we have not got it
            if self.chain_id.is_none() {
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
                trace!("auth is not ready");
                self.send_block_tx_hashes_req();
            }

            // process message from MQ
            match self.rx_sub.recv_timeout(Duration::new(3, 0)) {
                Ok((key, payload)) => {
                    let mut msg = Message::try_from(&payload).unwrap();
                    let rounting_key = RoutingKey::from(&key);
                    match rounting_key {
                        // we got this message when chain reach new height or response the BlockTxHashesReq
                        routing_key!(Chain >> BlockTxHashes) => {
                            let block_tx_hashes = msg.take_block_tx_hashes().unwrap();
                            let height = block_tx_hashes.get_height();
                            info!("get block tx hashes for height {:?}", height);

                            // because next height init value is 1
                            // the empty chain first msg height is 0 with quota info
                            if height >= self.history_heights.next_height() || height == 0 {
                                // get latest quota info from chain
                                let block_gas_limit = block_tx_hashes.get_block_gas_limit();
                                let account_gas_limit =
                                    block_tx_hashes.get_account_gas_limit().clone();
                                let check_quota = block_tx_hashes.get_check_quota();
                                self.check_quota = check_quota;
                                self.block_gas_limit = block_gas_limit;
                                self.account_gas_limit = account_gas_limit.clone();
                            }

                            // update history block tx hashes
                            let old_min_height = self.history_heights.min_height();

                            self.history_heights.add_height(height);

                            // remove unnecessary oldest history block tx hashes
                            for i in old_min_height..self.history_heights.min_height() {
                                self.history_hashes.remove(&i);
                            }

                            let tx_hashes = block_tx_hashes.get_tx_hashes();
                            let mut tx_hashes_h256 = HashSet::with_capacity(tx_hashes.len());
                            for data in tx_hashes.iter() {
                                let hash = H256::from_slice(data);
                                tx_hashes_h256.insert(hash);
                            }

                            // update tx pool
                            self.dispatcher.del_txs_from_pool_with_hash(&tx_hashes_h256);

                            // add new history block tx hashes
                            if !self.history_hashes.contains_key(&height) {
                                self.history_hashes.insert(height, tx_hashes_h256);
                            }

                            if self.is_ready() {
                                // process block verify if we have cached block request
                                if let Some(cache_request_id) = self.cache_block_request_id() {
                                    let cache_height = cache_request_id >> 16;
                                    if cache_height == self.history_heights.next_height() {
                                        let cache_block_req = self.cache_block_req.take().unwrap();
                                        self.process_block_verify(cache_block_req);
                                    }
                                }

                                // proposal new block
                                self.dispatcher.proposal_tx_list(
                                    (self.history_heights.next_height() - 1) as usize, // todo fix bft
                                    &self.tx_pub,
                                    self.block_gas_limit,
                                    self.account_gas_limit.clone(),
                                    self.check_quota,
                                );
                            } else {
                                trace!("auth is not ready");
                                // not ready means block hashes of some height we not have
                                // we will send request for these height
                                self.send_block_tx_hashes_req();
                            }
                        }
                        routing_key!(Consensus >> VerifyBlockReq) => {
                            let blk_req = msg.take_verify_block_req().unwrap();
                            let tx_cnt = blk_req.get_reqs().len();
                            info!("get block verify request with {:?} request", tx_cnt);

                            if tx_cnt == 0 {
                                error!(
                                    "Wrong block verify request with 0 tx request_id: {} from key: {}",
                                    blk_req.get_id(),
                                    key
                                );
                                continue;
                            }

                            if !self.is_ready() {
                                trace!("auth is not ready");
                                self.update_cache_block_req(blk_req);
                                continue;
                            }

                            self.process_block_verify(blk_req);
                        }
                        routing_key!(Net >> Request)
                        | routing_key!(Jsonrpc >> RequestNewTxBatch) => {
                            let is_local = rounting_key.is_sub_module(SubModules::Jsonrpc);
                            let newtx_req = msg.take_request().unwrap();
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
                                            self.publish_tx_failed_result(
                                                request_id,
                                                Ret::NotReady,
                                            );
                                        }
                                    }
                                    continue;
                                }

                                if self.is_flow_control(batch_new_tx.len()) {
                                    trace!("flow control ...");
                                    if is_local {
                                        for tx_req in batch_new_tx.iter() {
                                            let request_id = tx_req.get_request_id().to_vec();
                                            if is_local {
                                                self.publish_tx_failed_result(
                                                    request_id,
                                                    Ret::Busy,
                                                );
                                            }
                                        }
                                    }
                                    continue;
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
                                                self.publish_tx_failed_result(
                                                    request_id,
                                                    Ret::BadSig,
                                                );
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

                                let results: Vec<(
                                    H256,
                                    Option<Vec<u8>>,
                                )> = requests_no_cached
                                    .into_par_iter()
                                    .map(|(tx_hash, ref req)| {
                                        let result = verify_tx_sig(req);
                                        match result {
                                            Ok(pubkey) => (tx_hash, Some(pubkey)),
                                            Err(_) => (tx_hash, None),
                                        }
                                    })
                                    .collect();

                                for (tx_hash, option_pubkey) in results {
                                    self.save_ret_to_cache(tx_hash.clone(), option_pubkey.clone());
                                    if let Some(pubkey) = option_pubkey {
                                        if let Some(ref mut v) = requests.get_mut(&tx_hash) {
                                            v.0.set_signer(pubkey);
                                        }
                                    } else {
                                        if let Some(ref mut v) = requests.get_mut(&tx_hash) {
                                            if is_local {
                                                let request_id = v.1.get_request_id().to_vec();
                                                self.publish_tx_failed_result(
                                                    request_id,
                                                    Ret::BadSig,
                                                );
                                            }
                                            v.2 = false;
                                        }
                                    }
                                }

                                // other verify
                                requests
                                    .into_iter()
                                    .filter(|(_tx_hash, (_req, _tx_req, flag))| *flag)
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
                                        signed_tx
                                            .set_transaction_with_sig(tx_req.get_un_tx().clone());
                                        signed_tx.set_signer(req.get_signer().to_vec());
                                        signed_tx.set_tx_hash(tx_hash.to_vec());
                                        let request_id = tx_req.get_request_id().to_vec();
                                        if self.dispatcher.add_tx_to_pool(&signed_tx) {
                                            if is_local {
                                                self.publish_tx_success_result(
                                                    request_id,
                                                    Ret::OK,
                                                    tx_hash.clone(),
                                                );
                                            }
                                            // new tx need forward to other nodes
                                            self.forward_request(tx_req.clone());
                                        } else {
                                            // dup with transaction in tx pool
                                            if is_local {
                                                self.publish_tx_success_result(
                                                    request_id,
                                                    Ret::Dup,
                                                    tx_hash.clone(),
                                                );
                                            }
                                        }
                                    });
                            } else if newtx_req.has_un_tx() {
                                trace!("get single new tx request from Jsonrpc");
                                let request_id = newtx_req.get_request_id().to_vec();
                                if !self.is_ready() {
                                    trace!("auth is not ready");
                                    if is_local {
                                        self.publish_tx_failed_result(request_id, Ret::NotReady);
                                    }
                                    continue;
                                }
                                if self.is_flow_control(1) {
                                    trace!("flow control ...");
                                    if is_local {
                                        self.publish_tx_failed_result(request_id, Ret::Busy);
                                    }
                                    continue;
                                }
                                let mut req = newtx_req.get_un_tx().tx_verify_req_msg();
                                // verify with cache
                                let tx_hash = H256::from_slice(req.get_tx_hash());
                                if let Some(option_pubkey) = self.get_ret_from_cache(&tx_hash) {
                                    if option_pubkey.is_none() {
                                        self.publish_tx_failed_result(request_id, Ret::BadSig);
                                        continue;
                                    }
                                    req.set_signer(option_pubkey.unwrap());
                                } else {
                                    let result = verify_tx_sig(&req);
                                    self.save_ret_to_cache(tx_hash, result.clone().ok());
                                    match result {
                                        Ok(pubkey) => {
                                            req.set_signer(pubkey);
                                        }
                                        Err(_) => {
                                            if is_local {
                                                self.publish_tx_failed_result(
                                                    request_id,
                                                    Ret::BadSig,
                                                );
                                            }
                                            continue;
                                        }
                                    }
                                }
                                // other verify
                                let ret = self.verify_tx_req(&req);
                                if ret != Ret::OK {
                                    if is_local {
                                        self.publish_tx_failed_result(request_id, ret);
                                    }
                                    continue;
                                }

                                // add tx pool
                                let mut signed_tx = SignedTransaction::new();
                                signed_tx.set_transaction_with_sig(newtx_req.get_un_tx().clone());
                                signed_tx.set_signer(req.get_signer().to_vec());
                                signed_tx.set_tx_hash(tx_hash.to_vec());
                                if self.dispatcher.add_tx_to_pool(&signed_tx) {
                                    if is_local {
                                        self.publish_tx_success_result(
                                            request_id,
                                            Ret::OK,
                                            tx_hash,
                                        );
                                    }
                                    // new tx need forward to other nodes
                                    self.forward_request(newtx_req);
                                } else {
                                    // dup with transaction in tx pool
                                    if is_local {
                                        self.publish_tx_success_result(
                                            request_id,
                                            Ret::Dup,
                                            tx_hash,
                                        );
                                    }
                                }
                            }
                        }
                        routing_key!(Executor >> Miscellaneous) => {
                            let miscellaneous = msg.take_miscellaneous().unwrap();
                            info!("Get chain_id({}) from executor", miscellaneous.chain_id);
                            self.chain_id = Some(miscellaneous.chain_id);
                        }
                        routing_key!(Snapshot >> SnapshotReq) => {
                            let req = msg.take_snapshot_req().unwrap();
                            let mut resp = SnapshotResp::new();
                            match req.cmd {
                                Cmd::Begin => {
                                    resp.set_resp(Resp::BeginAck);
                                    let msg: Message = resp.into();
                                    info!("[snapshot] auth send BeginAck");
                                    self.tx_pub
                                        .send((
                                            routing_key!(Auth >> SnapshotResp).into(),
                                            (&msg).try_into().unwrap(),
                                        ))
                                        .unwrap();
                                }
                                Cmd::Clear => {
                                    resp.set_resp(Resp::ClearAck);
                                    self.is_recovery_mod = true;
                                    self.dispatcher.clear_txs_pool(0);
                                    let msg: Message = resp.into();
                                    info!("[snapshot] auth send ClearAck");
                                    self.tx_pub
                                        .send((
                                            routing_key!(Auth >> SnapshotResp).into(),
                                            (&msg).try_into().unwrap(),
                                        ))
                                        .unwrap();
                                }
                                Cmd::End => {
                                    resp.set_resp(Resp::EndAck);
                                    self.is_recovery_mod = false;
                                    let msg: Message = resp.into();
                                    info!("[snapshot] auth send EndAck");
                                    self.tx_pub
                                        .send((
                                            routing_key!(Auth >> SnapshotResp).into(),
                                            (&msg).try_into().unwrap(),
                                        ))
                                        .unwrap();
                                }
                                _ => {
                                    warn!(
                                        "[snapshot_req]receive: unexpected snapshot cmd = {:?}",
                                        req.cmd
                                    );
                                }
                            }
                        }
                        _ => {
                            error!("receive unexpected message key {}", key);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
