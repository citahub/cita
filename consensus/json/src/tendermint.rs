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
pub struct TendermintParams {
    /// Block duration.
    pub duration: u64,
    pub is_test: bool,
    /// Valid authorities
    pub authorities: Vec<Address>,
    pub signer: PrivKey,
    pub block_tx_limit: u64,
    pub tx_filter_size: u64,
    pub tx_pool_size: u64,

    #[serde(rename = "timeoutPropose")]
    pub timeout_propose: Option<u64>,
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
    use super::*;
    use serde_json;

    #[test]
    fn tendermint_params_deserialization() {
        let s = r#"{
            "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
            "duration": 3,
            "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65",
            "block_tx_limit": 1000,
            "tx_filter_size": 5000,
            "tx_pool_size": 50000,
            "is_test": true
        }"#;

        let _deserialize: TendermintParams = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn tendermint_deserialization() {
        let s = r#"{
            "params": {
                "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
                "duration": 3,
                "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65",
                "block_tx_limit": 1000,
                "tx_filter_size": 5000,
                "tx_pool_size": 50000,
                "is_test": true
            }
        }"#;

        let _deserialize: Tendermint = serde_json::from_str(s).unwrap();
    }
}
