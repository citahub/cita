use super::{PrivKey, PubKey, Address};
use error::Error;
use rustc_serialize::hex::ToHex;
use sodiumoxide::crypto::sign::{keypair_from_privkey, gen_keypair};
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
