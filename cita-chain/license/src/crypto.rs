// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_types::{H256, H512};
use ethereum_secp256k1::{
    constants,
    key::PublicKey,
    recovery::{RecoverableSignature, RecoveryId},
    Error as SecpError, Message as SecpMessage, Secp256k1,
};
use hex::encode;
use lazy_static::lazy_static;
use std::fmt;
use std::ops::{Deref, DerefMut};

/// Secp256k1 Public key
pub type Secp256k1PubKey = H512;
/// Sign Message
pub type Message = H256;

pub const SIGNATURE_BYTES_LEN: usize = 65;
pub const HASH_BYTES_LEN: usize = 32;

lazy_static! {
    pub static ref SECP256K1: Secp256k1<ethereum_secp256k1::All> = Secp256k1::new();
}

/// Signature
pub struct Secp256k1Signature(pub [u8; SIGNATURE_BYTES_LEN]);

impl Secp256k1Signature {
    /// Recover public key
    pub fn recover(&self, message: &Message) -> Result<Secp256k1PubKey, Error> {
        let context = &SECP256K1;
        let rsig = RecoverableSignature::from_compact(
            &self.0[0..64],
            RecoveryId::from_i32(i32::from(self.0[64] as i8))?,
        )?;
        let public = context.recover(&SecpMessage::from_slice(&message.0[..])?, &rsig)?;
        let serialized = public.serialize_uncompressed();

        let mut pubkey = Secp256k1PubKey::default();
        pubkey
            .0
            .copy_from_slice(&serialized[1..constants::UNCOMPRESSED_PUBLIC_KEY_SIZE]);
        Ok(pubkey)
    }

    /// Verify public key
    pub fn verify_public(
        &self,
        pubkey: &Secp256k1PubKey,
        message: &Message,
    ) -> Result<bool, Error> {
        let context = &SECP256K1;
        let rsig = RecoverableSignature::from_compact(
            &self.0[0..64],
            RecoveryId::from_i32(i32::from(self.0[64]))?,
        )?;
        let sig = rsig.to_standard();

        let pdata: [u8; SIGNATURE_BYTES_LEN] = {
            let mut temp = [4u8; SIGNATURE_BYTES_LEN];
            temp[1..SIGNATURE_BYTES_LEN].copy_from_slice(pubkey);
            temp
        };

        let publ = PublicKey::from_slice(&pdata)?;
        match context.verify(&SecpMessage::from_slice(&message.0[..])?, &sig, &publ) {
            Ok(_) => Ok(true),
            Err(SecpError::IncorrectSignature) => Ok(false),
            Err(x) => Err(Error::from(x)),
        }
    }
}

impl Deref for Secp256k1Signature {
    type Target = [u8; SIGNATURE_BYTES_LEN];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Secp256k1Signature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Secp256k1Signature {
    fn eq(&self, rhs: &Self) -> bool {
        self.0[..] == rhs.0[..]
    }
}

impl Eq for Secp256k1Signature {}

impl fmt::Debug for Secp256k1Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.debug_struct("Signature")
            .field("r", &encode(self.0[0..32].to_vec()))
            .field("s", &encode(self.0[32..64].to_vec()))
            .field("v", &encode(self.0[64..SIGNATURE_BYTES_LEN].to_vec()))
            .finish()
    }
}

impl fmt::Display for Secp256k1Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", encode(self.to_vec()))
    }
}

impl Default for Secp256k1Signature {
    fn default() -> Self {
        Secp256k1Signature([0; SIGNATURE_BYTES_LEN])
    }
}

impl<'a> From<&'a [u8]> for Secp256k1Signature {
    fn from(slice: &'a [u8]) -> Secp256k1Signature {
        assert_eq!(slice.len(), SIGNATURE_BYTES_LEN);
        let mut bytes = [0u8; SIGNATURE_BYTES_LEN];
        bytes.copy_from_slice(&slice[..]);
        Secp256k1Signature(bytes)
    }
}

/// Error of create secret key
#[derive(Debug)]
pub enum Error {
    /// Invalid private key
    InvalidPrivKey,
    /// Invalid public key
    InvalidPubKey,
    /// Invalid signature
    InvalidSignature,
    /// Invalid message
    InvalidMessage,
    /// Io error
    Io(::std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::InvalidPrivKey => "Invalid secret".into(),
            Error::InvalidPubKey => "Invalid public".into(),
            Error::InvalidSignature => "Invalid EC signature".into(),
            Error::InvalidMessage => "Invalid AES message".into(),
            Error::Io(ref err) => format!("I/O error: {}", err),
        };
        f.write_fmt(format_args!("Crypto error ({})", msg))
    }
}

impl From<::ethereum_secp256k1::Error> for Error {
    fn from(e: ::ethereum_secp256k1::Error) -> Error {
        match e {
            ::ethereum_secp256k1::Error::InvalidMessage => Error::InvalidMessage,
            ::ethereum_secp256k1::Error::InvalidPublicKey => Error::InvalidPubKey,
            ::ethereum_secp256k1::Error::InvalidSecretKey => Error::InvalidPrivKey,
            _ => Error::InvalidSignature,
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
