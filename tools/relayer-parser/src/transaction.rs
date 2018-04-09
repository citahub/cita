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
use std::convert::Into;

use cita_crypto::PrivKey;
use core::libchain::chain::TxProof;
use libproto::blockchain::{Transaction, UnverifiedTransaction};
use util::{H160, U256};

pub fn construct_transaction(
    pkey: &PrivKey,
    tx_proof_rlp: Vec<u8>,
    height: U256,
) -> Option<(u64, UnverifiedTransaction)> {
    let tx_proof = TxProof::from_bytes(tx_proof_rlp.clone());
    trace!("The input tx_proof is {:?}.", tx_proof);
    tx_proof.extract_relay_info().and_then(|relay_info| {
        trace!("relay_info {:?}", relay_info);
        encode(&relay_info.dest_hasher, tx_proof_rlp)
            .map(|code| sign(pkey, relay_info.dest_contract, code, height))
            .map(|utx| (relay_info.to_chain_id, utx))
    })
}

#[inline]
fn encode(dest_hasher: &str, tx_proof_rlp: Vec<u8>) -> Option<Vec<u8>> {
    FromHex::from_hex(dest_hasher)
        .map(|hasher| {
            let source: U256 = tx_proof_rlp.len().into();
            let mut target = [0u8; 32];
            source.to_big_endian(&mut target);
            let encoded = ethabi::encode(&[
                ethabi::Token::Uint(target),
                ethabi::Token::Bytes(tx_proof_rlp),
            ]);
            hasher.into_iter().chain(encoded.into_iter()).collect()
        })
        .ok()
}

#[inline]
fn sign(pkey: &PrivKey, addr: H160, code: Vec<u8>, height: U256) -> UnverifiedTransaction {
    let mut tx = Transaction::new();
    tx.set_data(code);
    tx.set_to(addr.hex());
    tx.set_valid_until_block(height.low_u64() + 100);
    tx.sign(*pkey).take_transaction_with_sig()
}
