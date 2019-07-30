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

extern crate cita_crypto as crypto;
#[macro_use]
extern crate rlp_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cita_logger as logger;
extern crate cita_database as cita_db;

pub extern crate bloomchain;

//pub mod account_diff;
//pub mod basic_account;
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
//pub mod state_diff;
pub mod transaction;

/// Type for block number.
pub type BlockNumber = u64;
