use std::error::Error;
use std::fmt;

use crate::types::errors::NativeError;
use cita_vm::Error as VMError;

// There is not reverted expcetion in VMError, so handle this in ExecutedException.
#[derive(Debug)]
pub enum ExecutedException {
    VM(VMError),
    NativeContract(NativeError),
    Reverted,
}

impl Error for ExecutedException {}

impl fmt::Display for ExecutedException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ExecutedException::VM(ref err) => format!("exception in vm: {:?}", err),
            ExecutedException::NativeContract(ref err) => {
                format!("exception in native contract: {:?}", err)
            }
            ExecutedException::Reverted => "execution reverted".to_owned(),
        };
        write!(f, "{}", printable)
    }
}

impl From<VMError> for ExecutedException {
    fn from(err: VMError) -> Self {
        ExecutedException::VM(err)
    }
}

impl From<NativeError> for ExecutedException {
    fn from(err: NativeError) -> Self {
        ExecutedException::NativeContract(err)
    }
}
