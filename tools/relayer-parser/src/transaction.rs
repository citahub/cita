// Copyright Cryptape Technologies LLC.
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

use ethabi;

use cita_crypto::PrivKey;
use cita_types::{H160, H256, U256};
use libproto::blockchain::{Transaction, UnverifiedTransaction};

pub fn construct_transaction(
    pkey: &PrivKey,
    tx_proof_rlp: &[u8],
    dest_hasher: [u8; 4],
    dest_contract: H160,
    chain_id: U256,
    height: U256,
) -> UnverifiedTransaction {
    let code = encode(dest_hasher, tx_proof_rlp);
    sign(pkey, dest_contract, code, chain_id, height)
}

#[inline]
fn encode(dest_hasher: [u8; 4], tx_proof_rlp: &[u8]) -> Vec<u8> {
    trace!("encode dest_hasher {:?}", dest_hasher);
    trace!("encode proof_len {:?}", tx_proof_rlp.len());
    trace!("encode proof_data {:?}", tx_proof_rlp);
    let encoded = ethabi::encode(&[ethabi::Token::Bytes(tx_proof_rlp.to_vec())]);
    let ret = Vec::from(&dest_hasher[..]).into_iter().chain(encoded.into_iter()).collect();
    trace!("encode result {:?}", ret);
    ret
}

#[inline]
fn sign(pkey: &PrivKey, addr: H160, code: Vec<u8>, chain_id: U256, height: U256) -> UnverifiedTransaction {
    let mut tx = Transaction::new();
    tx.set_data(code);
    tx.set_to_v1(addr.to_vec());
    tx.set_valid_until_block(height.low_u64() + 100);
    tx.set_quota(1_000_000);
    tx.set_chain_id_v1(H256::from(chain_id).to_vec());
    tx.set_version(2);
    tx.set_value(vec![0u8; 32]);
    tx.sign(*pkey).take_transaction_with_sig()
}
