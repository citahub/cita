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
use protobuf::{Message, RepeatedField};
use std::sync::mpsc::Sender;
use std::vec::*;
use util::H256;
use verify::Verifier;

pub fn handle_msg(payload: Vec<u8>, tx_pub: &Sender<(String, Vec<u8>)>, verifier: &mut Verifier) {
    let (_cmdid, _origin, content) = parse_msg(payload.as_slice());
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
        MsgClass::VERIFYREQ(req) => {
            trace!("get verify request {:?}", req);
            let mut resps = Vec::new();
            for req in req.get_reqs() {
                let ret = verifier.check_hash_exist(&H256::from_slice(req.get_tx_hash()));
                if ret {
                    let mut resp = VerifyRespMsg::new();
                    if verifier.is_inited() {
                        resp.set_ret(Ret::Dup);
                    } else {
                        resp.set_ret(Ret::NotReady);
                    }                            
                    resp.set_tx_hash(req.get_tx_hash().to_vec());
                    resps.push(resp);
                    continue;
                } 
                let ret = verifier.verify_sig(req);
                if ret.is_err() {
                    let mut resp = VerifyRespMsg::new();
                    resp.set_ret(Ret::BadSig);                            
                    resp.set_tx_hash(req.get_tx_hash().to_vec());
                    resps.push(resp);
                    continue;
                }
                if !verifier.verify_valid_until_block(req.get_valid_until_block()) {
                    let mut resp = VerifyRespMsg::new();
                    resp.set_ret(Ret::OutOfTime);                            
                    resp.set_tx_hash(req.get_tx_hash().to_vec());
                    resps.push(resp);
                    continue;
                }
                //ok
                {
                    let mut resp = VerifyRespMsg::new();
                    resp.set_ret(Ret::Ok);
                    resp.set_tx_hash(req.get_tx_hash().to_vec());
                    resp.set_signer(ret.unwrap().to_vec());
                    resps.push(resp);
                }
            }
            let mut vresq = VerifyResp::new();
            vresq.set_resps(RepeatedField::from_slice(&resps));
            let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_RESP, communication::MsgType::VERIFY_RESP, vresq.write_to_bytes().unwrap());
            tx_pub.send(("auth.verify_resp".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        }
        _ => {}
    }
}
