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
#![feature(custom_attribute)]
#![feature(try_from)]
#![allow(unused_attributes)]
#![cfg_attr(test, feature(test))]
extern crate bincode;
extern crate byteorder;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate lru_cache;
extern crate proof;
extern crate protobuf;
extern crate rlp;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate util;

extern crate bloomable;
extern crate bloomchain;
#[macro_use]
extern crate rlp_derive;
extern crate rustc_hex;

extern crate bit_set;
extern crate cita_ed25519;
extern crate cita_secp256k1;
extern crate common_types as types;
extern crate crossbeam;
extern crate crypto;
extern crate ethcore_io;
extern crate jsonrpc_types;
#[macro_use]
extern crate lazy_static;
extern crate sha3;
extern crate time;
extern crate transient_hashmap;

#[cfg(test)]
extern crate cita_crypto;
extern crate ethabi;
#[cfg(test)]
extern crate test;

pub mod state;
pub mod account_db;
pub mod executed;
pub mod factory;
#[cfg(test)]
pub mod tests;
pub mod action_params;
pub mod db;
pub mod state_db;
pub mod trace;
#[macro_use]
pub mod basic_types;
pub mod env_info;
pub mod builtin;
pub mod blooms;
pub mod header;
pub mod cache_manager;
pub mod executive;
pub mod externalities;
pub mod pod_account;
#[macro_use]
pub mod evm;
pub mod substate;
pub mod error;
pub mod engines;
pub mod native;

pub mod libexecutor;
pub mod contracts;

pub use factory::*;
pub use types::*;
pub use util::journaldb;
