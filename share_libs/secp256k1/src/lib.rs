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
#![allow(unused_extern_crates)]
#![feature(test)]

#[cfg(test)]
extern crate bincode;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate rlp;
extern crate rustc_serialize;
extern crate secp256k1;
extern crate serde;
extern crate test;
extern crate util;

pub type PrivKey = H256;
pub type PubKey = H512;
pub type Message = H256;
pub type Public = H512;

pub const ADDR_BYTES_LEN: usize = 20;
pub const PUBKEY_BYTES_LEN: usize = 64;
pub const PRIVKEY_BYTES_LEN: usize = 32;
pub const SIGNATURE_BYTES_LEN: usize = 65;
pub const HASH_BYTES_LEN: usize = 32;

mod error;
mod keypair;
mod signature;
mod signer;

pub use self::error::*;
pub use self::keypair::*;
pub use self::signature::*;
pub use self::signer::Signer;
use util::{Address, H256, H512};

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

#[test]
fn it_works() {}
