// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::header::{Header, OpenHeader};

use crate::transaction_index::TransactionIndex;
use cita_types::H256;
use std::collections::HashMap;

use libproto::blockchain::{
    Block as ProtoBlock, BlockBody as ProtoBlockBody, SignedTransaction as ProtoSignedTransaction,
};
use rlp::*;
use std::ops::{Deref, DerefMut};

use crate::transaction::SignedTransaction;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct OpenBlock {
    /// The header of this block.
    pub header: OpenHeader,
    /// The body of this block.
    pub body: BlockBody,
}

impl From<ProtoBlock> for OpenBlock {
    fn from(b: ProtoBlock) -> Self {
        let header = OpenHeader::from_protobuf(&b);
        Self {
            header,
            body: BlockBody::from(b.get_body().clone()),
        }
    }
}

impl Deref for OpenBlock {
    type Target = OpenHeader;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl DerefMut for OpenBlock {
    fn deref_mut(&mut self) -> &mut OpenHeader {
        &mut self.header
    }
}

impl OpenBlock {
    pub fn body(&self) -> &BlockBody {
        &self.body
    }

    pub fn header(&self) -> &OpenHeader {
        &self.header
    }

    pub fn set_header(&mut self, h: OpenHeader) {
        self.header = h;
    }

    pub fn set_body(&mut self, b: BlockBody) {
        self.body = b;
    }
}

/// A block, encoded as it is on the block chain.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block {
    /// The header of this block.
    pub header: Header,
    /// The body of this block.
    pub body: BlockBody,
}

impl Decodable for Block {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        if r.item_count()? != 2 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(Block {
            header: r.val_at(0)?,
            body: r.val_at(1)?,
        })
    }
}

impl Encodable for Block {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.header);
        s.append(&self.body);
    }
}

impl Deref for Block {
    type Target = Header;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl Block {
    pub fn body(&self) -> &BlockBody {
        &self.body
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn set_header(&mut self, h: Header) {
        self.header = h;
    }

    pub fn set_body(&mut self, b: BlockBody) {
        self.body = b;
    }

    pub fn protobuf(&self) -> ProtoBlock {
        let mut block = ProtoBlock::new();
        block.set_version(self.version());
        block.set_header(self.header.protobuf());
        block.set_body(self.body.protobuf());
        block
    }

    pub fn new(block: OpenBlock) -> Self {
        let header = Header::new(block.header);
        Self {
            header,
            body: block.body,
        }
    }
}

/// body of block.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockBody {
    /// The transactions in this body.
    pub transactions: Vec<SignedTransaction>,
}

impl Encodable for BlockBody {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.append_list(&self.transactions);
    }
}

impl Decodable for BlockBody {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        let block_body = BlockBody {
            transactions: r.as_list()?,
        };

        Ok(block_body)
    }
}

impl From<ProtoBlockBody> for BlockBody {
    fn from(body: ProtoBlockBody) -> Self {
        BlockBody {
            transactions: body
                .get_transactions()
                .iter()
                .map(|t| SignedTransaction::create(t).expect("transaction can not be converted"))
                .collect(),
        }
    }
}

impl BlockBody {
    pub fn transactions(&self) -> &[SignedTransaction] {
        &self.transactions
    }

    pub fn set_transactions(&mut self, txs: Vec<SignedTransaction>) {
        self.transactions = txs;
    }

    pub fn protobuf(&self) -> ProtoBlockBody {
        let mut body = ProtoBlockBody::new();
        let txs: Vec<ProtoSignedTransaction> = self
            .transactions
            .iter()
            .map(SignedTransaction::protobuf)
            .collect();
        body.set_transactions(txs.into());
        body
    }

    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.transactions().iter().map(|ts| ts.hash()).collect()
    }

    pub fn transaction_indexes(&self, hash: H256) -> HashMap<H256, TransactionIndex> {
        let tx_hashs = self.transaction_hashes();
        // Create TransactionIndex
        let mut tx_indexes = HashMap::new();
        for (i, tx_hash) in tx_hashs.into_iter().enumerate() {
            let index = TransactionIndex {
                block_hash: hash,
                index: i,
            };
            tx_indexes.insert(tx_hash, index);
        }

        trace!("closed block transactions {:?}", tx_indexes);
        tx_indexes
    }
}
