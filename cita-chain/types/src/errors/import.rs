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

#[derive(Debug, Clone, Copy, PartialEq)]
/// Import to the block queue result
pub enum ImportError {
    /// Already in the block chain.
    AlreadyInChain,
    /// Already in the block queue.
    AlreadyQueued,
    /// Already marked as bad from a previous import (could mean parent is bad).
    KnownBad,
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            ImportError::AlreadyInChain => "block already in chain",
            ImportError::AlreadyQueued => "block already in the block queue",
            ImportError::KnownBad => "block known to be bad",
        };

        f.write_fmt(format_args!("Block import error ({})", msg))
    }
}
