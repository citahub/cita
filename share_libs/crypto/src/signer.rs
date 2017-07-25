use super::{PrivKey, KeyPair, PubKey, Address};

#[derive(Default)]
pub struct Signer {
    pub keypair: KeyPair,
    pub address: Address,
}

impl Signer {
    pub fn privkey(&self) -> &PrivKey {
        self.keypair.privkey()
    }

    pub fn pubkey(&self) -> &PubKey {
        self.keypair.pubkey()
    }
}

impl From<PrivKey> for Signer {
    fn from(k: PrivKey) -> Self {
        let keypair = KeyPair::from_privkey(k).unwrap();
        Signer {
            address: keypair.address().clone(),
            keypair: keypair,
        }
    }
}