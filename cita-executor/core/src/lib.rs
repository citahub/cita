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

#[macro_use]
extern crate cita_logger as logger;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[macro_use]
extern crate crossbeam_channel;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate cita_crypto;
#[cfg(feature = "privatetx")]
extern crate zktx;
#[macro_use]
extern crate enum_primitive;
#[cfg(test)]
pub mod benches;
#[cfg(test)]
pub mod tests;

pub extern crate common_types as types;
pub extern crate core;

pub mod cita_executive;
pub mod contracts;
pub mod libexecutor;
pub mod storage;
pub mod tx_gas_schedule;

mod trie_db;
mod authentication;
mod exception;

pub use crate::types::*;
pub use trie_db::TrieDB;
pub use cita_database as cita_db;
