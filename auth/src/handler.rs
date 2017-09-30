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

use cache::{VerifyCache, VerifyBlockCache, VerifyResult, BlockVerifyStatus, BlockVerifyId};
use libproto::*;
use libproto::blockchain::SignedTransaction;
use protobuf::Message;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver};
use std::time::SystemTime;
use std::vec::*;
use util::{H256, RwLock};
use verify::Verifier;

#[derive(Debug, PartialEq, Clone)]
pub enum VerifyType {
    SingleVerify,
    BlockVerify,
}

#[derive(Debug, Clone)]
pub struct VerifyReqInfo {
    pub req: VerifyTxReq,
    pub info: (VerifyType, u64, u32, SystemTime, Origin),
}


fn verfiy_tx(req: &VerifyTxReq, verifier: &Verifier) -> VerifyTxResp {
    let mut resp = VerifyTxResp::new();
    resp.set_tx_hash(req.get_tx_hash().to_vec());

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
    if req_signer.len() != 0 {
        if req_signer != ret.unwrap().to_vec().as_slice() {
            resp.set_ret(Ret::BadSig);
            return resp;
        }
    }
    resp.set_signer(ret.unwrap().to_vec());
    resp.set_ret(Ret::Ok);
    trace!("verfiy_tx's result:tx_hash={:?}, ret={:?}, signer={:?}", resp.get_tx_hash(), resp.get_ret(), resp.get_signer());
    resp
}

pub fn verify_tx_group_service(mut req_grp: Vec<VerifyReqInfo>, verifier: Arc<RwLock<Verifier>>, cache: Arc<RwLock<VerifyCache>>, resp_sender: Sender<(VerifyType, u64, VerifyTxResp, u32, SystemTime, Origin)>) {
    let now = SystemTime::now();
    let len = req_grp.len();
    loop {
        if let Some(req_info) = req_grp.pop() {
            let req = req_info.req;
            let tx_hash = H256::from_slice(req.get_tx_hash());
            let response = verfiy_tx(&req, &verifier.read());
            cache.write().insert(tx_hash, response.clone());
            let (verify_type, id, sub_module, now, origin) = req_info.info;
            resp_sender.send((verify_type, id, response, sub_module, now, origin)).unwrap();
        } else {
            break;
        }
    }

    trace!("verify_tx_group_service Time cost {} ns for {} req ...", now.elapsed().unwrap().subsec_nanos(), len);
}

pub fn check_verify_request_preprocess(req_info: VerifyReqInfo, verifier: Arc<RwLock<Verifier>>, cache: Arc<RwLock<VerifyCache>>, resp_sender: Sender<(VerifyType, u64, VerifyTxResp, u32, SystemTime, Origin)>) -> bool {
    let req = req_info.req;
    let tx_hash = H256::from_slice(req.get_tx_hash());
    let mut final_response = VerifyTxResp::new();
    let mut processed = false;

    if !verifier.read().verify_valid_until_block(req.get_valid_until_block()) {
        let mut response = VerifyTxResp::new();
        response.set_tx_hash(req.get_tx_hash().to_vec());
        response.set_ret(Ret::OutOfTime);
        processed = true;
        final_response = response;
    } else {
        if let Some(resp) = get_resp_from_cache(&tx_hash, cache.clone()) {
            processed = true;
            final_response = resp;
        }
    }

    if true == processed {
        let (verify_type, id, sub_module, now, origin) = req_info.info;
        resp_sender.send((verify_type, id, final_response, sub_module, now, origin)).unwrap();
    }
    processed
}

fn get_resp_from_cache(tx_hash: &H256, cache: Arc<RwLock<VerifyCache>>) -> Option<VerifyTxResp> {
    if let Some(resp) = cache.read().get(tx_hash) { Some(resp.clone()) } else { None }
}

fn get_key(submodule: u32, is_blk: bool) -> String {
    "verify".to_owned() + if is_blk { "_blk_" } else { "_tx_" } + id_to_key(submodule)
}

