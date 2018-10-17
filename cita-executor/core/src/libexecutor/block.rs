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

use basic_types::LogBloom;
use cita_types::{Address, H256, U256};
use contracts::solc::PriceManagement;
use engines::Engine;
use error::Error;
use evm::env_info::{EnvInfo, LastHashes};
use factory::Factories;
use header::*;
use libexecutor::executor::{CheckOptions, EconomicalModel, Executor, GlobalSysConfig};
use libproto::blockchain::SignedTransaction as ProtoSignedTransaction;
use libproto::blockchain::{Block as ProtoBlock, BlockBody as ProtoBlockBody};
use libproto::executor::{ExecutedInfo, ReceiptWithOption};
use receipt::Receipt;
use rlp::*;
use state::State;
use state_db::StateDB;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Instant;
use trace::FlatTrace;
use types::ids::BlockId;
use types::transaction::SignedTransaction;
use util::{merklehash, HeapSizeOf};

/// Check the 256 transactions once
const CHECK_NUM: usize = 0xff;

lazy_static! {
    /// Block Reward
    /// HardFork if need to change block reward
    pub static ref BLOCK_REWARD: U256 = U256::from(5_000_000_000_000_000_000 as i64);
}

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

    /// Check whether the block should re-execute
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

/// Block that prepared to commit to db.
#[derive(Clone, Debug)]
pub struct ClosedBlock {
    /// Protobuf Block
    pub block: OpenBlock,
}

impl ClosedBlock {
    pub fn protobuf(&self) -> ExecutedInfo {
        let mut executed_info = ExecutedInfo::new();

        executed_info
            .mut_header()
            .set_prevhash(self.parent_hash().to_vec());
        executed_info.mut_header().set_timestamp(self.timestamp());
        executed_info.mut_header().set_height(self.number());
        executed_info
            .mut_header()
            .set_state_root(self.state_root().to_vec());
        executed_info
            .mut_header()
            .set_transactions_root(self.transactions_root().to_vec());
        executed_info
            .mut_header()
            .set_receipts_root(self.receipts_root().to_vec());
        executed_info
            .mut_header()
            .set_log_bloom(self.log_bloom().to_vec());
        executed_info
            .mut_header()
            .set_quota_used(u64::from(*self.quota_used()));
        executed_info
            .mut_header()
            .set_quota_limit(self.gas_limit().low_u64());

        executed_info.receipts = self
            .receipts
            .clone()
            .into_iter()
            .map(|receipt| {
                let mut receipt_proto_option = ReceiptWithOption::new();
                receipt_proto_option.set_receipt(receipt.protobuf());
                receipt_proto_option
            })
            .collect();
        executed_info
            .mut_header()
            .set_proposer(self.proposer().to_vec());
        executed_info
    }
}

impl Drain for ClosedBlock {
    /// Drop this object and return the underlieing database.
    fn drain(self) -> StateDB {
        self.block.drain()
    }
}

impl Deref for ClosedBlock {
    type Target = OpenBlock;

    fn deref(&self) -> &OpenBlock {
        &self.block
    }
}

impl DerefMut for ClosedBlock {
    fn deref_mut(&mut self) -> &mut OpenBlock {
        &mut self.block
    }
}

#[derive(Clone, Debug)]
pub struct ExecutedBlock {
    pub block: Block,
    pub receipts: Vec<Receipt>,
    pub state: State<StateDB>,
    pub current_quota_used: U256,
    traces: Option<Vec<Vec<FlatTrace>>>,
}

impl Drain for ExecutedBlock {
    fn drain(self) -> StateDB {
        self.state.drop().1
    }
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
            block,
            receipts: Default::default(),
            state,
            current_quota_used: U256::zero(),
            traces: if tracing { Some(Vec::new()) } else { None },
        }
    }

    pub fn transactions(&self) -> &[SignedTransaction] {
        self.body().transactions()
    }
}

#[derive(Clone, Debug)]
pub struct OpenBlock {
    exec_block: ExecutedBlock,
    last_hashes: Arc<LastHashes>,
    account_gas_limit: U256,
    account_gas: HashMap<Address, U256>,
}

