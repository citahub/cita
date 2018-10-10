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
use cita_types::H256;
use crypto::*;
use libproto::{Block, BlockWithProof, Message, SignedTransaction, Transaction};
use proof::BftProof;
use rustc_serialize::hex::FromHex;
use std::collections::HashMap;
use std::convert::{Into, TryInto};
use std::time::{Duration, UNIX_EPOCH};
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
        self.as_secs() * 1_000 + u64::from(self.subsec_millis())
    }
}

pub struct Generateblock;

impl Generateblock {
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
        // create contract if `to_address` empty
        tx.set_to(address);
        tx.set_valid_until_block(99_999);
        tx.sign(pv)
    }

    pub fn build_block_with_proof(
        txs: Vec<SignedTransaction>,
        pre_hash: H256,
        h: u64,
    ) -> (Vec<u8>, BlockWithProof) {
        let keypair = KeyPair::gen_keypair();
        let pv = keypair.privkey();
        let sender = keypair.address();

        let mut block = Block::new();
        let block_time = Self::unix_now();
        block
            .mut_header()
            .set_timestamp(AsMillis::as_millis(&block_time));
        block.mut_header().set_height(h);
        block.mut_header().set_prevhash(pre_hash.0.to_vec());
        block.mut_body().set_transactions(txs.into());
        let mut proof = BftProof::default();
        proof.height = (h - 1) as usize;
        proof.round = 0;
        proof.proposal = H256::default();
        let mut commits = HashMap::new();
        let msg = serialize(
            &(
                proof.height,
                proof.round,
                Step::Precommit,
                sender,
                Some(proof.proposal),
            ),
            Infinite,
        )
        .unwrap();
        let signature = Signature::sign(pv, &msg.crypt_hash()).unwrap();
        commits.insert((*sender).into(), signature);
        proof.commits = commits;
        block.mut_header().set_proof(proof.clone().into());
        let transactions_root = block.get_body().transactions_root();
        block
            .mut_header()
            .set_transactions_root(transactions_root.to_vec());
        let mut proof_blk = BlockWithProof::new();
        proof_blk.set_blk(block);
        proof_blk.set_proof(proof.into());

        let msg: Message = proof_blk.clone().into();
        (msg.try_into().unwrap(), proof_blk)
    }

    pub fn unix_now() -> Duration {
        UNIX_EPOCH.elapsed().unwrap()
    }
}
