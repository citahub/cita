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

use crate::authentication::AuthenticationError;
use crate::basic_types::LogBloom;
use crate::cita_executive::{CitaExecutive, EnvInfo, ExecutionError};
use crate::contracts::native::factory::Factory as NativeFactory;
use crate::error::Error;
// use crate::libexecutor::auto_exec::auto_exec;
use crate::core::env_info::LastHashes;
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::executor::CitaTrieDB;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::receipt::{Receipt, ReceiptError};
use crate::tx_gas_schedule::TxGasSchedule;
pub use crate::types::block::{Block, BlockBody, OpenBlock};
use crate::types::transaction::SignedTransaction;
use cita_merklehash;
use cita_types::{Address, H256, U256};
use cita_vm::BlockDataProvider;
use cita_vm::{evm::Error as EVMError, state::State as CitaState, Error as VMError};
use hashable::Hashable;
use libproto::executor::{ExecutedInfo, ReceiptWithOption};
use rlp::*;
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

lazy_static! {
    /// Block Reward
    /// HardFork if need to change block reward
    pub static ref BLOCK_REWARD: U256 = U256::from(5_000_000_000_000_000_000 as i64);
}

/// Trait for a object that has a state database.
pub trait Drain {
    /// Drop this object.
    fn drain(self);
}

pub struct ExecutedBlock {
    pub block: OpenBlock,
    pub receipts: Vec<Receipt>,
    pub state: Arc<RefCell<CitaState<CitaTrieDB>>>,
    pub current_quota_used: U256,
    pub state_root: H256,
    last_hashes: Arc<LastHashes>,
    account_gas_limit: U256,
    account_gas: HashMap<Address, U256>,
    eth_compatibility: bool,
}

impl Deref for ExecutedBlock {
    type Target = OpenBlock;

    fn deref(&self) -> &OpenBlock {
        &self.block
    }
}

impl DerefMut for ExecutedBlock {
    fn deref_mut(&mut self) -> &mut OpenBlock {
        &mut self.block
    }
}

