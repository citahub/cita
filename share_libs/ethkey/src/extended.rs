// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Extended keys

pub use self::derivation::Error as DerivationError;
use Public;
use bigint::hash::H256;
use secret::Secret;

/// Represents label that can be stored as a part of key derivation
pub trait Label {
    /// Length of the data that label occupies
    fn len() -> usize;

    /// Store label data to the key derivation sequence
    /// Must not use more than `len()` bytes from slice
    fn store(&self, target: &mut [u8]);
}

impl Label for u32 {
    fn len() -> usize {
        4
    }

    fn store(&self, target: &mut [u8]) {
        use byteorder::{BigEndian, ByteOrder};

        BigEndian::write_u32(&mut target[0..4], *self);
    }
}

/// Key derivation over generic label `T`
pub enum Derivation<T: Label> {
    /// Soft key derivation (allow proof of parent)
    Soft(T),
    /// Hard key derivation (does not allow proof of parent)
    Hard(T),
}

impl From<u32> for Derivation<u32> {
    fn from(index: u32) -> Self {
        if index < (2 << 30) { Derivation::Soft(index) } else { Derivation::Hard(index) }
    }
}

impl Label for H256 {
    fn len() -> usize {
        32
    }

    fn store(&self, target: &mut [u8]) {
        self.copy_to(&mut target[0..32]);
    }
}

/// Extended secret key, allows deterministic derivation of subsequent keys.
pub struct ExtendedSecret {
    secret: Secret,
    chain_code: H256,
}

impl ExtendedSecret {
    /// New extended key from given secret and chain code.
    pub fn with_code(secret: Secret, chain_code: H256) -> ExtendedSecret {
        ExtendedSecret {
            secret: secret,
            chain_code: chain_code,
        }
    }

    /// New extended key from given secret with the random chain code.
    pub fn new_random(secret: Secret) -> ExtendedSecret {
        ExtendedSecret::with_code(secret, H256::random())
    }

    /// New extended key from given secret.
    /// Chain code will be derived from the secret itself (in a deterministic way).
    pub fn new(secret: Secret) -> ExtendedSecret {
        let chain_code = derivation::chain_code(*secret);
        ExtendedSecret::with_code(secret, chain_code)
    }

    /// Derive new private key
    pub fn derive<T>(&self, index: Derivation<T>) -> ExtendedSecret
    where
        T: Label,
    {
        let (derived_key, next_chain_code) = derivation::private(*self.secret, self.chain_code, index);

        let derived_secret = Secret::from_slice(&*derived_key);

        ExtendedSecret::with_code(derived_secret, next_chain_code)
    }

    /// Private key component of the extended key.
    pub fn as_raw(&self) -> &Secret {
        &self.secret
    }
}

/// Extended public key, allows deterministic derivation of subsequent keys.
pub struct ExtendedPublic {
    public: Public,
    chain_code: H256,
}

impl ExtendedPublic {
    /// New extended public key from known parent and chain code
    pub fn new(public: Public, chain_code: H256) -> Self {
        ExtendedPublic {
            public: public,
            chain_code: chain_code,
        }
    }

    /// Create new extended public key from known secret
    pub fn from_secret(secret: &ExtendedSecret) -> Result<Self, DerivationError> {
        Ok(ExtendedPublic::new(derivation::point(**secret.as_raw())?, secret.chain_code.clone()))
    }

    /// Derive new public key
    /// Operation is defined only for index belongs [0..2^31)
    pub fn derive<T>(&self, index: Derivation<T>) -> Result<Self, DerivationError>
    where
        T: Label,
    {
        let (derived_key, next_chain_code) = derivation::public(self.public, self.chain_code, index)?;
        Ok(ExtendedPublic::new(derived_key, next_chain_code))
    }

    pub fn public(&self) -> &Public {
        &self.public
    }
}

pub struct ExtendedKeyPair {
    secret: ExtendedSecret,
    public: ExtendedPublic,
}

