use crypto::PrivKey;
use serde_types::hash::Address;

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
    use serde_json;
    use super::super::*;

    #[test]
    fn poa_params_deserialization() {
        let s = r#"{
            "duration": 3,
            "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
            "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65"
        }"#;

        let _deserialize: AuthorityRoundParams = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn poa_deserialization() {
        let s = r#"{
            "params": {
                "duration": 3,
                "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
                "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65"
            }
        }"#;

        let _deserialize: AuthorityRound = serde_json::from_str(s).unwrap();
    }
}
