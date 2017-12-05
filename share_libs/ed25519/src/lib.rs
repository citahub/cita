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

#[cfg(test)]
extern crate bincode;
extern crate rlp;
extern crate rustc_serialize;
extern crate serde;
extern crate sodiumoxide;
extern crate util;

mod keypair;
mod error;
mod signature;
mod signer;

use util::{Address, H256, H512};

pub const ADDR_BYTES_LEN: usize = 20;
pub const PUBKEY_BYTES_LEN: usize = 32;
pub const PRIVKEY_BYTES_LEN: usize = 64;
pub const SIGNATURE_BYTES_LEN: usize = 96;
pub const HASH_BYTES_LEN: usize = 32;

pub type PrivKey = H512;
pub type PubKey = H256;
pub type Message = H256;
pub type Public = H256;

pub use self::error::*;
pub use self::keypair::*;
pub use self::signature::*;
pub use self::signer::*;