impl ExtendedKeyPair {
    pub fn new(secret: Secret) -> Self {
        let extended_secret = ExtendedSecret::new(secret);
        let extended_public = ExtendedPublic::from_secret(&extended_secret).expect("Valid `Secret` always produces valid public; qed");
        ExtendedKeyPair {
            secret: extended_secret,
            public: extended_public,
        }
    }

    pub fn with_code(secret: Secret, public: Public, chain_code: H256) -> Self {
        ExtendedKeyPair {
            secret: ExtendedSecret::with_code(secret, chain_code.clone()),
            public: ExtendedPublic::new(public, chain_code),
        }
    }

    pub fn with_secret(secret: Secret, chain_code: H256) -> Self {
        let extended_secret = ExtendedSecret::with_code(secret, chain_code);
        let extended_public = ExtendedPublic::from_secret(&extended_secret).expect("Valid `Secret` always produces valid public; qed");
        ExtendedKeyPair {
            secret: extended_secret,
            public: extended_public,
        }
    }

    pub fn with_seed(seed: &[u8]) -> Result<ExtendedKeyPair, DerivationError> {
        let (master_key, chain_code) = derivation::seed_pair(seed);
        Ok(ExtendedKeyPair::with_secret(Secret::from_unsafe_slice(&*master_key).map_err(|_| DerivationError::InvalidSeed)?, chain_code))
    }

    pub fn secret(&self) -> &ExtendedSecret {
        &self.secret
    }

    pub fn public(&self) -> &ExtendedPublic {
        &self.public
    }

    pub fn derive<T>(&self, index: Derivation<T>) -> Result<Self, DerivationError>
    where
        T: Label,
    {
        let derived = self.secret.derive(index);

        Ok(ExtendedKeyPair {
               public: ExtendedPublic::from_secret(&derived)?,
               secret: derived,
           })
    }
}

// Derivation functions for private and public keys
// Work is based on BIP0032
// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
mod derivation {

    use super::{Label, Derivation};
    use SECP256K1;
    use bigint::{U256, U512};
    use bigint::hash::{H512, H256};
    use keccak;
    use math::curve_order;
    use rcrypto::hmac::Hmac;
    use rcrypto::mac::Mac;
    use rcrypto::sha2::Sha512;
    use secp256k1::key::{SecretKey, PublicKey};

    #[derive(Debug)]
    pub enum Error {
        InvalidHardenedUse,
        InvalidPoint,
        MissingIndex,
        InvalidSeed,
    }

    // Deterministic derivation of the key using secp256k1 elliptic curve.
    // Derivation can be either hardened or not.
    // For hardened derivation, pass u32 index at least 2^31 or custom Derivation::Hard(T) enum
    //
    // Can panic if passed `private_key` is not a valid secp256k1 private key
    // (outside of (0..curve_order()]) field
    pub fn private<T>(private_key: H256, chain_code: H256, index: Derivation<T>) -> (H256, H256)
    where
        T: Label,
    {
        match index {
            Derivation::Soft(index) => private_soft(private_key, chain_code, index),
            Derivation::Hard(index) => private_hard(private_key, chain_code, index),
        }
    }

    fn hmac_pair(data: &[u8], private_key: H256, chain_code: H256) -> (H256, H256) {
        let private: U256 = private_key.into();

        // produces 512-bit derived hmac (I)
        let mut hmac = Hmac::new(Sha512::new(), &*chain_code);
        let mut i_512 = [0u8; 64];
        hmac.input(&data[..]);
        hmac.raw_result(&mut i_512);

        // left most 256 bits are later added to original private key
        let hmac_key: U256 = H256::from_slice(&i_512[0..32]).into();
        // right most 256 bits are new chain code for later derivations
        let next_chain_code = H256::from(&i_512[32..64]);

        let child_key = private_add(hmac_key, private).into();
        (child_key, next_chain_code)
    }

