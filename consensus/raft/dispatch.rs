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

use libproto::*;
use libraft::{NotifyMessage, Command, decode};
use mio;
use std::sync::mpsc::Receiver;

pub fn dispatch(notifix: &mio::Sender<NotifyMessage>, rx: &Receiver<(u32, u32, MsgClass)>) {
    let (id, cmd_id, content_ext) = rx.recv().unwrap();
    match content_ext {
        MsgClass::REQUEST(req) => {}
        MsgClass::RESPONSE(rep) => {}
        MsgClass::HEADER(header) => {}
        MsgClass::BODY(body) => {}
        MsgClass::BLOCK(block) => {}
        MsgClass::TX(tx) => {}
        MsgClass::TXRESPONSE(content) => {}
        MsgClass::STATUS(status) => {
            if id == submodules::CHAIN {
                info!("received new status.");
                notifix.send(NotifyMessage::NewStatus(status.hash, status.height));
            }
        }
        MsgClass::MSG(content) => {
            match decode(&content) {
                Command::SpawnBlk(..) => {
                    info!("not expected");
                }
                Command::PoolSituation(height, hash, proof) => {
                    info!("receive pool situation");
                    notifix.send(NotifyMessage::NewStatus(hash.unwrap(), height));
                }
            }
        }
        MsgClass::VERIFYREQ(req) => {}
        MsgClass::VERIFYRESP(resp) => {}
        MsgClass::BLOCKTXHASHES(txhashes) => {}
    }
}
