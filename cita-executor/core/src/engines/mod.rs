// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use builtin::Builtin;
use cita_types::Address;
use header::BlockNumber;
use serde_json;
use spec::Builtin as SpecBuiltin;
use std::collections::BTreeMap;
use std::str::FromStr;
use types::reserved_addresses;

mod null_engine;
pub use self::null_engine::NullEngine;

pub trait Engine: Sync + Send {
    /// The name of this engine.
    fn name(&self) -> &str;

    /// Builtin-contracts we would like to see in the chain.
    /// (In principle these are just hints for the engine since that has the last word on them.)
    fn builtins(&self) -> &BTreeMap<Address, Builtin>;

    /// Attempt to get a handle to a built-in contract.
    /// Only returns references to activated built-ins.
    fn builtin(&self, a: &Address, block_number: BlockNumber) -> Option<&Builtin> {
        self.builtins().get(a).and_then(|b| {
            if b.is_active(block_number) {
                Some(b)
            } else {
                None
            }
        })
    }
}

impl NullEngine {
    // TODO: read from spec file
    pub fn cita() -> Self {
        let mut builtins = BTreeMap::new();

        let s = r#"{ "name": "ecrecover", "pricing": { "linear": { "base": 3000, "word": 0 } } }"#;
        let deserialized: SpecBuiltin = serde_json::from_str(s).unwrap();

        builtins.insert(
            Address::from_str(reserved_addresses::ECRECOVER_ADDRESS).unwrap(),
            Builtin::from(deserialized),
        );

        let s = r#"{ "name": "sha256", "pricing": { "linear": { "base": 60, "word": 12 } } }"#;
        let deserialized: SpecBuiltin = serde_json::from_str(s).unwrap();
        builtins.insert(
            Address::from_str(reserved_addresses::SHA256_ADDRESS).unwrap(),
            Builtin::from(deserialized),
        );

        let s = r#"{ "name": "ripemd160", "pricing": { "linear": { "base": 600, "word": 120 } } }"#;
        let deserialized: SpecBuiltin = serde_json::from_str(s).unwrap();
        builtins.insert(
            Address::from_str(reserved_addresses::RIPEMD160_ADDRESS).unwrap(),
            Builtin::from(deserialized),
        );

        let s = r#"{ "name": "identity", "pricing": { "linear": { "base": 15, "word": 3 } } }"#;
        let deserialized: SpecBuiltin = serde_json::from_str(s).unwrap();
        builtins.insert(
            Address::from_str(reserved_addresses::IDENTITY_ADDRESS).unwrap(),
            Builtin::from(deserialized),
        );

        let s = r#"{ "name": "edrecover", "pricing": { "linear": { "base": 3000, "word": 0 } } }"#;
        let deserialized: SpecBuiltin = serde_json::from_str(s).unwrap();
        builtins.insert(
            Address::from_str(reserved_addresses::EDRECOVER_ADDRESS).unwrap(),
            Builtin::from(deserialized),
        );

        Self::new(builtins)
    }
}

#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    use self::rustc_serialize::hex::FromHex;
    use super::*;
    use util::BytesRef;

    #[test]
    fn test_cita_correct() {
        let cita = NullEngine::cita();
        let sha256 = cita.builtin(
            &Address::from(0x0000000000000000000000000000000000000002),
            0,
        );

        assert!(sha256.is_some());

        let f = sha256.unwrap();
        let i = [0u8; 0];
        let mut o = [255u8; 32];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            )
            .unwrap())[..]
        );
    }
}
