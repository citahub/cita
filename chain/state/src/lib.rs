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

#![feature(plugin)]
#![feature(test)]
#![cfg_attr(test, plugin(stainless))]
extern crate test;
extern crate byteorder;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate util;
extern crate rustc_serialize;
extern crate lru_cache;
extern crate rlp;
extern crate ethcore_bloom_journal as bloom_journal;
extern crate bloomchain;
#[macro_use]
extern crate lazy_static;
extern crate bit_set;
extern crate crypto;
extern crate num;
extern crate rand;
extern crate futures;
extern crate ethkey;
extern crate bn;
extern crate time;
extern crate crossbeam;
extern crate ethcore_io;
extern crate transaction;
extern crate cita_crypto;

pub mod state;
pub mod account_db;
pub mod types;
pub mod factory;
#[cfg(test)]
pub mod tests;
pub mod action_params;
// TODO: Moved to libchain?
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

pub use types::*;
pub use factory::*;
pub use util::journaldb;
