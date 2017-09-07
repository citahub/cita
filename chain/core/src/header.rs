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

//! Block header.

use basic_types::{LogBloom, ZERO_LOGBLOOM};
use libproto::blockchain::{Proof, ProofType, BlockHeader};
use rlp::*;
use std::cell::Cell;
use std::cmp;
use std::ops::Deref;
use time::get_time;

pub use types::BlockNumber;
use util::*;

/// A block header.
///
#[derive(Debug, PartialEq, Clone, Eq)]
struct HashWrap(Cell<Option<H256>>);

unsafe impl Sync for HashWrap {}

impl Deref for HashWrap {
    type Target = Cell<Option<H256>>;

    fn deref(&self) -> &Cell<Option<H256>> {
        &self.0
    }
}

/// Doesn't do all that much on its own.
#[derive(Debug, Clone, Eq)]
pub struct Header {
    /// Parent hash.
    parent_hash: H256,
    /// Block timestamp.
    timestamp: u64,
    /// Block number.
    number: BlockNumber,
    /// Transactions root.
    transactions_root: H256,
    /// State root.
    state_root: H256,
    /// Block receipts root.
    receipts_root: H256,
    /// Block bloom.
    log_bloom: LogBloom,
    /// Gas used for contracts execution.
    gas_used: U256,
    /// Block gas limit.
    gas_limit: U256,
    /// the proof of the block
    proof: Proof,
    /// The hash of the header.
    hash: HashWrap,
    /// The version of the header.
    version: u32,
}

impl PartialEq for Header {
    fn eq(&self, c: &Header) -> bool {
        self.parent_hash == c.parent_hash && self.timestamp == c.timestamp && self.number == c.number && self.transactions_root == c.transactions_root && self.state_root == c.state_root && self.receipts_root == c.receipts_root && self.log_bloom == c.log_bloom && self.gas_used == c.gas_used && self.gas_limit == c.gas_limit && self.proof == c.proof

    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            parent_hash: H256::default(),
            timestamp: 0,
            number: 0,
            transactions_root: HASH_NULL_RLP,
            state_root: HASH_NULL_RLP,
            receipts_root: HASH_NULL_RLP,
            log_bloom: ZERO_LOGBLOOM.clone(),
            gas_used: U256::default(),
            gas_limit: U256::from(u64::max_value()),
            proof: Proof::new(),
            hash: HashWrap(Cell::new(None)),
            version: 0,
        }
    }
}

impl From<BlockHeader> for Header {
    fn from(bh: BlockHeader) -> Self {
        Header {
            parent_hash: H256::from(bh.get_prevhash()),
            timestamp: bh.get_timestamp(),
            number: bh.get_height(),
            transactions_root: H256::from(bh.get_transactions_root()),
            state_root: H256::default(),
            receipts_root: H256::default(),
            log_bloom: ZERO_LOGBLOOM,
            gas_used: U256::zero(),
            gas_limit: U256::from(u64::max_value()),
            proof: bh.get_proof().clone(),
            version: 0,
            hash: HashWrap(Cell::new(None)),
        }
    }
}

impl Header {
    /// Create a new, default-valued, header.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the parent_hash field of the header.
    pub fn parent_hash(&self) -> &H256 {
        &self.parent_hash
    }
    /// Get the timestamp field of the header.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    /// Get the number field of the header.
    pub fn number(&self) -> BlockNumber {
        self.number
    }
    /// Get the state root field of the header.
    pub fn state_root(&self) -> &H256 {
        &self.state_root
    }
    /// Get the receipts root field of the header.
    pub fn receipts_root(&self) -> &H256 {
        &self.receipts_root
    }
    /// Get the log bloom field of the header.
    pub fn log_bloom(&self) -> &LogBloom {
        &self.log_bloom
    }
    /// Get the transactions root field of the header.
    pub fn transactions_root(&self) -> &H256 {
        &self.transactions_root
    }
    /// Get the gas used field of the header.
    pub fn gas_used(&self) -> &U256 {
        &self.gas_used
    }
    /// Get the gas limit field of the header.
    pub fn gas_limit(&self) -> &U256 {
        &self.gas_limit
    }
    /// Get the proof field of the header.
    pub fn proof(&self) -> &Proof {
        &self.proof
    }
    /// Get the version of the block
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Get the proof type field of the header.
    pub fn proof_type(&self) -> Option<ProofType> {
        if self.proof == Proof::new() { None } else { Some(self.proof.get_field_type()) }
    }

