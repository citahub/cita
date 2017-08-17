// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use libproto::request::Call;
use util::{Bytes, Address};

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
            from: if call.get_from().is_empty() { None } else { Some(Address::from(call.get_from())) },
            to: Address::from(call.get_to()),
            data: if call.data.is_empty() { None } else { Some(Bytes::from(call.data)) },
        }
    }
}

#[cfg(test)]
mod tests {}
