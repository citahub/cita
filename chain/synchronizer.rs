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

use core::contracts::node_manager::NodeManager;
use core::libchain::block::Block;
use core::libchain::chain::Chain;
use libproto::*;
use libproto::blockchain::Status;
use protobuf::Message;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;

const BATCH_SYNC: u64 = 120;

pub struct Synchronizer {
    pub chain: Arc<Chain>,
    pub height_marker: AtomicUsize,
}

impl Synchronizer {
    pub fn new(chain: Arc<Chain>) -> Arc<Synchronizer> {
        Arc::new(Synchronizer {
                     height_marker: AtomicUsize::new(chain.get_current_height() as usize),
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
        let current_hash = self.chain.get_current_hash();
        let current_height = self.chain.get_current_height();
        let max_height = self.chain.get_max_height();
        trace!("chain sync status: current height {:?}  known height{:?}", current_height, max_height);

        let rich_status = factory::crate_rich_status(current_hash.clone(), current_height, NodeManager::read(&self.chain).into_string());
        drop(self);
        let msg = factory::create_msg(submodules::CHAIN, topics::RICH_STATUS, communication::MsgType::RICH_STATUS, rich_status.write_to_bytes().unwrap());
        ctx_pub.send(("chain.richstatus".to_string(), msg.write_to_bytes().unwrap())).unwrap();

        let mut status: Status = Status::new();
        status.set_hash(current_hash.to_vec());
        status.set_height(current_height);
        let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
        ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();
    }

    fn add_block(&self, ctx_pub: Sender<(String, Vec<u8>)>, blk: Block) {
        trace!("chain sync add blk {:?}", blk.number());
        let rich_status = self.chain.set_block(blk);

        if let Some(rich_status) = rich_status {
            let mut status = Status::new();
            status.set_hash(rich_status.get_hash().to_vec());
            status.set_height(rich_status.get_height());

            let msg = factory::create_msg(submodules::CHAIN, topics::RICH_STATUS, communication::MsgType::RICH_STATUS, rich_status.write_to_bytes().unwrap());
            ctx_pub.send(("chain.richstatus".to_string(), msg.write_to_bytes().unwrap())).unwrap();

            let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
            trace!("add_block chain.status height = {:?}, hash = {:?}", status.get_height(), status.get_hash());
            ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();
        }
    }
}
