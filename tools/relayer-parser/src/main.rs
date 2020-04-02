// Copyright Rivtower Technologies LLC.
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

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate cita_logger as logger;

mod arguments;
mod communication;
mod configuration;
mod transaction;

use cita_crypto::PrivKey;
use cita_types::H256;
use core::libchain::chain::{RelayInfo, TxProof};
use libproto::blockchain::UnverifiedTransaction;

use arguments::{build_commandline, parse_arguments};
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
            if metadata.chain_id_v1 == relay_info.to_chain_id.into() {
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
