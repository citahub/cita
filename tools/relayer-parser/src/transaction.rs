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
use rustc_hex::FromHex;

use cita_crypto::PrivKey;
use libproto::blockchain::{Transaction, UnverifiedTransaction};
use util::{H160, U256};

pub fn construct_transaction(
    pkey: &PrivKey,
    tx_proof_rlp: Vec<u8>,
    dest_hasher: &str,
    dest_contract: H160,
    chain_id: u32,
    height: U256,
) -> Option<UnverifiedTransaction> {
    encode(dest_hasher, tx_proof_rlp).map(|code| sign(pkey, dest_contract, code, chain_id, height))
}

#[inline]
fn encode(dest_hasher: &str, tx_proof_rlp: Vec<u8>) -> Option<Vec<u8>> {
    FromHex::from_hex(dest_hasher)
        .map(|hasher| {
            trace!("encode dest_hasher {:?}", hasher);
            trace!("encode proof_len {:?}", tx_proof_rlp.len());
            trace!("encode proof_data {:?}", tx_proof_rlp);
            let encoded = ethabi::encode(&[ethabi::Token::Bytes(tx_proof_rlp)]);
            let ret = hasher.into_iter().chain(encoded.into_iter()).collect();
            trace!("encode result {:?}", ret);
            ret
        })
        .ok()
}

#[inline]
fn sign(pkey: &PrivKey, addr: H160, code: Vec<u8>, chain_id: u32, height: U256) -> UnverifiedTransaction {
    let mut tx = Transaction::new();
    tx.set_data(code);
    tx.set_to(addr.hex());
    tx.set_valid_until_block(height.low_u64() + 100);
    tx.set_quota(10000000000);
    tx.set_chain_id(chain_id);
    tx.sign(*pkey).take_transaction_with_sig()
}
