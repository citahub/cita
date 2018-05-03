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

use error::ErrorCode;
use jsonrpc_types::rpctypes::TxResponse;
use libproto::{Message, Response, Ret, VerifyBlockResp, VerifyTxResp};
use libproto::blockchain::{AccountGasLimit, SignedTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotResp};
use std::collections::{HashMap, HashSet};
use std::convert::{Into, TryFrom, TryInto};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;
use std::vec::*;
use threadpool::ThreadPool;
use util::{H256, RwLock};
use verifier::*;

pub fn process_flow_control_failed(
    mut verify_info: VerifyRequestResponseInfo,
    resp_sender: &Sender<VerifyRequestResponseInfo>,
) {
    let mut response = VerifyTxResp::new();
    if let VerifyRequestResponse::AuthRequest(req) = verify_info.req_resp {
        response.set_tx_hash(req.get_tx_hash().to_vec());
        response.set_ret(Ret::Busy);
        verify_info.req_resp = VerifyRequestResponse::AuthResponse(response);
        resp_sender.send(verify_info).unwrap();
    }
}

#[cfg_attr(feature = "clippy", allow(needless_pass_by_value))]
pub fn verify_tx_group_service(
    mut req_grp: Vec<VerifyRequestResponseInfo>,
    verifier: Arc<RwLock<Verifier>>,
    cache: Arc<RwLock<HashMap<H256, VerifyTxResp>>>,
    resp_sender: Sender<VerifyRequestResponseInfo>,
) {
    let now = SystemTime::now();
    let len = req_grp.len();

    while let Some(mut req_info) = req_grp.pop() {
        if let VerifyRequestResponse::AuthRequest(req) = req_info.req_resp {
            let tx_hash = H256::from_slice(req.get_tx_hash());
            let response = { verifier.read().verify_tx(&req) };
            {
                cache.write().insert(tx_hash, response.clone());
            }
            req_info.req_resp = VerifyRequestResponse::AuthResponse(response);
            resp_sender.send(req_info).unwrap();
        }
    }

    trace!(
        "verify_tx_group_service Time cost {} ns for {} req ...",
        now.elapsed().unwrap().subsec_nanos(),
        len
    );
}

pub fn check_verify_request_preprocess(
    mut req_info: VerifyRequestResponseInfo,
    verifier: Arc<RwLock<Verifier>>,
    cache: Arc<RwLock<HashMap<H256, VerifyTxResp>>>,
    resp_sender: &Sender<VerifyRequestResponseInfo>,
) -> VerifyResult {
    if let VerifyRequestResponse::AuthRequest(req) = req_info.req_resp {
        let tx_hash = H256::from_slice(req.get_tx_hash());
        let mut final_response = VerifyTxResp::new();
        let mut processed = false;
        let mut result = VerifyResult::VerifyNotBegin;
        let is_single_verify = req_info.verify_type == VerifyType::SingleVerify;

        if is_single_verify
            && !verifier
                .read()
                .verify_valid_until_block(req.get_valid_until_block())
        {
            let mut response = VerifyTxResp::new();
            response.set_tx_hash(req.get_tx_hash().to_vec());
            response.set_ret(Ret::InvalidUntilBlock);
            processed = true;
            final_response = response;
        } else if let Some(resp) = get_resp_from_cache(&tx_hash, cache.clone()) {
            processed = true;
            final_response = resp;
        }

        if processed {
            match final_response.get_ret() {
                Ret::OK => result = VerifyResult::VerifySucceeded,
                _ => result = VerifyResult::VerifyFailed,
            }
            //only send result when the verify type is single
            if is_single_verify {
                req_info.req_resp = VerifyRequestResponse::AuthResponse(final_response);
                resp_sender.send(req_info).unwrap();
            }
        }
        result
    } else {
        VerifyResult::VerifyNotBegin
    }
}

fn get_resp_from_cache(tx_hash: &H256, cache: Arc<RwLock<HashMap<H256, VerifyTxResp>>>) -> Option<VerifyTxResp> {
    if let Some(resp) = cache.read().get(tx_hash) {
        Some(resp.clone())
    } else {
        None
    }
}

