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

use ed25519::PrivKey;
use util::Address;

/// Authority params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct AuthorityRoundParams {
    /// Block duration.
    pub duration: u64,
    /// Valid authorities
    pub authorities: Vec<Address>,
    pub signer: PrivKey,
}

/// Authority engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct AuthorityRound {
    pub params: AuthorityRoundParams,
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use serde_json;

    #[test]
    fn poa_params_deserialization() {
        let s = r#"{
            "duration": 3,
            "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
            "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65"
        }"#;

        let _deserialize: AuthorityRoundParams = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn poa_deserialization() {
        let s = r#"{
            "params": {
                "duration": 3,
                "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
                "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65"
            }
        }"#;

        let _deserialize: AuthorityRound = serde_json::from_str(s).unwrap();
    }
}
