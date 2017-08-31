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

use dispatch::PubType;
use engine::{unix_now, AsMillis};
use libproto::*;
use libproto::blockchain::*;
use protobuf::Message;
use protobuf::RepeatedField;
use std::sync::mpsc::Sender;
use tx_pool;

struct Situation {
    pub height: u64,
    pub hash: Option<Vec<u8>>,
}

pub struct CandidatePool(tx_pool::Pool, Situation);

impl CandidatePool {
    pub fn new(height: u64) -> Self {
        CandidatePool(tx_pool::Pool::new(10000, 3000), Situation { height: height, hash: None })
    }

    pub fn get_height(&self) -> u64 {
        self.1.height
    }

    pub fn meet_conditions(&self, height: u64) -> bool {
        self.1.height == (height - 1)
    }

    pub fn broadcast_tx(&self, tx: &SignedTransaction, sender: Sender<PubType>) -> Result<(), &'static str> {
        let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_TX, communication::MsgType::TX, tx.write_to_bytes().unwrap());
        trace!("broadcast new tx {:?}", tx);
        sender.send(("consensus.tx".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        Ok(())
    }

    pub fn add_tx(&mut self, tx: &mut SignedTransaction, sender: Sender<PubType>, is_from_broadcast: bool) {
        let mut content = blockchain::TxResponse::new();
        let ret = tx.recover();
        content.set_hash(tx.tx_hash.clone());
        if !ret {
            warn!("Transaction with bad signature, tx: {:?}", tx);
            content.set_result(String::from("BAG SIG").into_bytes());
        } else {
            let success = self.0.enqueue(tx.clone());
            if success {
                content.set_result(String::from("4:OK").into_bytes());
                self.broadcast_tx(tx, sender.clone()).unwrap();
            } else {
                content.set_result(String::from("4:DUP").into_bytes());
            }
        }

        if !is_from_broadcast {
            let msg = factory::create_msg(submodules::CONSENSUS, topics::TX_RESPONSE, communication::MsgType::TX_RESPONSE, content.write_to_bytes().unwrap());
            trace!("response new tx {:?}", tx);
            sender.send(("consensus.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        }
    }

    pub fn spawn_new_blk(&mut self, height: u64, hash: Vec<u8>) -> Block {
        let mut block = Block::new();
        info!("spawn new blk height:{:?}.", height);
        if height != self.1.height + 1 {
            warn!("block height is not match, expect: {}, but get {}", height, self.1.height);
        }
        let mut proof = Proof::new();
        proof.set_field_type(ProofType::Raft);

        self.1.height = height;
        block.mut_header().set_height(self.1.height);
        let block_time = unix_now();
        let txs: Vec<SignedTransaction> = self.0.package(height);
        block.mut_header().set_prevhash(hash);

        block.mut_header().set_timestamp(block_time.as_millis());
        block.mut_body().set_transactions(RepeatedField::from_slice(&txs[..]));
        let transaction_root = block.mut_body().transactions_root();
        block.mut_header().set_transactions_root(transaction_root.to_vec());
        block.mut_header().set_proof(proof);
        block
    }

    pub fn pub_block(&self, block: &Block, sender: Sender<PubType>) {
        let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_BLK, communication::MsgType::BLOCK, block.write_to_bytes().unwrap());
        trace!("publish block {:?}", block);
        sender.send(("consensus.blk".to_string(), msg.write_to_bytes().unwrap()));
    }

    pub fn update_txpool(&mut self, txs: &[SignedTransaction]) {
        info!("update txpool, current txpool size: {}", self.0.len());
        self.0.update(txs);
    }
}
