// FIXME: A lot of types will be defined here, and this will replace the types defines in Parity code,
// After this work finished, delete the Parity code.
pub mod errors;

// FIXME: upgrade ethereum-types in cita-types, and then replace this.
pub use errors::TypesError;