pub fn handle_remote_msg(payload: Vec<u8>, verifier: Arc<RwLock<Verifier>>, tx_req_block: Sender<(VerifyType, u64, VerifyTxReq, u32, SystemTime, Origin)>, tx_req_single: Sender<(VerifyType, u64, VerifyTxReq, u32, SystemTime, Origin)>, tx_pub: Sender<(String, Vec<u8>)>, block_cache: Arc<RwLock<VerifyBlockCache>>, cache: Arc<RwLock<VerifyCache>>, batch_new_tx_pool: Arc<Mutex<HashMap<H256, (u32, Request)>>>, txs_sender: Sender<(usize, Vec<H256>)>) {
    let (cmdid, origin, content) = parse_msg(payload.as_slice());
    let (submodule, _topic) = de_cmd_id(cmdid);
    match content {
        MsgClass::BLOCKTXHASHES(block_tx_hashes) => {
            let height = block_tx_hashes.get_height();
            trace!("get block tx hashs for height {:?}", height);
            let tx_hashes = block_tx_hashes.get_tx_hashes();
            let mut tx_hashes_in_h256: HashSet<H256> = HashSet::new();
            let mut tx_hashes_in_h256_vec: Vec<H256> = Vec::new();
            let mut cache_guard = cache.write();
            for data in tx_hashes.iter() {
                let hash = H256::from_slice(data);
                cache_guard.remove(&hash);
                tx_hashes_in_h256.insert(hash);
                tx_hashes_in_h256_vec.push(hash);
            }
            trace!("BLOCKTXHASHES come height {}, tx_hashs {:?}", height, tx_hashes_in_h256_vec.len());
            let res = txs_sender.send((height as usize, tx_hashes_in_h256_vec));
            trace!("BLOCKTXHASHES  txs_sender res is {:?}", res);
            verifier.write().update_hashes(height, tx_hashes_in_h256, &tx_pub);
        }
        MsgClass::VERIFYBLKREQ(blkreq) => {
            trace!("get block verify request with {:?} request", blkreq.get_reqs().len());
            let tx_cnt = blkreq.get_reqs().len();
            if tx_cnt > 0 {
                let block_verify_status = BlockVerifyStatus {
                    block_verify_result: VerifyResult::VerifyOngoing,
                    verify_success_cnt_required: blkreq.get_reqs().len(),
                    verify_success_cnt_capture: 0,
                };
                let id = blkreq.get_id();
                trace!("block verify request id: {}, and the init block_verify_status: {:?}", id, block_verify_status);
                let request_id = BlockVerifyId {
                    request_id: id,
                    sub_module: submodule,
                };
                block_cache.write().insert(request_id, block_verify_status);
                for req in blkreq.get_reqs() {
                    let now = SystemTime::now();
                    tx_req_block.send((VerifyType::BlockVerify, id, req.clone(), submodule, now, origin)).unwrap();
                }
            } else {
                error!("Wrong block verification request with 0 tx for block verify request id: {} from sub_module: {}", blkreq.get_id(), submodule);
            }
        }
        MsgClass::REQUEST(newtx_req) => {
            if true == newtx_req.has_batch_req() {
                let batch_new_tx = newtx_req.get_batch_req().get_new_tx_requests();
                let now = SystemTime::now();
                trace!("get batch new tx request from jsonrpc in system time :{:?}, and has got {} new tx ", now, batch_new_tx.len());

                let mut txs = batch_new_tx_pool.lock().unwrap();
                for tx_req in batch_new_tx.iter() {
                    let verify_tx_req = tx_verify_req_msg(tx_req.get_un_tx());
                    let hash: H256 = verify_tx_req.get_tx_hash().into();
                    txs.insert(hash, (submodule, tx_req.clone()));

                    let verify_tx_req = tx_verify_req_msg(tx_req.get_un_tx());
                    tx_req_single.send((VerifyType::SingleVerify, 0, verify_tx_req, submodule, now, origin)).unwrap();
                }
            } else if true == newtx_req.has_un_tx() {
                let now = SystemTime::now();
                trace!("get single new tx request from peer node with system time :{:?}", now);
                let verify_tx_req = tx_verify_req_msg(newtx_req.get_un_tx());
                let mut txs = batch_new_tx_pool.lock().unwrap();
                let hash = verify_tx_req.get_tx_hash().into();
                txs.insert(hash, (submodule, newtx_req.clone()));

                tx_req_single.send((VerifyType::SingleVerify, 0, verify_tx_req, submodule, now, origin)).unwrap();
            }

        }
        _ => {}

    }
    //trace!("single queue_req queues {} reqs, block queue_req queues {} reqs", single_req_queue.lock().unwrap().len(), block_req_queue.lock().unwrap().len());
}

