// Copyright Rivtower Technologies LLC.
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
pub mod cita_vm_helper;
pub mod contracts;
pub mod data_provider;
pub mod libexecutor;
pub mod storage;
pub mod tx_gas_schedule;

mod authentication;
mod exception;
mod trie_db;

pub use crate::types::*;
pub use cita_database as cita_db;
pub use trie_db::TrieDB;
