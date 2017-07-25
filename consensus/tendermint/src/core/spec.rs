use super::{Engine, Tendermint};
use std::sync::Arc;
use engine_json::{Engine as EngineJson, Spec as SpecJson};
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender, Receiver};
use serde_types::hash::{H256};

pub struct Spec {
    pub name: String,
    pub engine: Arc<Engine>,
    pub rx: Receiver<(usize,H256)>,
}

impl From<SpecJson> for Spec {
    fn from(s: SpecJson) -> Self {
        let (tx, rx) = channel();
        Spec {
            name: s.name.clone().into(),
            engine: Spec::engine(s.engine, tx),
            rx: rx,
        }
    }
}

impl Spec {
    fn engine(engine_json: EngineJson, tx: Sender<(usize,H256)>) -> Arc<Engine> {
        match engine_json {
            EngineJson::Tendermint(tendermint) => {
                Tendermint::new(From::from(tendermint.params), tx)
                    .expect("Failed to start Tendermint consensus engine.")
            }
            _ => panic!("Failed to start Tendermint consensus engine."),
        }
    }

    pub fn load<R>(reader: R) -> Result<Self, String>
        where R: Read
    {
        match SpecJson::load(reader) {
            Ok(spec) => Ok(spec.into()),
            _ => Err("Spec json is invalid".into()),
        }
    }

    pub fn new_test_tendermint(path: &str) -> Self {
        let config_file = File::open(path).unwrap();
        let fconfig = BufReader::new(config_file);
        Spec::load(fconfig).expect(concat!("spec is invalid."))
    }
}

#[cfg(test)]
mod tests {
    extern crate dotenv;

    use super::Spec;

    #[test]
    fn has_valid_metadata() {
        dotenv::dotenv().ok();
        let test_spec = ::std::env::current_dir()
            .unwrap()
            .join("../res/tendermint.json");
        println!("{}", test_spec.display());
        let engine = Spec::new_test_tendermint(test_spec.to_str().unwrap()).engine;
        assert!(!engine.name().is_empty());
        assert!(engine.version().major >= 1);
    }
}
