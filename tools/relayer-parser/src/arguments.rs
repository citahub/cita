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

use cita_types::{H256, U256};
use clap;
use std::str::FromStr;

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
