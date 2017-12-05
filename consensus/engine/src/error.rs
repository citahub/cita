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

use crypto::Signature;
use std::fmt;
use util::Address;

#[derive(Debug)]
pub enum EngineError {
    NotAuthorized(Address),
    NotProposer(Mismatch<Address>),
    DoubleVote(Address),
    FutureBlock(u64),
    NotAboveThreshold(usize),
    BadSignature(Signature),
    InvalidProof,
    /// Message was not expected.
    UnexpectedMessage,
    VoteMsgDelay(usize),
    VoteMsgForth(usize),
    InvalidSignature,
    InvalidTxInProposal,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EngineError::*;
        let msg = match *self {
            NotProposer(ref mis) => format!("Author is not a current proposer: {}", mis),
            NotAuthorized(ref address) => format!("Signer {} is not authorized.", address),
            DoubleVote(ref address) => format!("Author {} issued too many blocks.", address),
            BadSignature(ref signature) => format!("bad signature {}", signature),
            FutureBlock(time) => format!("Block from future: {}", time),
            InvalidProof => "Invalid proof.".into(),
            NotAboveThreshold(vote) => format!("Vote is not above threshold: {}", vote),
            UnexpectedMessage => "This Engine should not be fed messages.".into(),
            VoteMsgDelay(height) => format!(
                "The vote message is delayed and missed the current height:{}",
                height
            ),
            VoteMsgForth(height) => format!("The vote message is fulture height :{}", height),
            InvalidSignature => "Invalid Signature.".into(),
            InvalidTxInProposal => "Invalid Tx In Proposal.".into(),
        };
        f.write_fmt(format_args!("Engine error ({})", msg))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
/// Error indicating an expected value was not found.
pub struct Mismatch<T: fmt::Debug> {
    /// Value expected.
    pub expected: T,
    /// Value found.
    pub found: T,
}

impl<T: fmt::Debug + fmt::Display> fmt::Display for Mismatch<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "Expected {}, found {}",
            self.expected,
            self.found
        ))
    }
}
