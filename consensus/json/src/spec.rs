use std::io::Read;
use serde_json;
use serde_json::Error;
use super::Engine;

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
        where R: Read
    {
        serde_json::from_reader(reader)
    }
}


#[cfg(test)]
mod tests {
    use serde_json;
    use super::Spec;

    #[test]
    fn poa_spec_deserialization() {
        let s = r#"{
            "name": "TestTendermint",
            "engine": {
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
            }
        }"#;
        let _deserialized: Spec = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn tendermint_spec_deserialization() {
        let s = r#"{
            "name": "TestTendermint",
            "engine": {
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
            }
        }"#;
        let _deserialized: Spec = serde_json::from_str(s).unwrap();
    }
}
