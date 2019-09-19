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
use block_verify::BlockVerify;
use cita_types::traits::LowerHex;
use cita_types::{clean_0x, Address, H256, U256};
use crypto::{pubkey_to_address, PubKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use dispatcher::Dispatcher;
use error::ErrorCode;
use history::HistoryHeights;
use jsonrpc_types::rpc_types::TxResponse;
use libproto::auth::{Miscellaneous, MiscellaneousReq};
use libproto::blockchain::{AccountGasLimit, SignedTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
use libproto::{
    BlackList, BlockTxHashes, BlockTxHashesReq, BlockTxn, Crypto, GetBlockTxn, Message,
    OperateType, Origin, Request, Response, UnverifiedTransaction, VerifyBlockReq, VerifyTxReq,
};
use libproto::{TryFrom, TryInto};
use lru::LruCache;
use pubsub::channel::{Receiver, Sender};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::ThreadPoolBuilder;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::convert::Into;
use std::str::FromStr;
use std::time::Duration;
use transaction_verify::Error;
use util::BLOCKLIMIT;

const TX_OK: &str = "OK";

// verify signature
pub fn verify_tx_sig(crypto: Crypto, hash: &H256, sig_bytes: &[u8]) -> Result<Vec<u8>, ()> {
    if sig_bytes.len() != SIGNATURE_BYTES_LEN {
        return Err(());
    }

    let sig = Signature::from(sig_bytes);
    match crypto {
        Crypto::DEFAULT => sig
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
    history_hashes: HashMap<u64, HashSet<H256>>,
    dispatcher: Dispatcher,
    tx_request: Sender<Request>,
    tx_pool_limit: usize,
    is_snapshot: bool,
    black_list_cache: HashMap<Address, i8>,
    is_need_proposal_new_block: bool,
    config_info: SysConfigInfo,
    block_txn_req: Option<(BlockTxnReq)>,
    verify_block_req: Option<VerifyBlockReq>,
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
            verify_block_req: None,
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

    #[allow(unknown_lints, clippy::option_option)] // TODO clippy
    fn get_ret_from_cache(&self, tx_hash: &H256) -> Option<Option<Vec<u8>>> {
        self.cache.peek(tx_hash).cloned()
    }

    fn save_ret_to_cache(&mut self, tx_hash: H256, option_pubkey: Option<Vec<u8>>) {
        self.cache.put(tx_hash, option_pubkey);
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

    // verify to and version
    fn verify_request(&self, req: &Request) -> Result<(), Error> {
        let un_tx = req.get_un_tx();
        let tx = un_tx.get_transaction();
        let tx_version = tx.get_version();
        if tx_version != self.config_info.version.unwrap() {
            return Err(Error::InvalidVersion);
        }
        if tx_version == 0 {
            // new to must be empty
            if !tx.get_to_v1().is_empty() {
                return Err(Error::InvalidValue);
            }
            let to = clean_0x(tx.get_to());
            if !to.is_empty() && Address::from_str(to).is_err() {
                return Err(Error::InvalidValue);
            }
        } else if tx_version < 3 {
            // old to must be empty
            if !tx.get_to().is_empty() {
                return Err(Error::InvalidValue);
            }
            // check to_v1
            let to = tx.get_to_v1();
            if !to.is_empty() && to.len() != 20 {
                return Err(Error::InvalidValue);
            }
        } else {
            error!("unexpected version {}!", tx_version);
            return Err(Error::InvalidValue);
        }

        Ok(())
    }

    /// Verify black list
    fn verify_black_list(&self, req: &VerifyTxReq) -> Result<(), Error> {
        if let Some(credit) = self
            .black_list_cache
            .get(&pubkey_to_address(&PubKey::from_slice(req.get_signer())))
        {
            if *credit < 0 {
                Err(Error::Forbidden)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    fn verify_tx_req_chain_id(&self, req: &VerifyTxReq) -> Result<(), Error> {
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
            version if version < 3 => {
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
            return Err(Error::BadChainId);
        }

        Ok(())
    }

    // verify chain id, nonce, value, valid_until_block, dup, quota and black list
    fn verify_tx_req(&self, req: &VerifyTxReq) -> Result<(), Error> {
        let ret = self.verify_tx_req_chain_id(req);
        if ret.is_err() {
            return ret;
        }

        if req.get_nonce().len() > 128 {
            return Err(Error::InvalidNonce);
        }

        if req.get_value().len() != 32 {
            return Err(Error::InvalidValue);
        }

        if self
            .config_info
            .admin_address
            .map(|admin| pubkey_to_address(&PubKey::from_slice(req.get_signer())) != admin)
            .unwrap_or_else(|| false)
        {
            return Err(Error::Forbidden);
        }

        let valid_until_block = req.get_valid_until_block();
        let next_height = self.history_heights.next_height();
        if valid_until_block < next_height || valid_until_block >= (next_height + BLOCKLIMIT) {
            return Err(Error::InvalidUntilBlock);
        }

        let tx_hash = H256::from_slice(req.get_tx_hash());
        for (height, hashes) in &self.history_hashes {
            if hashes.contains(&tx_hash) {
                trace!(
                    "Tx with hash {:?} has already existed in height:{}",
                    tx_hash,
                    height
                );
                return Err(Error::Dup);
            }
        }

        if !self.verify_tx_quota(req.get_quota(), req.get_signer()) {
            return Err(Error::QuotaNotEnough);
        }

        Ok(())
    }

    fn publish_tx_failed_result(&self, request_id: Vec<u8>, ret: &Error) {
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

        let tx_response = TxResponse::new(tx_hash, TX_OK.to_string());
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

    fn daily_task(&mut self) {
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

    fn get_chain_id(&mut self) {
        if self.chain_id.is_none() && self.config_info.version.is_some() {
            trace!("chain id is not ready");
            let msg: Message = MiscellaneousReq::new().into();
            if let Ok(rabbit_mq_msg) = msg.try_into() {
                if let Err(e) = self
                    .tx_pub
                    .send((routing_key!(Auth >> MiscellaneousReq).into(), rabbit_mq_msg))
                {
                    error!("Send MiscellaneousReq message error {:?}", e);
                }
            } else {
                error!("Can not get rabbit mq message from MiscellaneousReq.");
            }
        }
    }

    fn process_msg(&mut self) {
        if let Ok((key, payload)) = self.rx_sub.recv_timeout(Duration::new(3, 0)) {
            if Message::try_from(&payload).is_err() {
                error!("Can not get message from payload {:?}", &payload);
                return;
            }

            let mut msg = Message::try_from(&payload).unwrap();
            let rounting_key = RoutingKey::from(&key);
            trace!("process message key = {}", key);
            match rounting_key {
                // we got this message when chain reach new height or response the BlockTxHashesReq
                routing_key!(Chain >> BlockTxHashes) => {
                    if let Some(block_tx_hashes) = msg.take_block_tx_hashes() {
                        self.deal_block_tx_hashes(&block_tx_hashes)
                    } else {
                        error!("Can not get block tx hashes from message {:?}.", msg);
                    }
                }
                routing_key!(Executor >> BlackList) => {
                    if let Some(black_list) = msg.take_black_list() {
                        self.deal_black_list(&black_list);
                    } else {
                        error!("Can not get black list from message {:?}.", msg);
                    }
                }
                routing_key!(Net >> Request) | routing_key!(Jsonrpc >> RequestNewTxBatch) => {
                    if let Some(newtx_req) = msg.take_request() {
                        let is_local = rounting_key.is_sub_module(SubModules::Jsonrpc);
                        self.deal_request(is_local, newtx_req);
                    } else {
                        error!("Can not get request from message {:?}.", msg);
                    }
                }
                routing_key!(Executor >> Miscellaneous) => {
                    if let Some(miscellaneous) = msg.take_miscellaneous() {
                        self.deal_miscellaneous(&miscellaneous);
                    } else {
                        error!("Can not get miscellaneous from message {:?}.", msg);
                    }
                }
                routing_key!(Snapshot >> SnapshotReq) => {
                    if let Some(snapshot_req) = msg.take_snapshot_req() {
                        self.deal_snapshot(&snapshot_req);
                    } else {
                        error!("Can not get snapshot from message {:?}.", msg);
                    }
                }
                routing_key!(Net >> GetBlockTxn) => {
                    if let Some(mut get_block_txn) = msg.take_get_block_txn() {
                        let origin = msg.get_origin();
                        self.deal_get_block_txn(&mut get_block_txn, origin);
                    } else {
                        error!("Can not get block txn from message {:?}.", msg);
                    }
                }
                // Compact proposal
                routing_key!(Consensus >> VerifyBlockReq) => {
                    if !self.is_ready() {
                        info!("Net/Consensus >> CompactProposal: auth is not ready");
                    } else {
                        self.deal_signed_proposal(msg);
                    }
                }
                routing_key!(Net >> BlockTxn) => {
                    if !self.is_ready() || self.verify_block_req.is_none() {
                        info!("Net >> BlockTxn: auth is not ready");
                    } else {
                        let verify_block_req = self.verify_block_req.clone();
                        if let Some(verify_block_req) = verify_block_req {
                            self.deal_block_txn(msg, verify_block_req);
                        }
                    }
                }
                _ => {
                    error!("receive unexpected message key {}", key);
                }
            }
        }
    }
    pub fn handle_remote_msg(&mut self) {
        loop {
            // send request to get chain id if we have not got it
            // chain id need version
            // so get chain id after get version
            self.get_chain_id();

            // block hashes of some height we not have
            // we will send request for these height
            if !self.history_heights.is_init() {
                trace!("history block hashes is not ready");
                self.send_block_tx_hashes_req(true);
            }

            // Daily tasks
            self.daily_task();

            // process message from MQ
            self.process_msg();
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
            let block_tx_version = block_tx_hashes.get_version();
            let check_version = block_tx_version > 0 && block_tx_version < 3;
            // Get chain id according to version
            if check_version && self.config_info.version == Some(0) {
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
            .iter()
            .for_each(|clear_list: &Vec<u8>| {
                self.black_list_cache
                    .remove(&Address::from_slice(clear_list.as_slice()));
            });

        black_list
            .get_black_list()
            .iter()
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
                        self.publish_tx_failed_result(request_id, &Error::NotReady);
                    }
                }
                return;
            }

            if self.is_flow_control(batch_new_tx.len()) {
                trace!("flow control ...");
                if is_local {
                    for tx_req in batch_new_tx.iter() {
                        let request_id = tx_req.get_request_id().to_vec();
                        self.publish_tx_failed_result(request_id, &Error::Busy);
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
                            self.publish_tx_failed_result(request_id, &Error::BadSig);
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
                        self.publish_tx_failed_result(request_id, &Error::BadSig);
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
                    if ret.is_err() {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, &ret.unwrap_err());
                        }
                        false
                    } else {
                        true
                    }
                })
                .filter(|(_tx_hash, (ref _req, ref tx_req, _flag))| {
                    let ret = self.verify_request(tx_req);
                    if ret.is_err() {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, &ret.unwrap_err());
                        }
                        false
                    } else {
                        true
                    }
                })
                .filter(|(_tx_hash, (ref req, ref tx_req, _flag))| {
                    let ret = self.verify_tx_req(&req);
                    if ret.is_err() {
                        if is_local {
                            let request_id = tx_req.get_request_id().to_vec();
                            self.publish_tx_failed_result(request_id, &ret.unwrap_err());
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
                        self.publish_tx_failed_result(request_id, &Error::Dup);
                    }
                });
        } else if newtx_req.has_un_tx() {
            trace!("get single new tx request from Jsonrpc");
            let request_id = newtx_req.get_request_id().to_vec();
            if !self.is_ready() {
                trace!("net || jsonrpc: auth is not ready");
                if is_local {
                    self.publish_tx_failed_result(request_id, &Error::NotReady);
                }
                return;
            }
            if self.is_flow_control(1) {
                trace!("flow control ...");
                if is_local {
                    self.publish_tx_failed_result(request_id, &Error::Busy);
                }
                return;
            }
            let mut req = newtx_req.get_un_tx().tx_verify_req_msg();
            // verify with cache
            let tx_hash = H256::from_slice(req.get_tx_hash());
            if let Some(option_pubkey) = self.get_ret_from_cache(&tx_hash) {
                if option_pubkey.is_none() {
                    self.publish_tx_failed_result(request_id, &Error::BadSig);
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
                            self.publish_tx_failed_result(request_id, &Error::BadSig);
                        }
                        return;
                    }
                }
            }

            // black verify
            let ret = self.verify_black_list(&req);
            if ret.is_err() {
                if is_local {
                    self.publish_tx_failed_result(request_id, &ret.unwrap_err());
                }
                return;
            }

            let ret = self.verify_request(&newtx_req);
            if ret.is_err() {
                if is_local {
                    self.publish_tx_failed_result(request_id, &ret.unwrap_err());
                }
                return;
            }

            // other verify
            let ret = self.verify_tx_req(&req);
            if ret.is_err() {
                if is_local {
                    self.publish_tx_failed_result(request_id, &ret.unwrap_err());
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
                self.publish_tx_failed_result(request_id, &Error::Dup);
            }
        }
    }

    fn deal_snapshot(&mut self, snapshot_req: &SnapshotReq) {
        match snapshot_req.cmd {
            Cmd::Snapshot => {
                info!("receive Snapshot::Snapshot: {:?}", snapshot_req);
                snapshot_response(&self.tx_pub, Resp::SnapshotAck, true);
            }
            Cmd::Begin => {
                info!("receive Snapshot::Begin: {:?}", snapshot_req);
                self.is_snapshot = true;
                snapshot_response(&self.tx_pub, Resp::BeginAck, true);
            }
            Cmd::Restore => {
                info!("receive Snapshot::Restore: {:?}", snapshot_req);
                snapshot_response(&self.tx_pub, Resp::RestoreAck, true);
            }
            Cmd::Clear => {
                info!("receive Snapshot::Clear: {:?}", snapshot_req);

                self.dispatcher.clear_txs_pool(0);
                self.cache.clear();
                self.history_heights.reset();
                self.history_hashes.clear();
                self.black_list_cache.clear();

                snapshot_response(&self.tx_pub, Resp::ClearAck, true);
            }
            Cmd::End => {
                info!("receive Snapshot::End: {:?}", snapshot_req);
                self.send_single_block_tx_hashes_req(snapshot_req.end_height);
                self.is_snapshot = false;
                snapshot_response(&self.tx_pub, Resp::EndAck, true);
            }
        }
    }

    fn deal_miscellaneous(&mut self, miscellaneous: &Miscellaneous) {
        if let Some(version) = self.config_info.version {
            self.chain_id = if version == 0 {
                Some(ChainId::V0(miscellaneous.chain_id))
            } else if version < 3 {
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
            .iter()
            .map(|id| H256::from(U256::from(id.as_slice())))
            .collect();
        let txs: Vec<UnverifiedTransaction> = self
            .dispatcher
            .get_txs(&short_ids)
            .into_iter()
            .map(|mut tx| tx.take_transaction_with_sig())
            .collect();

        info!("GetBlockTxn size: {}, origin: {}", txs.len(), origin);

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

    fn deal_signed_proposal(&mut self, mut msg: Message) {
        let verify_block_req = msg.take_verify_block_req().unwrap();
        let block_hash = verify_block_req.get_block().crypt_hash();
        let tx_hashes = verify_block_req.get_block().get_body().transaction_hashes();
        let origin = msg.get_origin();

        {
            if tx_hashes.is_empty() {
                return;
            };

            // Check tx hash in cache
            for tx_hash in tx_hashes.clone() {
                if let Some(option_pubkey) = self.get_ret_from_cache(&tx_hash) {
                    // BadSig
                    if option_pubkey.is_none() {
                        let resp = verify_block_req.reply(Err(()));
                        let msg = Message::init(OperateType::Single, origin, resp.into());
                        self.tx_pub
                            .send((
                                routing_key!(Auth >> VerifyBlockResp).into(),
                                (&msg).try_into().unwrap(),
                            ))
                            .unwrap();
                        return;
                    }
                }
            }
        }

        // Check missing hashes
        let missing_hashes = self.dispatcher.check_missing(tx_hashes.clone());

        if missing_hashes.is_empty() {
            // TODO: Refactor
            let transactions = self.dispatcher.get_txs(&tx_hashes);

            if let Err(err) = verify_block_req.check_txs(&transactions[..]) {
                error!("verify_block_req check txs failed {:?}", err);
            }
            let resp = verify_block_req.reply(Ok(transactions));

            let msg = Message::init(OperateType::Single, 0, resp.into());
            self.tx_pub
                .send((
                    routing_key!(Auth >> VerifyBlockResp).into(),
                    (&msg).try_into().unwrap(),
                ))
                .unwrap();
        } else {
            info!("missing_hashes len : {}", missing_hashes.len());
            self.verify_block_req = Some(verify_block_req);

            let mut get_block_txn = GetBlockTxn::new();
            get_block_txn.set_block_hash(block_hash.to_vec());
            let missing_hashes = missing_hashes
                .into_iter()
                .map(|hash| hash.to_vec())
                .collect();
            get_block_txn.set_short_ids(missing_hashes);

            self.block_txn_req = Some((origin, get_block_txn.clone()));

            let msg = Message::init(OperateType::Single, origin, get_block_txn.into());
            self.tx_pub
                .send((
                    routing_key!(Auth >> GetBlockTxn).into(),
                    (&msg).try_into().unwrap(),
                ))
                .unwrap();
        }
    }

    // TODO: Add test
    fn deal_block_txn(&mut self, mut msg: Message, verify_block_req: VerifyBlockReq) {
        let block_txn = msg.take_block_txn().unwrap();
        let origin = msg.get_origin();

        let block_txn_message = BlockTxnMessage { origin, block_txn };
        // Validate and add the transaction to the pool
        if self.validate_block_txn(block_txn_message) {
            let tx_hashes = verify_block_req.get_block().get_body().transaction_hashes();

            let transactions = self.dispatcher.get_txs(&tx_hashes);

            if transactions.len() != tx_hashes.len() {
                info!(
                    "block txn transactions number is not matched, expect: {}, got: {}",
                    tx_hashes.len(),
                    transactions.len()
                );
                return;
            }

            let result = {
                let block = BlockVerify {
                    transactions: &transactions,
                };

                block.verify_quota(
                    self.config_info.block_quota_limit,
                    &self.config_info.account_quota_limit,
                    self.config_info.check_quota,
                )
            };

            // TODO: Refactor
            let resp = if result {
                if let Err(err) = verify_block_req.check_txs(&transactions[..]) {
                    error!("verify_block_req check txs failed {:?}", err);
                }
                verify_block_req.reply(Ok(transactions))
            } else {
                verify_block_req.reply(Err(()))
            };
            let msg = Message::init(OperateType::Single, 0, resp.into());
            self.tx_pub
                .send((
                    routing_key!(Auth >> VerifyBlockResp).into(),
                    (&msg).try_into().unwrap(),
                ))
                .unwrap();
        };
    }

    // TODO: Add test
    fn validate_block_txn(&mut self, mut block_txn: BlockTxnMessage) -> bool {
        // TODO: Need NLL to avoid clone
        if let Some(ref block_txn_req) = self.block_txn_req.clone() {
            let result = block_txn.validate(block_txn_req);
            match result {
                Ok(signed_txn) => {
                    for tx in signed_txn.iter() {
                        let un_tx = tx.get_transaction_with_sig();
                        self.save_ret_to_cache(
                            H256::from_slice(tx.get_tx_hash()),
                            Some(un_tx.clone().get_signature().to_vec()),
                        );

                        let req = un_tx.tx_verify_req_msg();
                        if self.verify_tx_req(&req).is_ok() {
                            self.dispatcher.add_tx_to_pool(tx);
                        }
                    }
                    return true;
                }
                Err(error) => {
                    info!("Validate BlockTxn error: {}", error);
                    return false;
                }
            }
        } else {
            info!("Could not find cached block_txn_req");
            return false;
        }
    }
}

fn snapshot_response(sender: &Sender<(String, Vec<u8>)>, ack: Resp, flag: bool) {
    info!("snapshot_response ack: {:?}, flag: {}", ack, flag);

    let mut resp = SnapshotResp::new();
    resp.set_resp(ack);
    resp.set_flag(flag);
    let msg: Message = resp.into();
    sender
        .send((
            routing_key!(Auth >> SnapshotResp).into(),
            (&msg).try_into().unwrap(),
        ))
        .unwrap();
}
