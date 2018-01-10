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

use core::libchain::block::{Block, ClosedBlock};
use core::libchain::chain::{BlockInQueue, Chain, Stage};
use libproto::blockchain::Proof;
use proof::TendermintProof;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct BlockProcessor {
    chain: Arc<Chain>,
    ctx_pub: Sender<(String, Vec<u8>)>,
    closed_block: RefCell<Option<ClosedBlock>>,
}

impl BlockProcessor {
    pub fn new(chain: Arc<Chain>, ctx_pub: Sender<(String, Vec<u8>)>) -> Self {
        BlockProcessor {
            chain: chain,
            ctx_pub: ctx_pub,
            closed_block: RefCell::new(None),
        }
    }

    pub fn broadcast_current_status(&self) {
        self.chain.delivery_current_rich_status(&self.ctx_pub);
        if !self.chain.is_sync.load(Ordering::SeqCst) {
            self.chain.broadcast_status(&self.ctx_pub);
        }
    }

    pub fn set_block(&self, number: u64) {
        let block_in_queue = {
            let block_map = self.chain.block_map.read();
            block_map.get(&number).cloned()
        };
        let stage = { self.chain.stage.read().clone() };
        let mut need_clean_map = false;
        match block_in_queue {
            Some(BlockInQueue::ConsensusBlock(block, _)) => {
                if self.chain.validate_height(block.number()) && self.chain.validate_hash(block.parent_hash()) {
                    // Not Match before proposal
                    if self.chain.is_interrupted.load(Ordering::SeqCst) {
                        self.chain.is_interrupted.store(false, Ordering::SeqCst);
                        {
                            *self.chain.stage.write() = Stage::ExecutingBlock;
                        }
                        self.chain.set_block(block, &self.ctx_pub);
                    } else {
                        match stage {
                            // Match before proposal
                            Stage::WaitFinalized => {
                                if let Some(closed_block) = self.closed_block.replace(None) {
                                    {
                                        *self.chain.stage.write() = Stage::ExecutingBlock;
                                    }
                                    self.chain
                                        .finalize_proposal(closed_block, block, &self.ctx_pub);
                                } else {
                                    // Maybe never reach
                                    warn!("at WaitFinalized, but no closed block found!");
                                    {
                                        *self.chain.stage.write() = Stage::ExecutingBlock;
                                    }
                                    self.chain.set_block(block, &self.ctx_pub);
                                };
                            }
                            // Not receive proposal
                            Stage::Idle => {
                                {
                                    *self.chain.stage.write() = Stage::ExecutingBlock;
                                }
                                self.chain.set_block(block, &self.ctx_pub);
                            }
                            _ => {
                                // Maybe never reach
                                warn!(
                                    "something wrong, comming consensus block, but wrong stage {:?}",
                                    stage
                                );
                            }
                        }
                    }
                    {
                        *self.chain.stage.write() = Stage::Idle;
                    }
                    self.chain.broadcast_status(&self.ctx_pub);
                    info!("set consensus block-{}", number);
                    need_clean_map = true;
                }
            }
            Some(BlockInQueue::SyncBlock((_, Some(_)))) => {
                if number == self.chain.get_current_height() + 1 {
                    {
                        *self.chain.stage.write() = Stage::ExecutingBlock;
                    }
                    self.sync_blocks(number);
                    {
                        *self.chain.stage.write() = Stage::Idle;
                    }
                    self.chain.broadcast_status(&self.ctx_pub);
                    need_clean_map = true;
                };
            }
            Some(BlockInQueue::Proposal(proposal)) => {
                // Interrupte pre proposal
                if self.chain.is_interrupted.load(Ordering::SeqCst) {
                    self.chain.is_interrupted.store(false, Ordering::SeqCst);
                }
                {
                    *self.chain.stage.write() = Stage::ExecutingProposal;
                }
                if let Some(closed_block) = self.chain.set_proposal(proposal) {
                    // Interruppted by laster proposal/consensus block
                    if self.chain.is_interrupted.load(Ordering::SeqCst) {
                        self.chain.is_interrupted.store(false, Ordering::SeqCst);
                        return;
                    }
                    // After execute proposal, check whether block-in-map is changed
                    let in_queue = {
                        let block_map = self.chain.block_map.read();
                        block_map.get(&number).cloned()
                    };
                    match in_queue {
                        Some(BlockInQueue::ConsensusBlock(comming, _)) => {
                            if comming.header().transactions_root() == closed_block.header().transactions_root() {
                                self.chain
                                    .finalize_proposal(closed_block, comming, &self.ctx_pub);
                                {
                                    *self.chain.stage.write() = Stage::Idle;
                                }
                                self.chain.broadcast_status(&self.ctx_pub);
                                info!("finalize proposal block-{}", number);
                            } else {
                                // Maybe never reach
                                warn!("something is wrong, go into no-man's-land")
                            }
                        }
                        Some(BlockInQueue::Proposal(_)) => {
                            let mut cb = self.closed_block.borrow_mut();
                            *cb = Some(closed_block);
                            *self.chain.stage.write() = Stage::WaitFinalized;
                            info!("wait finalized");
                        }
                        _ => {
                            // Maybe never reach
                            warn!("Block in queue is wrong, go into no-man's-land");
                        }
                    }
                } else {
                    warn!("executing proposal is interrupted.");
                }
            }
            _ => {
                info!("block-{} in queue is without proof", number);
            }
        }

        if need_clean_map {
            let mut guard = self.chain.block_map.write();
            let new_map = guard.split_off(&self.chain.get_current_height());
            *guard = new_map;
        }
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
        let mut block_map = {
            let guard = self.chain.block_map.read();
            guard.clone()
        };
        loop {
            let block_in_queue = block_map.remove(&number);
            match block_in_queue {
                Some(BlockInQueue::SyncBlock((block, Some(proof)))) => {
                    if self.set_sync_block(block, proof) {
                        number += 1;
                    } else {
                        invalid_block_in_queue = true;
                        // Reach here only in byzantine condition
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
