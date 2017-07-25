use crypto::PrivKey;
use serde_types::hash::Address;

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

    #[serde(rename="timeoutPropose")]
    pub timeout_propose: Option<u64>,
    /// Prevote step timeout in milliseconds.
    #[serde(rename="timeoutPrevote")]
    pub timeout_prevote: Option<u64>,
    /// Precommit step timeout in milliseconds.
    #[serde(rename="timeoutPrecommit")]
    pub timeout_precommit: Option<u64>,
    /// Commit step timeout in milliseconds.
    #[serde(rename="timeoutCommit")]
    pub timeout_commit: Option<u64>,
}

/// Authority engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Tendermint {
    pub params: TendermintParams,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use super::*;

    #[test]
    fn tendermint_params_deserialization() {
        let s = r#"{
            "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
            "duration": 3,
            "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65",
            "block_tx_limit": 1000,
            "tx_filter_size": 5000,
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
                "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65",
                "block_tx_limit": 1000,
                "tx_filter_size": 5000,
                "is_test": true
            }
        }"#;

        let _deserialize: Tendermint = serde_json::from_str(s).unwrap();
    }
}
