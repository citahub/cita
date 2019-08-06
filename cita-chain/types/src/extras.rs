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

use crate::block::BlockBody;
use crate::block_number::BlockNumber;
use crate::block_receipts::BlockReceipts;
use crate::header::Header;
use crate::log_blooms::LogBloomGroup;
use crate::receipt::Receipt;
use crate::transaction_index::TransactionIndex;
use bloomchain::group::GroupPosition;
use cita_types::{H256, H264};
use libproto::blockchain::Proof;
use std::ops::{Deref, Index};

/// Represents index of extra data in database
#[derive(Copy, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExtrasIndex {
    /// Transaction address index
    TransactionIndex = 0,
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

pub trait DBIndex<T> {
    type Item: Deref<Target = [u8]>;

    fn get_index(&self) -> Self::Item;
}

pub struct CurrentHash;

impl DBIndex<H256> for CurrentHash {
    type Item = H256;

    fn get_index(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f66")
    }
}

pub struct CurrentProof;

impl DBIndex<Proof> for CurrentProof {
    type Item = H256;

    fn get_index(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f67")
    }
}

pub struct CurrentHeight;

impl DBIndex<BlockNumber> for CurrentHeight {
    type Item = H256;

    fn get_index(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f68")
    }
}

impl DBIndex<Header> for H256 {
    type Item = H256;

    fn get_index(&self) -> H256 {
        *self
    }
}

impl DBIndex<BlockBody> for H256 {
    type Item = H256;

    fn get_index(&self) -> H256 {
        *self
    }
}

impl DBIndex<BlockNumber> for H256 {
    type Item = H256;

    fn get_index(&self) -> H256 {
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

impl DBIndex<Header> for BlockNumber {
    type Item = BlockNumberKeyLong;

    fn get_index(&self) -> Self::Item {
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

impl DBIndex<BlockBody> for BlockNumber {
    type Item = BlockNumberKeyLong;

    fn get_index(&self) -> Self::Item {
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

impl DBIndex<H256> for BlockNumber {
    type Item = BlockNumberKey;

    fn get_index(&self) -> Self::Item {
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

impl DBIndex<TransactionIndex> for H256 {
    type Item = H264;

    fn get_index(&self) -> H264 {
        with_index(self, ExtrasIndex::TransactionIndex)
    }
}

impl DBIndex<BlockReceipts> for H256 {
    type Item = H264;

    fn get_index(&self) -> H264 {
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

impl DBIndex<LogBloomGroup> for LogGroupPosition {
    type Item = LogGroupKey;

    fn get_index(&self) -> Self::Item {
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

impl Index<usize> for BlockReceipts {
    type Output = Receipt;
    fn index(&self, i: usize) -> &Receipt {
        &self.receipts[i]
    }
}
