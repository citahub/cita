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

use super::{pubkey_to_address, Address, Error, Message, PrivKey, PubKey, sm2_recover, sm2_sign, GROUP,
            PUBKEY_BYTES_LEN, SIGNATURE_BYTES_LEN};
use rlp::*;
use rustc_serialize::hex::ToHex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as SerdeError, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use util::H520;
use util::crypto::Sign;

pub struct Signature(pub [u8; 65]);

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        &self.0[..] == &other.0[..]
    }
}

impl Decodable for Signature {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| {
            let mut sig = [0u8; 65];
            sig[0..65].copy_from_slice(bytes);
            Ok(Signature(sig))
        })
    }
}

impl Encodable for Signature {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(&self.0[0..65]);
    }
}

// TODO: Maybe it should be implemented with rust macro(https://github.com/rust-lang/rfcs/issues/1038)
impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignatureVisitor;

        impl<'de> Visitor<'de> for SignatureVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("sm2 signature")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut signature = Signature([0u8; SIGNATURE_BYTES_LEN]);
                for i in 0..SIGNATURE_BYTES_LEN {
                    signature.0[i] = match visitor.next_element()? {
                        Some(val) => val,
                        None => return Err(SerdeError::invalid_length(SIGNATURE_BYTES_LEN, &self)),
                    }
                }
                Ok(signature)
            }
        }

        let visitor = SignatureVisitor;
        deserializer.deserialize_seq(visitor)
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(SIGNATURE_BYTES_LEN))?;
        for i in 0..SIGNATURE_BYTES_LEN {
            seq.serialize_element(&self.0[i])?;
        }
        seq.end()
    }
}

// manual implementation required in Rust 1.13+, see `std::cmp::AssertParamIsEq`.
impl Eq for Signature {}

// also manual for the same reason, but the pretty printing might be useful.
impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Signature")
            .field("r", &self.0[1..33].to_hex())
            .field("s", &self.0[33..65].to_hex())
            .field("v", &self.0[0..1].to_hex())
            .finish()
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_hex())
    }
}

impl Default for Signature {
    fn default() -> Self {
        Signature([0; 65])
    }
}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Clone for Signature {
    fn clone(&self) -> Self {
        Signature(self.0)
    }
}

impl From<[u8; 65]> for Signature {
    fn from(s: [u8; 65]) -> Self {
        Signature(s)
    }
}

impl Into<[u8; 65]> for Signature {
    fn into(self) -> [u8; 65] {
        self.0
    }
}

impl<'a> From<&'a [u8]> for Signature {
    fn from(slice: &'a [u8]) -> Signature {
        assert_eq!(slice.len(), SIGNATURE_BYTES_LEN);
        let mut bytes = [0u8; 65];
        bytes.copy_from_slice(&slice[..]);
        Signature(bytes)
    }
}

impl<'a> Into<&'a [u8]> for &'a Signature {
    fn into(self) -> &'a [u8] {
        &self.0[..]
    }
}

impl From<Signature> for H520 {
    fn from(s: Signature) -> Self {
        s.0.into()
    }
}

impl From<H520> for Signature {
    fn from(bytes: H520) -> Self {
        Signature(bytes.into())
    }
}

impl Deref for Signature {
    type Target = [u8; 65];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Sign for Signature {
    type PrivKey = PrivKey;
    type PubKey = PubKey;
    type Message = Message;
    type Error = Error;

    fn sign(privkey: &Self::PrivKey, message: &Self::Message) -> Result<Self, Error> {
        let mut signature: [u8; SIGNATURE_BYTES_LEN] = [0; SIGNATURE_BYTES_LEN];
        unsafe {
            sm2_sign(
                GROUP.as_ptr(),
                privkey.as_ref().as_ptr(),
                message.as_ref().as_ptr(),
                signature.as_mut_ptr(),
            );
        }
        Ok(Signature(signature))
    }

    fn recover(&self, message: &Message) -> Result<Self::PubKey, Error> {
        let mut pubkey: [u8; PUBKEY_BYTES_LEN] = [0; PUBKEY_BYTES_LEN];
        unsafe {
            let result = sm2_recover(
                GROUP.as_ptr(),
                self.0.as_ptr(),
                message.as_ptr(),
                pubkey.as_mut_ptr(),
            );
            if result <= 0 {
                return Err(Error::RecoverError);
            }
        }
        Ok(PubKey::from(pubkey))
    }

    fn verify_public(&self, pubkey: &Self::PubKey, message: &Self::Message) -> Result<bool, Error> {
        self.recover(message).map(|key| *pubkey == key)
    }

    fn verify_address(&self, address: &Address, message: &Self::Message) -> Result<bool, Error> {
        let pubkey = self.recover(message)?;
        let recover_address = pubkey_to_address(&pubkey);
        Ok(address == &recover_address)
    }
}

/*
#[cfg(test)]
mod tests {
    extern crate bincode;
    use self::bincode::{serialize, deserialize, Infinite};
    use super::{Signature, Message};
    use super::super::KeyPair;
    use util::crypto::{CreateKey, Sign};

    #[test]
    fn test_sign_verify() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        assert!(sig.verify_public(keypair.pubkey(), &msg).unwrap());
    }

    #[test]
    fn test_verify_address() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        assert_eq!(keypair.pubkey(), &sig.recover(&msg).unwrap());
    }

    #[test]
    fn test_recover() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        assert_eq!(keypair.pubkey(), &sig.recover(&msg).unwrap());
    }

    #[test]
    fn test_into_slice() {
        let keypair = KeyPair::gen_keypair();
        let msg = Message::default();
        let sig = Signature::sign(keypair.privkey(), &msg).unwrap();
        let sig = &sig;
        let slice: &[u8] = sig.into();
        assert_eq!(Signature::from(slice), *sig);
    }

    #[test]
    fn test_de_serialize() {
        let keypair = KeyPair::gen_keypair();
        let message = Message::default();
        let signature = Signature::sign(keypair.privkey().into(), &message.into()).unwrap();
        let se_result = serialize(&signature, Infinite).unwrap();
        let de_result: Signature = deserialize(&se_result).unwrap();
        assert_eq!(signature, de_result);
    }
}
*/
