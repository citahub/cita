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
use libproto::communication::*;
use protobuf::core::parse_from_bytes;
use std::sync::mpsc::Sender;
use verify::Verifier;
use protobuf::{Message, RepeatedField};
use util::H256;

pub fn handle_msg(payload: Vec<u8>, tx_pub: &Sender<(String, Vec<u8>)>, verifier: &mut Verifier) {

    if let Ok(msg) = parse_from_bytes::<communication::Message>(payload.as_ref()) {
        let t = msg.get_field_type();
        let cid = msg.get_cmd_id();
        if cid == cmd_id(submodules::CHAIN, topics::NEW_STATUS) && t == MsgType::STATUS {
            let (_, _, content) = parse_msg(payload.as_slice());
            match content {
                MsgClass::STATUS(status) => {
                    let height = status.get_height();
                    trace!("got height {:?}", height);
                    verifier.set_height(height);
                }
                MsgClass::VERIFYREQ(req) => {
                    trace!("get verify request {:?}", req);
                    let mut resps = Vec::new();
                    for req in req.get_reqs() {
                        let ret = verifier.check_hash_exist(&H256::from_slice(req.get_tx_hash()));
                        if ret {
                            let mut resp = VerifyRespMsg::new();
                            resp.set_ret(Ret::Dup);
                            resp.set_tx_hash(req.get_tx_hash().to_vec());
                            resps.push(resp);
                        } else {
                            let mut resp = VerifyRespMsg::new();
                            resp.set_ret(Ret::Ok);
                            resp.set_tx_hash(req.get_tx_hash().to_vec());
                            resps.push(resp);
                        }                        
                    }
                    let mut vresq = VerifyResp::new();
                    vresq.set_resps(RepeatedField::from_slice(&resps));
                    tx_pub.send(("auth.verify_resp".to_string(), vresq.write_to_bytes().unwrap())).unwrap();
                }
                _ => {}
            }
        }
    }

}
