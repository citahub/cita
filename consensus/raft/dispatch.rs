#![allow(unused_variables)]

use std::sync::mpsc::Receiver;
use libproto::*;
use mio;
use libraft::{NotifyMessage, Command, decode};

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
                info!("receved new status.");
                notifix.send(NotifyMessage::NewStatus(status.hash, status.height));
            }
        }
        MsgClass::MSG(content) => {
            match decode(&content) {
                Command::SpawnBlk(_) => {
                    info!("not expected");
                }
                Command::PoolSituation(height, hash, proof) => {
                    notifix.send(NotifyMessage::NewStatus(hash.unwrap(), height));
                }
            }
        }
    }
}
