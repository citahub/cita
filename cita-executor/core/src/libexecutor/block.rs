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

use basic_types::LogBloom;
use env_info::EnvInfo;
use env_info::LastHashes;
use error::{Error, ExecutionError};
use factory::Factories;
use header::*;
use libexecutor::executor::Executor;
use libexecutor::executor::GlobalSysConfig;
use libproto::blockchain::{Block as ProtoBlock, BlockBody as ProtoBlockBody};
use libproto::blockchain::SignedTransaction as ProtoSignedTransaction;
use libproto::executor::{ExecutedInfo, ReceiptWithOption};
use protobuf::RepeatedField;
use receipt::{Receipt, ReceiptError};
use rlp::*;
use state::State;
use state_db::StateDB;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Instant;
use trace::FlatTrace;
use types::transaction::SignedTransaction;
use util::{merklehash, Address, H256, HeapSizeOf, U256};

/// Check the 256 transactions once
const CHECK_NUM: usize = 0xff;

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
            transactions: body.get_transactions()
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
        let txs: Vec<ProtoSignedTransaction> = self.transactions.iter().map(|t| t.protobuf()).collect();
        body.set_transactions(RepeatedField::from_slice(&txs[..]));
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
            .set_gas_used(u64::from(*self.gas_used()));
        executed_info
            .mut_header()
            .set_gas_limit(self.gas_limit().low_u64());

        executed_info.receipts = self.receipts
            .clone()
            .into_iter()
            .map(|receipt| {
                let mut receipt_proto_option = ReceiptWithOption::new();
                if let Some(value) = receipt {
                    receipt_proto_option.set_receipt(value.protobuf());
                }
                receipt_proto_option
            })
            .collect();
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
    pub receipts: Vec<Option<Receipt>>,
    pub state: State<StateDB>,
    pub current_gas_used: U256,
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
            block: block,
            receipts: Default::default(),
            state: state,
            current_gas_used: U256::zero(),
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

        let r = OpenBlock {
            exec_block: ExecutedBlock::new(block, state, tracing),
            last_hashes: last_hashes,
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
            author: Address::default(),
            timestamp: self.timestamp(),
            difficulty: U256::default(),
            last_hashes: Arc::clone(&self.last_hashes),
            gas_used: self.current_gas_used,
            gas_limit: *self.gas_limit(),
            account_gas_limit: 0.into(),
        }
    }

    /// Execute transactions
    pub fn apply_transactions(&mut self, executor: &Executor, check_permission: bool, check_quota: bool) -> bool {
        for (index, t) in self.body.transactions.clone().into_iter().enumerate() {
            if index & CHECK_NUM == 0 {
                if executor.is_interrupted.load(Ordering::SeqCst) {
                    return false;
                }
            }
            // Apply transaction and set account nonce
            self.apply_transaction(&t, check_permission, check_quota);
        }

        let now = Instant::now();
        self.state.commit().expect("commit trie error");
        let new_now = Instant::now();
        debug!("state root use {:?}", new_now.duration_since(now));

        let gas_used = self.current_gas_used;
        self.set_gas_used(gas_used);
        true
    }

    pub fn apply_transaction(&mut self, t: &SignedTransaction, check_permission: bool, check_quota: bool) {
        let mut env_info = self.env_info();
        if !self.account_gas.contains_key(t.sender()) {
            self.account_gas.insert(*t.sender(), self.account_gas_limit);
            env_info.account_gas_limit = self.account_gas_limit;
        }
        env_info.account_gas_limit = *self.account_gas
            .get(t.sender())
            .expect("account should exist in account_gas_limit");

        let has_traces = self.traces.is_some();
        match self.state
            .apply(&env_info, t, has_traces, check_permission, check_quota)
        {
            Ok(outcome) => {
                let trace = outcome.trace;
                trace!("apply signed transaction {} success", t.hash());
                self.traces.as_mut().map(|tr| tr.push(trace));
                let transaction_gas_used = outcome.receipt.gas_used - self.current_gas_used;
                self.current_gas_used = outcome.receipt.gas_used;
                if check_quota {
                    if let Some(value) = self.account_gas.get_mut(t.sender()) {
                        *value = *value - transaction_gas_used;
                    }
                }
                self.receipts.push(Some(outcome.receipt));
            }
            Err(Error::Execution(execution_error)) => {
                self.receipts.push(
                    match execution_error {
                        ExecutionError::NoTransactionPermission => Some(ReceiptError::NoTransactionPermission),
                        ExecutionError::NoContractPermission => Some(ReceiptError::NoContractPermission),
                        ExecutionError::NoCallPermission => Some(ReceiptError::NoCallPermission),
                        ExecutionError::NotEnoughBaseGas { .. } => Some(ReceiptError::NotEnoughBaseGas),
                        ExecutionError::BlockGasLimitReached { .. } => Some(ReceiptError::BlockGasLimitReached),
                        ExecutionError::AccountGasLimitReached { .. } => Some(ReceiptError::AccountGasLimitReached),
                        _ => None,
                    }.map(|receipt_error| {
                        Receipt::new(
                            None,
                            0.into(),
                            Vec::new(),
                            Some(receipt_error),
                            0.into(),
                            t.get_transaction_hash(),
                        )
                    }),
                );
            }
            Err(_) => {
                self.receipts.push(None);
            }
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn into_closed_block(mut self) -> ClosedBlock {
        // Rebuild block
        let state_root = *self.state.root();
        let receipts_root =
            merklehash::MerkleTree::from_bytes(self.receipts.iter().map(|r| r.rlp_bytes().to_vec())).get_root_hash();
        self.set_state_root(state_root);
        self.set_receipts_root(receipts_root);

        // blocks blooms
        let log_bloom = self.receipts
            .clone()
            .into_iter()
            .filter_map(|r| r)
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
        let body = BlockBody {
            transactions: transactions,
        };
        let body_rlp = rlp::encode(&body);
        let body: BlockBody = rlp::decode(&body_rlp);
        let body_encoded = rlp::encode(&body).into_vec();

        assert_eq!(body_rlp, body_encoded);
    }

}
