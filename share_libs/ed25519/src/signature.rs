use super::{PrivKey, PubKey, Address, Message, Error, KeyPair, pubkey_to_address};
use rlp::*;
use rustc_serialize::hex::ToHex;
use sodiumoxide::crypto::sign::{sign_detached, verify_detached, SecretKey, Signature as EdSignature, PublicKey as EdPublicKey};
use std::fmt;
use std::ops::{Deref, DerefMut};
use util::H768;

pub struct Signature(pub [u8; 96]);

impl Signature {
    pub fn sig(&self) -> &[u8] {
        &self.0[0..64]
    }

    pub fn pk(&self) -> &[u8] {
        &self.0[64..96]
    }
}

impl PartialEq for Signature {
    fn eq(&self, rhs: &Self) -> bool {
        &self.0[..] == &rhs.0[..]
    }
}

impl Decodable for Signature {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
                                       let mut sig = [0u8; 96];
                                       sig[0..96].copy_from_slice(bytes);
                                       Ok(Signature(sig))
                                   })
    }
}

impl Encodable for Signature {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(&self.0[0..96]);
    }
}


impl Eq for Signature {}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Signature")
         .field("signature", &self.0[0..64].to_hex())
         .field("pubkey", &self.0[64..96].to_hex())
         .finish()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0[..].to_hex())
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature([0u8; 96])
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        Signature(self.0)
    }
}

impl From<[u8; 96]> for Signature {
    fn from(bytes: [u8; 96]) -> Self {
        Signature(bytes)
    }
}

impl Into<[u8; 96]> for Signature {
    fn into(self) -> [u8; 96] {
        self.0
    }
}

impl From<Signature> for H768 {
    fn from(s: Signature) -> Self {
        s.0.into()
    }
}

impl From<H768> for Signature {
    fn from(h: H768) -> Self {
        Signature(h.into())
    }
}

impl Deref for Signature {
    type Target = [u8; 96];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn sign(privkey: &PrivKey, message: &Message) -> Result<Signature, Error> {
    let keypair = KeyPair::from_privkey(*privkey)?;
    let secret_key = SecretKey::from_slice(privkey.as_ref()).unwrap();
    let pubkey = keypair.pubkey();
    let mut ret = [0u8; 96];
    let sig = sign_detached(message.as_ref(), &secret_key);

    ret[0..64].copy_from_slice(&sig.0[..]);
    ret[64..96].copy_from_slice(&pubkey.as_ref()[..]);
    Ok(Signature(ret))
}

pub fn recover(signature: &Signature, message: &Message) -> Result<PubKey, Error> {
    let sig = signature.sig();
    let pubkey = signature.pk();
    let is_valid = verify_detached(&EdSignature::from_slice(&sig).unwrap(), message.as_ref(), &EdPublicKey::from_slice(&pubkey).unwrap());

    if !is_valid { Err(Error::InvalidSignature) } else { Ok(PubKey::from_slice(&pubkey)) }
}

pub fn verify_public(pubkey: &PubKey, signature: &Signature, message: &Message) -> Result<bool, Error> {
    let sig = signature.sig();
    let pk = signature.pk();
    if pk != pubkey.as_ref() {
        return Err(Error::InvalidPubKey);
    }

    let is_valid = verify_detached(&EdSignature::from_slice(&sig).unwrap(), message.as_ref(), &EdPublicKey::from_slice(&pubkey).unwrap());
    if !is_valid { Err(Error::InvalidSignature) } else { Ok(true) }
}

pub fn verify_address(address: &Address, signature: &Signature, message: &Message) -> Result<bool, Error> {
    let pubkey = recover(signature, message)?;
    let recover_address = pubkey_to_address(&pubkey);
    Ok(address == &recover_address)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MESSAGE: [u8; 32] = [
        0x01,
        0x02,
        0x03,
        0x04,
        0x19,
        0xab,
        0xfe,
        0x39,
        0x6f,
        0x28,
        0x79,
        0x00,
        0x08,
        0xdf,
        0x9a,
        0xef,
        0xfb,
        0x77,
        0x42,
        0xae,
        0xad,
        0xfc,
        0xcf,
        0x12,
        0x24,
        0x45,
        0x29,
        0x89,
        0x29,
        0x45,
        0x3f,
        0xf8,
    ];

    #[test]
    fn test_sign_verify() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::from_slice(&MESSAGE[..]);
        let sig = sign(keypair.privkey(), &msg).unwrap();
        assert!(verify_public(keypair.pubkey(), &sig, &msg).unwrap());
    }

    #[test]
    fn test_verify_address() {
        let keypair = KeyPair::gen_keypair();
        let address = pubkey_to_address(keypair.pubkey());
        let msg = Message::from_slice(&MESSAGE[..]);
        let sig = sign(keypair.privkey(), &msg).unwrap();
        assert!(verify_address(&address, &sig, &msg).unwrap());
    }

    #[test]
    fn test_recover() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::from_slice(&MESSAGE[..]);
        let sig = sign(keypair.privkey(), &msg).unwrap();
        assert_eq!(keypair.pubkey(), &recover(&sig, &msg).unwrap());
    }
}
