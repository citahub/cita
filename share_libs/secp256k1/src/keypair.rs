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

use super::{Address, Error, PrivKey, PubKey, SECP256K1};
use rand::thread_rng;
use rustc_serialize::hex::ToHex;
use secp256k1::key;
use std::fmt;
use util::H160 as Hash160;
use util::Hashable;
use util::crypto::CreateKey;

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

impl CreateKey for KeyPair {
    type PrivKey = PrivKey;
    type PubKey = PubKey;
    type Error = Error;

    /// Create a pair from secret key
    fn from_privkey(privkey: Self::PrivKey) -> Result<Self, Self::Error> {
        let context = &SECP256K1;
        let s: key::SecretKey = key::SecretKey::from_slice(context, &privkey.0[..])?;
        let pubkey = key::PublicKey::from_secret_key(context, &s)?;
        let serialized = pubkey.serialize_vec(context, false);

        let mut pubkey = PubKey::default();
        pubkey.0.copy_from_slice(&serialized[1..65]);

        let keypair = KeyPair {
            privkey: privkey,
            pubkey: pubkey,
        };

        Ok(keypair)
    }

    fn gen_keypair() -> Self {
        let context = &SECP256K1;
        let (s, p) = context.generate_keypair(&mut thread_rng()).unwrap();
        let serialized = p.serialize_vec(context, false);
        let mut privkey = PrivKey::default();
        privkey.0.copy_from_slice(&s[0..32]);
        let mut pubkey = PubKey::default();
        pubkey.0.copy_from_slice(&serialized[1..65]);
        KeyPair {
            privkey: privkey,
            pubkey: pubkey,
        }
    }

    fn privkey(&self) -> &Self::PrivKey {
        &self.privkey
    }

    fn pubkey(&self) -> &Self::PubKey {
        &self.pubkey
    }

    fn address(&self) -> Address {
        pubkey_to_address(&self.pubkey)
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyPair, PrivKey};
    use std::str::FromStr;
    use util::H256 as Hash256;
    use util::crypto::CreateKey;

    #[test]
    fn from_privkey() {
        let privkey = PrivKey::from(
            Hash256::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap(),
        );
        let _ = KeyPair::from_privkey(privkey).unwrap();
    }
}
