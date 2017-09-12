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

use bytes::Bytes;
use util::Address;

/// Call request
#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct CallRequest {
    /// From
    pub from: Option<Address>,
    /// To
    pub to: Address,
    /// Data
    pub data: Option<Bytes>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use serde_json::Error;

    #[test]
    fn call_deserialization() {
        let s = r#"{"from":"0xd46e8dd67c5d32be8058bb8eb970870f07244567", "to":"0xb60e8dd61c5d32be8058bb8eb970870f07233155", "data":"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"}"#;
        let call: Result<CallRequest, Error> = serde_json::from_str(s);
        println!("call1 = {:?}", call);
        assert!(call.is_ok());

        let s = r#"{"from":"d46e8dd67c5d32be8058bb8eb970870f07244567", "to":"b60e8dd61c5d32be8058bb8eb970870f07233155", "data":"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"}"#;
        let call: Result<CallRequest, Error> = serde_json::from_str(s);
        println!("call2 = {:?}", call);

        assert!(call.is_ok());
    }
}
