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

extern crate util;
extern crate rlp;
extern crate bloomable;
extern crate cita_crypto as crypto;
extern crate libproto;
pub mod account_diff;
pub mod basic_account;
pub mod blockchain_info;
pub mod call_analytics;
pub mod filter;
pub mod ids;
pub mod log_entry;
pub mod receipt;
pub mod state_diff;
pub mod transaction;

/// Type for block number.
pub type BlockNumber = u64;
