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
use core::libchain::chain::{BlockInQueue, Chain};
use libproto::blockchain::Proof;
use proof::TendermintProof;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct BlockProcessor {
    chain: Arc<Chain>,
    ctx_pub: Sender<(String, Vec<u8>)>,
}

impl BlockProcessor {
    pub fn new(chain: Arc<Chain>, ctx_pub: Sender<(String, Vec<u8>)>) -> Self {
        BlockProcessor {
            chain: chain,
            ctx_pub: ctx_pub,
        }
    }

    pub fn broadcast_current_status(&self) {
        self.chain.delivery_current_rich_status(&self.ctx_pub);
        if !self.chain.is_sync.load(Ordering::SeqCst) {
            self.chain.broadcast_status(&self.ctx_pub);
        }
    }

    pub fn set_block(&self, number: u64) {
        let block_in_queue: Option<BlockInQueue>;
        {
            let block_map = self.chain.block_map.read();
            block_in_queue = block_map.get(&number).cloned();
        }
        match block_in_queue {
            Some(BlockInQueue::ConsensusBlock(block, _)) => {
                if self.chain.validate_height(block.number()) && self.chain.validate_hash(block.parent_hash()) {
                    self.chain.set_block(block, &self.ctx_pub);
                    self.chain.broadcast_status(&self.ctx_pub);
                    info!("set consensus block-{}", number);
                }
            }
            Some(BlockInQueue::SyncBlock((_, Some(_)))) => {
                if number == self.chain.get_current_height() + 1 {
                    self.sync_blocks(number);
                    self.chain.broadcast_status(&self.ctx_pub);
                    info!("finish sync blocks to {}", number);
                };
            }
            _ => {
                info!("block-{} in queue is invalid", number);
            }
        }

        let mut guard = self.chain.block_map.write();
        let new_map = guard.split_off(&self.chain.get_current_height());
        *guard = new_map;
    }

    fn set_sync_block(&self, block: Block, proto_proof: Proof) -> bool {
        let number = block.number();
        info!("set sync block-{}", number);
        let proof = TendermintProof::from(proto_proof);
        let proof_height = if proof.height == ::std::usize::MAX {
            0
        } else {
            proof.height as u64
        };
        let authorities = self.chain.nodes.read().clone();
        if self.chain.validate_height(number) && self.chain.validate_hash(block.parent_hash())
            && proof.check(proof_height as usize, &authorities)
        {
            self.chain.set_block(block, &self.ctx_pub);
            info!("set sync block-{} is finished", number);
            true
        } else {
            info!("sync block-{} is invalid", number);
            false
        }
    }

    fn sync_blocks(&self, mut number: u64) {
        self.chain.is_sync.store(true, Ordering::SeqCst);
        info!("set sync block start from {}", number);
        let mut invalid_block_in_queue = false;
        let mut block_map;
        {
            block_map = self.chain.block_map.read().clone();
        }
        loop {
            let block_in_queue = block_map.remove(&number);
            match block_in_queue {
                Some(BlockInQueue::SyncBlock((block, Some(proof)))) => {
                    if self.set_sync_block(block, proof) {
                        number += 1;
                    } else {
                        invalid_block_in_queue = true;
                        info!("set sync block end to {} as invalid block", number - 1);
                        break;
                    }
                }
                _ => {
                    info!("set sync block end to {}", number - 1);
                    break;
                }
            }
        }

        if invalid_block_in_queue {
            let mut guard = self.chain.block_map.write();
            guard.clear();
        }

        self.chain.is_sync.store(false, Ordering::SeqCst);
    }
}
