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

//! Engine deserialization.

use super::AuthorityRound;
use super::Tendermint;


/// Engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub enum Engine {
    AuthorityRound(AuthorityRound),
    Tendermint(Tendermint),
}

#[cfg(test)]
mod tests {
    extern crate cita_crypto as crypto;

    use super::super::Engine;
    use crypto::SIGNATURE_NAME;
    use serde_json;

    fn generate_signer() -> String {
        if SIGNATURE_NAME == "ed25519" {
            "a100df7a048e50ed308ea696dc60021\
             5098141cb391e9527329df289f9383f6\
             5a100df7a048e50ed308ea696dc60021\
             5098141cb391e9527329df289f9383f65"
                .to_string()
        } else if SIGNATURE_NAME == "secp256k1" {
            "a100df7a048e50ed308ea696dc60021\
             5098141cb391e9527329df289f9383f65"
                .to_string()
        } else {
            "".to_string()
        }
    }

    #[test]
    fn poa_engine_deserialization() {
        let signer = generate_signer();
        let s = format!(
            r#"{{
                "AuthorityRound": {{
                    "params": {{
                        "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
                        "duration": 3,
                        "signer": "{}",
                        "is_test": true
                    }}
                }}
            }}"#,
            signer
        );

        let _deserialized: Engine = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn tendermint_engine_deserialization() {
        let signer = generate_signer();
        let s = format!(
            r#"{{
            "Tendermint": {{
                "params": {{
                    "duration": 3,
                    "signer": "{}",
                    "is_test": true
                }}
            }}
        }}"#,
            signer
        );

        let _deserialized: Engine = serde_json::from_str(&s).unwrap();
    }
}
