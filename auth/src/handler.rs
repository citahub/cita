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
use verifier::Verifier;



pub fn handle_msg(payload: Vec<u8>, tx_pub: &Sender<(String, Vec<u8>)>, v: &Verifier) {
    let (_, _, content) = parse_msg(payload.as_slice());
    match content {
        MsgClass::STATUS(status) => {
            let height = status.get_height();
            info!("got height {:?}", height);
        }
        MsgClass::VERIFYREQ(req) => {
            trace!("got verify request {:?}", req);
            let req_msgs = req.get_reqs();
            let mut resps = Vec::new();
            for req in req_msgs {
                let mut resp = VerifyRespMsg::new();
                if v.verify_valid_until_block(req.get_valid_until_block()) {
                    resp.set_ret(Ret::Ok);
                } else {
                    resp.set_ret(Ret::OutOfTime);
                }

                match v.verify_sig(req) {
                    Ok(pubkey) => {
                        resp.set_signer(pubkey.to_vec());
                        resp.set_ret(Ret::Ok);
                        resps.push(resp)
                    }
                    Err(_) => {
                        resp.set_ret(Ret::BadSig);
                        resps.push(resp)
                    }
                }
                let mut vresp = VerifyResp::new();
                vresp.set_resps(RepeatedField::from_slice(&resps));

                let msg = factory::create_msg(submodules::AUTH, topics::VERIFY_RESP, communication::MsgType::VERIFY_RESP, vresp.write_to_bytes().unwrap());
                tx_pub.send(("auth.verify_resp".to_string(), msg.write_to_bytes().unwrap())).unwrap();
            }

        }
        _ => {}
    }



}
