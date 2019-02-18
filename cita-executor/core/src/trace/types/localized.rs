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

//! Localized traces type definitions

use super::trace::{Action, Res};
use cita_types::H256;
use header::BlockNumber;

/// Localized trace.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ipc", binary)]
pub struct LocalizedTrace {
    /// Type of action performed by a transaction.
    pub action: Action,
    /// Result of this action.
    pub result: Res,
    /// Number of subtraces.
    pub subtraces: usize,
    /// Exact location of trace.
    ///
    /// [index in root, index in first CALL, index in second CALL, ...]
    pub trace_address: Vec<usize>,
    /// Transaction number within the block.
    pub transaction_number: usize,
    /// Signed transaction hash.
    pub transaction_hash: H256,
    /// Block number.
    pub block_number: BlockNumber,
    /// Block hash.
    pub block_hash: H256,
}
