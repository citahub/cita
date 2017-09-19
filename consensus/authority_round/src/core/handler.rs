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

use super::Engine;
use libproto::*;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver};
use threadpool::*;

pub fn receive(pool: &ThreadPool, tx: &Sender<(u32, u32, u32, MsgClass)>, id: u32, msg: Vec<u8>) {
    let tx = tx.clone();
    pool.execute(move || {
                     let (cmd_id, origin, content) = parse_msg(msg.as_slice());
                     tx.send((id, cmd_id, origin, content)).unwrap();
                 });
}

pub fn process(engine: Arc<Engine>, rx: &Receiver<(u32, u32, u32, MsgClass)>, tx_pub: Sender<(String, Vec<u8>)>) {
    let (id, cmd_id, _origin, content_ext) = rx.recv().unwrap();
    let from_broadcast = id == submodules::NET;
    if from_broadcast {
        trace!("from_broadcast cmd_id {:?}", cmd_id);
        if (cmd_id >> 16) == submodules::CONSENSUS {
            match content_ext {
                MsgClass::REQUEST(tx_req) => {
                    trace!("get new broadcast tx {:?}", tx_req);
                    engine.receive_new_transaction(&tx_req, tx_pub, _origin, from_broadcast);
                }
                MsgClass::BLOCK(block) => {
                    trace!("get new broadcast block {:?}", block);
                    engine.receive_new_block(&block, tx_pub);
                }
                _ => error!("match not exsit msg content!!!"),
            }
        }
    } else {
        match content_ext {
            MsgClass::REQUEST(tx_req) => {
                trace!("get new local tx {:?}", tx_req);
                engine.receive_new_transaction(&tx_req, tx_pub, _origin, from_broadcast);
            }
            MsgClass::STATUS(status) => {
                trace!("get new local status {:?}", status.height);
                engine.receive_new_status(status);
            }
            _ => {}
        }
    }
}

pub fn seal(engine: Arc<Engine>, tx_pub: Sender<(String, Vec<u8>)>) {
    trace!("new seal worker");
    engine.new_block(tx_pub);
}
