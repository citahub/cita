// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use cita_ed25519::{Message as ED_Message, Signature as ED_Signature};
use cita_secp256k1::Signature;
use cita_types::{H256, U256};
use crypto::digest::Digest;
use crypto::ripemd160::Ripemd160 as Ripemd160Digest;
use crypto::sha2::Sha256 as Sha256Digest;
use spec;
use std::cmp::min;
use util::crypto::Sign;
use util::{BytesRef, Hashable};

/// Native implementation of a built-in contract.
pub trait Impl: Send + Sync {
    /// execute this built-in on the given input, writing to the given output.
    fn execute(&self, input: &[u8], output: &mut BytesRef);
}

/// A gas pricing scheme for built-in contracts.
pub trait Pricer: Send + Sync {
    /// The gas cost of running this built-in for the given size of input data.
    fn cost(&self, input: &[u8]) -> U256;
}

/// A linear pricing model. This computes a price using a base cost and a cost per-word.
struct Linear {
    base: usize,
    word: usize,
}

impl Pricer for Linear {
    fn cost(&self, input: &[u8]) -> U256 {
        U256::from(self.base) + U256::from(self.word) * U256::from((input.len() + 31) / 32)
    }
}

/// Pricing scheme and execution definition for a built-in contract.
pub struct Builtin {
    pricer: Box<Pricer>,
    native: Box<Impl>,
    activate_at: u64,
}

impl Builtin {
    /// Simple forwarder for cost.
    pub fn cost(&self, input: &[u8]) -> U256 {
        self.pricer.cost(input)
    }

    /// Simple forwarder for execute.
    pub fn execute(&self, input: &[u8], output: &mut BytesRef) {
        self.native.execute(input, output)
    }

    /// Whether the builtin is activated at the given block number.
    pub fn is_active(&self, at: u64) -> bool {
        at >= self.activate_at
    }
}

impl From<spec::Builtin> for Builtin {
    fn from(b: spec::Builtin) -> Self {
        let pricer = match b.pricing {
            spec::Pricing::Linear(linear) => Box::new(Linear {
                base: linear.base,
                word: linear.word,
            }),
        };

        Builtin {
            pricer,
            native: ethereum_builtin(&b.name),
            activate_at: b.activate_at.unwrap_or(0),
        }
    }
}

// Ethereum builtin creator.
fn ethereum_builtin(name: &str) -> Box<Impl> {
    match name {
        "identity" => Box::new(Identity) as Box<Impl>,
        "ecrecover" => Box::new(EcRecover) as Box<Impl>,
        "sha256" => Box::new(Sha256) as Box<Impl>,
        "ripemd160" => Box::new(Ripemd160) as Box<Impl>,
        "edrecover" => Box::new(EdRecover) as Box<Impl>,
        _ => panic!("invalid builtin name: {}", name),
    }
}

// Ethereum builtins:
//
// - The identity function
// - ec recovery
// - sha256
// - ripemd160

#[derive(Debug)]
struct Identity;

#[derive(Debug)]
struct EcRecover;

#[derive(Debug)]
struct Sha256;

#[derive(Debug)]
struct Ripemd160;

#[derive(Debug)]
struct EdRecover;

impl Impl for Identity {
    fn execute(&self, input: &[u8], output: &mut BytesRef) {
        output.write(0, input);
    }
}

impl Impl for EcRecover {
    fn execute(&self, i: &[u8], output: &mut BytesRef) {
        let len = min(i.len(), 128);

        let mut input = [0; 128];
        input[..len].copy_from_slice(&i[..len]);

        let hash = H256::from_slice(&input[0..32]);
        let v = H256::from_slice(&input[32..64]);
        let r = H256::from_slice(&input[64..96]);
        let s = H256::from_slice(&input[96..128]);

        let bit = match v[31] {
            27 | 28 if v.0[..31] == [0; 31] => v[31] - 27,
            _ => return,
        };

        let s = Signature::from_rsv(&r, &s, bit);
        if s.is_valid() {
            if let Ok(p) = s.recover(&hash) {
                let r = p.crypt_hash();
                output.write(0, &[0; 12]);
                output.write(12, &r[12..r.len()]);
            }
        }
    }
}

impl Impl for Sha256 {
    fn execute(&self, input: &[u8], output: &mut BytesRef) {
        let mut sha = Sha256Digest::new();
        sha.input(input);

        let mut out = [0; 32];
        sha.result(&mut out);

        output.write(0, &out);
    }
}

impl Impl for Ripemd160 {
    fn execute(&self, input: &[u8], output: &mut BytesRef) {
        let mut sha = Ripemd160Digest::new();
        sha.input(input);

        let mut out = [0; 32];
        sha.result(&mut out[12..32]);

        output.write(0, &out);
    }
}

