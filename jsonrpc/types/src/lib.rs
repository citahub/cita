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