impl Drain for OpenBlock {
    fn drain(self) -> StateDB {
        self.exec_block.drain()
    }
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
    pub fn new(
        factories: Factories,
        conf: GlobalSysConfig,
        tracing: bool,
        block: Block,
        db: StateDB,
        state_root: H256,
        last_hashes: Arc<LastHashes>,
    ) -> Result<Self, Error> {
        let mut state = State::from_existing(db, state_root, U256::default(), factories)?;
        state.account_permissions = conf.account_permissions;
        state.group_accounts = conf.group_accounts;
        state.super_admin_account = conf.super_admin_account;

        let r = OpenBlock {
            exec_block: ExecutedBlock::new(block, state, tracing),
            last_hashes,
            account_gas_limit: conf.account_gas_limit.common_gas_limit.into(),
            account_gas: conf.account_gas_limit.specific_gas_limit.iter().fold(
                HashMap::new(),
                |mut acc, (key, value)| {
                    acc.insert(*key, (*value).into());
                    acc
                },
            ),
        };

        Ok(r)
    }

    /// Transaction execution env info.
    pub fn env_info(&self) -> EnvInfo {
        EnvInfo {
            number: self.number(),
            author: *self.header.proposer(),
            timestamp: self.timestamp(),
            difficulty: U256::default(),
            last_hashes: Arc::clone(&self.last_hashes),
            gas_used: self.current_quota_used,
            gas_limit: *self.gas_limit(),
            account_gas_limit: 0.into(),
        }
    }

    /// Execute transactions
    /// Return false if be interrupted
    pub fn apply_transactions(
        &mut self,
        executor: &Executor,
        chain_owner: Address,
        check_options: &CheckOptions,
    ) -> bool {
        let price_management = PriceManagement::new(executor);
        let quota_price = price_management
            .quota_price(BlockId::Pending)
            .unwrap_or_else(PriceManagement::default_quota_price);
        for (index, mut t) in self.body.transactions.clone().into_iter().enumerate() {
            if index & CHECK_NUM == 0 && executor.is_interrupted.load(Ordering::SeqCst) {
                executor.is_interrupted.store(false, Ordering::SeqCst);
                return false;
            }

            let economical_model: EconomicalModel = *executor.economical_model.read();
            if economical_model == EconomicalModel::Charge {
                t.gas_price = quota_price;
            }

            self.apply_transaction(
                &*executor.engine,
                &t,
                *executor.economical_model.read(),
                chain_owner,
                check_options,
            );
        }

        let now = Instant::now();
        self.state.commit().expect("commit trie error");
        let new_now = Instant::now();
        debug!("state root use {:?}", new_now.duration_since(now));

        let quota_used = self.current_quota_used;
        self.set_quota_used(quota_used);
        true
    }

    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn apply_transaction(
        &mut self,
        engine: &Engine,
        t: &SignedTransaction,
        economical_model: EconomicalModel,
        chain_owner: Address,
        check_options: &CheckOptions,
    ) {
        let mut env_info = self.env_info();
        self.account_gas
            .entry(*t.sender())
            .or_insert(self.account_gas_limit);
        env_info.account_gas_limit = *self
            .account_gas
            .get(t.sender())
            .expect("account should exist in account_gas_limit");

        let has_traces = self.traces.is_some();
        match self.state.apply(
            &env_info,
            engine,
            t,
            has_traces,
            economical_model,
            chain_owner,
            check_options,
        ) {
            Ok(outcome) => {
                trace!("apply signed transaction {} success", t.hash());
                if let Some(ref mut traces) = self.traces {
                    traces.push(outcome.trace);
                }
                let transaction_quota_used = outcome.receipt.quota_used - self.current_quota_used;
                self.current_quota_used = outcome.receipt.quota_used;
                if check_options.quota {
                    if let Some(value) = self.account_gas.get_mut(t.sender()) {
                        *value = *value - transaction_quota_used;
                    }
                }
                self.receipts.push(outcome.receipt);
            }
            _ => panic!("apply_transaction: There must be something wrong!"),
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(mut self) -> ClosedBlock {
        // Rebuild block
        let state_root = *self.state.root();
        self.set_state_root(state_root);
        let receipts_root = merklehash::MerkleTree::from_bytes(
            self.receipts.iter().map(|r| r.rlp_bytes().to_vec()),
        )
        .get_root_hash();
        self.set_receipts_root(receipts_root);

        // blocks blooms
        let log_bloom = self
            .receipts
            .clone()
            .into_iter()
            .fold(LogBloom::zero(), |mut b, r| {
                b = b | r.log_bloom;
                b
            });

        self.set_log_bloom(log_bloom);

        ClosedBlock { block: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlp;

    #[test]
    fn test_encode_and_decode() {
        let mut stx = SignedTransaction::default();
        stx.data = vec![1; 200];
        let transactions = vec![stx; 200];
        let body = BlockBody { transactions };
        let body_rlp = rlp::encode(&body);
        let body: BlockBody = rlp::decode(&body_rlp);
        let body_encoded = rlp::encode(&body).into_vec();

        assert_eq!(body_rlp, body_encoded);
    }
}
