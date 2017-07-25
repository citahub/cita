#[macro_use]
extern crate lazy_static;
extern crate secp256k1;
extern crate sha3;
extern crate rustc_serialize;
extern crate util;
extern crate rand;
extern crate serde_types;

pub type PrivKey = H256;
pub type PubKey = H512;
pub type Message = H256;
pub type Public = H512;

mod error;
mod keypair;
mod signature;
mod signer;

pub use self::error::*;
pub use self::keypair::*;
pub use self::signature::*;
pub use self::signer::Signer;
use serde_types::hash::Address;
use serde_types::hash::{H256, H512};


lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}

#[test]
fn it_works() {}
