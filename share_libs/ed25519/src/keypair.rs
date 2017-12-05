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

use super::{Address, PrivKey, PubKey};
use error::Error;
use rustc_serialize::hex::ToHex;
use sodiumoxide::crypto::sign::{gen_keypair, keypair_from_privkey};
use std::fmt;
use util::{H160, Hashable};
use util::crypto::CreateKey;

pub fn pubkey_to_address(pubkey: &PubKey) -> Address {
    Address::from(H160::from(pubkey.crypt_hash()))
}

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

    fn from_privkey(privkey: Self::PrivKey) -> Result<Self, Self::Error> {
        let keypair = keypair_from_privkey(privkey.as_ref());
        match keypair {
            None => Err(Error::InvalidPrivKey),
            Some((pk, sk)) => Ok(KeyPair {
                privkey: PrivKey::from(sk.0),
                pubkey: PubKey::from(pk.0),
            }),
        }
    }

    fn gen_keypair() -> Self {
        let (pk, sk) = gen_keypair();
        KeyPair {
            privkey: PrivKey::from(sk.0),
            pubkey: PubKey::from(pk.0),
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
    use super::*;
    use util::crypto::CreateKey;

    #[test]
    fn test_from_privkey() {
        let keypair1 = KeyPair::gen_keypair();
        let keypair2 = KeyPair::from_privkey(keypair1.privkey).unwrap();
        assert_eq!(keypair1.pubkey, keypair2.pubkey);
        assert_eq!(keypair1.privkey, keypair2.privkey);
    }
}
