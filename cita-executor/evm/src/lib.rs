// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Ethereum virtual machine.

#![feature(tool_lints)]

extern crate bit_set;
extern crate cita_types;
extern crate common_types as types;
#[cfg_attr(feature = "evm-debug", macro_use)]
extern crate logger;
extern crate rlp;
extern crate rustc_hex;
extern crate util;

pub mod action_params;
pub mod call_type;
pub mod env_info;
pub mod error;
pub mod evm;
pub mod ext;
pub mod interpreter;
pub mod return_data;
#[macro_use]
pub mod storage;
#[macro_use]
pub mod factory;
pub mod instructions;
pub mod schedule;
#[macro_use]
extern crate lazy_static;

#[cfg(all(feature = "benches", test))]
mod benches;
#[cfg(test)]
pub mod tests;

pub mod fake_tests;

pub use self::error::{Error, Result};
pub use self::evm::{CostType, Evm, FinalizationResult, Finalize};
pub use self::ext::{ContractCreateResult, Ext, MessageCallResult};
pub use self::factory::{Factory, VMType};
pub use self::instructions::*;
pub use self::return_data::{GasLeft, ReturnData};
pub use self::schedule::Schedule;
