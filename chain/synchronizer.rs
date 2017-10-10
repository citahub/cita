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

use core::libchain::block::Block;
use core::libchain::chain::Chain;
use libproto::*;
use libproto::blockchain::{Status, RichStatus};
use protobuf::{Message, RepeatedField};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use types::ids::BlockId;
use util::Address;

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
                    let value = block_map[&(height)].clone();
                    let block = value.1;
                    let is_verified = value.2;
                    if is_verified {
                        self.add_block(ctx_pub.clone(), block);
                    } else {
                        trace!("chain proof not ok height: {}, wait next sync", height);
                        break;
                    }
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
        let nodes: Vec<Address> = self.chain.nodes.read().clone();

        drop(self);
        info!("sync_status {:?}, {:?}", current_hash, current_height);

        let mut rich_status = RichStatus::new();
        rich_status.set_hash(current_hash.0.to_vec());
        rich_status.set_height(current_height);
        let node_list = nodes.into_iter().map(|address| address.to_vec()).collect();
        rich_status.set_nodes(RepeatedField::from_vec(node_list));

        let msg = factory::create_msg(submodules::CHAIN, topics::RICH_STATUS, communication::MsgType::RICH_STATUS, rich_status.write_to_bytes().unwrap());
        trace!("chain after sync current height {:?}  known height{:?}", current_height, max_height);
        ctx_pub.send(("chain.richstatus".to_string(), msg.write_to_bytes().unwrap())).unwrap();

        let status: Status = rich_status.into();
        let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
        trace!("add_block chain.status {:?}, {:?}", status.get_height(), status.get_hash());
        ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();
    }

    fn add_block(&self, ctx_pub: Sender<(String, Vec<u8>)>, blk: Block) {
        trace!("chain sync add blk {:?}", blk.number());
        let rich_status = self.chain.set_block(blk);

        if let Some(rich_status) = rich_status {
            let msg = factory::create_msg(submodules::CHAIN, topics::RICH_STATUS, communication::MsgType::RICH_STATUS, rich_status.write_to_bytes().unwrap());
            trace!("chain after sync current height {:?}  known height{:?}", self.chain.get_current_height(), self.chain.get_max_height());
            ctx_pub.send(("chain.richstatus".to_string(), msg.write_to_bytes().unwrap())).unwrap();

            let status: Status = rich_status.into();
            let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
            trace!("add_block chain.status {:?}, {:?}", status.get_height(), status.get_hash());
            ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();
            let block_height = status.get_height();
            self.sync_block_tx_hashes(block_height, ctx_pub);
        }
    }

    pub fn sync_block_tx_hashes(&self, block_height: u64, ctx_pub: Sender<(String, Vec<u8>)>) {
        if let Some(tx_hashes) = self.chain.transaction_hashes(BlockId::Number(block_height)) {
            //prepare and send the block tx hashes to auth
            let mut block_tx_hashes = BlockTxHashes::new();
            block_tx_hashes.set_height(block_height);
            let mut tx_hashes_in_u8 = Vec::new();
            for tx_hash_in_h256 in tx_hashes.iter() {
                tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());
            }
            block_tx_hashes.set_tx_hashes(RepeatedField::from_slice(&tx_hashes_in_u8[..]));

            let msg = factory::create_msg(submodules::CHAIN, topics::BLOCK_TXHASHES, communication::MsgType::BLOCK_TXHASHES, block_tx_hashes.write_to_bytes().unwrap());

            ctx_pub.send(("chain.txhashes".to_string(), msg.write_to_bytes().unwrap())).unwrap();
            trace!("sync block's tx hashes for height:{}", block_height);
        }
    }
}
