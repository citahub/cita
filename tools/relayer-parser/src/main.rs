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

#![feature(try_from)]
extern crate cita_crypto;
#[macro_use]
extern crate clap;
extern crate ethabi;
extern crate futures;
extern crate hyper;
extern crate parking_lot;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

extern crate cita_types;
extern crate core;
extern crate jsonrpc_types;
extern crate libproto;
#[macro_use]
extern crate logger;

mod arguments;
mod communication;
mod configuration;
mod transaction;

use cita_crypto::PrivKey;
use cita_types::H256;
use core::libchain::chain::{RelayInfo, TxProof};
use libproto::blockchain::UnverifiedTransaction;

use arguments::{build_commandline, parse_arguments};
use cita_types::traits::LowerHex;
use configuration::{parse_configfile, UpStream};

fn main() {
    logger::init();

    let matches = build_commandline();
    let args = parse_arguments(&matches);
    let cfg = parse_configfile(&args.cfg_file);

    let mut retcode = 1;
    // Get servers list from the config file by the input chain id.
    // Try to get transaction proof from servers in server list.
    // If we got a tx proof, then construct transaction from the tx proof.
    // and relay the transaction to the to-chain.
    // The chain id of to-chain is in the tx proof.
    // Relay the transaction to each server in to-chain servers list, until succeed.
    let _ = cfg
        .get_servers(args.chain_id)
        .and_then(|servers| fetch_txproof(&servers[..], args.tx_hash))
        .and_then(|tx_proof_rlp| {
            deconstruct_txproof(&tx_proof_rlp[..]).map(|relay_info| (tx_proof_rlp, relay_info))
        })
        .and_then(|(tx_proof_rlp, relay_info)| {
            cfg.get_servers(relay_info.to_chain_id)
                .map(|to_servers| (to_servers, tx_proof_rlp, relay_info))
        })
        .and_then(|(to_servers, tx_proof_rlp, relay_info)| {
            relay_transaction(
                &to_servers[..],
                &cfg.get_private_key(),
                &tx_proof_rlp[..],
                &relay_info,
            )
        })
        .map(|tx_hash| {
            println!("{:?}", tx_hash);
            retcode = 0;
        });
    ::std::process::exit(retcode);
}

#[inline]
fn fetch_txproof(servers: &[UpStream], tx_hash: H256) -> Option<Vec<u8>> {
    let mut ret = None;
    for upstream in servers.iter() {
        if let Ok(tx_proof_rlp) = communication::cita_get_transaction_proof(upstream, tx_hash) {
            ret = Some(tx_proof_rlp);
            break;
        }
    }
    ret
}

#[inline]
fn deconstruct_txproof(tx_proof_rlp: &[u8]) -> Option<RelayInfo> {
    trace!("proof_len {:?}", tx_proof_rlp.len());
    trace!("proof_data {:?}", tx_proof_rlp);
    let tx_proof = TxProof::from_bytes(tx_proof_rlp);
    trace!("The input tx_proof is {:?}.", tx_proof);
    tx_proof.extract_relay_info().map(|relay_info| {
        trace!("relay_info {:?}", relay_info);
        relay_info
    })
}

#[inline]
fn construct_transaction(
    upstream: &UpStream,
    pkey: &PrivKey,
    tx_proof_rlp: &[u8],
    relay_info: &RelayInfo,
) -> Option<UnverifiedTransaction> {
    communication::cita_get_metadata(upstream)
        .ok()
        .and_then(|metadata| {
            if metadata.chain_id_v1 == relay_info.to_chain_id.lower_hex() {
                Some(relay_info.to_chain_id)
            } else {
                error!(
                    "chain id is not right {} != {}",
                    metadata.chain_id, relay_info.to_chain_id
                );
                None
            }
        })
        .and_then(|chain_id| {
            communication::cita_block_number(upstream)
                .ok()
                .map(|height| (chain_id, height))
        })
        .map(|(chain_id, height)| {
            transaction::construct_transaction(
                pkey,
                tx_proof_rlp,
                relay_info.dest_hasher,
                relay_info.dest_contract,
                chain_id,
                height,
            )
        })
}

#[inline]
fn relay_transaction(
    servers: &[UpStream],
    pkey: &PrivKey,
    tx_proof_rlp: &[u8],
    relay_info: &RelayInfo,
) -> Option<H256> {
    let mut ret = None;
    for upstream in servers.iter() {
        if let Some(utx) = construct_transaction(upstream, pkey, tx_proof_rlp, relay_info) {
            if let Ok(hash) = communication::cita_send_transaction(upstream, &utx) {
                ret = Some(hash);
                break;
            }
        }
    }
    ret
}