    // Can panic if passed `private_key` is not a valid secp256k1 private key
    // (outside of (0..curve_order()]) field
    fn private_soft<T>(private_key: H256, chain_code: H256, index: T) -> (H256, H256)
    where
        T: Label,
    {
        let mut data = vec![0u8; 33 + T::len()];

        let sec_private = SecretKey::from_slice(&SECP256K1, &*private_key).expect("Caller should provide valid private key");
        let sec_public = PublicKey::from_secret_key(&SECP256K1, &sec_private).expect("Caller should provide valid private key");
        let public_serialized = sec_public.serialize_vec(&SECP256K1, true);

        // curve point (compressed public key) --  index
        //             0.33                    --  33..end
        data[0..33].copy_from_slice(&public_serialized);
        index.store(&mut data[33..]);

        hmac_pair(&data, private_key, chain_code)
    }

    // Deterministic derivation of the key using secp256k1 elliptic curve
    // This is hardened derivation and does not allow to associate
    // corresponding public keys of the original and derived private keys
    fn private_hard<T>(private_key: H256, chain_code: H256, index: T) -> (H256, H256)
    where
        T: Label,
    {
        let mut data: Vec<u8> = vec![0u8; 33 + T::len()];
        let private: U256 = private_key.into();

        // 0x00 (padding) -- private_key --  index
        //  0             --    1..33    -- 33..end
        private.to_big_endian(&mut data[1..33]);
        index.store(&mut data[33..(33 + T::len())]);

        hmac_pair(&data, private_key, chain_code)
    }

    fn private_add(k1: U256, k2: U256) -> U256 {
        let sum = U512::from(k1) + U512::from(k2);
        modulo(sum, curve_order())
    }

    // todo: surely can be optimized
    fn modulo(u1: U512, u2: U256) -> U256 {
        let dv = u1 / U512::from(u2);
        let md = u1 - (dv * U512::from(u2));
        md.into()
    }

    pub fn public<T>(public_key: H512, chain_code: H256, derivation: Derivation<T>) -> Result<(H512, H256), Error>
    where
        T: Label,
    {
        let index = match derivation {
            Derivation::Soft(index) => index,
            Derivation::Hard(_) => {
                return Err(Error::InvalidHardenedUse);
            }
        };

        let mut public_sec_raw = [0u8; 65];
        public_sec_raw[0] = 4;
        public_sec_raw[1..65].copy_from_slice(&*public_key);
        let public_sec = PublicKey::from_slice(&SECP256K1, &public_sec_raw).map_err(|_| Error::InvalidPoint)?;
        let public_serialized = public_sec.serialize_vec(&SECP256K1, true);

        let mut data = vec![0u8; 33 + T::len()];
        // curve point (compressed public key) --  index
        //             0.33                    --  33..end
        data[0..33].copy_from_slice(&public_serialized);
        index.store(&mut data[33..(33 + T::len())]);

        // HMAC512SHA produces [derived private(256); new chain code(256)]
        let mut hmac = Hmac::new(Sha512::new(), &*chain_code);
        let mut i_512 = [0u8; 64];
        hmac.input(&data[..]);
        hmac.raw_result(&mut i_512);

        let new_private = H256::from(&i_512[0..32]);
        let new_chain_code = H256::from(&i_512[32..64]);

        // Generated private key can (extremely rarely) be out of secp256k1 key field
        if curve_order() <= new_private.clone().into() {
            return Err(Error::MissingIndex);
        }
        let new_private_sec = SecretKey::from_slice(&SECP256K1, &*new_private).expect("Private key belongs to the field [0..CURVE_ORDER) (checked above); So initializing can never fail; qed");
        let mut new_public = PublicKey::from_secret_key(&SECP256K1, &new_private_sec).expect("Valid private key produces valid public key");

        // Adding two points on the elliptic curves (combining two public keys)
        new_public.add_assign(&SECP256K1, &public_sec)
                  .expect("Addition of two valid points produce valid point");

        let serialized = new_public.serialize_vec(&SECP256K1, false);

        Ok((H512::from(&serialized[1..65]), new_chain_code))
    }

