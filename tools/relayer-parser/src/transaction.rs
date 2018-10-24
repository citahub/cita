// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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
    let ret = Vec::from(&dest_hasher[..])
        .into_iter()
        .chain(encoded.into_iter())
        .collect();
    trace!("encode result {:?}", ret);
    ret
}

#[inline]
fn sign(
    pkey: &PrivKey,
    addr: H160,
    code: Vec<u8>,
    chain_id: U256,
    height: U256,
) -> UnverifiedTransaction {
    let mut tx = Transaction::new();
    tx.set_data(code);
    tx.set_to_v1(addr.to_vec());
    tx.set_valid_until_block(height.low_u64() + 100);
    tx.set_quota(1_000_000);
    tx.set_chain_id_v1(H256::from(chain_id).to_vec());
    tx.set_version(1);
    tx.set_value(vec![0u8; 32]);
    tx.sign(*pkey).take_transaction_with_sig()
}