impl ExecutedBlock {
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        conf: &BlockSysConfig,
        block: OpenBlock,
        trie_db: Arc<CitaTrieDB>,
        state_root: H256,
        last_hashes: Arc<LastHashes>,
        eth_compatibility: bool,
    ) -> Result<Self, Error> {
        let state = CitaState::from_existing(Arc::<CitaTrieDB>::clone(&trie_db), state_root)
            .expect("Get state from trie db");

        // Need only one state reference for the whole block transaction.
        let state = Arc::new(RefCell::new(state));

        let r = ExecutedBlock {
            block,
            state,
            state_root,
            last_hashes,
            account_gas_limit: conf.account_quota_limit.common_quota_limit.into(),
            account_gas: conf.account_quota_limit.specific_quota_limit.iter().fold(
                HashMap::new(),
                |mut acc, (key, value)| {
                    acc.insert(*key, (*value).into());
                    acc
                },
            ),
            current_quota_used: Default::default(),
            receipts: Default::default(),
            eth_compatibility,
        };

        Ok(r)
    }

    pub fn transactions(&self) -> &[SignedTransaction] {
        self.body.transactions()
    }

    /// Transaction execution env info.
    pub fn env_info(&self) -> EnvInfo {
        EnvInfo {
            number: self.number(),
            coin_base: *self.proposer(),
            timestamp: if self.eth_compatibility {
                self.timestamp() / 1000
            } else {
                self.timestamp()
            },
            difficulty: U256::default(),
            last_hashes: Arc::clone(&self.last_hashes),
            gas_used: self.current_quota_used,
            gas_limit: *self.quota_limit(),
            account_gas_limit: 0.into(),
        }
    }

    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn apply_transaction(&mut self, t: &SignedTransaction, conf: &BlockSysConfig) {
        let mut env_info = self.env_info();
        self.account_gas
            .entry(*t.sender())
            .or_insert(self.account_gas_limit);

        //FIXME: set coin_base according to conf.
        env_info.account_gas_limit = *self
            .account_gas
            .get(t.sender())
            .expect("account should exist in account_gas_limit");

        // Reset coin_base
        if (*conf).check_options.fee_back_platform {
            // Set coin_base to chain_owner if check_fee_back_platform is true, and chain_owner is set.
            if (*conf).chain_owner != Address::from(0) {
                env_info.coin_base = (*conf).chain_owner.clone();
            }
        }
        let block_data_provider = EVMBlockDataProvider::new(env_info.clone());
        let native_factory = NativeFactory::default();

        match CitaExecutive::new(
            Arc::new(block_data_provider),
            self.state.clone(),
            &native_factory,
            &env_info,
            conf.economical_model,
        )
        .exec(t, conf)
        {
            Ok(ret) => {
                // FIXME: logic from old cita, but there is some confuse about this logic.
                let quota_used = U256::from(ret.quota_used);
                let transaction_quota_used = quota_used - self.current_quota_used;
                self.current_quota_used = U256::from(quota_used);
                if conf.check_options.quota {
                    if let Some(value) = self.account_gas.get_mut(t.sender()) {
                        *value = *value - transaction_quota_used;
                    }
                }

                // FIXME: hasn't handle some errors
                let receipt_error = ret.exception.and_then(|error| match error {
                    ExecutionError::VM(VMError::Evm(EVMError::OutOfGas)) => {
                        Some(ReceiptError::OutOfQuota)
                    }
                    ExecutionError::VM(VMError::Evm(EVMError::InvalidJumpDestination)) => {
                        Some(ReceiptError::BadJumpDestination)
                    }
                    ExecutionError::VM(VMError::Evm(EVMError::InvalidOpcode)) => {
                        Some(ReceiptError::BadInstruction)
                    }
                    // ExecutionError::VM(VMError::Evm(EVMError::OutOfStack)) => Some(ReceiptError::StackUnderflow),
                    ExecutionError::VM(VMError::Evm(EVMError::OutOfStack)) => {
                        Some(ReceiptError::OutOfStack)
                    }
                    ExecutionError::VM(VMError::Evm(EVMError::MutableCallInStaticContext)) => {
                        Some(ReceiptError::MutableCallInStaticContext)
                    }
                    ExecutionError::VM(VMError::Evm(EVMError::Internal(_))) => {
                        Some(ReceiptError::Internal)
                    }
                    ExecutionError::VM(VMError::Evm(EVMError::OutOfBounds)) => {
                        Some(ReceiptError::OutOfBounds)
                    }
                    _ => Some(ReceiptError::Internal),
                    //EvmError::Reverted => Some(ReceiptError::Reverted),
                });

                // FIXME: change the quota_used to cumulative_gas_used.
                let receipt = Receipt::new(
                    None,
                    quota_used,
                    ret.logs,
                    receipt_error,
                    ret.account_nonce,
                    t.get_transaction_hash(),
                );

                self.receipts.push(receipt);
            }
            Err(err) => {
                // FIXME: hasn't handle some errors.
                let receipt_error = match err {
                    ExecutionError::VM(VMError::NotEnoughBaseGas) => {
                        Some(ReceiptError::NotEnoughBaseQuota)
                    }
                    // FIXME: need to handle this two situation.
                    ExecutionError::VM(VMError::ExccedMaxBlockGasLimit) => {
                        Some(ReceiptError::BlockQuotaLimitReached)
                    }
                    //                    ExecutionError::AccountGasLimitReached { .. } => {
                    //                        Some(ReceiptError::AccountQuotaLimitReached)
                    //                    }
                    ExecutionError::VM(VMError::InvalidNonce) => Some(ReceiptError::InvalidNonce),
                    ExecutionError::VM(VMError::NotEnoughBalance) => {
                        Some(ReceiptError::NotEnoughCash)
                    }
                    ExecutionError::Authentication(
                        AuthenticationError::NoTransactionPermission,
                    ) => Some(ReceiptError::NoTransactionPermission),
                    ExecutionError::Authentication(AuthenticationError::NoContractPermission) => {
                        Some(ReceiptError::NoContractPermission)
                    }
                    ExecutionError::Authentication(AuthenticationError::NoCallPermission) => {
                        Some(ReceiptError::NoCallPermission)
                    }
                    ExecutionError::Internal { .. } => Some(ReceiptError::ExecutionInternal),
                    ExecutionError::TransactionMalformed { .. } => {
                        Some(ReceiptError::TransactionMalformed)
                    }
                    _ => Some(ReceiptError::Internal),
                };

                let schedule = TxGasSchedule::default();
                //                let sender = *t.sender();
                let tx_gas_used = schedule.tx_gas;
                //match err {
                //                    ExecutionError::Internal(_) => t.gas,
                //                    _ => cmp::min(
                //                        state_db.balance(&sender).unwrap_or_else(|_| U256::from(0)),
                //                        U256::from(schedule.tx_gas),
                //                    ),
                //                };

                // FIXME: Handle the economical_model. Check for the detail of the fee.
                if (*conf).economical_model == EconomicalModel::Charge {
                    unimplemented!()
                }

                let cumulative_gas_used = env_info.gas_used + tx_gas_used;
                let receipt = Receipt::new(
                    None,
                    cumulative_gas_used,
                    Vec::new(),
                    receipt_error,
                    0.into(),
                    t.get_transaction_hash(),
                );

                self.receipts.push(receipt);
            }
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(self, conf: &BlockSysConfig) -> ClosedBlock {
        let mut env_info = self.env_info();
        // In protocol version 0, 1:
        // Auto Execution's env info author is default address
        // In protocol version > 1:
        // Auto Execution's env info author is block author
        if conf.chain_version < 2 {
            env_info.coin_base = Address::default();
        }

        // FIXME
        // if conf.auto_exec {
        //     auto_exec(
        //         &mut self.state,
        //         conf.auto_exec_quota_limit,
        //         conf.economical_model,
        //         env_info,
        //         conf.chain_version,
        //     );
        //     self.state.commit().expect("commit trie error");
        // }
        // Rebuild block
        let mut block = Block::new(self.block);
        let state_root = self.state_root;
        block.set_state_root(state_root);
        let receipts_root = cita_merklehash::Tree::from_hashes(
            self.receipts
                .iter()
                .map(|r| r.rlp_bytes().to_vec().crypt_hash())
                .collect::<Vec<_>>(),
            cita_merklehash::merge,
        )
        .get_root_hash()
        .cloned()
        .unwrap_or(cita_merklehash::HASH_NULL);

        block.set_receipts_root(receipts_root);
        block.set_quota_used(self.current_quota_used);

        // blocks blooms
        let log_bloom = self
            .receipts
            .clone()
            .into_iter()
            .fold(LogBloom::zero(), |mut b, r| {
                b = b | r.log_bloom;
                b
            });

        block.set_log_bloom(log_bloom);
        block.rehash();

        // Clear state cache.
        self.state.borrow_mut().clear();

        // Note: It is ok to new a state, because no cache and checkpoint used.
        let state = CitaState::from_existing(
            Arc::<CitaTrieDB>::clone(&self.state.borrow().db),
            self.state.borrow().root,
        )
        .expect("Get state from trie db");

        ClosedBlock {
            block,
            receipts: self.receipts,
            state,
        }
    }
}

// Block that prepared to commit to db.
// The CloseBlock will be share in two thread.
// #[derive(Debug)]
pub struct ClosedBlock {
    /// Protobuf Block
    pub block: Block,
    pub receipts: Vec<Receipt>,
    pub state: CitaState<CitaTrieDB>,
}

impl Drain for ClosedBlock {
    /// Drop this object
    fn drain(mut self) {
        self.state.clear();
    }
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
            .set_quota_limit(self.quota_limit().low_u64());

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

pub struct EVMBlockDataProvider {
    env_info: EnvInfo,
}

impl EVMBlockDataProvider {
    pub fn new(env_info: EnvInfo) -> Self {
        EVMBlockDataProvider { env_info }
    }
}

impl BlockDataProvider for EVMBlockDataProvider {
    fn get_block_hash(&self, number: &U256) -> H256 {
        // TODO: comment out what this function expects from env_info, since it will produce panics if the latter is inconsistent
        if *number < U256::from(self.env_info.number)
            && number.low_u64() >= cmp::max(256, self.env_info.number) - 256
        {
            let index = self.env_info.number - number.low_u64() - 1;
            assert!(
                index < self.env_info.last_hashes.len() as u64,
                format!(
                    "Inconsistent env_info, should contain at least {:?} last hashes",
                    index + 1
                )
            );
            let r = self.env_info.last_hashes[index as usize];
            trace!(
                "ext: blockhash({}) -> {} self.env_info.number={}\n",
                number,
                r,
                self.env_info.number
            );
            r
        } else {
            trace!(
                "ext: blockhash({}) -> null self.env_info.number={}\n",
                number,
                self.env_info.number
            );
            H256::zero()
        }
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
