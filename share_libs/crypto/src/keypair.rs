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

use super::{PrivKey, PubKey, Address, SECP256K1, Error};
use rustc_serialize::hex::ToHex;
use secp256k1::key;
use std::fmt;
use util::H160 as Hash160;
use util::Hashable;

pub fn pubkey_to_address(pubkey: &PubKey) -> Address {
    Address::from(Hash160::from(pubkey.crypt_hash()))
}

/// key pair
#[derive(Default)]
pub struct KeyPair {
    privkey: PrivKey,
    pubkey: PubKey,
}

impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "privkey:  {}", self.privkey.0.to_hex())?;
        writeln!(f, "pubkey:  {}", self.pubkey.0.to_hex())?;
        write!(f, "address:  {}", self.address().0.to_hex())
    }
}

impl KeyPair {
    /// Create a pair from secret key
    pub fn from_privkey(privkey: PrivKey) -> Result<KeyPair, Error> {
        let context = &SECP256K1;
        let s: key::SecretKey = key::SecretKey::from_slice(context, &privkey.0[..])?;
        let pubkey = key::PublicKey::from_secret_key(context, &s)?;
        let serialized = pubkey.serialize_vec(context, false);

        let mut pubkey = PubKey::default();
        pubkey.0.copy_from_slice(&serialized[1..65]);

        let keypair = KeyPair { privkey: privkey, pubkey: pubkey };

        Ok(keypair)
    }

    pub fn from_keypair(sec: key::SecretKey, publ: key::PublicKey) -> Self {
        let context = &SECP256K1;
        let serialized = publ.serialize_vec(context, false);
        let mut privkey = PrivKey::default();
        privkey.0.copy_from_slice(&sec[0..32]);
        let mut pubkey = PubKey::default();
        pubkey.0.copy_from_slice(&serialized[1..65]);

        KeyPair { privkey: privkey, pubkey: pubkey }
    }

    pub fn privkey(&self) -> &PrivKey {
        &self.privkey
    }

    pub fn pubkey(&self) -> &PubKey {
        &self.pubkey
    }

    pub fn address(&self) -> Address {
        pubkey_to_address(&self.pubkey)
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyPair, PrivKey};
    use std::str::FromStr;
    use util::H256 as Hash256;

    #[test]
    fn from_privkey() {
        let privkey = PrivKey::from(Hash256::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap());
        let _ = KeyPair::from_privkey(privkey).unwrap();
    }

    #[test]
    fn keypair_display() {
        let expected = "privkey:  a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65\npubkey:  8ce0db0b0359ffc5866ba61903cc2518c3675ef2cf380a7e54bde7ea20e6fa1ab45b7617346cd11b7610001ee6ae5b0155c41cad9527cbcdff44ec67848943a4\naddress:  5b073e9233944b5e729e46d618f0d8edf3d9c34a".to_owned();
        let privkey = PrivKey::from(Hash256::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap());
        let kp = KeyPair::from_privkey(privkey).unwrap();
        assert_eq!(format!("{}", kp), expected);
    }
}
