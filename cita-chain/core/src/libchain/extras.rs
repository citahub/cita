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

//! Blockchain DB extras.

use basic_types::LogBloomGroup;
use bloomchain::group::GroupPosition;
use cita_types::{H256, H264};
use db::Key;
use header::{BlockNumber, Header};
use libchain::block::BlockBody;
use libproto::blockchain::Proof;
use receipt::Receipt;
use rlp::*;
use std::ops::{Deref, Index};
use util::*;

/// Represents index of extra data in database
#[derive(Copy, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExtrasIndex {
    /// Transaction address index
    TransactionAddress = 0,
    /// Block receipts index
    BlockReceipts = 1,
    /// Block blooms index
    BlocksBlooms = 2,
    /// Block hash index
    BlockHash = 3,
    /// Block head index
    BlockHeadHash = 4,
    /// Block body index
    BlockBodyHash = 5,
}

pub struct CurrentHash;

impl Key<H256> for CurrentHash {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f66")
    }
}

pub struct CurrentProof;

impl Key<Proof> for CurrentProof {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f67")
    }
}

pub struct CurrentHeight;

impl Key<BlockNumber> for CurrentHeight {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f68")
    }
}

impl Key<Header> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

impl Key<BlockBody> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

impl Key<BlockNumber> for H256 {
    type Target = H256;

    fn key(&self) -> H256 {
        *self
    }
}

pub struct BlockNumberKey([u8; 5]);

impl Deref for BlockNumberKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct BlockNumberKeyLong([u8; 9]);

impl Deref for BlockNumberKeyLong {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Key<Header> for BlockNumber {
    type Target = BlockNumberKeyLong;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockHeadHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl Key<BlockBody> for BlockNumber {
    type Target = BlockNumberKeyLong;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 9];
        result[0] = ExtrasIndex::BlockBodyHash as u8;
        result[1] = (self >> 56) as u8;
        result[2] = (self >> 48) as u8;
        result[3] = (self >> 40) as u8;
        result[4] = (self >> 32) as u8;
        result[5] = (self >> 24) as u8;
        result[6] = (self >> 16) as u8;
        result[7] = (self >> 8) as u8;
        result[8] = *self as u8;
        BlockNumberKeyLong(result)
    }
}

impl Key<H256> for BlockNumber {
    type Target = BlockNumberKey;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 5];
        result[0] = ExtrasIndex::BlockHash as u8;
        result[1] = (self >> 24) as u8;
        result[2] = (self >> 16) as u8;
        result[3] = (self >> 8) as u8;
        result[4] = *self as u8;
        BlockNumberKey(result)
    }
}

fn with_index(hash: &H256, i: ExtrasIndex) -> H264 {
    let mut result = H264::default();
    result[0] = i as u8;
    (*result)[1..].clone_from_slice(hash);
    result
}

impl Key<TransactionAddress> for H256 {
    type Target = H264;

    fn key(&self) -> H264 {
        with_index(self, ExtrasIndex::TransactionAddress)
    }
}

impl Key<BlockReceipts> for H256 {
    type Target = H264;

    fn key(&self) -> H264 {
        with_index(self, ExtrasIndex::BlockReceipts)
    }
}

pub struct LogGroupKey([u8; 6]);

impl Deref for LogGroupKey {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LogGroupPosition(GroupPosition);

impl From<GroupPosition> for LogGroupPosition {
    fn from(position: GroupPosition) -> Self {
        LogGroupPosition(position)
    }
}

impl HeapSizeOf for LogGroupPosition {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl Key<LogBloomGroup> for LogGroupPosition {
    type Target = LogGroupKey;

    fn key(&self) -> Self::Target {
        let mut result = [0u8; 6];
        result[0] = ExtrasIndex::BlocksBlooms as u8;
        result[1] = self.0.level as u8;
        result[2] = (self.0.index >> 24) as u8;
        result[3] = (self.0.index >> 16) as u8;
        result[4] = (self.0.index >> 8) as u8;
        result[5] = self.0.index as u8;
        LogGroupKey(result)
    }
}

/// Represents address of certain transaction within block
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAddress {
    /// Block hash
    pub block_hash: H256,
    /// Transaction index within the block
    pub index: usize,
}

impl HeapSizeOf for TransactionAddress {
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl Decodable for TransactionAddress {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        let tx_address = TransactionAddress {
            block_hash: rlp.val_at(0)?,
            index: rlp.val_at(1)?,
        };

        Ok(tx_address)
    }
}

impl Encodable for TransactionAddress {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.block_hash);
        s.append(&self.index);
    }
}

/// Contains all block receipts.
#[derive(Clone)]
pub struct BlockReceipts {
    pub receipts: Vec<Receipt>,
}

impl BlockReceipts {
    pub fn new(receipts: Vec<Receipt>) -> Self {
        BlockReceipts { receipts }
    }
}

impl Decodable for BlockReceipts {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(BlockReceipts {
            receipts: rlp.as_list()?,
        })
    }
}

impl Encodable for BlockReceipts {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.receipts);
    }
}

impl HeapSizeOf for BlockReceipts {
    fn heap_size_of_children(&self) -> usize {
        self.receipts.heap_size_of_children()
    }
}

impl Index<usize> for BlockReceipts {
    type Output = Receipt;
    fn index(&self, i: usize) -> &Receipt {
        &self.receipts[i]
    }
}

#[cfg(test)]
mod tests {
    use super::BlockReceipts;
    use rlp::*;

    #[test]
    fn encode_block_receipts() {
        let br = BlockReceipts::new(Vec::new());

        let mut s = RlpStream::new_list(2);
        s.append(&br);
        assert!(!s.is_finished(), "List shouldn't finished yet");
        s.append(&br);
        assert!(s.is_finished(), "List should be finished now");
        s.out();
    }
}
