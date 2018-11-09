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

use header::*;

use cita_types::H256;
use extras::TransactionAddress;
use std::collections::HashMap;

use libproto::blockchain::SignedTransaction as ProtoSignedTransaction;
use libproto::blockchain::{Block as ProtoBlock, BlockBody as ProtoBlockBody};
use rlp::*;
use std::ops::{Deref, DerefMut};

use transaction::SignedTransaction;
use util::HeapSizeOf;

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

impl From<ProtoBlock> for Block {
    fn from(b: ProtoBlock) -> Self {
        let mut header = Header::from(b.get_header().clone());
        header.set_version(b.get_version());
        Block {
            header,
            body: BlockBody::from(b.get_body().clone()),
        }
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
    pub fn new() -> Self {
        Block {
            header: Header::new(),
            body: BlockBody::new(),
        }
    }

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

    pub fn transaction_addresses(&self, hash: H256) -> HashMap<H256, TransactionAddress> {
        let tx_hashs = self.body().transaction_hashes();
        // Create TransactionAddress
        let mut transactions = HashMap::new();
        for (i, tx_hash) in tx_hashs.into_iter().enumerate() {
            let address = TransactionAddress {
                block_hash: hash,
                index: i,
            };
            transactions.insert(tx_hash, address);
        }

        trace!("closed block transactions {:?}", transactions);
        transactions
    }

    /// Check whether the block should re-execute
    // TODO: check version and others
    pub fn is_equivalent(&self, block: &Block) -> bool {
        self.transactions_root() == block.transactions_root()
            && self.timestamp() == block.timestamp()
            && self.proposer() == block.proposer()
    }
}

/// body of block.
#[derive(Default, Debug, Clone, PartialEq, RlpEncodableWrapper, RlpDecodableWrapper)]
pub struct BlockBody {
    /// The transactions in this body.
    pub transactions: Vec<SignedTransaction>,
}

impl HeapSizeOf for BlockBody {
    fn heap_size_of_children(&self) -> usize {
        self.transactions.heap_size_of_children()
    }
}

impl From<ProtoBlockBody> for BlockBody {
    fn from(body: ProtoBlockBody) -> Self {
        BlockBody {
            transactions: body
                .get_transactions()
                .iter()
                .map(|t| SignedTransaction::new(t).expect("transaction can not be converted"))
                .collect(),
        }
    }
}

impl BlockBody {
    pub fn new() -> Self {
        BlockBody {
            transactions: Vec::new(),
        }
    }

    pub fn transactions(&self) -> &[SignedTransaction] {
        &self.transactions
    }

    pub fn set_transactions(&mut self, txs: Vec<SignedTransaction>) {
        self.transactions = txs;
    }

    pub fn protobuf(&self) -> ProtoBlockBody {
        let mut body = ProtoBlockBody::new();
        let txs: Vec<ProtoSignedTransaction> =
            self.transactions.iter().map(|t| t.protobuf()).collect();
        body.set_transactions(txs.into());
        body
    }

    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.transactions().iter().map(|ts| ts.hash()).collect()
    }
}
