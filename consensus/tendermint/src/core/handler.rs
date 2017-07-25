use threadpool::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use pubsub::Pub;
use super::Engine;
use libproto::*;

const ID_NEW_TX: u32 = (submodules::CONSENSUS << 16) + topics::NEW_TX as u32;
const ID_NEW_BLK: u32 = (submodules::CONSENSUS << 16) + topics::NEW_BLK as u32;
const ID_CONSENSUS_MSG: u32 = (submodules::CONSENSUS << 16) + topics::CONSENSUS_MSG as u32;
const ID_NEW_PROPOSAL: u32 = (submodules::CONSENSUS << 16) + topics::NEW_PROPOSAL as u32;


pub fn receive(pool: &ThreadPool, tx: &Sender<(u32, u32, u32, MsgClass)>, id: u32, msg: Vec<u8>) {
    let tx = tx.clone();
    pool.execute(move || {
                     let (cmd_id, origin, content) = parse_msg(msg.as_slice());
                     tx.send((id, cmd_id, origin, content)).unwrap();
                 });
}

pub fn process(engine: Arc<Engine>, rx: &Receiver<(u32, u32, u32, MsgClass)>, _pub: &mut Pub) {
    let res = rx.recv().unwrap();
    let (id, cmd_id, _origin, content_ext) = res;
    let from_broadcast = id == submodules::NET;

    if from_broadcast {
        match cmd_id {
            ID_NEW_TX => {
                if let MsgClass::TX(tx) = content_ext {
                    engine.receive_new_transaction(&tx, _pub, _origin, from_broadcast);
                }
            }
            ID_NEW_BLK => {
                // unused
                trace!("net receive_new_block");
                if let MsgClass::BLOCK(block) = content_ext {
                    engine.receive_new_block(&block, _pub);
                }
            }
            ID_CONSENSUS_MSG => {
                if let MsgClass::MSG(msg) = content_ext {
                    let _ = engine.handle_message(msg, _pub);
                }
            }
            ID_NEW_PROPOSAL => {
                if let MsgClass::MSG(msg) = content_ext {
                    trace!("receive proposal");
                    let ret = engine.handle_proposal(msg, _pub);
                    trace!("handle_proposal {:?}", ret);
                }
            }
            _ => {}
        }
    } else {
        match content_ext {
            MsgClass::TX(tx) => {
                engine.receive_new_transaction(&tx, _pub, _origin, from_broadcast);
            }
            MsgClass::STATUS(status) => {
                trace!("get new local status {:?}", status.height);
                engine.receive_new_status(status);
            }
            _ => {}
        }
    }
}