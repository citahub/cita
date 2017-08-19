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

use env_info::EnvInfo;
use env_info::LastHashes;
use error::{Error, ExecutionError};
use factory::Factories;
use header::*;
use libchain::chain::TransactionHash;
use libchain::extras::TransactionAddress;

use libproto::blockchain::{Block as ProtoBlock, BlockBody as ProtoBlockBody};
use libproto::blockchain::SignedTransaction as ProtoSignedTransaction;
use protobuf::RepeatedField;
use receipt::Receipt;
use rlp::*;
use state::State;
use state_db::StateDB;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use trace::FlatTrace;
use types::transaction::SignedTransaction;
use util::{U256, H256, merklehash, HeapSizeOf};

/// Trait for a object that has a state database.
pub trait Drain {
    /// Drop this object and return the underlieing database.
    fn drain(self) -> StateDB;
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

impl From<ProtoBlock> for Block {
    fn from(b: ProtoBlock) -> Self {
        let mut header = Header::from(b.get_header().clone());
        header.set_version(b.get_version());
        Block {
            header: header,
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
}


/// body of block.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BlockBody {
    /// The transactions in this body.
    pub transactions: Vec<SignedTransaction>,
}

impl Decodable for BlockBody {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(BlockBody { transactions: rlp.as_list()? })
    }
}

impl Encodable for BlockBody {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.transactions);
    }
}

impl HeapSizeOf for BlockBody {
    fn heap_size_of_children(&self) -> usize {
        self.transactions.heap_size_of_children()
    }
}

impl From<ProtoBlockBody> for BlockBody {
    fn from(body: ProtoBlockBody) -> Self {
        BlockBody {
            transactions: body.get_transactions()
                              .iter()
                              .map(|t| SignedTransaction::new(t).expect("transaction can not be converted"))
                              .collect(),
        }
    }
}

impl BlockBody {
    pub fn new() -> Self {
        BlockBody { transactions: Vec::new() }
    }

    pub fn transactions(&self) -> &[SignedTransaction] {
        &self.transactions
    }

    pub fn set_transactions(&mut self, txs: Vec<SignedTransaction>) {
        self.transactions = txs;
    }

    pub fn protobuf(&self) -> ProtoBlockBody {
        let mut body = ProtoBlockBody::new();
        let txs: Vec<ProtoSignedTransaction> = self.transactions.iter().map(|t| t.protobuf()).collect();
        body.set_transactions(RepeatedField::from_slice(&txs[..]));
        body
    }

    pub fn transaction_hashes(&self) -> Vec<H256> {
        self.transactions().iter().map(|ts| ts.hash()).collect()
    }
}

/// Block that prepared to commit to db.
#[derive(Clone)]
pub struct ClosedBlock {
    /// Protobuf Block
    pub block: Block,
    /// Hash
    pub hash: H256,
    // TODO: cache hash
    pub transactions_uni: HashMap<H256, TransactionAddress>,
    pub transactions_dup: HashMap<H256, TransactionAddress>,
    pub receipts: Vec<Option<Receipt>>,
    pub state: State<StateDB>,
    // TODO: add blocks_blooms
}

impl Drain for ClosedBlock {
    /// Drop this object and return the underlieing database.
    fn drain(self) -> StateDB {
        self.state.drop().1
    }
}

impl Deref for ClosedBlock {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for ClosedBlock {
    fn deref_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}

#[derive(Clone)]
pub struct ExecutedBlock {
    pub block: Block,
    pub receipts: Vec<Option<Receipt>>,
    pub state: State<StateDB>,
    pub gas_used: U256,
    traces: Option<Vec<Vec<FlatTrace>>>,
}

impl Deref for ExecutedBlock {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.block
    }
}

impl DerefMut for ExecutedBlock {
    fn deref_mut(&mut self) -> &mut Block {
        &mut self.block
    }
}

