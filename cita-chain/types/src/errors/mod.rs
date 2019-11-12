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

mod authentication;
mod call;
mod contract;
mod execution;
mod native;
mod receipt;

pub use authentication::AuthenticationError;
pub use call::CallError;
pub use contract::ContractError;
pub use execution::ExecutionError;
pub use native::NativeError;
pub use receipt::ReceiptError;

#[derive(Debug)]
pub enum Error {
    Execution(ExecutionError),
    Receipt(ReceiptError),
    Call(CallError),
    Native(NativeError),
    Authentication(AuthenticationError),
    Contract(ContractError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err = match self {
            Error::Execution(ref err) => format!("Execution error {:?}", err),
            Error::Receipt(ref err) => format!("Receipt error {:?}", err),
            Error::Call(ref err) => format!("Call error {:?}", err),
            Error::Native(ref err) => format!("Native error {:?}", err),
            Error::Authentication(ref err) => format!("Authentication error {:?}", err),
            Error::Contract(ref err) => format!("Contract error {:?}", err),
        };
        write!(f, "{}", err)
    }
}

impl From<ExecutionError> for Error {
    fn from(err: ExecutionError) -> Error {
        Error::Execution(err)
    }
}

impl From<ReceiptError> for Error {
    fn from(err: ReceiptError) -> Error {
        Error::Receipt(err)
    }
}
impl From<CallError> for Error {
    fn from(err: CallError) -> Error {
        Error::Call(err)
    }
}
impl From<NativeError> for Error {
    fn from(err: NativeError) -> Error {
        Error::Native(err)
    }
}
impl From<AuthenticationError> for Error {
    fn from(err: AuthenticationError) -> Error {
        Error::Authentication(err)
    }
}

impl From<ContractError> for Error {
    fn from(err: ContractError) -> Error {
        Error::Contract(err)
    }
}
