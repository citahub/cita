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
