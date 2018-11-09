// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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
#![cfg_attr(test, feature(test))]
#![feature(tool_lints)]

extern crate bincode;
extern crate byteorder;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate lru_cache;
extern crate proof;
extern crate rlp;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[macro_use]
extern crate util;

#[macro_use]
extern crate rlp_derive;
extern crate rustc_hex;

extern crate bit_set;
extern crate cita_ed25519;
extern crate cita_secp256k1;
extern crate cita_types;
extern crate common_types as types;
extern crate crossbeam;
extern crate crypto;
extern crate evm;
extern crate jsonrpc_types;
#[macro_use]
extern crate lazy_static;
extern crate time;
extern crate transient_hashmap;

#[cfg(test)]
extern crate cita_crypto;
extern crate core;
extern crate ethabi;
#[cfg(test)]
extern crate test;

extern crate grpc;
#[cfg(feature = "privatetx")]
extern crate zktx;

#[macro_use]
extern crate enum_primitive;
extern crate ethcore_bloom_journal;
extern crate largest_remainder_method;
extern crate num;
extern crate rand;

pub mod account_db;
pub mod builtin;
pub mod executed;
pub mod executive;
pub mod externalities;
pub mod factory;
pub mod pod_account;
pub mod state;
pub mod state_db;
#[cfg(test)]
pub mod tests;
pub mod trace;
#[macro_use]
pub mod engines;
pub mod error;
pub mod substate;

pub mod contracts;
pub mod libexecutor;
pub mod snapshot;

mod spec;

pub use evm::Error;
pub use factory::*;
pub use types::*;
pub use util::journaldb;
