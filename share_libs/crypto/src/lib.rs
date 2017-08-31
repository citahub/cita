#[cfg(feature = "ed25519")]
extern crate cita_ed25519;
#[cfg(feature = "secp256k1")]
extern crate cita_secp256k1;
extern crate util;

#[cfg(feature = "ed25519")]
pub use cita_ed25519::*;
#[cfg(feature = "secp256k1")]
pub use cita_secp256k1::*;
pub use util::crypto::{Sign, CreateKey};
