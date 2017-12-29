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
use libproto::*;
use libproto::blockchain::{AccountGasLimit, SignedTransaction, UnverifiedTransaction};
use protobuf::Message;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{ATOMIC_U64_INIT, AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;
use std::vec::*;
use threadpool::ThreadPool;
use util::{H256, Mutex, RwLock};
use verify::Verifier;

#[derive(Debug, Clone)]
pub enum VerifyRequestID {
    SingleVerifyRequestID(Vec<u8>),
    BlockVerifyRequestID(u64),
}

#[derive(Debug, Clone)]
pub enum VerifyRequestResponse {
    AuthRequest(VerifyTxReq),
    AuthResponse(VerifyTxResp),
}

#[derive(Debug, Clone)]
pub struct VerifyRequestResponseInfo {
    pub sub_module: u32,
    pub verify_type: VerifyType,
    pub request_id: VerifyRequestID,
    pub time_stamp: SystemTime,
    pub req_resp: VerifyRequestResponse,
    pub un_tx: Option<UnverifiedTransaction>,
}

#[derive(Debug, PartialEq)]
pub enum VerifyResult {
    VerifyNotBegin,
    VerifyOngoing,
    VerifyFailed,
    VerifySucceeded,
}

#[derive(Debug)]
pub struct BlockVerifyStatus {
    pub request_id: u64,
    pub block_verify_result: VerifyResult,
    pub verify_success_cnt_required: usize,
    pub verify_success_cnt_capture: usize,
    pub cache_hit: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VerifyType {
    SingleVerify,
    BlockVerify,
}

static mut MAX_HEIGHT: AtomicU64 = ATOMIC_U64_INIT;

fn verfiy_tx(req: &VerifyTxReq, verifier: &Verifier) -> VerifyTxResp {
    let mut resp = VerifyTxResp::new();
    resp.set_tx_hash(req.get_tx_hash().to_vec());

    if req.get_nonce().len() > 128 {
        resp.set_ret(Ret::InvalidNonce);
        return resp;
    }

    let tx_hash = H256::from_slice(req.get_tx_hash());
    let ret = verifier.check_hash_exist(&tx_hash);
    if ret {
        if verifier.is_inited() {
            resp.set_ret(Ret::Dup);
        } else {
            resp.set_ret(Ret::NotReady);
        }
        return resp;
    }
    let ret = verifier.verify_sig(req);
    if ret.is_err() {
        resp.set_ret(Ret::BadSig);
        return resp;
    }
    //check signer if req have
    let req_signer = req.get_signer();
    if !req_signer.is_empty() && req_signer != ret.unwrap().to_vec().as_slice() {
        resp.set_ret(Ret::BadSig);
        return resp;
    }
    resp.set_signer(ret.unwrap().to_vec());
    resp.set_ret(Ret::OK);
    trace!(
        "verfiy_tx's result:tx_hash={:?}, ret={:?}, signer={:?}",
        resp.get_tx_hash(),
        resp.get_ret(),
        resp.get_signer()
    );
    resp
}


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
            let response = verfiy_tx(&req, &verifier.read());
            cache.write().insert(tx_hash, response.clone());
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
    payload: Vec<u8>,
    on_proposal: Arc<AtomicBool>,
    threadpool: &Mutex<ThreadPool>,
    proposal_tx_verify_num_per_thread: usize,
    verifier: Arc<RwLock<Verifier>>,
    tx_req_single: &Sender<VerifyRequestResponseInfo>,
    tx_pub: &Sender<(String, Vec<u8>)>,
    block_verify_status: Arc<RwLock<BlockVerifyStatus>>,
    cache: Arc<RwLock<HashMap<H256, VerifyTxResp>>>,
    txs_sender: &Sender<(usize, HashSet<H256>, u64, AccountGasLimit)>,
    resp_sender: &Sender<VerifyRequestResponseInfo>,
) {
    let (cmdid, _origin, content) = parse_msg(payload.as_slice());
    let (submodule, _topic) = de_cmd_id(cmdid);
    match content {
        MsgClass::BLOCKTXHASHES(block_tx_hashes) => {
            let height = block_tx_hashes.get_height();
            info!("get block tx hashs for height {:?}", height);
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
                let mut flag = false;
                unsafe {
                    if height > MAX_HEIGHT.load(Ordering::SeqCst) {
                        MAX_HEIGHT.store(height, Ordering::SeqCst);
                        flag = true;
                    }
                }
                if flag {
                    info!(
                        "BLOCKTXHASHES come height {}, tx_hashes count is: {:?}",
                        height,
                        tx_hashes_in_h256.len()
                    );
                    let block_gas_limit = block_tx_hashes.get_block_gas_limit();
                    let account_gas_limit = block_tx_hashes.get_account_gas_limit().clone();
                    info!(
                        "Auth rich status block gas limit: {:?}, account gas limit {:?}",
                        block_gas_limit,
                        account_gas_limit
                    );

                    let _ = txs_sender.send((
                        height as usize,
                        tx_hashes_in_h256.clone(),
                        block_gas_limit,
                        account_gas_limit,
                    ));
                }
            }
            verifier
                .write()
                .update_hashes(height, tx_hashes_in_h256, tx_pub);
        }
        // TODO: Add ProposalVerifier { status, request_id, threadpool }, Status: On, Failed, Successed, Experied
        // TODO: Make most of the logic asynchronous
        // Verify Proposal from consensus
        MsgClass::VERIFYBLKREQ(blkreq) => {
            info!(
                "get block verify request with {:?} request",
                blkreq.get_reqs().len()
            );
            let tx_cnt = blkreq.get_reqs().len();
            let mut tx_need_verify = Vec::new();
            if tx_cnt > 0 {
                let request_id = blkreq.get_id();
                let new_block_verify_status = BlockVerifyStatus {
                    request_id: request_id,
                    block_verify_result: VerifyResult::VerifyOngoing,
                    verify_success_cnt_required: blkreq.get_reqs().len(),
                    verify_success_cnt_capture: 0,
                    cache_hit: 0,
                };

                info!(
                    "Coming new block verify request with request_id: {}, and the init block_verify_status: {:?}",
                    request_id,
                    new_block_verify_status
                );
                //add big brace here to release write lock as soon as poobible
                {
                    let mut block_verify_status_guard = block_verify_status.write();
                    if block_verify_status_guard.block_verify_result == VerifyResult::VerifyOngoing {
                        warn!(
                            "block verification request with request_id: {:?} \
                             has been expired, and the current info is: {:?}",
                            block_verify_status_guard.request_id,
                            *block_verify_status_guard
                        );
                    }
                    let block_verify_stamp = SystemTime::now();
                    *block_verify_status_guard = new_block_verify_status;
                    let now = SystemTime::now();
                    for req in blkreq.get_reqs() {
                        let verify_request_info = VerifyRequestResponseInfo {
                            sub_module: submodule,
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
                                    request_id,
                                    resp_ret
                                );
                                publish_block_verification_result(request_id, resp_ret, tx_pub);
                                break;
                            }
                            _ => {
                                let verify_request_info = VerifyRequestResponseInfo {
                                    sub_module: submodule,
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
                        let pool = threadpool.lock();
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
                    "Wrong block verification request with 0 tx for block verify request_id: {} from sub_module: {}",
                    blkreq.get_id(),
                    submodule
                );
            }
        }
        MsgClass::REQUEST(newtx_req) => {
            if newtx_req.has_batch_req() {
                let batch_new_tx = newtx_req.get_batch_req().get_new_tx_requests();
                let now = SystemTime::now();
                trace!(
                    "get batch new tx request from module:{:?} in system time :{:?}, and has got {} new tx ",
                    id_to_key(submodule),
                    now,
                    batch_new_tx.len()
                );

                for tx_req in batch_new_tx.iter() {
                    let verify_tx_req = tx_req.get_un_tx().tx_verify_req_msg();
                    let verify_request_info = VerifyRequestResponseInfo {
                        sub_module: submodule,
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
                    sub_module: submodule,
                    verify_type: VerifyType::SingleVerify,
                    request_id: VerifyRequestID::SingleVerifyRequestID(newtx_req.get_request_id().to_vec()),
                    time_stamp: now,
                    req_resp: VerifyRequestResponse::AuthRequest(verify_tx_req),
                    un_tx: Some(newtx_req.get_un_tx().clone()),
                };

                tx_req_single.send(verify_request_info).unwrap();
            }
        }
        _ => {}
    }
}

pub fn handle_verificaton_result(
    result_receiver: &Receiver<VerifyRequestResponseInfo>,
    tx_pub: &Sender<(String, Vec<u8>)>,
    block_verify_status: Arc<RwLock<BlockVerifyStatus>>,
    tx_sender: &Sender<(u32, Vec<u8>, TxResponse, SignedTransaction)>,
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
                                        verify_response_info.sub_module,
                                        request_id.clone(),
                                        tx_response,
                                        signed_tx.clone(),
                                    ));
                                    trace!("Send singed tx to txpool");
                                }
                                _ => {
                                    if verify_response_info.sub_module == submodules::JSON_RPC {
                                        let tx_response = TxResponse::new(tx_hash, result);

                                        let mut response = Response::new();
                                        response.set_request_id(request_id);
                                        response.set_code(ErrorCode::tx_auth_error());
                                        response.set_error_msg(tx_response.status);

                                        let msg = factory::create_msg(
                                            submodules::AUTH,
                                            topics::RESPONSE,
                                            communication::MsgType::RESPONSE,
                                            response.write_to_bytes().unwrap(),
                                        );
                                        trace!("response new tx {:?}", response);
                                        tx_pub
                                            .send(("auth.rpc".to_string(), msg.write_to_bytes().unwrap()))
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
                                        "Failed to do verify blk req for request_id: {}, ret: {:?}, from submodule: {}",
                                        request_id,
                                        result,
                                        verify_response_info.sub_module
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

    let msg = factory::create_msg(
        submodules::AUTH,
        topics::VERIFY_BLK_RESP,
        communication::MsgType::VERIFY_BLK_RESP,
        blkresp.write_to_bytes().unwrap(),
    );
    tx_pub
        .send((
            String::from("auth.verify_blk_res"),
            msg.write_to_bytes().unwrap(),
        ))
        .unwrap();
}
