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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]
#![feature(try_from)]
#![allow(unused_attributes)]
#![allow(unused_extern_crates)]
extern crate bincode;
extern crate cita_types;
extern crate common_types as types;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate proof;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(not(test))]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
extern crate uuid;

mod id;
mod params;
pub mod error;
pub mod bytes;
pub mod request;
pub mod response;
pub mod rpctypes;
pub mod method;
pub use self::error::*;
pub use self::id::*;
pub use self::params::*;
pub use self::request::*;
pub use serde_json::Value;
pub use serde_json::to_string;
pub use serde_json::value::to_value;
