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
    use serde_json;
    use super::super::Engine;

    #[test]
    fn poa_engine_deserialization() {
        let s = r#"{
            "AuthorityRound": {
                "params": {
                    "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
                    "duration": 3,
                    "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65",
		    "block_tx_limit": 300,
                    "tx_filter_size": 100000,
                    "is_test": true
                }
            }
        }"#;

        let _deserialized: Engine = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn tendermint_engine_deserialization() {
        let s = r#"{
            "Tendermint": {
                "params": {
                    "authorities" : ["0x5b073e9233944b5e729e46d618f0d8edf3d9c34a"],
                    "duration": 3,
                    "signer": "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65",
		    "block_tx_limit": 300,
                    "tx_filter_size": 100000,	
                    "is_test": true
                }
            }
        }"#;

        let _deserialized: Engine = serde_json::from_str(s).unwrap();
    }
}
