use std::fmt;

use super::authentication::AuthenticationError;
use super::call::CallError;
use super::native::NativeError;
use cita_vm::state::Error as StateError;

#[derive(Debug, PartialEq)]
pub enum ExecutionError {
    Internal(String),
    Authentication(AuthenticationError),
    InvalidTransaction,
    NotEnoughBaseGas,
    InvalidNonce,
    NotEnoughBalance,
    BlockQuotaLimitReached,
    AccountQuotaLimitReached,
}

impl std::error::Error for ExecutionError {}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ExecutionError::Internal(ref err) => format!("internal error: {:?}", err),
            ExecutionError::Authentication(ref err) => format!("internal error: {:?}", err),
            ExecutionError::InvalidTransaction => "invalid transaction".to_owned(),
            ExecutionError::NotEnoughBaseGas => "not enough base gas".to_owned(),
            ExecutionError::InvalidNonce => "invalid nonce".to_owned(),
            ExecutionError::NotEnoughBalance => "not enough balance".to_owned(),
            ExecutionError::BlockQuotaLimitReached => "block quota limit reached".to_owned(),
            ExecutionError::AccountQuotaLimitReached => "account quota limit reached".to_owned(),
        };
        write!(f, "{}", printable)
    }
}

impl From<NativeError> for ExecutionError {
    fn from(err: NativeError) -> Self {
        match err {
            NativeError::Internal(err_str) => ExecutionError::Internal(err_str),
        }
    }
}

impl From<AuthenticationError> for ExecutionError {
    fn from(err: AuthenticationError) -> Self {
        ExecutionError::Authentication(err)
    }
}

impl From<StateError> for ExecutionError {
    fn from(err: StateError) -> Self {
        ExecutionError::Internal(format!("{}", err))
    }
}

impl From<ExecutionError> for CallError {
    fn from(error: ExecutionError) -> Self {
        CallError::Execution(error)
    }
}
