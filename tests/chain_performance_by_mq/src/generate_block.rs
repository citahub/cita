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

use bincode::{serialize, Infinite};
use crypto::*;
use libproto::{factory, submodules, topics, MsgClass};
use libproto::blockchain::{Block, BlockWithProof, SignedTransaction, Transaction};
use proof::TendermintProof;
use protobuf::RepeatedField;
use protobuf::core::Message;
use rustc_serialize::hex::FromHex;
use std::collections::HashMap;
use std::time::{Duration, UNIX_EPOCH};
use util::H256;
use util::Hashable;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

pub trait AsMillis {
    fn as_millis(&self) -> u64;
}

impl AsMillis for Duration {
    fn as_millis(&self) -> u64 {
        self.as_secs() * 1_000 + (self.subsec_nanos() / 1_000_000) as u64
    }
}

#[allow(unused_variables, dead_code)]
pub struct Generateblock {
    pre_hash: H256,
}

#[allow(unused_variables, dead_code)]
impl Generateblock {
    pub fn new() -> Self {
        Generateblock {
            pre_hash: H256::default(),
        }
    }

    pub fn set_pre_hash(&mut self, pre_hash: H256) {
        self.pre_hash = pre_hash;
        println!("{:?}", self.pre_hash);
    }

    pub fn generate_tx(
        code: &str,
        address: String,
        quota: u64,
        flag_multi_sender: i32,
        kp: PrivKey,
    ) -> SignedTransaction {
        let pv: PrivKey;
        pv = if flag_multi_sender > 0 {
            let keypair = KeyPair::gen_keypair();
            *keypair.privkey()
        } else {
            kp
        };

        let data = code.from_hex().unwrap();
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce("0".to_string());
        tx.set_quota(quota);
        //设置空，则创建合约
        tx.set_to(address);
        tx.set_valid_until_block(99999);
        tx.sign(pv)
    }

    pub fn build_block_with_proof(txs: Vec<SignedTransaction>, pre_hash: H256, h: u64) -> (Vec<u8>, BlockWithProof) {
        let keypair = KeyPair::gen_keypair();
        let pv = keypair.privkey();
        let pk = keypair.pubkey();
        let sender = keypair.address().clone();

        let mut block = Block::new();
        let block_time = Self::unix_now();
        block.mut_header().set_timestamp(block_time.as_millis());
        block.mut_header().set_height(h);
        block.mut_header().set_prevhash(pre_hash.0.to_vec());
        block
            .mut_body()
            .set_transactions(RepeatedField::from_vec(txs));
        let mut proof = TendermintProof::default();
        proof.height = (h - 1) as usize;
        proof.round = 0;
        proof.proposal = H256::default();
        let mut commits = HashMap::new();
        let msg = serialize(
            &(
                proof.height,
                proof.round,
                Step::Precommit,
                sender.clone(),
                Some(proof.proposal.clone()),
            ),
            Infinite,
        ).unwrap();
        let signature = Signature::sign(pv, &msg.crypt_hash().into()).unwrap();
        commits.insert((*sender).into(), signature.into());
        proof.commits = commits;
        block.mut_header().set_proof(proof.clone().into());
        let transactions_root = block.get_body().transactions_root();
        block
            .mut_header()
            .set_transactions_root(transactions_root.to_vec());
        let mut proof_blk = BlockWithProof::new();
        proof_blk.set_blk(block);
        proof_blk.set_proof(proof.into());

        let msg = factory::create_msg(
            submodules::CONSENSUS,
            topics::NEW_PROOF_BLOCK,
            MsgClass::BLOCKWITHPROOF(proof_blk.clone()),
        );
        (msg.write_to_bytes().unwrap(), proof_blk)
    }

    pub fn unix_now() -> Duration {
        UNIX_EPOCH.elapsed().unwrap()
    }
}