// this function has too many arguments
// the function has a cyclomatic complexity of 29
// consider changing the type to: `&[u8]`
pub fn handle_remote_msg(
    key: String,
    payload: Vec<u8>,
    on_proposal: Arc<AtomicBool>,
    pool: &ThreadPool,
    proposal_tx_verify_num_per_thread: usize,
    verifier: Arc<RwLock<Verifier>>,
    tx_req_single: &Sender<VerifyRequestResponseInfo>,
    tx_pub: &Sender<(String, Vec<u8>)>,
    block_verify_status: Arc<RwLock<BlockVerifyStatus>>,
    cache: Arc<RwLock<HashMap<H256, VerifyTxResp>>>,
    txs_sender: &Sender<(usize, HashSet<H256>, u64, AccountGasLimit)>,
    resp_sender: &Sender<VerifyRequestResponseInfo>,
    clear_txs_pool: Arc<AtomicBool>,
) {
    let mut msg = Message::try_from(&payload).unwrap();
    match RoutingKey::from(&key) {
        routing_key!(Chain >> BlockTxHashes) => {
            let block_tx_hashes = msg.take_block_tx_hashes().unwrap();
            let height = block_tx_hashes.get_height();
            debug!("get block tx hashs for height {:?}", height);
            let tx_hashes = block_tx_hashes.get_tx_hashes();
            let mut tx_hashes_in_h256 = HashSet::with_capacity(tx_hashes.len());
            {
                let mut cache_guard = cache.write();

                for data in tx_hashes.iter() {
                    let hash = H256::from_slice(data);
                    cache_guard.remove(&hash);
                    tx_hashes_in_h256.insert(hash);
                }
            }
            {
                verifier
                    .write()
                    .update_hashes(height, tx_hashes_in_h256.clone(), tx_pub);
            }
            let mut flag = true;
            if let Some(h) = verifier.read().get_height_latest() {
                if height != h {
                    flag = false;
                }
            }
            if flag {
                debug!(
                    "BLOCKTXHASHES come height {}, tx_hashes count is: {:?}",
                    height,
                    tx_hashes_in_h256.len()
                );
                let block_gas_limit = block_tx_hashes.get_block_gas_limit();
                let account_gas_limit = block_tx_hashes.get_account_gas_limit().clone();
                debug!(
                    "Auth rich status block gas limit: {:?}, account gas limit {:?}",
                    block_gas_limit, account_gas_limit
                );
                let _ = txs_sender.send((
                    height as usize,
                    tx_hashes_in_h256,
                    block_gas_limit,
                    account_gas_limit,
                ));
            }
        }
        // TODO: Add ProposalVerifier { status, request_id, threadpool }, Status: On, Failed, Successed, Experied
        // TODO: Make most of the logic asynchronous
        // Verify Proposal from consensus
        routing_key!(Consensus >> VerifyBlockReq) => {
            let blkreq = msg.take_verify_block_req().unwrap();
            let tx_cnt = blkreq.get_reqs().len();
            info!("get block verify request with {:?} request", tx_cnt);
            let mut tx_need_verify = Vec::new();
            if tx_cnt > 0 {
                let request_id = blkreq.get_id();
                let new_block_verify_status = BlockVerifyStatus {
                    request_id: request_id,
                    block_verify_result: VerifyResult::VerifyOngoing,
                    verify_success_cnt_required: tx_cnt,
                    verify_success_cnt_capture: 0,
                    cache_hit: 0,
                };

                info!(
                    "Coming new block verify request with request_id: {}, \
                     and the init block_verify_status: {:?}",
                    request_id, new_block_verify_status
                );
                //add big brace here to release write lock as soon as possible
                {
                    let mut block_verify_status_guard = block_verify_status.write();
                    if block_verify_status_guard.block_verify_result == VerifyResult::VerifyOngoing {
                        warn!(
                            "block verification request with request_id: {:?} \
                             has been expired, and the current info is: {:?}",
                            block_verify_status_guard.request_id, *block_verify_status_guard
                        );
                    }
                    let block_verify_stamp = SystemTime::now();
                    *block_verify_status_guard = new_block_verify_status;
                    let now = SystemTime::now();
                    for req in blkreq.get_reqs() {
                        let verify_request_info = VerifyRequestResponseInfo {
                            key: key.clone(),
                            verify_type: VerifyType::BlockVerify,
                            request_id: VerifyRequestID::BlockVerifyRequestID(request_id),
                            time_stamp: now,
                            req_resp: VerifyRequestResponse::AuthRequest(req.clone()),
                            un_tx: None,
                        };
                        let result = check_verify_request_preprocess(
                            verify_request_info,
                            verifier.clone(),
                            cache.clone(),
                            resp_sender,
                        );
                        match result {
                            VerifyResult::VerifySucceeded => {
                                block_verify_status_guard.verify_success_cnt_capture += 1;
                                block_verify_status_guard.cache_hit += 1;
                                trace!(
                                    "The verification request： {:?} has been cached already",
                                    req
                                );
                            }
                            VerifyResult::VerifyFailed => {
                                // statement with no effect, bug here?
                                block_verify_status_guard.block_verify_result == VerifyResult::VerifyFailed;

                                let tx_hash = H256::from_slice(req.get_tx_hash());
                                let resp_ret;
                                if let Some(resp) = get_resp_from_cache(&tx_hash, cache.clone()) {
                                    resp_ret = resp.get_ret();
                                } else {
                                    error!("Can't get response from cache but could get it just right now");
                                    resp_ret = Ret::BadSig;
                                }
                                warn!(
                                    "Failed to do verify blk req for request_id: {}, ret: {:?}",
                                    request_id, resp_ret
                                );
                                publish_block_verification_result(request_id, resp_ret, tx_pub);
                                break;
                            }
                            _ => {
                                let verify_request_info = VerifyRequestResponseInfo {
                                    key: key.clone(),
                                    verify_type: VerifyType::BlockVerify,
                                    request_id: VerifyRequestID::BlockVerifyRequestID(request_id),
                                    time_stamp: now,
                                    req_resp: VerifyRequestResponse::AuthRequest(req.clone()),
                                    un_tx: None,
                                };
                                tx_need_verify.push(verify_request_info);
                            }
                        }
                    }
                    let tx_need_verify_len = tx_need_verify.len();
                    // Most of time nearly all the transactions hit the cache.
                    if tx_need_verify_len > 0 {
                        on_proposal.store(true, Ordering::SeqCst);
                        let iter = tx_need_verify.chunks(proposal_tx_verify_num_per_thread);
                        for group in iter {
                            let verifier_clone = verifier.clone();
                            let cache_clone = cache.clone();
                            let resp_sender_clone = resp_sender.clone();
                            let group_for_pool = group.to_vec().clone();
                            pool.execute(move || {
                                verify_tx_group_service(
                                    group_for_pool,
                                    verifier_clone,
                                    cache_clone,
                                    resp_sender_clone,
                                );
                            });
                        }
                        on_proposal.store(false, Ordering::SeqCst);
                    } else if block_verify_status_guard.verify_success_cnt_capture
                        == block_verify_status_guard.verify_success_cnt_required
                    {
                        block_verify_status_guard.block_verify_result = VerifyResult::VerifySucceeded;
                        info!(
                            "Succeed to do verify blk req for request_id: {}, \
                             ret: {:?}, \
                             time cost: {:?}, \
                             and final status is: {:?}",
                            request_id,
                            Ret::OK,
                            block_verify_stamp.elapsed().unwrap(),
                            *block_verify_status_guard
                        );
                        publish_block_verification_result(request_id, Ret::OK, tx_pub);
                    }
                }
            } else {
                error!(
                    "Wrong block verification request with 0 tx for block verify request_id: {} from key: {}",
                    blkreq.get_id(),
                    key
                );
            }
        }
        routing_key!(Net >> Request) | routing_key!(Jsonrpc >> RequestNewTxBatch) => {
            let newtx_req = msg.take_request().unwrap();
            if newtx_req.has_batch_req() {
                let batch_new_tx = newtx_req.get_batch_req().get_new_tx_requests();
                let now = SystemTime::now();
                trace!(
                    "get batch new tx request from key:{} in system time :{:?}, and has got {} new tx ",
                    key,
                    now,
                    batch_new_tx.len()
                );

                for tx_req in batch_new_tx.iter() {
                    let verify_tx_req = tx_req.get_un_tx().tx_verify_req_msg();
                    let verify_request_info = VerifyRequestResponseInfo {
                        key: key.clone(),
                        verify_type: VerifyType::SingleVerify,
                        request_id: VerifyRequestID::SingleVerifyRequestID(tx_req.get_request_id().to_vec()),
                        time_stamp: now,
                        req_resp: VerifyRequestResponse::AuthRequest(verify_tx_req),
                        un_tx: Some(tx_req.get_un_tx().clone()),
                    };
                    tx_req_single.send(verify_request_info).unwrap();
                }
            } else if newtx_req.has_un_tx() {
                let now = SystemTime::now();
                trace!(
                    "get single new tx request from peer node with system time :{:?}",
                    now
                );
                let verify_tx_req = newtx_req.get_un_tx().tx_verify_req_msg();
                let verify_request_info = VerifyRequestResponseInfo {
                    key: key.clone(),
                    verify_type: VerifyType::SingleVerify,
                    request_id: VerifyRequestID::SingleVerifyRequestID(newtx_req.get_request_id().to_vec()),
                    time_stamp: now,
                    req_resp: VerifyRequestResponse::AuthRequest(verify_tx_req),
                    un_tx: Some(newtx_req.get_un_tx().clone()),
                };

                tx_req_single.send(verify_request_info).unwrap();
            }
        }
        routing_key!(Snapshot >> SnapshotReq) => {
            let req = msg.take_snapshot_req().unwrap();
            let mut resp = SnapshotResp::new();
            match req.cmd {
                Cmd::Begin => {
                    resp.set_resp(Resp::BeginAck);
                    let msg: Message = resp.into();
                    info!("auth resp BeginAck");
                    tx_pub
                        .send((
                            routing_key!(Auth >> SnapshotResp).into(),
                            (&msg).try_into().unwrap(),
                        ))
                        .unwrap();
                }
                Cmd::Clear => {
                    resp.set_resp(Resp::ClearAck);
                    clear_txs_pool.store(true, Ordering::SeqCst);
                    let msg: Message = resp.into();
                    info!("auth resp ClearAck");
                    tx_pub
                        .send((
                            routing_key!(Auth >> SnapshotResp).into(),
                            (&msg).try_into().unwrap(),
                        ))
                        .unwrap();
                }
                Cmd::End => {
                    resp.set_resp(Resp::EndAck);
                    clear_txs_pool.store(false, Ordering::SeqCst);
                    let msg: Message = resp.into();
                    info!("auth resp EndAck");
                    tx_pub
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
        routing_key!(Executor >> Miscellaneous) => {
            let miscellaneous = msg.take_miscellaneous().unwrap();
            info!("The chain_id from executor is {}", miscellaneous.chain_id);
            verifier
                .try_write()
                .unwrap()
                .set_chain_id(miscellaneous.chain_id);
        }
        _ => {}
    }
}

pub fn handle_verification_result(
    result_receiver: &Receiver<VerifyRequestResponseInfo>,
    tx_pub: &Sender<(String, Vec<u8>)>,
    block_verify_status: Arc<RwLock<BlockVerifyStatus>>,
    tx_sender: &Sender<(String, Vec<u8>, TxResponse, SignedTransaction)>,
) {
    match result_receiver.recv() {
        Ok(verify_response_info) => {
            if let VerifyRequestResponse::AuthResponse(resp) = verify_response_info.req_resp {
                match verify_response_info.verify_type {
                    VerifyType::SingleVerify => {
                        let tx_hash: H256 = resp.get_tx_hash().into();
                        trace!(
                            "SingleVerify Time cost {} ns for tx hash: {:?}",
                            verify_response_info
                                .time_stamp
                                .elapsed()
                                .unwrap()
                                .subsec_nanos(),
                            resp.get_tx_hash()
                        );
                        trace!(
                            "receive verify resp, hash: {:?}, ret: {:?}",
                            tx_hash,
                            resp.get_ret()
                        );

                        if let VerifyRequestID::SingleVerifyRequestID(request_id) = verify_response_info.request_id {
                            let result = format!("{:?}", resp.get_ret());
                            match resp.get_ret() {
                                Ret::OK => {
                                    let mut signed_tx = SignedTransaction::new();
                                    signed_tx.set_transaction_with_sig(verify_response_info.un_tx.unwrap());
                                    signed_tx.set_signer(resp.get_signer().to_vec());
                                    signed_tx.set_tx_hash(tx_hash.to_vec());
                                    let tx_response = TxResponse::new(tx_hash, result.clone());
                                    let _ = tx_sender.send((
                                        verify_response_info.key,
                                        request_id.clone(),
                                        tx_response,
                                        signed_tx.clone(),
                                    ));
                                    trace!("Send singed tx to txpool");
                                }
                                _ => {
                                    if RoutingKey::from(&verify_response_info.key).is_sub_module(SubModules::Jsonrpc) {
                                        let tx_response = TxResponse::new(tx_hash, result);

                                        let mut response = Response::new();
                                        response.set_request_id(request_id);
                                        response.set_code(ErrorCode::tx_auth_error());
                                        response.set_error_msg(tx_response.status);

                                        trace!("response new tx {:?}", response);
                                        let msg: Message = response.into();
                                        tx_pub
                                            .send((
                                                routing_key!(Auth >> Response).into(),
                                                msg.try_into().unwrap(),
                                            ))
                                            .unwrap();
                                    }
                                }
                            }
                        }
                    }
                    VerifyType::BlockVerify => {
                        if let VerifyRequestID::BlockVerifyRequestID(request_id) = verify_response_info.request_id {
                            let result = resp.get_ret();
                            if Ret::OK != result {
                                let mut block_verify_status_guard = block_verify_status.write();
                                if request_id == block_verify_status_guard.request_id
                                    && VerifyResult::VerifyFailed != block_verify_status_guard.block_verify_result
                                {
                                    block_verify_status_guard.block_verify_result = VerifyResult::VerifyFailed;
                                    warn!(
                                        "Failed to do verify blk req for request_id: {}, ret: {:?}, from key: {}",
                                        request_id, result, verify_response_info.key
                                    );
                                    publish_block_verification_result(request_id, result, tx_pub);
                                }
                            } else {
                                let mut block_verify_status_guard = block_verify_status.write();
                                if request_id == block_verify_status_guard.request_id {
                                    trace!(
                                        "BlockVerify Time cost {:?} for tx hash: {:?}",
                                        verify_response_info.time_stamp.elapsed().unwrap(),
                                        resp.get_tx_hash()
                                    );
                                    block_verify_status_guard.verify_success_cnt_capture += 1;
                                    if block_verify_status_guard.verify_success_cnt_capture
                                        == block_verify_status_guard.verify_success_cnt_required
                                    {
                                        block_verify_status_guard.block_verify_result = VerifyResult::VerifySucceeded;
                                        info!(
                                            "Succeed to do verify blk req for request_id: {}, \
                                             ret: {:?}, \
                                             time cost: {:?}, \
                                             and final status is: {:?}",
                                            request_id,
                                            Ret::OK,
                                            verify_response_info.time_stamp.elapsed().unwrap(),
                                            *block_verify_status_guard
                                        );
                                        publish_block_verification_result(request_id, Ret::OK, tx_pub);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(err_info) => {
            error!(
                "Failed to receive message from result_receiver due to {:?}",
                err_info
            );
        }
    }
}

pub fn publish_block_verification_fail_result(
    request_id: u64,
    hash: &H256,
    cache: Arc<RwLock<HashMap<H256, VerifyTxResp>>>,
    tx_pub: &Sender<(String, Vec<u8>)>,
) {
    let ret: Ret;
    if let Some(resp) = get_resp_from_cache(hash, cache) {
        ret = resp.get_ret();
    } else {
        ret = Ret::BadSig;
        error!(
            "Failed to get response from cache for tx with hash: {:?}",
            hash
        );
    }
    publish_block_verification_result(request_id, ret, tx_pub);
}

fn publish_block_verification_result(request_id: u64, ret: Ret, tx_pub: &Sender<(String, Vec<u8>)>) {
    let mut blkresp = VerifyBlockResp::new();
    blkresp.set_id(request_id);
    blkresp.set_ret(ret);

    let msg: Message = blkresp.into();
    tx_pub
        .send((
            routing_key!(Auth >> VerifyBlockResp).into(),
            msg.try_into().unwrap(),
        ))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::*;
    use libproto::{BlockTxHashes, Message, Request, Ret, SignedTransaction, Transaction, VerifyBlockReq, VerifyTxReq};
    use libproto::router::{MsgType, RoutingKey, SubModules};
    use protobuf::RepeatedField;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;
    use threadpool;
    use util::{H256, U256};
    use util::Hashable;
    use uuid::Uuid;

    const BLOCK_REQUEST_ID: u64 = 0x0123456789abcdef;

    fn generate_tx(data: Vec<u8>, valid_until_block: u64, privkey: &PrivKey, chain_id: u32) -> SignedTransaction {
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_to("1234567".to_string());
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(valid_until_block);
        tx.set_chain_id(chain_id);
        let signed_tx = tx.sign(*privkey);
        signed_tx
    }

    fn generate_request(tx: SignedTransaction) -> Request {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = Request::new();
        request.set_un_tx(tx.get_transaction_with_sig().clone());
        request.set_request_id(request_id);
        request
    }

    fn generate_msg_from_request(request: Request) -> Vec<u8> {
        let msg: Message = request.into();
        msg.try_into().unwrap()
    }

    fn generate_msg(tx: SignedTransaction) -> Vec<u8> {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = Request::new();
        request.set_un_tx(tx.get_transaction_with_sig().clone());
        request.set_request_id(request_id);
        let msg: Message = request.into();
        msg.try_into().unwrap()
    }

    fn generate_blk_msg(tx: SignedTransaction) -> Vec<u8> {
        //create verify message
        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(
            tx.get_transaction_with_sig()
                .get_transaction()
                .get_valid_until_block(),
        );
        let signature = tx.get_transaction_with_sig().get_signature().to_vec();
        req.set_signature(signature);
        let bytes: Vec<u8> = tx.get_transaction_with_sig()
            .get_transaction()
            .try_into()
            .unwrap();
        let hash = bytes.crypt_hash().to_vec();
        req.set_hash(hash);
        req.set_tx_hash(tx.get_tx_hash().to_vec());
        req.set_chain_id(
            tx.get_transaction_with_sig()
                .get_transaction()
                .get_chain_id(),
        );

        let mut blkreq = VerifyBlockReq::new();
        blkreq.set_id(BLOCK_REQUEST_ID);
        blkreq.set_reqs(RepeatedField::from_slice(&[req]));

        let msg: Message = blkreq.into();
        msg.try_into().unwrap()
    }

    fn generate_blk_msg_with_fake_signature(tx: SignedTransaction, pubkey: PubKey) -> Vec<u8> {
        //create verify message
        let mut req = VerifyTxReq::new();
        req.set_valid_until_block(
            tx.get_transaction_with_sig()
                .get_transaction()
                .get_valid_until_block(),
        );
        let signature = tx.get_transaction_with_sig().get_signature().to_vec();
        req.set_signature(signature[0..16].to_vec());
        let bytes: Vec<u8> = tx.get_transaction_with_sig()
            .get_transaction()
            .try_into()
            .unwrap();
        let hash = bytes.crypt_hash().to_vec();
        req.set_hash(hash);
        req.set_tx_hash(tx.get_tx_hash().to_vec());
        req.set_signer(pubkey.to_vec());

        let mut blkreq = VerifyBlockReq::new();
        blkreq.set_id(BLOCK_REQUEST_ID);
        blkreq.set_reqs(RepeatedField::from_slice(&[req]));

        let msg: Message = blkreq.into();
        msg.try_into().unwrap()
    }

    fn generate_sync_blk_hash_msg(height: u64) -> Vec<u8> {
        //prepare and send the block tx hashes to auth
        let mut block_tx_hashes = BlockTxHashes::new();
        block_tx_hashes.set_height(height);
        let mut tx_hashes_in_u8 = Vec::new();

        let u: U256 = 0x123456789abcdef0u64.into();
        let tx_hash_in_h256 = H256::from(u);
        tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());

        let u: U256 = 0x1122334455667788u64.into();
        let tx_hash_in_h256 = H256::from(u);
        tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());

        block_tx_hashes.set_tx_hashes(RepeatedField::from_slice(&tx_hashes_in_u8[..]));

        let msg: Message = block_tx_hashes.into();
        msg.try_into().unwrap()
    }

    fn generate_verifier(chain_id: u32) -> Arc<RwLock<Verifier>> {
        let v = Arc::new(RwLock::new(Verifier::new()));
        v.write().set_chain_id(chain_id);
        v
    }

    #[test]
    fn verify_sync_block_hash() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = generate_verifier(7);

        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        let height = 0;
        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal,
            &pool,
            tx_verify_num_per_thread,
            v.clone(),
            &req_sender,
            &tx_pub,
            c,
            cache,
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );
        assert_eq!(rx_pub.try_recv().is_err(), true);

        let u: U256 = 0x123456789abcdef0u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x1122334455667788u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x3344.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), false);
        assert_eq!(v.read().is_inited(), true);
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_txs_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_txs_receiver
        );
    }

    #[test]
    fn verify_request_sync_block_hash() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = generate_verifier(7);
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        let height = 1;
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;

        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal,
            &pool,
            tx_verify_num_per_thread,
            v.clone(),
            &req_sender,
            &tx_pub,
            c,
            cache,
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let u: U256 = 0x123456789abcdef0u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x1122334455667788u64.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);

        let u: U256 = 0x3344.into();
        let tx_hash_in_h256 = H256::from(u);
        assert_eq!(v.read().check_hash_exist(&tx_hash_in_h256), true);
        assert_eq!(v.read().is_inited(), false);

        let (key, sync_request) = rx_pub.recv().unwrap();
        assert_eq!(
            routing_key!(Auth >> BlockTxHashesReq),
            RoutingKey::from(&key)
        );
        let mut msg = Message::try_from(&sync_request).unwrap();
        match msg.take_block_tx_hashes_req() {
            Some(req) => {
                assert_eq!(req.get_height(), 0);
            }
            None => panic!("test failed"),
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_txs_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_txs_receiver
        );
    }

    #[test]
    fn verify_single_tx_request_dispatch_success() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = Arc::new(RwLock::new(Verifier::new()));
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_tx_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        // we set chain_id is 7 without any preference
        let tx = generate_tx(vec![1], 99, privkey, 7);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        let req = generate_request(tx);
        let request_id = req.get_request_id().to_vec();
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        handle_remote_msg(
            routing_key!(Jsonrpc >> RequestNewTxBatch).into(),
            generate_msg_from_request(req),
            on_proposal,
            &pool,
            tx_verify_num_per_thread,
            v.clone(),
            &req_sender,
            &tx_pub,
            c,
            cache,
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );
        let verify_req_info: VerifyRequestResponseInfo = req_receiver.recv().unwrap();
        assert_eq!(verify_req_info.verify_type, VerifyType::SingleVerify);
        if let VerifyRequestID::SingleVerifyRequestID(single_request_id) = verify_req_info.request_id {
            assert_eq!(request_id, single_request_id);
        }

        assert!(RoutingKey::from(&verify_req_info.key).is_sub_module(SubModules::Jsonrpc));
        if let VerifyRequestResponse::AuthRequest(req) = verify_req_info.req_resp {
            assert_eq!(req.get_tx_hash().to_vec().clone(), tx_hash);
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_tx_receiver
        );
    }

    #[test]
    fn verify_block_tx_request_dispatch_success() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        //verify tx
        let v = generate_verifier(7);
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let c = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let verify_cache = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache));
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            v.clone(),
            &req_sender,
            &tx_pub,
            c.clone(),
            cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        // we set chain_id is 7 without any preference
        let tx = generate_tx(vec![1], 99, privkey, 7);
        handle_remote_msg(
            routing_key!(Consensus >> VerifyBlockReq).into(),
            generate_blk_msg(tx),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            v.clone(),
            &req_sender,
            &tx_pub,
            c.clone(),
            cache,
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let block_verify_status = c.read();
        assert_eq!(
            block_verify_status.block_verify_result,
            VerifyResult::VerifyOngoing
        );
        assert_eq!(block_verify_status.verify_success_cnt_required, 1);
        assert_eq!(block_verify_status.verify_success_cnt_capture, 0);
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_txs_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_txs_receiver
        );
    }

    #[test]
    fn handle_verification_result_single_tx() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let verify_cache_hashmap = HashMap::new();
        let verify_cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let verifier = generate_verifier(7);
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        // we set chain_id is 7 without any preference
        let tx = generate_tx(vec![1], 99, privkey, 7);
        let tx_hash = tx.get_tx_hash().to_vec().clone();
        handle_remote_msg(
            routing_key!(Jsonrpc >> RequestNewTxBatch).into(),
            generate_msg(tx),
            on_proposal,
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let verify_req_info: VerifyRequestResponseInfo = req_receiver.recv().unwrap();
        let mut req_grp: Vec<VerifyRequestResponseInfo> = Vec::new();
        req_grp.push(verify_req_info);
        verify_tx_group_service(req_grp, verifier, verify_cache, resp_sender);

        handle_verification_result(
            &resp_receiver,
            &tx_pub,
            block_verify_status,
            &pool_tx_sender,
        );
        let (_, _, resp_msg, _) = pool_tx_receiver.try_recv().unwrap();
        let ok_result = format!("{:?}", Ret::OK);
        assert_eq!(resp_msg.status, ok_result);
        assert_eq!(tx_hash, resp_msg.hash.to_vec());
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_tx_receiver
        );
    }

    #[test]
    fn handle_verification_result_block_tx() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let verifier = generate_verifier(7);
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        // we set chain_id is 7 without any preference
        let tx = generate_tx(vec![1], 99, privkey, 7);
        handle_remote_msg(
            routing_key!(Consensus >> VerifyBlockReq).into(),
            generate_blk_msg(tx),
            on_proposal,
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            cache,
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );
        handle_verification_result(
            &resp_receiver,
            &tx_pub,
            block_verify_status,
            &pool_tx_sender,
        );

        let (_, resp_msg) = rx_pub.recv().unwrap();
        let mut msg = Message::try_from(&resp_msg).unwrap();
        match msg.take_verify_block_resp() {
            Some(resp) => {
                assert_eq!(resp.get_ret(), Ret::OK);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => panic!("test failed"),
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}, pool_txs_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_tx_receiver, pool_txs_receiver
        );
    }

    #[test]
    fn block_verificaton_failed() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, req_receiver) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let verifier = generate_verifier(7);
        let (pool_txs_sender, pool_txs_receiver) = channel();
        let (pool_tx_sender, pool_tx_receiver) = channel();
        let verify_cache_hashmap = HashMap::new();
        let cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let height = 0;
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let pubkey = keypair.pubkey().clone();
        // we set chain_id is 7 without any preference
        let tx = generate_tx(vec![1], 99, privkey, 7);

        handle_remote_msg(
            routing_key!(Consensus >> VerifyBlockReq).into(),
            generate_blk_msg_with_fake_signature(tx, pubkey),
            on_proposal,
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            cache,
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool,
        );

        handle_verification_result(
            &resp_receiver,
            &tx_pub,
            block_verify_status,
            &pool_tx_sender,
        );

        let (_, resp_msg) = rx_pub.recv().unwrap();
        let mut msg = Message::try_from(&resp_msg).unwrap();
        match msg.take_verify_block_resp() {
            Some(resp) => {
                assert_eq!(resp.get_ret(), Ret::BadSig);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => panic!("test failed"),
        }
        // keep the receiver live long enough
        thread::sleep(Duration::new(0, 9000000));
        println!(
            "rx_pub {:?}, req_receiver {:?}, resp_receiver {:?}, pool_tx_receiver {:?}, pool_txs_receiver {:?}",
            rx_pub, req_receiver, resp_receiver, pool_tx_receiver, pool_txs_receiver
        );
    }

    #[test]
    fn get_tx_verificaton_from_cache() {
        let (tx_pub, rx_pub) = channel();
        let (req_sender, _) = channel();
        let (resp_sender, resp_receiver) = channel();
        let block_verify_status = BlockVerifyStatus {
            request_id: 0,
            block_verify_result: VerifyResult::VerifyNotBegin,
            verify_success_cnt_required: 0,
            verify_success_cnt_capture: 0,
            cache_hit: 0,
        };
        let block_verify_status = Arc::new(RwLock::new(block_verify_status));
        let verify_cache_hashmap = HashMap::new();
        let verify_cache = Arc::new(RwLock::new(verify_cache_hashmap));
        let verifier = generate_verifier(7);
        let (pool_txs_sender, _) = channel();
        let (pool_tx_sender, _) = channel();
        let pool = threadpool::ThreadPool::new(10);
        let tx_verify_num_per_thread = 30;
        let on_proposal = Arc::new(AtomicBool::new(false));
        let clear_txs_pool = Arc::new(AtomicBool::new(false));

        let height = 0;
        handle_remote_msg(
            routing_key!(Chain >> BlockTxHashes).into(),
            generate_sync_blk_hash_msg(height),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            verify_cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        // we set chain_id is 7 without any preference
        let tx = generate_tx(vec![1], 99, privkey, 7);

        handle_remote_msg(
            routing_key!(Consensus >> VerifyBlockReq).into(),
            generate_blk_msg(tx.clone()),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            verify_cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );
        handle_verification_result(
            &resp_receiver,
            &tx_pub,
            block_verify_status.clone(),
            &pool_tx_sender,
        );
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let mut msg = Message::try_from(&resp_msg).unwrap();
        match msg.take_verify_block_resp() {
            Some(resp) => {
                assert_eq!(resp.get_ret(), Ret::OK);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => panic!("test failed"),
        }

        thread::sleep(Duration::new(0, 9000000));
        // Begin to construct the same tx's verification request
        handle_remote_msg(
            routing_key!(Consensus >> VerifyBlockReq).into(),
            generate_blk_msg(tx.clone()),
            on_proposal.clone(),
            &pool,
            tx_verify_num_per_thread,
            verifier.clone(),
            &req_sender,
            &tx_pub,
            block_verify_status.clone(),
            verify_cache.clone(),
            &pool_txs_sender,
            &resp_sender,
            clear_txs_pool.clone(),
        );
        let (_, resp_msg) = rx_pub.recv().unwrap();
        let mut msg = Message::try_from(&resp_msg).unwrap();
        match msg.take_verify_block_resp() {
            Some(resp) => {
                assert_eq!(resp.get_ret(), Ret::OK);
                assert_eq!(resp.get_id(), BLOCK_REQUEST_ID);
            }
            _ => panic!("test failed"),
        }
    }

}
