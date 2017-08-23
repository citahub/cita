extern crate sodiumoxide;
extern crate rustc_serialize;
extern crate util;
extern crate rlp;

mod keypair;
mod error;
mod signature;
mod signer;

use util::{H256, H512, Address};

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
