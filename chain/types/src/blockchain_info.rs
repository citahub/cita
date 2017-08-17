// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Blockhain info type definition

use BlockNumber;
use std::fmt;
use util::{U256, H256};

/// Information about the blockchain gathered together.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "ipc", binary)]
pub struct BlockChainInfo {
    /// Blockchain difficulty.
    pub total_difficulty: U256,
    /// Block queue difficulty.
    pub pending_total_difficulty: U256,
    /// Genesis block hash.
    pub genesis_hash: H256,
    /// Best blockchain block hash.
    pub best_block_hash: H256,
    /// Best blockchain block number.
    pub best_block_number: BlockNumber,
    /// Best blockchain block timestamp.
    pub best_block_timestamp: u64,
    /// Best ancient block hash.
    pub ancient_block_hash: Option<H256>,
    /// Best ancient block number.
    pub ancient_block_number: Option<BlockNumber>,
    /// First block on the best sequence.
    pub first_block_hash: Option<H256>,
    /// Number of the first block on the best sequence.
    pub first_block_number: Option<BlockNumber>,
}


impl fmt::Display for BlockChainInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}.{}", self.best_block_number, self.best_block_hash)
    }
}
