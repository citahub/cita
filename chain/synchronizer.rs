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
use libproto::blockchain::{Status, RichStatus, ProofType};
use proof::TendermintProof;
use protobuf::{Message, RepeatedField};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
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

    pub fn sync(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let flag = !self.chain.block_map.read().is_empty();
        if flag {
            let start_height = self.chain.get_current_height() + 1;
            if !self.chain.is_sync.load(Ordering::SeqCst) {
                self.chain.is_sync.store(true, Ordering::SeqCst);
            }

            for height in start_height..start_height + BATCH_SYNC {
                let mut val = None;

                {
                    let block_map = self.chain.block_map.read();
                    if block_map.contains_key(&height) {
                        val = Some(block_map[&(height)].clone());
                    }
                }
                if let Some(value) = val {
                    let proof_option = value.0;
                    let block = value.1;
                    let is_local = value.2;
                    if let Some(proof) = proof_option {
                        if is_local {
                            self.add_block(ctx_pub, block);
                        } else {
                            // Check block proof
                            let block = Block::from(block.clone());
                            if let Some(proof_type) = block.proof_type() {
                                if proof_type == ProofType::Tendermint {
                                    let authorities = self.chain.nodes.read().clone();
                                    let proof = TendermintProof::from(proof.clone());
                                    if proof.check(height as usize, &authorities) {
                                        self.add_block(ctx_pub, block);
                                    }
                                }
                            };
                            trace!("block {} proof is invalid ", height);
                            {
                                let mut block_map = self.chain.block_map.write();
                                block_map.remove(&height);
                            }
                        }
                    } else {
                        trace!("chain proof not exist height: {}, wait next sync", height);
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

    pub fn sync_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
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

    fn add_block(&self, ctx_pub: &Sender<(String, Vec<u8>)>, blk: Block) {
        trace!("chain sync add blk {:?}", blk.number());
        let status = self.chain.set_block(blk, &ctx_pub);

        if let Some(status) = status {
            let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
            trace!("add_block chain.status {:?}, {:?}", status.get_height(), status.get_hash());
            ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();
        }
    }
}