impl ExecutedBlock {
    fn new(block: Block, state: State<StateDB>, tracing: bool) -> ExecutedBlock {
        ExecutedBlock {
            block: block,
            receipts: Default::default(),
            state: state,
            gas_used: U256::default(),
            traces: if tracing { Some(Vec::new()) } else { None },
        }
    }

    pub fn transactions(&self) -> &[SignedTransaction] {
        self.body().transactions()
    }
}

impl TransactionHash for ExecutedBlock {
    fn transaction_hashes(&self) -> Vec<H256> {
        self.body().transaction_hashes()
    }
}

pub struct OpenBlock {
    exec_block: ExecutedBlock,
    last_hashes: Arc<LastHashes>,
    tx_hashes: Vec<bool>,
}

impl Deref for OpenBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &ExecutedBlock {
        &self.exec_block
    }
}

impl DerefMut for OpenBlock {
    fn deref_mut(&mut self) -> &mut ExecutedBlock {
        &mut self.exec_block
    }
}

impl OpenBlock {
    pub fn new(factories: Factories, tracing: bool, block: Block, db: StateDB, state_root: H256, last_hashes: Arc<LastHashes>) -> Result<Self, Error> {
        let state = State::from_existing(db, state_root, U256::default(), factories)?;
        let r = OpenBlock {
            exec_block: ExecutedBlock::new(block, state, tracing),
            last_hashes: last_hashes,
            tx_hashes: Vec::new(),
        };

        Ok(r)
    }

    /// Transaction execution env info.
    pub fn env_info(&self) -> EnvInfo {
        EnvInfo {
            number: self.number(),
            author: self.author().clone(),
            timestamp: self.timestamp(),
            difficulty: U256::default(),
            last_hashes: self.last_hashes.clone(),
            gas_used: self.gas_used().clone(),
            gas_limit: self.gas_limit().clone(),
        }
    }

    // TODO:
    // 1. 存证的Transaction
    // 2. 在Precompile中处理的合约
    pub fn apply_transactions(&mut self) {
        for t in self.body.transactions.clone() {
            self.apply_transaction(&t);
        }
        self.state.commit().expect("commit trie error");
    }

    pub fn apply_transaction(&mut self, t: &SignedTransaction) {
        let env_info = self.env_info();
        let has_traces = self.traces.is_some();
        info!("env_info says gas_used={}", env_info.gas_used);
        match self.state.apply(&env_info, &t, has_traces) {
            Ok(outcome) => {
                let trace = outcome.trace;
                trace!("apply signed transaction {} success", t.hash());
                self.traces.as_mut().map(|tr| tr.push(trace));
                self.set_gas_used(outcome.receipt.gas_used);
                self.receipts.push(Some(outcome.receipt));
                self.tx_hashes.push(false);
            }
            Err(Error::Execution(ExecutionError::InvalidNonce { expected: _, got: _ })) => {
                self.receipts.push(None);
                self.tx_hashes.push(true);
            }
            Err(_) => {
                self.receipts.push(None);
                self.tx_hashes.push(false);
            } 
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(&mut self) -> ClosedBlock {
        let tx_hashs = self.body().transaction_hashes();

        // Rebuild block
        let state_root = self.state.root().clone();
        let receipts_root = merklehash::complete_merkle_root(self.receipts.iter().map(|r| r.rlp_bytes().to_vec()));
        self.set_state_root(state_root);
        self.set_receipts_root(receipts_root);

        // Create TransactionAddress
        let hash = self.hash();
        let mut transactions_uni = HashMap::new();
        let mut transactions_dup = HashMap::new();
        for (i, tx_hash) in tx_hashs.into_iter().enumerate() {
            let address = TransactionAddress { block_hash: hash, index: i };
            if self.tx_hashes[i] {
                transactions_dup.insert(tx_hash, address);
            } else {
                transactions_uni.insert(tx_hash, address);
            }
        }

        ClosedBlock {
            block: self.block.clone(),
            hash: hash,
            transactions_uni: transactions_uni,
            transactions_dup: transactions_dup,
            receipts: self.receipts.clone(),
            state: self.state.clone(),
        }
    }
}