pub fn handle_verificaton_result(result_receiver: &Receiver<(VerifyType, u64, VerifyTxResp, u32, SystemTime, Origin)>, tx_pub: &Sender<(String, Vec<u8>)>, block_cache: Arc<RwLock<VerifyBlockCache>>, batch_new_tx_pool: Arc<Mutex<HashMap<H256, (u32, Request)>>>, tx_sender: Sender<(u32, Vec<u8>, TxResponse, SignedTransaction, Origin)>) {
    let (verify_type, id, resp, sub_module, now, origin) = result_receiver.recv().unwrap();
    match verify_type {
        VerifyType::SingleVerify => {
            trace!("SingleVerify Time cost {} ns for tx hash: {:?}", now.elapsed().unwrap().subsec_nanos(), resp.get_tx_hash());

            let tx_hash: H256 = resp.get_tx_hash().into();
            let unverified_tx = {
                let mut txs = batch_new_tx_pool.lock().unwrap();
                txs.remove(&tx_hash)
            };
            trace!("receive verify resp, hash: {:?}, ret: {:?}", tx_hash, resp.get_ret());

            unverified_tx.map(|(sub_module_id, mut req)| {

                let request_id = req.get_request_id().to_vec();
                let result = format!("{:?}", resp.get_ret());
                match resp.get_ret() {
                    Ret::Ok => {
                        let mut signed_tx = SignedTransaction::new();
                        signed_tx.set_transaction_with_sig(req.take_un_tx());
                        signed_tx.set_signer(resp.get_signer().to_vec());
                        signed_tx.set_tx_hash(tx_hash.to_vec());
                        //add to the newtx pool and broadcast to other nodes if not received from network
                        //.....................
                        //.....................
                        let tx_response = TxResponse::new(tx_hash, result.clone());
                        let _ = tx_sender.send((sub_module_id, request_id.clone(), tx_response, signed_tx.clone(), origin)).unwrap();
                        trace!("Send singed tx to txpool");
                    }
                    _ => {
                        if sub_module_id == submodules::JSON_RPC {
                            let tx_response = TxResponse::new(tx_hash, result);

                            let mut response = Response::new();
                            response.set_request_id(request_id);
                            response.set_code(submodules::AUTH as i64);
                            response.set_error_msg(format!("{:?}", tx_response));

                            let msg = factory::create_msg(submodules::AUTH, topics::RESPONSE, communication::MsgType::RESPONSE, response.write_to_bytes().unwrap());
                            trace!("response new tx {:?}", response);
                            tx_pub.send(("auth.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                        }
                    }
                }
            });
        }
        VerifyType::BlockVerify => {
            let request_id = BlockVerifyId {
                request_id: id,
                sub_module: sub_module,
            };
            if Ret::Ok != resp.get_ret() {
                if let Some(block_verify_status) = block_cache.write().get_mut(&request_id) {
                    block_verify_status.block_verify_result = VerifyResult::VerifyFailed;

                    let mut blkresp = VerifyBlockResp::new();
                    blkresp.set_id(id);
                    blkresp.set_ret(resp.get_ret());

                    let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_BLK_RESP, communication::MsgType::VERIFY_BLK_RESP, blkresp.write_to_bytes().unwrap());
                    trace!("Failed to do verify blk req for request id: {}, ret: {:?}, from submodule: {}", id, blkresp.get_ret(), sub_module);
                    tx_pub.send((get_key(sub_module, true), msg.write_to_bytes().unwrap())).unwrap();
                } else {
                    error!("Failed to get block verify status for request id: {:?} from submodule: {}", id, sub_module);
                }
            } else {
                if let Some(block_verify_status) = block_cache.write().get_mut(&request_id) {
                    trace!("BlockVerify Time cost {} ns for tx hash: {:?}", now.elapsed().unwrap().subsec_nanos(), resp.get_tx_hash());
                    block_verify_status.verify_success_cnt_capture += 1;
                    if block_verify_status.verify_success_cnt_capture == block_verify_status.verify_success_cnt_required {
                        let mut blkresp = VerifyBlockResp::new();
                        blkresp.set_id(id);
                        blkresp.set_ret(resp.get_ret());

                        let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_BLK_RESP, communication::MsgType::VERIFY_BLK_RESP, blkresp.write_to_bytes().unwrap());
                        trace!("Succeed to do verify blk req for request id: {}, ret: {:?}, from submodule: {}", id, blkresp.get_ret(), sub_module);
                        tx_pub.send((get_key(sub_module, true), msg.write_to_bytes().unwrap())).unwrap();
                    }
                } else {
                    error!("Failed to get block verify status for request id: {:?} from submodule: {}", id, sub_module);
                }
            }

        }
    }
}
