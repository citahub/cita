use cita_vm::evm::Error as EVMError;
use cita_vm::Error as VMError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ContractError {
    AdminError(String),
    Internal(String),
}

impl Into<VMError> for ContractError {
    fn into(self) -> VMError {
        match self {
            ContractError::AdminError(str) => VMError::Evm(EVMError::Internal(str)),
            ContractError::Internal(str) => VMError::Evm(EVMError::Internal(str)),
        }
    }
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            ContractError::AdminError(ref e) => format!("Admin sys contract error {}", e),
            ContractError::Internal(ref e) => format!("interval error {}", e),
        };

        f.write_fmt(format_args!("System contract error ({}).", msg))
    }
}
