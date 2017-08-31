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

use super::{PrivKey, PubKey, SECP256K1, Error, Message, pubkey_to_address, Address, SIGNATURE_BYTES_LEN};
use rlp::*;
use rustc_serialize::hex::ToHex;
use secp256k1::{Message as SecpMessage, RecoverableSignature, RecoveryId, Error as SecpError};
use secp256k1::key::{SecretKey, PublicKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error as SerdeError, Visitor, SeqAccess};
use serde::ser::SerializeSeq;
use std::{mem, fmt};
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use util::{H520, H256};

pub struct Signature(pub [u8; 65]);

impl Signature {
    /// Get a slice into the 'r' portion of the data.
    pub fn r(&self) -> &[u8] {
        &self.0[0..32]
    }

    /// Get a slice into the 's' portion of the data.
    pub fn s(&self) -> &[u8] {
        &self.0[32..64]
    }

    /// Get the recovery byte.
    pub fn v(&self) -> u8 {
        self.0[64]
    }

    /// Create a signature object from the sig.
    pub fn from_rsv(r: &H256, s: &H256, v: u8) -> Signature {
        let mut sig = [0u8; 65];
        sig[0..32].copy_from_slice(&r.0);
        sig[32..64].copy_from_slice(&s.0);
        sig[64] = v;
        Signature(sig)
    }

    /// Check if this is a "low" signature.
    pub fn is_low_s(&self) -> bool {
        H256::from_slice(self.s()) <= "7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0".into()
    }

    /// Check if each component of the signature is in range.
    pub fn is_valid(&self) -> bool {
        self.v() <= 1 && H256::from_slice(self.r()) < "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".into() && H256::from_slice(self.r()) >= 1.into() && H256::from_slice(self.s()) < "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".into() && H256::from_slice(self.s()) >= 1.into()
    }
}

// manual implementation large arrays don't have trait impls by default.
// remove when integer generics exist
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
                formatter.write_str("secp256k1 signature")
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
         .field("r", &self.0[0..32].to_hex())
         .field("s", &self.0[32..64].to_hex())
         .field("v", &self.0[64..65].to_hex())
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

pub fn sign(privkey: &PrivKey, message: &Message) -> Result<Signature, Error> {
    let context = &SECP256K1;
    // no way to create from raw byte array.
    let sec: &SecretKey = unsafe { mem::transmute(privkey) };
    let s = context.sign_recoverable(&SecpMessage::from_slice(&message.0[..])?, sec)?;
    let (rec_id, data) = s.serialize_compact(context);
    let mut data_arr = [0; 65];

    // no need to check if s is low, it always is
    data_arr[0..64].copy_from_slice(&data[0..64]);
    data_arr[64] = rec_id.to_i32() as u8;
    Ok(Signature(data_arr))
}

pub fn verify_public(pubkey: &PubKey, signature: &Signature, message: &Message) -> Result<bool, Error> {
    let context = &SECP256K1;
    let rsig = RecoverableSignature::from_compact(context, &signature[0..64], RecoveryId::from_i32(signature[64] as i32)?)?;
    let sig = rsig.to_standard(context);

    let pdata: [u8; 65] = {
        let mut temp = [4u8; 65];
        temp[1..65].copy_from_slice(pubkey);
        temp
    };

    let publ = PublicKey::from_slice(context, &pdata)?;
    match context.verify(&SecpMessage::from_slice(&message.0[..])?, &sig, &publ) {
        Ok(_) => Ok(true),
        Err(SecpError::IncorrectSignature) => Ok(false),
        Err(x) => Err(Error::from(x)),
    }
}

pub fn verify_address(address: &Address, signature: &Signature, message: &Message) -> Result<bool, Error> {
    let pubkey = recover(signature, message)?;
    let recovered_address = pubkey_to_address(&pubkey);
    Ok(address == &recovered_address)
}

pub fn recover(signature: &Signature, message: &Message) -> Result<PubKey, Error> {
    let context = &SECP256K1;
    let rsig = RecoverableSignature::from_compact(context, &signature[0..64], RecoveryId::from_i32(signature[64] as i32)?)?;
    let publ = context.recover(&SecpMessage::from_slice(&message.0[..])?, &rsig)?;
    let serialized = publ.serialize_vec(context, false);

    let mut pubkey = PubKey::default();
    pubkey.0.copy_from_slice(&serialized[1..65]);
    Ok(pubkey)
}


#[cfg(test)]
mod tests {
    use super::{Signature, sign};
    use super::super::KeyPair;
    use bincode::{serialize, deserialize, Infinite};
    use util::H256;

    #[test]
    fn test_de_serialize() {
        let keypair = KeyPair::gen_keypair();
        let message = H256::default();
        let signature = sign(keypair.privkey().into(), &message.into()).unwrap();
        let se_result = serialize(&signature, Infinite).unwrap();
        let de_result: Signature = deserialize(&se_result).unwrap();
        assert_eq!(signature, de_result);
    }
}
