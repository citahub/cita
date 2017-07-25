use threadpool::*;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use libproto::*;
use pubsub::Pub;
use super::Engine;

pub fn receive(pool: &ThreadPool, tx: &Sender<(u32, u32, u32, MsgClass)>, id: u32, msg: Vec<u8>) {
    let tx = tx.clone();
    pool.execute(move || {
                     let (cmd_id, origin, content) = parse_msg(msg.as_slice());
                     tx.send((id, cmd_id, origin, content)).unwrap();
                 });
}

pub fn process(engine: Arc<Engine>, rx: &Receiver<(u32, u32, u32, MsgClass)>, _pub: &mut Pub) {
    let (id, cmd_id, _origin, content_ext) = rx.recv().unwrap();
    let from_broadcast = id == submodules::NET;
    if from_broadcast {
        trace!("from_broadcast cmd_id {:?}", cmd_id);
        if (cmd_id >> 16) == submodules::CONSENSUS {
            match content_ext {
                MsgClass::TX(tx) => {
                    trace!("get new broadcast tx {:?}", tx);
                    engine.receive_new_transaction(&tx, _pub, _origin, from_broadcast);
                }
                MsgClass::BLOCK(block) => {
                    trace!("get new broadcast block {:?}", block);
                    engine.receive_new_block(&block, _pub);
                }
                _ => {}
            }
        }
    } else {
        match content_ext {
            MsgClass::TX(tx) => {
                trace!("get new local tx {:?}", tx);
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

pub fn seal(engine: Arc<Engine>, _pub: &mut Pub) {
    trace!("new seal worker");
    engine.new_block(_pub);
}