    fn sha3(slc: &[u8]) -> H256 {
        keccak::Keccak256::keccak256(slc).into()
    }

    pub fn chain_code(secret: H256) -> H256 {
        // 10,000 rounds of sha3
        let mut running_sha3 = sha3(&*secret);
        for _ in 0..99999 {
            running_sha3 = sha3(&*running_sha3);
        }
        running_sha3
    }

    pub fn point(secret: H256) -> Result<H512, Error> {
        let sec = SecretKey::from_slice(&SECP256K1, &*secret).map_err(|_| Error::InvalidPoint)?;
        let public_sec = PublicKey::from_secret_key(&SECP256K1, &sec).map_err(|_| Error::InvalidPoint)?;
        let serialized = public_sec.serialize_vec(&SECP256K1, false);
        Ok(H512::from(&serialized[1..65]))
    }

    pub fn seed_pair(seed: &[u8]) -> (H256, H256) {
        let mut hmac = Hmac::new(Sha512::new(), b"Bitcoin seed");
        let mut i_512 = [0u8; 64];
        hmac.input(seed);
        hmac.raw_result(&mut i_512);

        let master_key = H256::from_slice(&i_512[0..32]);
        let chain_code = H256::from_slice(&i_512[32..64]);

        (master_key, chain_code)
    }
}

#[cfg(test)]
mod tests {
    use super::{derivation, Derivation};
    use super::{ExtendedSecret, ExtendedPublic, ExtendedKeyPair};
    use bigint::hash::{H128, H256};
    use secret::Secret;
    use std::str::FromStr;

    fn master_chain_basic() -> (H256, H256) {
        let seed = H128::from_str("000102030405060708090a0b0c0d0e0f").expect("Seed should be valid H128").to_vec();

        derivation::seed_pair(&*seed)
    }

    fn test_extended<F>(f: F, test_private: H256)
    where
        F: Fn(ExtendedSecret) -> ExtendedSecret,
    {
        let (private_seed, chain_code) = master_chain_basic();
        let extended_secret = ExtendedSecret::with_code(Secret::from_slice(&*private_seed), chain_code);
        let derived = f(extended_secret);
        assert_eq!(**derived.as_raw(), test_private);
    }

    #[test]
    fn smoky() {
        let secret = Secret::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap();
        let extended_secret = ExtendedSecret::with_code(secret.clone(), 0u64.into());

        // hardened
        assert_eq!(&**extended_secret.as_raw(), &*secret);
        assert_eq!(&**extended_secret.derive(2147483648.into()).as_raw(), &"0927453daed47839608e414a3738dfad10aed17c459bbd9ab53f89b026c834b6".into());
        assert_eq!(&**extended_secret.derive(2147483649.into()).as_raw(), &"44238b6a29c6dcbe9b401364141ba11e2198c289a5fed243a1c11af35c19dc0f".into());

        // normal
        assert_eq!(&**extended_secret.derive(0.into()).as_raw(), &"bf6a74e3f7b36fc4c96a1e12f31abc817f9f5904f5a8fc27713163d1f0b713f6".into());
        assert_eq!(&**extended_secret.derive(1.into()).as_raw(), &"bd4fca9eb1f9c201e9448c1eecd66e302d68d4d313ce895b8c134f512205c1bc".into());
        assert_eq!(&**extended_secret.derive(2.into()).as_raw(), &"86932b542d6cab4d9c65490c7ef502d89ecc0e2a5f4852157649e3251e2a3268".into());

        let extended_public = ExtendedPublic::from_secret(&extended_secret).expect("Extended public should be created");
        let derived_public = extended_public.derive(0.into()).expect("First derivation of public should succeed");
        assert_eq!(&*derived_public.public(), &"f7b3244c96688f92372bfd4def26dc4151529747bab9f188a4ad34e141d47bd66522ff048bc6f19a0a4429b04318b1a8796c000265b4fa200dae5f6dda92dd94".into());

        let keypair = ExtendedKeyPair::with_secret(Secret::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap(), 064.into());
        assert_eq!(&**keypair.derive(2147483648u32.into())
                             .expect("Derivation of keypair should succeed")
                             .secret()
                             .as_raw(),
                   &"edef54414c03196557cf73774bc97a645c9a1df2164ed34f0c2a78d1375a930c".into());
    }

