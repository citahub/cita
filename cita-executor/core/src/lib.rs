// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

extern crate bincode;
extern crate byteorder;
extern crate cita_crypto_trait;
extern crate libproto;
extern crate snappy;
#[macro_use]
extern crate cita_logger as logger;
extern crate lru_cache;
extern crate proof;
extern crate rlp;
#[macro_use]
extern crate serde_derive;
pub extern crate cita_database as cita_db;
extern crate cita_merklehash;
extern crate hashable;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate util;

#[macro_use]
extern crate crossbeam_channel;

extern crate rustc_hex;

extern crate bit_set;
extern crate cita_ed25519;
extern crate cita_secp256k1;
extern crate cita_types;
pub extern crate common_types as types;
extern crate crossbeam;
extern crate crypto;
extern crate jsonrpc_types;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate transient_hashmap;

#[cfg(test)]
extern crate cita_crypto;
extern crate core;
extern crate ethabi;

#[cfg(feature = "privatetx")]
extern crate zktx;

#[macro_use]
extern crate enum_primitive;
extern crate cita_database;
extern crate largest_remainder_method;
extern crate num;
extern crate rand;

#[cfg(test)]
pub mod benches;
pub mod cita_executive;
pub mod storage;
pub mod tx_gas_schedule;

// FIXME: Rename this after this work finished
pub mod core_types;
pub mod executed;
pub mod factory;
#[cfg(test)]
pub mod tests;

pub mod authentication;
pub mod contracts;
pub mod exception;
pub mod libexecutor;
pub mod trie_db;

pub use crate::factory::*;
pub use crate::types::*;
pub use trie_db::TrieDB;
