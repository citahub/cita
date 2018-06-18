// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use rpctypes::{Data, Data20};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct CallRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Data20>,
    pub to: Data20,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl CallRequest {
    pub fn new(from: Option<Data20>, to: Data20, data: Option<Data>) -> Self {
        CallRequest { from, to, data }
    }
}

#[cfg(test)]
mod tests {
    use super::CallRequest;
    use cita_types::H160;
    use serde_json;

    #[test]
    fn deserialize() {
        let testdata = vec![(
            json!({
                "from": "0x0000000000000000000000000000000000000001",
                "to": "0x0000000000000000000000000000000000000002",
                "data": "0xabcdef"
            }).to_string(),
            Some(CallRequest::new(
                Some(H160::from(1).into()),
                H160::from(2).into(),
                Some(vec![0xab, 0xcd, 0xef].into()),
            )),
        )];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<CallRequest, serde_json::Error> = serde_json::from_str(&data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
