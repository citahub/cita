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
#![allow(unused_imports)]


use byteorder::{BigEndian, ByteOrder};
use core::libchain::block::Block;
use core::libchain::chain::Chain;
use libproto;
use libproto::*;
use libproto::blockchain::Status;
use protobuf::Message;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::sync::mpsc::{Sender, Receiver};
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

    pub fn sync(&self, ctx_pub: Sender<(String, Vec<u8>)>) {
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
                    self.add_block(ctx_pub.clone(), value.1);
                } else {
                    trace!("chain sync break {:?}", height);
                    break;
                }
            }
        }
        self.chain.is_sync.store(false, Ordering::SeqCst);
    }

    pub fn sync_status(&self, ctx_pub: Sender<(String, Vec<u8>)>) {
        self.chain.is_sync.store(false, Ordering::SeqCst);
        let current_hash = *self.chain.current_hash.read();
        let current_height = self.chain.get_current_height();
        info!("sync_status {:?}, {:?}", current_hash, current_height);
        let mut status = Status::new();
        status.set_hash(current_hash.0.to_vec());
        status.set_height(current_height);

        let msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
        ctx_pub.send(("chain.status".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }

    fn add_block(&self, ctx_pub: Sender<(String, Vec<u8>)>, blk: Block) {
        trace!("chain sync add blk {:?}", blk.number());
        if let Some(st) = self.chain.set_block(blk) {
            let msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, st.write_to_bytes().unwrap());
            info!("chain after sync current height {:?}  known height{:?}", self.chain.get_current_height(), self.chain.get_max_height());
            ctx_pub.send(("chain.status".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        }
    }
}
