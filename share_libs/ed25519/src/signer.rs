use super::{PubKey, PrivKey, KeyPair, Address};

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
    fn from(privkey: PrivKey) -> Self {
        let keypair = KeyPair::from_privkey(privkey).unwrap();
        Signer {
            address: keypair.address(),
            keypair: keypair,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer() {
        let keypair = KeyPair::gen_keypair();
        let signer = Signer::from(keypair.privkey().clone());
        assert_eq!(signer.keypair.privkey(), keypair.privkey());
        assert_eq!(signer.keypair.pubkey(), keypair.pubkey());
        assert_eq!(signer.address, keypair.address());
    }
}
