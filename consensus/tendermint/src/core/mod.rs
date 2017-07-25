pub mod tendermint;
pub mod params;
pub mod spec;
pub mod handler;
pub mod voteset;

pub use self::tendermint::*;
pub use self::params::*;
pub use self::spec::*;
pub use self::voteset::*;

pub use libproto::blockchain::{BlockHeader, Block, Transaction, BlockBody, Proof, Status};
use engine::*;
use serde_types::hash::Address;
use serde_types::hash::H256;

pub trait BareHash {
    fn bare_hash(&self) -> H256;
}