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

use super::Engine;
use serde_json;
use serde_json::Error;
use std::io::Read;

/// Spec deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Spec {
    /// User friendly spec name
    pub name: String,
    /// Engine.
    pub engine: Engine,
}


impl Spec {
    /// Loads test from json.
    pub fn load<R>(reader: R) -> Result<Self, Error>
    where
        R: Read,
    {
        serde_json::from_reader(reader)
    }
}


#[cfg(test)]
mod tests {
    extern crate cita_crypto as crypto;

    use super::Spec;
    use crypto::SIGNATURE_NAME;
    use serde_json;

    fn generate_signer() -> String {
        if SIGNATURE_NAME == "ed25519" {
            "a100df7a048e50ed308ea696dc6002150981\
             41cb391e9527329df289f9383f65a100df7a0\
             48e50ed308ea696dc600215098141cb391e95\
             27329df289f9383f65"
                .to_string()
        } else if SIGNATURE_NAME == "secp256k1" {
            "a100df7a048e50ed308ea696dc6002150981\
             41cb391e9527329df289f9383f65"
                .to_string()
        } else {
            "".to_string()
        }
    }

    #[test]
    fn poa_spec_deserialization() {
        let signer = generate_signer();
        let s = format!(
            r#"{{
                "name": "TestPOA",
                "engine": {{
                    "AuthorityRound": {{
                        "params": {{
                            "authorities": [
                                "0x5b073e9233944b5e729e46d618f0d8edf3d9c34a",
                                "0x9cce34f7ab185c7aba1b7c8140d620b4bda941d6"
                            ],
                            "duration": 3000,
                            "signer": "{}"
                        }}
                    }}
                }}
            }}"#,
            signer
        );
        let _deserialized: Spec = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn tendermint_spec_deserialization() {
        let signer = generate_signer();
        let s = format!(
            r#"{{
                "name": "TestTendermint",
                "engine": {{
                    "Tendermint": {{
                        "params": {{
                            "duration": 3,
                            "signer": "{}",
                            "is_test": true
                        }}
                    }}
                }}
            }}"#,
            signer
        );
        let _deserialized: Spec = serde_json::from_str(&s).unwrap();
    }
}
