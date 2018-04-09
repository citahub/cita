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
extern crate rustc_hex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;

extern crate core;
extern crate jsonrpc_types;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate util;

mod arguments;
mod configuration;
mod communication;
mod transaction;

use cita_crypto::PrivKey;
use libproto::blockchain::UnverifiedTransaction;
use util::H256;

use arguments::{build_commandline, parse_arguments};
use configuration::{parse_configfile, UpStream};

fn main() {
    logger::init();

    let matches = build_commandline();
    let args = parse_arguments(&matches);
    let cfg = parse_configfile(&args.cfg_file);

    let mut retcode = 1;
    // Get servers list from the config file by the input chain id.
    cfg.get_servers(args.chain_id).map(|servers| {
        // Try to get transaction proof from servers in server list.
        // If we got a tx proof, then construct transaction from the tx proof.
        // and relay the transaction to the to-chain.
        // The chain id of to-chain is in the tx proof.
        // Relay the transaction to each server in to-chain servers list, until succeed.
        for upstream in servers.iter() {
            if let Some(tx_hash) = communication::cita_get_transaction_proof(upstream, args.tx_hash)
                .ok()
                .and_then(|tx_proof_rlp| construct_transaction(upstream, &cfg.get_private_key(), tx_proof_rlp))
                .and_then(|(to_chain_id, utx)| {
                    cfg.get_servers(to_chain_id)
                        .map(|to_servers| (to_servers, utx))
                })
                .and_then(|(to_servers, utx)| relay_transaction(to_servers, utx))
            {
                println!("{:?}", tx_hash);
                retcode = 0;
                break;
            };
        }
    });
    ::std::process::exit(retcode);
}

#[inline]
fn construct_transaction(
    upstream: &UpStream,
    pkey: &PrivKey,
    tx_proof_rlp: Vec<u8>,
) -> Option<(u64, UnverifiedTransaction)> {
    communication::cita_block_number(upstream)
        .ok()
        .and_then(|height| transaction::construct_transaction(pkey, tx_proof_rlp, height))
}

#[inline]
fn relay_transaction(servers: &Vec<UpStream>, utx: UnverifiedTransaction) -> Option<H256> {
    let mut ret = None;
    for upstream in servers.iter() {
        if let Ok(hash) = communication::cita_send_transaction(upstream, &utx) {
            ret = Some(hash);
            break;
        }
    }
    ret
}
