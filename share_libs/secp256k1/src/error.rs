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

use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPrivKey,
    InvalidPubKey,
    InvalidAddress,
    InvalidSignature,
    InvalidMessage,
    Io(::std::io::Error),
    Unexpected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::InvalidPrivKey => "Invalid secret".into(),
            Error::InvalidPubKey => "Invalid public".into(),
            Error::InvalidAddress => "Invalid address".into(),
            Error::InvalidSignature => "Invalid EC signature".into(),
            Error::InvalidMessage => "Invalid AES message".into(),
            Error::Io(ref err) => format!("I/O error: {}", err),
            Error::Unexpected(ref s) => s.clone(),
        };
        f.write_fmt(format_args!("Crypto error ({})", msg))
    }
}

impl From<::secp256k1::Error> for Error {
    fn from(e: ::secp256k1::Error) -> Error {
        match e {
            ::secp256k1::Error::InvalidMessage => Error::InvalidMessage,
            ::secp256k1::Error::InvalidPublicKey => Error::InvalidPubKey,
            ::secp256k1::Error::InvalidSecretKey => Error::InvalidPrivKey,
            _ => Error::InvalidSignature,
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::Io(err)
    }
}
