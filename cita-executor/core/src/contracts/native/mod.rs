mod crosschain_verify;
pub mod factory;
// Fix: uncommented cfg
// #[cfg(test)]
mod simple_storage;

#[cfg(feature = "privatetx")]
mod zk_privacy;

pub use factory::{Contract, NativeError};
