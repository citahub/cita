pub mod handler;
pub mod authority_round;
pub mod spec;

pub use engine::*;
pub use self::authority_round::AuthorityRound;
pub use self::spec::Spec;
pub use libproto::blockchain::{BlockHeader, Block, Transaction, BlockBody, Proof};
use util::Address;
use util::hash::H256;
use crypto::{PrivKey, Signature, sign, recover, pubkey_to_address, Error as CryptoError};

pub trait Signable {
    fn bare_hash(&self) -> H256;
    fn sign_with_privkey(&self, privkey: &PrivKey) -> Result<Signature, CryptoError> {
        sign(privkey, &self.bare_hash().into())
    }
    fn recover_address_with_signature(&self,
                                      signature: &Signature)
                                      -> Result<Address, CryptoError> {
        let pubkey = recover(signature, &self.bare_hash().into()).unwrap();
        Ok(pubkey_to_address(&pubkey).into())
    }
}