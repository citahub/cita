// Copyrighttape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::crypto::{CreateKey, KeyPair, PrivKey, Sign, Signature};
use bincode::{serialize, Infinite};
use cita_types::{Address, H256, U256};
use hashable::Hashable;
use libproto::TryInto;
use libproto::{Block, BlockWithProof, Message, SignedTransaction, Transaction};
use proof::BftProof;
use rustc_serialize::hex::FromHex;
use std::collections::HashMap;
use std::convert::Into;
use std::time::Duration;

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

pub struct BuildBlock {}

impl BuildBlock {
    pub fn build_contract_address(sender: &Address, nonce: &U256) -> Address {
        use rlp::RlpStream;

        let mut stream = RlpStream::new_list(2);
        stream.append(sender);
        stream.append(nonce);
        From::from(stream.out().crypt_hash())
    }

    pub fn build_tx(
        to_address: &str,
        data: &str,
        quota: u64,
        nonce: u32,
        valid_until_block: u64,
        privkey: &PrivKey,
    ) -> SignedTransaction {
        let data = data.from_hex().unwrap();
        let mut tx = Transaction::new();
        tx.set_data(data);
        tx.set_nonce(format!("{}", nonce));
        tx.set_quota(quota);
        // create contract if `to_address` empty
        tx.set_to(to_address.to_string());
        tx.set_valid_until_block(valid_until_block);
        tx.set_value(vec![0u8; 32]);
        tx.set_chain_id(123);
        tx.set_chain_id_v1(vec![]);
        tx.sign(*privkey)
    }

    /// Build a signed block with given transactions
    pub fn build_block_with_proof(
        txs: &[SignedTransaction],
        pre_hash: H256,
        height: u64,
        privkey: &PrivKey,
        timestamp: u64,
    ) -> (Vec<u8>, BlockWithProof) {
        let sender = KeyPair::from_privkey(*privkey).unwrap().address();
        let mut block = Block::new();
        block.mut_header().set_timestamp(timestamp * 1000);
        block.mut_header().set_height(height);
        block.mut_header().set_prevhash(pre_hash.0.to_vec());
        block.mut_body().set_transactions(txs.into());
        let mut proof = BftProof::default();
        proof.height = height as usize;
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
        let signature = Signature::sign(privkey, &msg.crypt_hash()).unwrap();
        commits.insert((*sender).into(), signature);
        proof.commits = commits;
        let mut previous_proof = proof.clone();
        previous_proof.height = height as usize - 1;
        block.mut_header().set_proof(previous_proof.into());
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
}