    #[test]
    fn h256_soft_match() {
        let secret = Secret::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap();
        let derivation_secret = H256::from_str("51eaf04f9dbbc1417dc97e789edd0c37ecda88bac490434e367ea81b71b7b015").unwrap();

        let extended_secret = ExtendedSecret::with_code(secret.clone(), 0u64.into());
        let extended_public = ExtendedPublic::from_secret(&extended_secret).expect("Extended public should be created");

        let derived_secret0 = extended_secret.derive(Derivation::Soft(derivation_secret));
        let derived_public0 = extended_public.derive(Derivation::Soft(derivation_secret))
                                             .expect("First derivation of public should succeed");

        let public_from_secret0 = ExtendedPublic::from_secret(&derived_secret0).expect("Extended public should be created");

        assert_eq!(public_from_secret0.public(), derived_public0.public());
    }

    #[test]
    fn h256_hard() {
        let secret = Secret::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap();
        let derivation_secret = H256::from_str("51eaf04f9dbbc1417dc97e789edd0c37ecda88bac490434e367ea81b71b7b015").unwrap();
        let extended_secret = ExtendedSecret::with_code(secret.clone(), 1u64.into());

        assert_eq!(&**extended_secret.derive(Derivation::Hard(derivation_secret)).as_raw(), &"2bc2d696fb744d77ff813b4a1ef0ad64e1e5188b622c54ba917acc5ebc7c5486".into());
    }

    #[test]
    fn match_() {
        let secret = Secret::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap();
        let extended_secret = ExtendedSecret::with_code(secret.clone(), 1.into());
        let extended_public = ExtendedPublic::from_secret(&extended_secret).expect("Extended public should be created");

        let derived_secret0 = extended_secret.derive(0.into());
        let derived_public0 = extended_public.derive(0.into()).expect("First derivation of public should succeed");

        let public_from_secret0 = ExtendedPublic::from_secret(&derived_secret0).expect("Extended public should be created");

        assert_eq!(public_from_secret0.public(), derived_public0.public());
    }

    #[test]
    fn test_seeds() {
        let seed = H128::from_str("000102030405060708090a0b0c0d0e0f").expect("Seed should be valid H128").to_vec();

        // private key from bitcoin test vector
        // xprv9wTYmMFdV23N2TdNG573QoEsfRrWKQgWeibmLntzniatZvR9BmLnvSxqu53Kw1UmYPxLgboyZQaXwTCg8MSY3H2EU4pWcQDnRnrVA1xe8fs
        let test_private = H256::from_str("e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35").expect("Private should be decoded ok");

        let (private_seed, _) = derivation::seed_pair(&*seed);

        assert_eq!(private_seed, test_private);
    }

    #[test]
    fn test_vector_1() {
        test_extended(|secret| secret.derive(2147483648.into()), H256::from_str("edb2e14f9ee77d26dd93b4ecede8d16ed408ce149b6cd80b0715a2d911a0afea").expect("Private should be decoded ok"));
    }

    #[test]
    fn test_vector_2() {
        test_extended(|secret| secret.derive(2147483648.into()).derive(1.into()), H256::from_str("3c6cb8d0f6a264c91ea8b5030fadaa8e538b020f0a387421a12de9319dc93368").expect("Private should be decoded ok"));
    }
}