    /// Set the number field of the header.
    pub fn set_parent_hash(&mut self, a: H256) {
        self.parent_hash = a;
        self.note_dirty();
    }
    /// Set the state root field of the header.
    pub fn set_state_root(&mut self, a: H256) {
        self.state_root = a;
        self.note_dirty();
    }
    /// Set the transactions root field of the header.
    pub fn set_transactions_root(&mut self, a: H256) {
        self.transactions_root = a;
        self.note_dirty()
    }
    /// Set the receipts root field of the header.
    pub fn set_receipts_root(&mut self, a: H256) {
        self.receipts_root = a;
        self.note_dirty()
    }
    /// Set the log bloom field of the header.
    pub fn set_log_bloom(&mut self, a: LogBloom) {
        self.log_bloom = a;
        self.note_dirty()
    }
    /// Set the timestamp field of the header.
    pub fn set_timestamp(&mut self, a: u64) {
        self.timestamp = a;
        self.note_dirty();
    }
    /// Set the timestamp field of the header to the current time.
    pub fn set_timestamp_now(&mut self, but_later_than: u64) {
        self.timestamp = cmp::max(get_time().sec as u64, but_later_than + 1);
        self.note_dirty();
    }
    /// Set the number field of the header.
    pub fn set_number(&mut self, a: BlockNumber) {
        self.number = a;
        self.note_dirty();
    }
    /// Set the gas used field of the header.
    pub fn set_gas_used(&mut self, a: U256) {
        self.gas_used = a;
        self.note_dirty();
    }
    /// Set the gas limit field of the header.
    pub fn set_gas_limit(&mut self, a: U256) {
        self.gas_limit = a;
        self.note_dirty();
    }
    /// Set the version of the header.
    pub fn set_version(&mut self, a: u32) {
        self.version = a;
        self.note_dirty();
    }
    /// Set the proof the block.
    pub fn set_proof(&mut self, a: Proof) {
        self.proof = a;
        self.note_dirty();
    }

    /// Get the hash of this header (sha3 of the RLP).
    pub fn hash(&self) -> H256 {
        let hash = self.hash.get();
        match hash {
            Some(h) => h,
            None => {
                let h = self.rlp_hash();
                self.hash.set(Some(h.clone()));
                h
            }
        }
    }

    /// Note that some fields have changed. Resets the memoised hash.
    pub fn note_dirty(&self) {
        self.hash.set(None);
    }

    // TODO: make these functions traity
    /// Place this header into an RLP stream `s`.
    pub fn stream_rlp(&self, s: &mut RlpStream) {
        s.begin_list(11);
        s.append(&self.parent_hash);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.log_bloom);
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);
        s.append(&self.version);
        s.append(&self.proof);

    }

    /// Get the RLP of this header.
    pub fn rlp(&self) -> Bytes {
        let mut s = RlpStream::new();
        self.stream_rlp(&mut s);
        s.out()
    }

    /// Get the crypt_hash (Keccak or blake2b) of this header.
    pub fn rlp_hash(&self) -> H256 {
        self.rlp().crypt_hash()
    }

    /// Generate the protobuf header.
    pub fn protobuf(&self) -> BlockHeader {
        let mut bh = BlockHeader::new();
        bh.set_prevhash(self.parent_hash.to_vec());
        bh.set_timestamp(self.timestamp);
        bh.set_height(self.number);
        bh.set_state_root(self.state_root.to_vec());
        bh.set_receipts_root(self.receipts_root.to_vec());
        bh.set_transactions_root(self.transactions_root.to_vec());
        bh.set_gas_used(u64::from(self.gas_used));
        bh.set_proof(self.proof.clone());
        bh
    }
}

impl Decodable for Header {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        let blockheader = Header {
            parent_hash: r.val_at(0)?,
            state_root: r.val_at(1)?,
            transactions_root: r.val_at(2)?,
            receipts_root: r.val_at(3)?,
            log_bloom: r.val_at(4)?,
            number: r.val_at(5)?,
            gas_limit: r.val_at(6)?,
            gas_used: r.val_at(7)?,
            timestamp: cmp::min(r.val_at::<U256>(8)?, u64::max_value().into()).as_u64(),
            version: r.val_at(9)?,
            proof: r.val_at(10)?,
            hash: HashWrap(Cell::new(Some(r.as_raw().crypt_hash()))),
        };

        Ok(blockheader)
    }
}

impl Encodable for Header {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.stream_rlp(s);
    }
}

impl HeapSizeOf for Header {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Header;
    use rlp;

    #[test]
    fn decode_and_encode_header() {
        // that's rlp of block header created with ethash engine.
        let header = Header::new();
        let header_rlp = rlp::encode(&header).into_vec();
        let header: Header = rlp::decode(&header_rlp);
        let encoded_header = rlp::encode(&header).into_vec();

        assert_eq!(header_rlp, encoded_header);
    }
}
