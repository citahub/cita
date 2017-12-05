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

#![feature(libc)]
#![feature(unique)]
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate rlp;
extern crate rustc_serialize;
extern crate serde;
extern crate util;

use libc::c_int;
use std::ptr::Unique;
use util::{Address, H256, H512};

mod keypair;
mod error;
mod signature;
mod signer;

pub use self::error::*;
pub use self::keypair::*;
pub use self::signature::*;
pub use self::signer::*;

pub type PrivKey = H256;
pub type PubKey = H512;
pub type Message = H256;
pub type Public = H512;

pub const ADDR_BYTES_LEN: usize = 20;
pub const PUBKEY_BYTES_LEN: usize = 64;
pub const PRIVKEY_BYTES_LEN: usize = 32;
pub const SIGNATURE_BYTES_LEN: usize = 65;
pub const HASH_BYTES_LEN: usize = 32;

pub enum EcGroup {}
unsafe impl std::marker::Sync for EcGroup {}

#[link(name = "gmssl")]
extern "C" {
    pub fn ec_group() -> *mut EcGroup;
    //not used
    //pub fn EC_GROUP_free(group: *mut EcGroup);
    pub fn sm2_generate_key(group: *const EcGroup, privkey: *mut u8, pubkey: *mut u8);
    pub fn sm2_pubkey_from_privkey(group: *const EcGroup, privkey: *const u8, pubkey: *mut u8);
    pub fn sm2_sign(group: *const EcGroup, privkey: *const u8, message: *const u8, signature: *mut u8);
    pub fn sm2_recover(group: *const EcGroup, signature: *const u8, message: *const u8, pubkey: *mut u8) -> c_int;
}

lazy_static! {
    pub static ref GROUP: Unique<EcGroup> = unsafe {
        Unique::new(ec_group()).unwrap()
    };
}
