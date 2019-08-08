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
use std::fmt;

use super::util::{Mismatch, OutOfBounds};
use crate::header::BlockNumber;
use crate::log_entry::LogBloom;
use cita_types::{H256, U256};

#[allow(unknown_lints, clippy::large_enum_variant)] // TODO clippy
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
/// Errors concerning block processing.
pub enum BlockError {
    /// Extra data is of an invalid length.
    ExtraDataOutOfBounds(OutOfBounds<usize>),
    /// Seal is incorrect format.
    InvalidSealArity(Mismatch<usize>),
    /// Block has too much gas used.
    TooMuchGasUsed(OutOfBounds<U256>),
    /// State root header field is invalid.
    InvalidStateRoot(Mismatch<H256>),
    /// Gas used header field is invalid.
    InvalidGasUsed(Mismatch<U256>),
    /// Transactions root header field is invalid.
    InvalidTransactionsRoot(Mismatch<H256>),
    /// Difficulty is out of range; this can be used as an looser error prior to getting a definitive
    /// value for difficulty. This error needs only provide bounds of which it is out.
    DifficultyOutOfBounds(OutOfBounds<U256>),
    /// Difficulty header field is invalid; this is a strong error used after getting a definitive
    /// value for difficulty (which is provided).
    InvalidDifficulty(Mismatch<U256>),
    /// Seal element of type H256 (max_hash for Ethash, but could be something else for
    /// other seal engines) is out of bounds.
    MismatchedH256SealElement(Mismatch<H256>),
    /// Proof-of-work aspect of seal, which we assume is a 256-bit value, is invalid.
    InvalidProofOfWork(OutOfBounds<U256>),
    /// Some low-level aspect of the seal is incorrect.
    InvalidSeal,
    /// Gas limit header field is invalid.
    InvalidGasLimit(OutOfBounds<U256>),
    /// Receipts trie root header field is invalid.
    InvalidReceiptsRoot(Mismatch<H256>),
    /// Timestamp header field is invalid.
    InvalidTimestamp(OutOfBounds<u64>),
    /// Log bloom header field is invalid.
    InvalidLogBloom(Mismatch<LogBloom>),
    /// Parent hash field of header is invalid; this is an invalid error indicating a logic flaw in the codebase.
    /// TODO: remove and favour an assert!/panic!.
    InvalidParentHash(Mismatch<H256>),
    /// Number field of header is invalid.
    InvalidNumber(Mismatch<BlockNumber>),
    /// Block number isn't sensible.
    RidiculousNumber(OutOfBounds<BlockNumber>),
    /// Parent given is unknown.
    UnknownParent(H256),
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::BlockError::*;

        let msg = match *self {
            ExtraDataOutOfBounds(ref oob) => format!("Extra block data too long. {}", oob),
            InvalidSealArity(ref mis) => format!("Block seal in incorrect format: {}", mis),
            TooMuchGasUsed(ref oob) => format!("Block has too much gas used. {}", oob),
            InvalidStateRoot(ref mis) => format!("Invalid state root in header: {}", mis),
            InvalidGasUsed(ref mis) => format!("Invalid gas used in header: {}", mis),
            InvalidTransactionsRoot(ref mis) => {
                format!("Invalid transactions root in header: {}", mis)
            }
            DifficultyOutOfBounds(ref oob) => format!("Invalid block difficulty: {}", oob),
            InvalidDifficulty(ref mis) => format!("Invalid block difficulty: {}", mis),
            MismatchedH256SealElement(ref mis) => format!("Seal element out of bounds: {}", mis),
            InvalidProofOfWork(ref oob) => format!("Block has invalid PoW: {}", oob),
            InvalidSeal => "Block has invalid seal.".into(),
            InvalidGasLimit(ref oob) => format!("Invalid gas limit: {}", oob),
            InvalidReceiptsRoot(ref mis) => {
                format!("Invalid receipts trie root in header: {}", mis)
            }
            InvalidTimestamp(ref oob) => format!("Invalid timestamp in header: {}", oob),
            InvalidLogBloom(ref oob) => format!("Invalid log bloom in header: {}", oob),
            InvalidParentHash(ref mis) => format!("Invalid parent hash: {}", mis),
            InvalidNumber(ref mis) => format!("Invalid number in header: {}", mis),
            RidiculousNumber(ref oob) => format!("Implausible block number. {}", oob),
            UnknownParent(ref hash) => format!("Unknown parent: {}", hash),
        };

        f.write_fmt(format_args!("Block error ({})", msg))
    }
}
