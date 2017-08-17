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

use cmd::{Command, encode};
use libproto::*;
use libproto::blockchain::*;
use protobuf::Message;
use protobuf::RepeatedField;
use pubsub::Pub;
use tx_pool;
use util::*;

struct Situation {
    pub height: u64,
    pub is_leader: bool,
    pub hash: Option<Vec<u8>>,
}

pub struct CandidatePool(tx_pool::Pool, Situation);

impl CandidatePool {
    pub fn new(height: u64) -> Self {
        CandidatePool(tx_pool::Pool::new(10000, 3000),
                      Situation {
                          height: height,
                          is_leader: false,
                          hash: None,
                      })
    }
    /*
    pub fn set_leader(&mut self, is_leader: bool) {
        self.1.is_leader = is_leader
    }

    pub fn is_leader(&self) -> bool {
        self.1.is_leader
    }
*/
    pub fn get_height(&self) -> u64 {
        self.1.height
    }

    pub fn update_height(&mut self, height: u64) {
        self.1.height = height;
    }

    pub fn update_hash(&mut self, hash: Vec<u8>) {
        self.1.hash = Some(hash);
    }

    pub fn meet_conditions(&self, height: u64) -> bool {
        self.1.height == (height - 1)
    }

    pub fn reflect_situation(&self, _pub: &mut Pub) {
        let cmd = Command::PoolSituation(self.1.height, self.1.hash.clone(), None);
        let msg = factory::create_msg(submodules::CONSENSUS, topics::DEFAULT, communication::MsgType::MSG, encode(&cmd));
        _pub.publish("consensus.default", msg.write_to_bytes().unwrap());
    }

    pub fn broadcast_tx(&self, tx: &Transaction, _pub: &mut Pub) -> Result<(), &'static str> {
        let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_TX, communication::MsgType::TX, tx.write_to_bytes().unwrap());
        trace!("broadcast new tx {:?}", tx);
        _pub.publish("consensus.tx", msg.write_to_bytes().unwrap());
        Ok(())
    }

    pub fn add_tx(&mut self, tx: &Transaction, _pub: &mut Pub, is_from_broadcast: bool) {
        let mut content = blockchain::TxResponse::new();
        let hash: H256 = tx.crypt_hash();
        {
            content.set_hash(hash.to_vec());
            let success = self.0.enqueue(tx.clone(), hash);
            if success {
                content.set_result(String::from("4:OK").into_bytes());
                self.broadcast_tx(tx, _pub).unwrap();
            } else {
                content.set_result(String::from("4:DUP").into_bytes());
            }
            if !is_from_broadcast {
                let msg = factory::create_msg(submodules::CONSENSUS, topics::TX_RESPONSE, communication::MsgType::TX_RESPONSE, content.write_to_bytes().unwrap());
                trace!("response new tx {:?}", tx);
                _pub.publish("consensus.rpc", msg.write_to_bytes().unwrap());
            }
        }
    }

    pub fn spawn_new_blk(&mut self, height: u64) -> Block {
        let mut block = Block::new();
        info!("spawn new blk height:{:?}.", self.1.height);
        if height != self.1.height + 1 {}
        self.1.height = height;
        block.mut_header().set_height(self.1.height);
        let txs: Vec<Transaction> = self.0.package(height);
        block.mut_body().set_transactions(RepeatedField::from_slice(&txs[..]));
        //block.mut_header().set_timestamp(block_time.as_millis());
        //block.mut_header().set_proof(proof);
        block
    }

    pub fn pub_block(&self, block: &Block, _pub: &mut Pub) {
        let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_BLK, communication::MsgType::BLOCK, block.write_to_bytes().unwrap());
        trace!("publish block {:?}", block);
        _pub.publish("consensus.blk", msg.write_to_bytes().unwrap());
    }
}
