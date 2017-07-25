//! Unique identifiers.

use util::hash::H256;
use header::BlockNumber;

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
