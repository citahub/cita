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
use libproto::blockchain::{Status, RichStatus, AccountGasLimit};
use protobuf::{Message, RepeatedField};
use std::collections::HashMap;
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
        let nodes: Vec<Address> = NodeManager::read(&self.chain);
        //todo get block gas limit
        let block_gas_limit = 30000;
        //todo (get account gas limit. key: Address  value: account_gas_limit)
        let mut specific_gas_limit = HashMap::new();
        specific_gas_limit.insert(Address::new().hex(), 30000);
        let mut account_gas_limit = AccountGasLimit::new();
        account_gas_limit.set_common_gas_limit(1000000);
        account_gas_limit.set_specific_gas_limit(specific_gas_limit);

        drop(self);
        info!("sync_status {:?}, {:?}", current_hash, current_height);

        let mut rich_status = RichStatus::new();
        rich_status.set_hash(current_hash.0.to_vec());
        rich_status.set_height(current_height);
        let node_list = nodes.into_iter().map(|address| address.to_vec()).collect();
        rich_status.set_nodes(RepeatedField::from_vec(node_list));
        rich_status.set_block_gas_limit(block_gas_limit);
        rich_status.set_account_gas_limit(account_gas_limit);

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
        }
    }
}
