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

use cita_transaction::eth_transaction::SignedTransaction;
use env_info::EnvInfo;
use env_info::LastHashes;
use error::{Error, ExecutionError};
use factory::Factories;
use libchain::chain::TransactionHash;
use libchain::extras::TransactionAddress;

pub use libproto::blockchain::{Block, BlockHeader, BlockBody, Status};
use libproto::blockchain::SignedTransaction as ProtoSignedTransaction;
use receipt::Receipt;
use rlp::Encodable;
use state::State;
use state_db::StateDB;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use trace::FlatTrace;
use util::{Address, U256, H256, merklehash};

/// Trait for a object that has a state database.
pub trait Drain {
    /// Drop this object and return the underlieing database.
    fn drain(self) -> StateDB;
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

    pub fn header(&self) -> &BlockHeader {
        self.block.get_header()
    }

    pub fn transactions(&self) -> &[ProtoSignedTransaction] {
        let body = self.block.get_body();
        body.get_transactions()
    }
}

impl TransactionHash for ExecutedBlock {
    fn transaction_hashes(&self) -> Vec<H256> {
        self.transactions().iter().map(|ts| H256::from_slice(ts.get_tx_hash())).collect()
    }
}

pub struct OpenBlock {
    block: ExecutedBlock,
    last_hashes: Arc<LastHashes>,
    tx_hashes: Vec<bool>,
}

impl Deref for OpenBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &ExecutedBlock {
        &self.block
    }
}

impl OpenBlock {
    pub fn new(factories: Factories, tracing: bool, block: Block, db: StateDB, state_root: H256, last_hashes: Arc<LastHashes>) -> Result<Self, Error> {
        let state = State::from_existing(db, state_root, U256::default(), factories)?;
        let r = OpenBlock {
            block: ExecutedBlock::new(block, state, tracing),
            last_hashes: last_hashes,
            tx_hashes: Vec::new(),
        };

        Ok(r)
    }

    /// Transaction execution env info.
    pub fn env_info(&self) -> EnvInfo {
        let header = self.block.header();

        EnvInfo {
            number: header.get_height(),
            author: Address::default(),
            timestamp: header.get_timestamp(),
            difficulty: U256::default(),
            last_hashes: self.last_hashes.clone(),
            gas_used: self.block.gas_used,
            gas_limit: U256::from(u64::max_value()),
        }
    }

    // TODO:
    // 1. 存证的Transaction
    // 2. 在Precompile中处理的合约
    pub fn apply_transactions(&mut self) {
        let mut block = self.block.block.clone();
        let mut body = block.take_body();
        let transactions = body.take_transactions();
        for t in transactions.into_iter() {
            match SignedTransaction::new(t) {
                Ok(signed_transaction) => self.apply_transaction(&signed_transaction),
                _ => {
                    self.block.receipts.push(None);
                    self.tx_hashes.push(true);
                }
            }
        }
        self.block.state.commit().expect("commit trie error");
    }

    pub fn apply_transaction(&mut self, t: &SignedTransaction) {
        let env_info = self.env_info();
        info!("env_info says gas_used={}", env_info.gas_used);
        match self.block.state.apply(&env_info, &t, self.block.traces.is_some()) {
            Ok(outcome) => {
                trace!("apply signed transaction {} success", t.hash);
                let t = outcome.trace;
                self.block.traces.as_mut().map(|traces| traces.push(t));
                self.block.gas_used = outcome.receipt.gas_used;
                self.block.receipts.push(Some(outcome.receipt));
                self.tx_hashes.push(false);
            }
            Err(Error::Execution(ExecutionError::InvalidNonce { expected: _, got: _ })) => {
                self.block.receipts.push(None);
                self.tx_hashes.push(true);
            }
            Err(_) => {
                self.block.receipts.push(None);
                self.tx_hashes.push(false);
            }
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(&self) -> ClosedBlock {
        let mut block = self.block.block.clone();
        let mut header = block.take_header();
        let tx_hashs = self.block.transaction_hashes();

        // Rebuild block
        header.set_state_root(self.block.state.root().to_vec().clone());
        header.set_receipts_root(merklehash::complete_merkle_root(self.block.receipts.iter().map(|r| r.rlp_bytes().to_vec())).to_vec());
        block.set_header(header);

        // Create TransactionAddress
        let hash = block.crypt_hash();
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
            block: block.clone(),
            hash: hash,
            transactions_uni: transactions_uni,
            transactions_dup: transactions_dup,
            receipts: self.block.receipts.clone(),
            state: self.state.clone(),
        }
    }
}
