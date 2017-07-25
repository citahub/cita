#![allow(unused_variables)]
#![allow(unused_imports)]


use libproto::*;
use byteorder::{BigEndian, ByteOrder};
use pubsub::Pub;
use libproto;
use libproto::blockchain::Status;
use protobuf::Message;
use std::sync::Arc;
use chain::Chain;
use block::Block;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::thread;
use std::time::Duration;

const BATCH_SYNC: u64 = 120;

pub struct Synchronizer {
    pub chain: Arc<Chain>,
    pub height_marker: AtomicUsize,
}

impl Synchronizer {
    pub fn new(chain: Arc<Chain>) -> Arc<Synchronizer> {
        Arc::new(Synchronizer {
            height_marker: AtomicUsize::new(chain.current_height.load(Ordering::Relaxed)),
            chain: chain,
        })
    }

    pub fn sync(&self, _pub: &mut Pub) {
        let block_map = self.chain.block_map.read();
        if !block_map.is_empty() {
            let start_height = self.chain.get_current_height() + 1;
            if !self.chain.is_sync.load(Ordering::SeqCst) {
                self.chain.is_sync.store(true, Ordering::SeqCst);
            }
            for height in start_height..start_height + BATCH_SYNC {
                if block_map.contains_key(&height) {
                    trace!("chain sync loop {:?}", height);

                    let value = block_map[&(height)].clone();
                    self.add_block(_pub, value.1);
                } else {
                    trace!("chain sync break {:?}", height);
                    break;
                }
            }
        }
        self.chain.is_sync.store(false, Ordering::SeqCst);
    }

    pub fn sync_status(&self, _pub: &mut Pub) {
        self.chain.is_sync.store(false, Ordering::SeqCst);
        let current_hash = *self.chain.current_hash.read();
        let current_height = self.chain.get_current_height();
        info!("sync_status {:?}, {:?}", current_hash, current_height);
        let mut status = Status::new();
        status.set_hash(current_hash.0.to_vec());
        status.set_height(current_height);

        let msg = factory::create_msg(submodules::CHAIN,
                                      topics::NEW_STATUS,
                                      communication::MsgType::STATUS,
                                      status.write_to_bytes().unwrap());
        _pub.publish("chain.status", msg.write_to_bytes().unwrap());
    }

    fn add_block(&self, _pub: &mut Pub, blk: Block) {
        trace!("chain sync add blk-----{:?}", blk.get_header().get_height());
        if let Some(st) = self.chain.set_block(blk) {
            let msg = factory::create_msg(submodules::CHAIN,
                                          topics::NEW_STATUS,
                                          communication::MsgType::STATUS,
                                          st);
            info!("chain after sync-----{:?}-----{:?}",
            self.chain.get_current_height(),
            self.chain.get_max_height());
            _pub.publish("chain.status", msg.write_to_bytes().unwrap());
        }
    }
}
