#![allow(unused_variables)]

use threadpool::*;
use std::sync::mpsc::{Sender, Receiver};
use libproto;
use libproto::*;
use candidate_pool::CandidatePool;
use pubsub::Pub;
use cmd::{Command, decode};

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

pub fn dispatch(candidate_pool: &mut CandidatePool,
                _pub: &mut Pub,
                rx: &Receiver<(u32, u32, u32, MsgClass)>) {
    let (id, cmd_id, _origin,content_ext) = rx.recv().unwrap();
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
            info!("received chain status:({:?},{:?})",
                  status.height,
                  status.hash);
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
                            warn!("tx_pool's height:{:?}, received from consensus's height:{:?}",
                                  candidate_pool.get_height(),
                                  height);
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
