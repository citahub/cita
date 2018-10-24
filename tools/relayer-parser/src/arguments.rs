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

use clap;
use std::str::FromStr;

use cita_types::{H256, U256};

pub struct AppArgs {
    pub cfg_file: String,
    pub chain_id: U256,
    pub tx_hash: H256,
}

impl<'a> From<&'a clap::ArgMatches<'a>> for AppArgs {
    fn from(matches: &'a clap::ArgMatches) -> Self {
        let cfg_file = matches.value_of("ConfigFile").unwrap();
        let chain_id_str = matches.value_of("ChainId").unwrap();
        let chain_id_str = if chain_id_str.starts_with("0x") {
            &chain_id_str[2..]
        } else {
            chain_id_str
        };

        let chain_id = U256::from_str(chain_id_str).unwrap();

        let tx_hash_str = matches.value_of("TxHash").unwrap();
        let tx_hash_str = if tx_hash_str.starts_with("0x") {
            &tx_hash_str[2..]
        } else {
            tx_hash_str
        };
        let tx_hash = H256::from_str(tx_hash_str).unwrap();
        AppArgs {
            cfg_file: cfg_file.to_owned(),
            chain_id: chain_id.to_owned(),
            tx_hash,
        }
    }
}

impl ::std::fmt::Display for AppArgs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("AppArgs")
            .field("cfg_file", &self.cfg_file)
            .field("chain_id", &self.chain_id)
            .field("tx_hash", &self.tx_hash)
            .finish()
    }
}

pub fn build_commandline<'a>() -> clap::ArgMatches<'a> {
    let matches = clap_app!(RelayerParser =>
        (version: "0.1")
        (author: "Cryptape Technologies")
        (about: "CITA Relay Info Parser by Rust")
        (@arg ConfigFile: -f --config_file +takes_value +required "Input a toml configuration file.")
        (@arg ChainId: -c --chain_id +takes_value +required "Input a chain id for the transaction hash.")
        (@arg TxHash: -t --tx_hash +takes_value +required "Input a hex string of the transaction hash.")
    ).get_matches();
    trace!("matches = {:?}", matches);
    matches
}

pub fn parse_arguments(matches: &clap::ArgMatches) -> AppArgs {
    let args = AppArgs::from(matches);
    trace!("args = {}", args);
    args
}
