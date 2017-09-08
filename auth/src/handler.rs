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

use libproto::*;
use protobuf::Message;
use std::sync::mpsc::Sender;
use std::vec::*;
use util::H256;
use verify::Verifier;
use cache::VerifyCache;


fn verfiy_tx(req: &VerifyTxReq, verifier: &Verifier) -> VerifyTxResp {
    let mut resp = VerifyTxResp::new();
    resp.set_tx_hash(req.get_tx_hash().to_vec());

    if !verifier.verify_valid_until_block(req.get_valid_until_block()) {
        resp.set_ret(Ret::OutOfTime);                            
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
    if req_signer.len() != 0 {
        if req_signer != ret.unwrap().to_vec().as_slice() {
            resp.set_ret(Ret::BadSig);                            
            return resp;
        }
    }
    resp.set_signer(ret.unwrap().to_vec());
    resp.set_ret(Ret::Ok);
    resp
}

fn get_key(submodule: u32, is_blk: bool) -> String {
    "verify".to_owned() + 
    if is_blk {
        "_blk_"
    } else {
        "_tx_"
    } +
    id_to_key(submodule)
}

pub fn handle_msg(payload: Vec<u8>, tx_pub: &Sender<(String, Vec<u8>)>, verifier: &mut Verifier, cache: &mut VerifyCache) {
    let (cmdid, _origin, content) = parse_msg(payload.as_slice());
    let (submodule, _topic) = de_cmd_id(cmdid);
    match content {
        MsgClass::BLOCKTXHASHES(block_tx_hashes) => {
            let height = block_tx_hashes.get_height();
            trace!("get block tx hashs for height {:?}", height);
            let tx_hashes = block_tx_hashes.get_tx_hashes();
            let mut tx_hashes_in_h256: Vec<H256> = Vec::new();
            for data in tx_hashes.iter() {
                tx_hashes_in_h256.push(H256::from_slice(data));
            }
            verifier.update_hashes(height, tx_hashes_in_h256, tx_pub);
        }
        MsgClass::VERIFYTXREQ(req) => {
            trace!("get verify request {:?}", req);
            let tx_hash = H256::from_slice(req.get_tx_hash());
            if let Some(resp) = cache.get(&tx_hash) {
                let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_TX_RESP, communication::MsgType::VERIFY_TX_RESP, resp.write_to_bytes().unwrap());
                tx_pub.send((get_key(submodule, false), msg.write_to_bytes().unwrap())).unwrap();
                return;
            }
            let resp = verfiy_tx(&req, verifier);
            let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_TX_RESP, communication::MsgType::VERIFY_TX_RESP, resp.write_to_bytes().unwrap());
            tx_pub.send((get_key(submodule, false), msg.write_to_bytes().unwrap())).unwrap();
            cache.insert(tx_hash, resp);
        }
        MsgClass::VERIFYBLKREQ(blkreq) => {
            let id = blkreq.get_id();
            let mut blkresp = VerifyBlockResp::new();
            blkresp.set_id(id);
            blkresp.set_ret(Ret::Ok);
            for req in blkreq.get_reqs() {
                let tx_hash = H256::from_slice(req.get_tx_hash());
                if let Some(resp) = cache.get(&tx_hash) {
                    if resp.get_ret() == Ret::Ok {
                        trace!("tx {:?} verify result: {:?}", tx_hash, resp);
                        continue;
                    }
                    blkresp.set_ret(Ret::Err);
                    break;
                }
                let resp = verfiy_tx(req, verifier);
                let ret = resp.get_ret();
                cache.insert(tx_hash, resp);
                if ret == Ret::Ok {
                    continue;
                }
                blkresp.set_ret(Ret::Err);
                break;
            }
            let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_BLK_RESP, communication::MsgType::VERIFY_BLK_RESP, blkresp.write_to_bytes().unwrap());
            trace!("receive verify blk req, id: {}, ret: {:?}, from: {}", id, blkresp.get_ret(), submodule);
            tx_pub.send((get_key(submodule, true), msg.write_to_bytes().unwrap())).unwrap();
        }
        _ => {}
    }
}
