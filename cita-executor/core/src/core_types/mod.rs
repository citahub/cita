// FIXME: A lot of types will be defined here, and this will replace the types defines in Parity code,
// After this work finished, delete the Parity code.
pub mod common;
pub mod errors;
pub mod receipt;

pub use receipt::{LogEntry, Receipt};

// FIXME: upgrade ethereum-types in cita-types, and then replace this.
pub use common::{Address, Balance, Hash, H256, U256};
pub use errors::TypesError;
pub use ethbloom::{Bloom, BloomRef, Input as BloomInput};
