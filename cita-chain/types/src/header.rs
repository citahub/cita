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

//! Block header.

use basic_types::{LogBloom, ZERO_LOGBLOOM};
use cita_types::{Address, H256, U256};
use libproto::blockchain::{
    Block as ProtoBlock, BlockHeader as ProtoBlockHeader, Proof as ProtoProof, ProofType,
};
use libproto::executor::{ExecutedHeader, ExecutedInfo};
use proof::BftProof;
use rlp::{self, Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
use std::cmp;
use std::ops::{Deref, DerefMut};
use time::get_time;

use util::{Bytes, Hashable, HeapSizeOf, HASH_NULL_RLP};
pub use BlockNumber;

#[derive(Debug, Clone, Eq)]
pub struct OpenHeader {
    /// Parent hash.
    parent_hash: H256,
    /// Block timestamp.
    timestamp: u64,
    /// Block number.
    number: BlockNumber,
    /// Transactions root.
    transactions_root: H256,
    /// Block gas limit.
    quota_limit: U256,
    /// the proof of the block
    proof: ProtoProof,
    /// The version of the header.
    version: u32,
    /// the selected proposer address
    proposer: Address,
}

impl PartialEq for OpenHeader {
    fn eq(&self, c: &OpenHeader) -> bool {
        self.parent_hash == c.parent_hash
            && self.timestamp == c.timestamp
            && self.number == c.number
            && self.transactions_root == c.transactions_root
            && self.quota_limit == c.quota_limit
            && self.proof == c.proof
            && self.version == c.version
            && self.proposer == c.proposer
    }
}

impl Default for OpenHeader {
    fn default() -> Self {
        OpenHeader {
            parent_hash: H256::default(),
            timestamp: 0,
            number: 0,
            transactions_root: HASH_NULL_RLP,
            quota_limit: U256::from(u64::max_value()),
            proof: ProtoProof::new(),
            version: 0,
            proposer: Address::default(),
        }
    }
}

impl OpenHeader {
    // TODO: trait
    pub fn from_protobuf(block: &ProtoBlock) -> Self {
        let header = block.get_header();
        let version = block.get_version();
        Self {
            parent_hash: H256::from(header.get_prevhash()),
            timestamp: header.get_timestamp(),
            number: header.get_height(),
            transactions_root: H256::from(header.get_transactions_root()),
            quota_limit: U256::from(header.get_quota_limit()),
            proof: header.get_proof().clone(),
            version,
            proposer: Address::from(header.get_proposer()),
        }
    }

    pub fn is_equivalent(&self, header: &OpenHeader) -> bool {
        self.transactions_root() == header.transactions_root()
            && self.timestamp() == header.timestamp()
            && self.proposer() == header.proposer()
            && self.parent_hash() == header.parent_hash()
            && self.number() == header.number()
            && self.version() == header.version()
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
    /// Get the transactions root field of the header.
    pub fn transactions_root(&self) -> &H256 {
        &self.transactions_root
    }
    /// Get the quota limit field of the header.
    pub fn quota_limit(&self) -> &U256 {
        &self.quota_limit
    }
    /// Get the proof field of the header.
    pub fn proof(&self) -> &ProtoProof {
        &self.proof
    }
    /// Get the version of the block
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Get the proof type field of the header.
    pub fn proof_type(&self) -> Option<ProofType> {
        if self.proof == ProtoProof::new() {
            None
        } else {
            Some(self.proof.get_field_type())
        }
    }
    /// Get the selected proposer address of the header
    pub fn proposer(&self) -> &Address {
        &self.proposer
    }
    /// Set the number field of the header.
    pub fn set_parent_hash(&mut self, a: H256) {
        self.parent_hash = a;
    }
    /// Set the quota limit field of the header.
    pub fn set_quota_limit(&mut self, a: U256) {
        self.quota_limit = a;
    }
    /// Set the version of the header.
    pub fn set_version(&mut self, a: u32) {
        self.version = a;
    }
    /// Set the proof the block.
    pub fn set_proof(&mut self, a: ProtoProof) {
        self.proof = a;
    }
    /// Set the timestamp field of the header.
    pub fn set_timestamp(&mut self, a: u64) {
        self.timestamp = a;
    }
    /// Set the timestamp field of the header to the current time.
    pub fn set_timestamp_now(&mut self, but_later_than: u64) {
        self.timestamp = cmp::max(get_time().sec as u64, but_later_than + 1);
    }
    /// Set the number field of the header.
    pub fn set_number(&mut self, a: BlockNumber) {
        self.number = a;
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Header {
    open_header: OpenHeader,
    /// State root.
    state_root: H256,
    /// Block receipts root.
    receipts_root: H256,
    /// Block bloom.
    log_bloom: LogBloom,
    /// Quota used for contracts execution.
    quota_used: U256,
    /// The hash of the header.
    hash: Option<H256>,
}

impl Deref for Header {
    type Target = OpenHeader;

    fn deref(&self) -> &Self::Target {
        &self.open_header
    }
}

impl DerefMut for Header {
    fn deref_mut(&mut self) -> &mut OpenHeader {
        &mut self.open_header
    }
}

impl PartialEq for Header {
    fn eq(&self, c: &Header) -> bool {
        self.parent_hash() == c.parent_hash()
            && self.timestamp() == c.timestamp()
            && self.number() == c.number()
            && self.transactions_root() == c.transactions_root()
            && self.state_root() == c.state_root()
            && self.receipts_root() == c.receipts_root()
            && self.log_bloom() == c.log_bloom()
            && self.quota_used() == c.quota_used()
            && self.quota_limit() == c.quota_limit()
            && self.proof() == c.proof()
            && self.version() == c.version()
            && self.proposer() == c.proposer()
    }
}

impl Default for Header {
    fn default() -> Self {
        let mut header = Header {
            open_header: OpenHeader::default(),
            state_root: HASH_NULL_RLP,
            receipts_root: HASH_NULL_RLP,
            log_bloom: *ZERO_LOGBLOOM,
            quota_used: U256::default(),
            hash: None,
        };
        header.rehash();
        header
    }
}

impl Header {
    pub fn new(header: OpenHeader) -> Self {
        Header {
            open_header: header,
            state_root: HASH_NULL_RLP,
            receipts_root: HASH_NULL_RLP,
            log_bloom: *ZERO_LOGBLOOM,
            quota_used: U256::default(),
            hash: None,
        }
    }

    pub fn open_header(&self) -> &OpenHeader {
        &self.open_header
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
    /// Get the quota used field of the header.
    pub fn quota_used(&self) -> &U256 {
        &self.quota_used
    }
    /// Set the state root field of the header.
    pub fn set_state_root(&mut self, a: H256) {
        self.state_root = a;
        self.note_dirty();
    }
    /// Set the receipts root field of the header.
    pub fn set_receipts_root(&mut self, a: H256) {
        self.receipts_root = a;
        self.note_dirty();
    }
    /// Set the log bloom field of the header.
    pub fn set_log_bloom(&mut self, a: LogBloom) {
        self.log_bloom = a;
    }
    /// Set the quota used field of the header.
    pub fn set_quota_used(&mut self, a: U256) {
        self.quota_used = a;
        self.note_dirty();
    }
    /// Set the quota limit field of the header.
    pub fn set_quota_limit(&mut self, a: U256) {
        self.quota_limit = a;
        self.note_dirty();
    }
    /// Set the version of the header.
    pub fn set_version(&mut self, a: u32) {
        self.version = a;
        self.note_dirty();
    }
    /// Set the proof the block.
    pub fn set_proof(&mut self, a: ProtoProof) {
        self.proof = a;
        self.note_dirty();
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
    /// Set the number field of the header.
    pub fn set_parent_hash(&mut self, a: H256) {
        self.parent_hash = a;
        self.note_dirty();
    }
    /// Get the hash of this header (sha3 of the RLP).
    pub fn hash(&self) -> Option<H256> {
        self.hash
    }

    pub fn rehash(&mut self) {
        if self.hash().is_none() {
            let h = self.rlp_hash();
            self.hash = Some(h);
        }
    }

    /// Note that some fields have changed. Resets the memoised hash.
    pub fn note_dirty(&mut self) -> &Self {
        self.hash = None;
        self
    }

    // TODO: make these functions traity
    /// Place this header into an RLP stream `s`.
    pub fn stream_rlp(&self, s: &mut RlpStream) {
        s.begin_list(12);
        s.append(&self.parent_hash);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.log_bloom);
        s.append(&self.number);
        s.append(&self.quota_limit);
        s.append(&self.quota_used);
        s.append(&self.timestamp);
        s.append(&self.version);
        s.append(&self.proof);
        s.append(&self.proposer);
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
    pub fn protobuf(&self) -> ProtoBlockHeader {
        let mut bh = ProtoBlockHeader::new();
        bh.set_prevhash(self.parent_hash.to_vec());
        bh.set_timestamp(self.timestamp);
        bh.set_height(self.number);
        bh.set_state_root(self.state_root.to_vec());
        bh.set_receipts_root(self.receipts_root.to_vec());
        bh.set_transactions_root(self.transactions_root.to_vec());
        bh.set_quota_used(u64::from(self.quota_used));
        bh.set_quota_limit(self.quota_limit.low_u64());
        bh.set_proof(self.proof.clone());
        bh.set_proposer(self.proposer.to_vec());
        bh
    }

    /// Generate a header, only set the fields which has been set in new proposal.
    pub fn proposal(&self) -> Header {
        let mut header = Header {
            open_header: OpenHeader {
                parent_hash: self.open_header.parent_hash,
                timestamp: self.open_header.timestamp,
                number: self.open_header.number,
                transactions_root: self.open_header.transactions_root,
                quota_limit: self.open_header.quota_limit,
                proof: self.proof().clone(),
                version: self.open_header.version,
                proposer: self.open_header.proposer,
            },
            hash: None,
            log_bloom: *ZERO_LOGBLOOM,
            state_root: HASH_NULL_RLP,
            receipts_root: HASH_NULL_RLP,
            quota_used: U256::zero(),
        };
        header.rehash();
        header
    }

    /// Generate the protobuf header, only set the fields which has been set in new proposal.
    pub fn proposal_protobuf(&self) -> ProtoBlockHeader {
        let mut bh = ProtoBlockHeader::new();
        bh.set_prevhash(self.parent_hash.to_vec());
        bh.set_timestamp(self.timestamp);
        bh.set_height(self.number);
        bh.set_transactions_root(self.transactions_root.to_vec());
        bh.set_proof(self.proof.clone());
        bh.set_proposer(self.proposer.to_vec());
        bh
    }

    pub fn generate_executed_header(self) -> ExecutedHeader {
        let mut executed_header = ExecutedHeader::new();
        executed_header.set_prevhash(self.parent_hash.to_vec());
        executed_header.set_timestamp(self.timestamp);
        executed_header.set_height(self.number);
        executed_header.set_state_root(self.state_root.to_vec());
        executed_header.set_transactions_root(self.transactions_root.to_vec());
        executed_header.set_receipts_root(self.receipts_root.to_vec());
        executed_header.set_log_bloom(self.log_bloom.to_vec());
        executed_header.set_quota_used(u64::from(self.quota_used));
        executed_header.set_quota_limit(self.quota_limit.low_u64());
        executed_header.set_proposer(self.proposer.to_vec());
        executed_header
    }

    pub fn from_executed_info(info: &ExecutedInfo, open_header: &OpenHeader) -> Header {
        let mut header = Header {
            open_header: OpenHeader {
                number: info.get_header().get_height(),
                quota_limit: U256::from(info.get_header().get_quota_limit()),
                timestamp: info.get_header().get_timestamp(),
                transactions_root: H256::from(info.get_header().get_transactions_root()),
                proof: open_header.proof.clone(),
                proposer: Address::from(info.get_header().get_proposer()),
                version: open_header.version,
                parent_hash: H256::from_slice(info.get_header().get_prevhash()),
            },
            log_bloom: LogBloom::from(info.get_header().get_log_bloom()),
            quota_used: U256::from(info.get_header().get_quota_used()),
            receipts_root: H256::from(info.get_header().get_receipts_root()),
            state_root: H256::from(info.get_header().get_state_root()),
            hash: None,
        };
        header.rehash();
        header
    }

    /// Recover a header from rlp bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        rlp::decode(bytes)
    }
    /// Verify if a header is the next header.
    pub fn verify_next(&self, next: &Header, authorities: &[Address]) -> bool {
        // Calculate block header hash, and is should be same as the parent_hash in next header
        if self.number() + 1 == next.number() {
        } else {
            warn!("verify next block header block number failed");
            return false;
        }
        if self.hash().unwrap() == *next.parent_hash() {
        } else {
            warn!("verify next block header parent hash failed");
            return false;
        };
        let next_proof = BftProof::from(next.proof().clone());
        // Verify block header, use proof.proposal
        if self.number() == 0 || self.proposal_protobuf().crypt_hash() == next_proof.proposal {
        } else {
            warn!("verify next block header proposal failed");
            return false;
        };
        // Verify signatures in proposal proof.
        if next_proof.check(self.number() as usize, authorities) {
        } else {
            warn!("verify signatures for next block header failed");
            return false;
        };
        true
    }
}

impl Decodable for Header {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        let blockheader = Header {
            open_header: OpenHeader {
                parent_hash: r.val_at(0)?,
                transactions_root: r.val_at(2)?,
                number: r.val_at(5)?,
                quota_limit: r.val_at(6)?,
                timestamp: cmp::min(r.val_at::<U256>(8)?, u64::max_value().into()).as_u64(),
                version: r.val_at(9)?,
                proof: r.val_at(10)?,
                proposer: r.val_at(11)?,
            },
            state_root: r.val_at(1)?,
            receipts_root: r.val_at(3)?,
            log_bloom: r.val_at(4)?,
            quota_used: r.val_at(7)?,
            hash: Some(r.as_raw().crypt_hash()),
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
    use super::{Header, OpenHeader};
    use rlp;

    #[test]
    fn decode_and_encode_header() {
        // that's rlp of block header created with ethash engine.
        let open_header = OpenHeader::default();

        let header = Header::new(open_header);
        let header_rlp = rlp::encode(&header).into_vec();
        let header: Header = rlp::decode(&header_rlp);
        let encoded_header = rlp::encode(&header).into_vec();

        assert_eq!(header_rlp, encoded_header);
    }
}
