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

use crypto::PrivKey;

/// Authority params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct TendermintParams {
    /// Block duration.
    pub duration: u64,
    pub is_test: bool,
    pub signer: PrivKey,

    #[serde(rename = "timeoutPropose")] pub timeout_propose: Option<u64>,
    /// Prevote step timeout in milliseconds.
    #[serde(rename = "timeoutPrevote")]
    pub timeout_prevote: Option<u64>,
    /// Precommit step timeout in milliseconds.
    #[serde(rename = "timeoutPrecommit")]
    pub timeout_precommit: Option<u64>,
    /// Commit step timeout in milliseconds.
    #[serde(rename = "timeoutCommit")]
    pub timeout_commit: Option<u64>,
}

/// Authority engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Tendermint {
    pub params: TendermintParams,
}

#[cfg(test)]
mod tests {
    extern crate cita_crypto as crypto;

    use super::*;
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
    fn tendermint_params_deserialization() {
        let signer = generate_signer();
        let s = format!(
            r#"{{
                "duration": 3,
                "signer": "{}",
                "is_test": true
            }}"#,
            signer
        );

        let _deserialize: TendermintParams = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn tendermint_deserialization() {
        let signer = generate_signer();
        let s = format!(
            r#"{{
                "params": {{
                    "duration": 3,
                    "signer": "{}",
                    "is_test": true
                }}
            }}"#,
            signer
        );

        let _deserialize: Tendermint = serde_json::from_str(&s).unwrap();
    }
}
