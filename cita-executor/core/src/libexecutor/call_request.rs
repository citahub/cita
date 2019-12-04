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

use cita_types::Address;
use libproto::request::Call;
use types::Bytes;

/// Call request
#[derive(Debug, Default, PartialEq)]
pub struct CallRequest {
    /// From
    pub from: Option<Address>,
    /// To
    pub to: Address,
    /// Data
    pub data: Option<Bytes>,
}

impl From<Call> for CallRequest {
    fn from(call: Call) -> Self {
        CallRequest {
            from: if call.get_from().is_empty() {
                None
            } else {
                Some(Address::from(call.get_from()))
            },
            to: Address::from(call.get_to()),
            data: if call.data.is_empty() {
                None
            } else {
                Some(call.data)
            },
        }
    }
}

#[cfg(test)]
mod tests {}
