// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
