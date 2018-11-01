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

#![feature(tool_lints)]
extern crate cita_crypto as crypto;
extern crate cita_types;
extern crate jsonrpc_types;
extern crate libproto;
extern crate rlp;
#[macro_use]
extern crate rlp_derive;
#[macro_use]
extern crate serde_derive;
extern crate util;
#[macro_use]
extern crate lazy_static;
extern crate time;
#[macro_use]
extern crate logger;
extern crate proof;

pub extern crate bloomchain;

pub mod account_diff;
pub mod basic_account;
pub mod basic_types;
pub mod block;
pub mod cache_manager;
pub mod call_analytics;
pub mod db;
pub mod extras;
pub mod filter;
pub mod header;
pub mod ids;
pub mod log_blooms;
pub mod log_entry;
pub mod receipt;
pub mod reserved_addresses;
pub mod state_diff;
pub mod transaction;

/// Type for block number.
pub type BlockNumber = u64;
