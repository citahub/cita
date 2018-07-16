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

//! Unique identifiers.

use cita_types::H256;
use jsonrpc_types::rpctypes::{BlockNumber as RpcBlockNumber, BlockTag};
use BlockNumber;

/// Uniquely identifies block.
#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum BlockId {
    /// Block's sha3.
    // TODO: Query by number faster
    /// Querying by hash is always faster.
    Hash(H256),
    /// Block number within canon blockchain.
    // TODO: Change to Height
    Number(BlockNumber),
    /// Earliest block (genesis).
    Earliest,
    /// Latest mined block.
    Latest,
}

pub type TransactionId = H256;

impl From<RpcBlockNumber> for BlockId {
    fn from(v: RpcBlockNumber) -> BlockId {
        match v {
            RpcBlockNumber::Height(height) => BlockId::Number(height.into()),
            RpcBlockNumber::Tag(BlockTag::Latest) => BlockId::Latest,
            RpcBlockNumber::Tag(BlockTag::Earliest) => BlockId::Earliest,
        }
    }
}
