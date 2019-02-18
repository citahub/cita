// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidNonce,
    Dup,
    InvalidUntilBlock,
    BadSig,
    NotReady,
    Busy,
    BadChainId,
    // TODO: rename to QuotaOverflow
    QuotaNotEnough,
    Forbidden,
    InvalidValue,
    InvalidVersion,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            InvalidNonce => write!(f, "InvalidNonce"),
            Dup => write!(f, "Dup"),
            InvalidUntilBlock => write!(f, "InvalidUntilBlock"),
            BadSig => write!(f, "BadSig"),
            NotReady => write!(f, "NotReady"),
            Busy => write!(f, "Busy"),
            BadChainId => write!(f, "BadChainId"),
            QuotaNotEnough => write!(f, "QuotaNotEnough"),
            Forbidden => write!(f, "Forbidden"),
            InvalidValue => write!(f, "InvalidValue"),
            InvalidVersion => write!(f, "InvalidVersion"),
        }
    }
}
