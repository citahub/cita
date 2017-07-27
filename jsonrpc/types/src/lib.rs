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
#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
extern crate futures;
extern crate hyper;
extern crate libproto;
extern crate protobuf;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate sha3;
extern crate util;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate rustc_serialize;
extern crate amqp;
extern crate pubsub;
extern crate time;
extern crate proof;
extern crate serde_types;
extern crate state;

mod id;
mod params;
mod error;
pub mod bytes;
pub mod rpc_request;
pub mod rpc_response;
pub mod rpctypes;

pub use self::id::*;
pub use self::params::*;
pub use self::error::RpcError;
pub use serde_json::Value;
pub use serde_json::value::to_value;
pub use serde_json::to_string;
