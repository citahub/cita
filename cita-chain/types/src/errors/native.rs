// Copyright Cryptape Technologies LLC.
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

use cita_vm::evm::Error as EVMError;
use cita_vm::Error as VMError;
use std::fmt;

#[derive(Debug)]
pub enum NativeError {
    Internal(String),
}

impl Into<VMError> for NativeError {
    fn into(self) -> VMError {
        match self {
            NativeError::Internal(str) => VMError::Evm(EVMError::Internal(str)),
        }
    }
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            NativeError::Internal(str) => format!("Internal error {:?}", str),
        };
        write!(f, "{}", printable)
    }
}
