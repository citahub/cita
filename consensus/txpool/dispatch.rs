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

#![allow(unused_variables)]

use candidate_pool::CandidatePool;
use cmd::{Command, decode};
use libproto;
use libproto::*;
use pubsub::Pub;
use std::sync::mpsc::{Sender, Receiver};
use threadpool::*;

pub fn extract(pool: &ThreadPool, tx: &Sender<(u32, u32, u32, MsgClass)>, id: u32, msg: Vec<u8>) {
    let tx = tx.clone();
    pool.execute(move || {
                     let (cmd_id, origin, content) = parse_msg(msg.as_slice());
                     tx.send((id, cmd_id, origin, content)).unwrap();
                 });
}

pub fn wait(rx: &Receiver<(u32, u32, u32, MsgClass)>) -> (u64, Vec<u8>) {
    info!("waiting chain's new status.");
    loop {
        let (id, cmd_id, _origin, content_ext) = rx.recv().unwrap();
        if let MsgClass::STATUS(status) = content_ext {
            if id == submodules::CHAIN {
                return (status.height, status.hash);
            }
        }
    }
}

pub fn dispatch(candidate_pool: &mut CandidatePool, _pub: &mut Pub, rx: &Receiver<(u32, u32, u32, MsgClass)>) {
    let (id, cmd_id, _origin, content_ext) = rx.recv().unwrap();
    match content_ext {
        MsgClass::REQUEST(req) => {}
        MsgClass::RESPONSE(rep) => {}
        MsgClass::HEADER(header) => {}
        MsgClass::BODY(body) => {}
        MsgClass::BLOCK(block) => {
            if cmd_id == libproto::cmd_id(submodules::CONSENSUS, topics::NEW_BLK) {
                if block.get_header().get_height() < candidate_pool.get_height() {}
            }
        }
        MsgClass::TX(tx) => {
            if id == submodules::JSON_RPC {
                candidate_pool.add_tx(&tx, _pub, false);
            } else {
                candidate_pool.add_tx(&tx, _pub, true);
            }
        }
        MsgClass::TXRESPONSE(content) => {}
        MsgClass::STATUS(status) => {
            info!("received chain status:({:?},{:?})", status.height, status.hash);
        }
        MsgClass::MSG(content) => {
            if id == submodules::CONSENSUS_CMD {
                //to do: consensus cmd.
                match decode(&content) {
                    Command::SpawnBlk(height) => {
                        if candidate_pool.meet_conditions(height) {
                            info!("recieved command spawn new blk.");
                            let blk = candidate_pool.spawn_new_blk(height);
                            candidate_pool.pub_block(&blk, _pub);
                            candidate_pool.reflect_situation(_pub);
                        } else {
                            warn!("tx_pool's height:{:?}, received from consensus's height:{:?}", candidate_pool.get_height(), height);
                        }
                    }
                    Command::PoolSituation(_, _, _) => {
                        info!("not expected.");
                    }
                }
            }
        }
    }
}
