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

use core::params::TendermintParams;
use engine_json::{Engine as EngineJson, Spec as SpecJson};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Spec {
    pub name: String,
    pub params: TendermintParams,
}

impl From<SpecJson> for Spec {
    fn from(s: SpecJson) -> Self {
        Spec {
            name: s.name.clone(),
            params: Spec::params(s.engine),
        }
    }
}

impl Spec {
    fn params(engine_json: EngineJson) -> TendermintParams {
        match engine_json {
            EngineJson::Tendermint(tendermint) => From::from(tendermint.params),
            _ => panic!("Failed to start Tendermint consensus engine."),
        }
    }

    pub fn load<R>(reader: R) -> Result<Self, String>
    where
        R: Read,
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
    extern crate cita_crypto as crypto;

    use super::Spec;
    use crypto::SIGNATURE_NAME;

    #[test]
    fn has_valid_metadata() {
        let config_path = if SIGNATURE_NAME == "ed25519" {
            "../res/tendermint.json".to_string()
        } else if SIGNATURE_NAME == "secp256k1" {
            "../res/tendermint_secp256k1.json".to_string()
        } else {
            "not exist".to_string()
        };
        let test_spec = ::std::env::current_dir().unwrap().join(&config_path);
        println!("{}", test_spec.display());
        let spec = Spec::new_test_tendermint(test_spec.to_str().unwrap());
        assert_eq!(spec.name, "TestTendermint");
    }
}