impl Impl for EdRecover {
    fn execute(&self, i: &[u8], output: &mut BytesRef) {
        let len = min(i.len(), 128);

        let mut input = [0; 128];
        input[..len].copy_from_slice(&i[..len]);

        let hash = ED_Message::from_slice(&input[0..32]);
        let sig = ED_Signature::from(&input[32..128]);

        if let Ok(p) = sig.recover(&hash) {
            let r = p.crypt_hash();
            output.write(0, &[0; 12]);
            output.write(12, &r[12..r.len()]);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize;

    use super::{ethereum_builtin, Builtin, Linear, Pricer};
    use cita_ed25519::{pubkey_to_address as ED_pubkey_to_address, KeyPair, Signature};
    use cita_types::{H256, U256};
    use spec;
    use util::crypto::{CreateKey, Sign};
    use util::BytesRef;

    #[test]
    fn identity() {
        let f = ethereum_builtin("identity");

        let i = [0u8, 1, 2, 3];

        let mut o2 = [255u8; 2];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o2[..]));
        assert_eq!(i[0..2], o2);

        let mut o4 = [255u8; 4];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o4[..]));
        assert_eq!(i, o4);

        let mut o8 = [255u8; 8];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o8[..]));
        assert_eq!(i, o8[..4]);
        assert_eq!([255u8; 4], o8[4..]);
    }

    #[test]
    fn sha256() {
        use self::rustc_serialize::hex::FromHex;
        let f = ethereum_builtin("sha256");

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

        let mut o8 = [255u8; 8];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o8[..]));
        assert_eq!(
            &o8[..],
            &(FromHex::from_hex("e3b0c44298fc1c14").unwrap())[..]
        );

        let mut o34 = [255u8; 34];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o34[..]));
        assert_eq!(
            &o34[..],
            &(FromHex::from_hex(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855ffff"
            )
            .unwrap())[..]
        );

        let mut ov = vec![];
        f.execute(&i[..], &mut BytesRef::Flexible(&mut ov));
        assert_eq!(
            &ov[..],
            &(FromHex::from_hex(
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            )
            .unwrap())[..]
        );
    }

    #[test]
    fn ripemd160() {
        use self::rustc_serialize::hex::FromHex;
        let f = ethereum_builtin("ripemd160");

        let i = [0u8; 0];

        let mut o = [255u8; 32];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "0000000000000000000000009c1185a5c5e9fc54612808977ee8f548b2258d31"
            )
            .unwrap())[..]
        );

        let mut o8 = [255u8; 8];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o8[..]));
        assert_eq!(
            &o8[..],
            &(FromHex::from_hex("0000000000000000").unwrap())[..]
        );

        let mut o34 = [255u8; 34];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o34[..]));
        assert_eq!(
            &o34[..],
            &(FromHex::from_hex(
                "0000000000000000000000009c1185a5c5e9fc54612808977ee8f548b2258d31ffff"
            )
            .unwrap())[..]
        );
    }

    #[test]
    fn ecrecover() {
        use self::rustc_serialize::hex::FromHex;
        /*let k = KeyPair::from_secret(b"test".crypt_hash()).unwrap();
        let a: Address = From::from(k.public().crypt_hash());
        println!("Address: {}", a);
        let m = b"hello world".crypt_hash();
        println!("Message: {}", m);
        let s = k.sign(&m).unwrap();
        println!("Signed: {}", s);*/

        let f = ethereum_builtin("ecrecover");

        let i = FromHex::from_hex("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001b650acf9d3f5f0a2c799776a1254355d5f4061762a237396a99a0e0e3fc2bcd6729514a0dacb2e623ac4abd157cb18163ff942280db4d5caad66ddf941ba12e03").unwrap();

        let mut o = [255u8; 32];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o[..]));
        #[cfg(feature = "sha3hash")]
        let expected = "000000000000000000000000c08b5542d177ac6686946920409741463a15dddb";
        #[cfg(feature = "blake2bhash")]
        let expected = "0000000000000000000000009f374781e8bf2e7dc910b0ee56baf9c2d475f1d9";
        #[cfg(feature = "sm3hash")]
        let expected = "000000000000000000000000040888ccac3826b7b6faf17cef6d6b6f861452c4";
        assert_eq!(&o[..], &(FromHex::from_hex(expected).unwrap())[..]);

        let mut o8 = [255u8; 8];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o8[..]));
        assert_eq!(
            &o8[..],
            &(FromHex::from_hex("0000000000000000").unwrap())[..]
        );

        let mut o34 = [255u8; 34];
        f.execute(&i[..], &mut BytesRef::Fixed(&mut o34[..]));
        #[cfg(feature = "sha3hash")]
        let expected = "000000000000000000000000c08b5542d177ac6686946920409741463a15dddbffff";
        #[cfg(feature = "blake2bhash")]
        let expected = "0000000000000000000000009f374781e8bf2e7dc910b0ee56baf9c2d475f1d9ffff";
        #[cfg(feature = "sm3hash")]
        let expected = "000000000000000000000000040888ccac3826b7b6faf17cef6d6b6f861452c4ffff";
        assert_eq!(&o34[..], &(FromHex::from_hex(expected).unwrap())[..]);

        let i_bad = FromHex::from_hex("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001a650acf9d3f5f0a2c799776a1254355d5f4061762a237396a99a0e0e3fc2bcd6729514a0dacb2e623ac4abd157cb18163ff942280db4d5caad66ddf941ba12e03").unwrap();
        let mut o = [255u8; 32];
        f.execute(&i_bad[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            )
            .unwrap())[..]
        );

        let i_bad = FromHex::from_hex("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001b000000000000000000000000000000000000000000000000000000000000001b0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let mut o = [255u8; 32];
        f.execute(&i_bad[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            )
            .unwrap())[..]
        );

        let i_bad = FromHex::from_hex("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001b").unwrap();
        let mut o = [255u8; 32];
        f.execute(&i_bad[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            )
            .unwrap())[..]
        );

        let i_bad = FromHex::from_hex("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001bffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000000000000000000000000000000000000000000000000000000001b").unwrap();
        let mut o = [255u8; 32];
        f.execute(&i_bad[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            )
            .unwrap())[..]
        );

        let i_bad = FromHex::from_hex("47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001b000000000000000000000000000000000000000000000000000000000000001bffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
        let mut o = [255u8; 32];
        f.execute(&i_bad[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(
            &o[..],
            &(FromHex::from_hex(
                "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
            )
            .unwrap())[..]
        );

        // TODO: Should this (corrupted version of the above) fail rather than returning some address?
        /*    let i_bad = FromHex::from_hex("48173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad000000000000000000000000000000000000000000000000000000000000001b650acf9d3f5f0a2c799776a1254355d5f4061762a237396a99a0e0e3fc2bcd6729514a0dacb2e623ac4abd157cb18163ff942280db4d5caad66ddf941ba12e03").unwrap();
        let mut o = [255u8; 32];
        f.execute(&i_bad[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(&o[..], &(FromHex::from_hex("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap())[..]);*/    }

    #[test]
    fn edrecover() {
        let key_pair = KeyPair::gen_keypair();
        let message: [u8; 32] = [
            0x01, 0x02, 0x03, 0x04, 0x19, 0xab, 0xfe, 0x39, 0x6f, 0x28, 0x79, 0x00, 0x08, 0xdf,
            0x9a, 0xef, 0xfb, 0x77, 0x42, 0xae, 0xad, 0xfc, 0xcf, 0x12, 0x24, 0x45, 0x29, 0x89,
            0x29, 0x45, 0x3f, 0xf8,
        ];
        let hash = H256::from(message);
        let privkey = key_pair.privkey();
        let pubkey = key_pair.pubkey();
        let address = ED_pubkey_to_address(pubkey);
        let signature = Signature::sign(privkey, &hash).unwrap();
        let mut buf = Vec::<u8>::with_capacity(128);
        buf.extend_from_slice(&message[..]);
        buf.extend_from_slice(&signature.0[..]);

        let f = ethereum_builtin("edrecover");
        let mut output = [255u8; 32];
        f.execute(&buf, &mut BytesRef::Fixed(&mut output[..]));

        assert_eq!(&output[0..12], &[0u8; 12]);
        assert_eq!(&output[12..], &address.0[..]);
    }

    #[test]
    #[should_panic]
    fn from_unknown_linear() {
        let _ = ethereum_builtin("foo");
    }

    #[test]
    fn from_named_linear() {
        let pricer = Box::new(Linear { base: 10, word: 20 });
        let b = Builtin {
            pricer: pricer as Box<Pricer>,
            native: ethereum_builtin("identity"),
            activate_at: 0,
        };

        assert_eq!(b.cost(&[0; 0]), U256::from(10));
        assert_eq!(b.cost(&[0; 1]), U256::from(30));
        assert_eq!(b.cost(&[0; 32]), U256::from(30));
        assert_eq!(b.cost(&[0; 33]), U256::from(50));

        let i = [0u8, 1, 2, 3];
        let mut o = [255u8; 4];
        b.execute(&i[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(i, o);
    }

    #[test]
    fn from_json() {
        let b = Builtin::from(spec::Builtin {
            name: "identity".to_owned(),
            pricing: spec::Pricing::Linear(spec::Linear { base: 10, word: 20 }),
            activate_at: Some(10000),
        });

        assert_eq!(b.cost(&[0; 0]), U256::from(10));
        assert_eq!(b.cost(&[0; 1]), U256::from(30));
        assert_eq!(b.cost(&[0; 32]), U256::from(30));
        assert_eq!(b.cost(&[0; 33]), U256::from(50));
        assert_eq!(b.activate_at, 10000);

        let i = [0u8, 1, 2, 3];
        let mut o = [255u8; 4];
        b.execute(&i[..], &mut BytesRef::Fixed(&mut o[..]));
        assert_eq!(i, o);
    }
}
