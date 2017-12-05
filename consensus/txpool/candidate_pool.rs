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
use error::ErrorCode;
use libproto::*;
use libproto::blockchain::*;
use protobuf::Message;
use protobuf::RepeatedField;
use serde_json;
use std::sync::mpsc::Sender;
use tx_pool;
use util::H256;

pub struct CandidatePool {
    pool: tx_pool::Pool,
    height: u64,
    sender: Sender<PubType>,
}

impl CandidatePool {
    pub fn new(sender: Sender<PubType>) -> Self {
        CandidatePool {
            pool: tx_pool::Pool::new(3000),
            height: 0,
            sender: sender,
        }
    }


    pub fn get_height(&self) -> u64 {
        self.height
    }

    pub fn meet_conditions(&self, height: u64) -> bool {
        self.height == (height - 1)
    }

    pub fn broadcast_tx(&self, tx_req: &Request) {
        let msg = factory::create_msg(
            submodules::CONSENSUS,
            topics::REQUEST,
            communication::MsgType::REQUEST,
            tx_req.write_to_bytes().unwrap(),
        );
        trace!("broadcast new tx {:?}", tx_req);
        self.sender
            .send(("consensus.tx".to_string(), msg.write_to_bytes().unwrap()));
    }

    pub fn add_tx(&mut self, tx_req: &Request, is_from_broadcast: bool) {
        let unverified_tx = tx_req.get_un_tx();
        let trans = SignedTransaction::verify_transaction(unverified_tx.clone());
        let mut response = Response::new();
        let error_code = submodules::CONSENSUS as i64;
        response.set_request_id(tx_req.get_request_id().to_vec());

        match trans {
            Err(hash) => {
                response.set_code(ErrorCode::tx_auth_error());
                warn!("Transaction with bad signature, tx: {:?}", hash);
                response.set_error_msg("BadSig".to_string());
            }

            Ok(tx) => {
                let hash = H256::from_slice(&tx.tx_hash);
                let success = self.pool.enqueue(tx);
                if success {
                    let tx_response = TxResponse::new(hash.clone(), String::from("Ok"));
                    let tx_state = serde_json::to_string(&tx_response).unwrap();
                    response.set_tx_state(tx_state);
                    self.broadcast_tx(tx_req);
                } else {
                    response.set_code(ErrorCode::tx_auth_error());
                    response.set_error_msg("Dup".to_string());
                }
            }
        }

        // Response RPC
        if !is_from_broadcast {
            let msg = factory::create_msg(
                submodules::CONSENSUS,
                topics::RESPONSE,
                communication::MsgType::RESPONSE,
                response.write_to_bytes().unwrap(),
            );
            self.sender
                .send(("consensus.rpc".to_string(), msg.write_to_bytes().unwrap()))
                .unwrap();
        }
    }

    pub fn spawn_new_blk(&mut self, height: u64, hash: Vec<u8>) -> Block {
        let mut block = Block::new();
        info!("spawn new blk height:{:?}.", height);
        if height != self.height + 1 {
            warn!(
                "block height is not match, expect: {}, but get {}",
                height,
                self.height
            );
        }
        let mut proof = Proof::new();
        proof.set_field_type(ProofType::Raft);

        self.height = height;
        block.mut_header().set_height(self.height);
        let block_time = unix_now();
        let txs: Vec<SignedTransaction> = self.pool.package_backword_compatible(height);

        block.mut_header().set_prevhash(hash);
        block.mut_header().set_timestamp(block_time.as_millis());
        block
            .mut_body()
            .set_transactions(RepeatedField::from_slice(&txs[..]));
        let transaction_root = block.mut_body().transactions_root();
        block
            .mut_header()
            .set_transactions_root(transaction_root.to_vec());
        block.mut_header().set_proof(proof);
        block
    }

    pub fn pub_block(&self, block: &Block) {
        let msg = factory::create_msg(
            submodules::CONSENSUS,
            topics::NEW_BLK,
            communication::MsgType::BLOCK,
            block.write_to_bytes().unwrap(),
        );
        trace!("publish block {:?}", block);
        self.sender
            .send(("consensus.blk".to_string(), msg.write_to_bytes().unwrap()));
    }

    pub fn update_txpool(&mut self, txs: &[SignedTransaction]) {
        trace!("update txpool, current txpool size: {}", self.pool.len());
        self.pool.update(txs);
    }
}
