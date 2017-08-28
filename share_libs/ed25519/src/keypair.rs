
use super::{PrivKey, PubKey, Address};
use error::Error;
use sodiumoxide::crypto::sign::{keypair_from_privkey, gen_keypair};
use util::{H160, Hashable};

pub fn pubkey_to_address(pubkey: &PubKey) -> Address {
    Address::from(H160::from(pubkey.crypt_hash()))
}

#[derive(Default)]
pub struct KeyPair {
    privkey: PrivKey,
    pubkey: PubKey,
}

impl KeyPair {
    pub fn from_privkey(privkey: PrivKey) -> Result<Self, Error> {
        let keypair = keypair_from_privkey(privkey.as_ref());
        match keypair {
            None => Err(Error::InvalidPrivKey),
            Some((pk, sk)) => Ok(KeyPair {
                                     privkey: PrivKey::from(sk.0),
                                     pubkey: PubKey::from(pk.0),
                                 }),
        }
    }

    pub fn gen_keypair() -> Self {
        let (pk, sk) = gen_keypair();
        KeyPair {
            privkey: PrivKey::from(sk.0),
            pubkey: PubKey::from(pk.0),
        }
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
    use super::*;

    #[test]
    fn test_from_privkey() {
        let keypair1 = KeyPair::gen_keypair();
        let keypair2 = KeyPair::from_privkey(keypair1.privkey).unwrap();
        assert_eq!(keypair1.pubkey, keypair2.pubkey);
        assert_eq!(keypair1.privkey, keypair2.privkey);
    }
}
