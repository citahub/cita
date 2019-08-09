// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! General error types for use in ethcore.

use cita_ed25519::Error as EthkeyError;

use cita_trie::TrieError;
use std::fmt;

use super::execution::ExecutionError;
use super::util::UtilError;

#[allow(unknown_lints, clippy::large_enum_variant)] // TODO clippy
#[derive(Debug)]
/// General error type which should be capable of representing all errors in ethcore.
pub enum Error {
    /// Error concerning a utility.
    Util(UtilError),
    /// Unknown engine given.
    UnknownEngineName(String),
    /// Error concerning EVM code execution.
    Execution(ExecutionError),
    /// PoW hash is invalid or out of date.
    PowHashInvalid,
    /// The value of the nonce or mishash is invalid.
    PowInvalid,
    /// Error concerning TrieDBs
    Trie(TrieError),
    /// Standard io error.
    StdIo(::std::io::Error),
    /// Snappy error.
    Snappy(snappy::SnappyError),
    /// Ethkey error.
    Ethkey(EthkeyError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Util(ref err) => err.fmt(f),
            // Error::Block(ref err) => err.fmt(f),
            Error::Execution(ref err) => err.fmt(f),
            Error::UnknownEngineName(ref name) => {
                f.write_fmt(format_args!("Unknown engine name ({})", name))
            }
            Error::PowHashInvalid => f.write_str("Invalid or out of date PoW hash."),
            Error::PowInvalid => f.write_str("Invalid nonce or mishash"),
            Error::Trie(ref err) => err.fmt(f),
            Error::StdIo(ref err) => err.fmt(f),
            Error::Snappy(ref err) => err.fmt(f),
            Error::Ethkey(ref err) => err.fmt(f),
        }
    }
}

impl From<ExecutionError> for Error {
    fn from(err: ExecutionError) -> Error {
        Error::Execution(err)
    }
}

impl From<::rlp::DecoderError> for Error {
    fn from(err: ::rlp::DecoderError) -> Error {
        Error::Util(UtilError::Decoder(err))
    }
}

impl From<UtilError> for Error {
    fn from(err: UtilError) -> Error {
        Error::Util(err)
    }
}

impl From<TrieError> for Error {
    fn from(err: TrieError) -> Error {
        Error::Trie(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Error {
        Error::StdIo(err)
    }
}

impl From<snappy::SnappyError> for Error {
    fn from(err: snappy::SnappyError) -> Error {
        Error::Snappy(err)
    }
}

impl From<EthkeyError> for Error {
    fn from(err: EthkeyError) -> Error {
        Error::Ethkey(err)
    }
}
