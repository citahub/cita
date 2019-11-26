// Copyright Cryptape Technologies LLC.
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

use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::cita_executive::CitaExecutive;
use crate::core::context::{Context, LastHashes};
use crate::data_provider::BlockDataProvider;
use crate::exception::ExecutedException;
use crate::libexecutor::auto_exec::auto_exec;
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::executor::CitaTrieDB;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::libexecutor::sys_config::GlobalSysConfig;
use crate::receipt::Receipt;
use crate::tx_gas_schedule::TxGasSchedule;
pub use crate::types::block::{Block, BlockBody, OpenBlock};
use crate::types::errors::Error;
use crate::types::errors::ReceiptError;
use crate::types::errors::{AuthenticationError, ExecutionError};
use crate::types::transaction::SignedTransaction;
use cita_merklehash;
use cita_types::{Address, Bloom as LogBloom, H256, U256};
use cita_vm::{
    evm::Error as EVMError, state::State as CitaState, state::StateObjectInfo, Error as VMError,
};
use hashable::Hashable;
use libproto::executor::{ExecutedInfo, ReceiptWithOption};
use rlp::Encodable;

use crate::rs_contracts::storage::db_contracts::ContractsDB;

lazy_static! {
    /// Block Reward
    /// HardFork if need to change block reward
    pub static ref BLOCK_REWARD: U256 = U256::from(5_000_000_000_000_000_000 as i64);
}

pub struct ExecutedBlock {
    pub block: OpenBlock,
    pub receipts: Vec<Receipt>,
    pub state: Arc<RefCell<CitaState<CitaTrieDB>>>,
    pub contracts_db: Arc<ContractsDB>,
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
        contracts_db: Arc<ContractsDB>,
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
            contracts_db,
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
    pub fn get_context(&self) -> Context {
        Context {
            block_number: self.number(),
            coin_base: *self.proposer(),
            timestamp: if self.eth_compatibility {
                self.timestamp() / 1000
            } else {
                self.timestamp()
            },
            difficulty: U256::default(),
            last_hashes: Arc::clone(&self.last_hashes),
            quota_used: self.current_quota_used,
            block_quota_limit: *self.quota_limit(),
            account_quota_limit: 0.into(),
        }
    }

    pub fn apply_transaction(&mut self, t: &SignedTransaction, sys_config: &GlobalSysConfig) {
        let mut context = self.get_context();
        context.block_quota_limit = U256::from(sys_config.block_quota_limit);
        trace!("block quota limit is {:?}", context.block_quota_limit);

        let conf = sys_config.block_sys_config.clone();
        self.account_gas
            .entry(*t.sender())
            .or_insert(self.account_gas_limit);

        //FIXME: set coin_base according to conf.
        context.account_quota_limit = *self
            .account_gas
            .get(t.sender())
            .expect("account should exist in account_gas_limit");

        // Reset coin_base
        if conf.check_options.fee_back_platform {
            // Set coin_base to chain_owner if check_fee_back_platform is true, and chain_owner is set.
            if conf.chain_owner != Address::from(0) {
                context.coin_base = conf.chain_owner;
            }
        }
        let block_data_provider = EVMBlockDataProvider::new(context.clone());

        let tx_quota_used = match CitaExecutive::new(
            Arc::new(block_data_provider),
            self.state.clone(),
            self.contracts_db.clone(),
            &context,
            conf.economical_model,
        )
        .exec(t, &conf)
        {
            Ok(ret) => {
                // Note: ret.quota_used was a current transaction quota used.
                // FIXME: hasn't handle some errors
                let receipt_error = ret.exception.and_then(|error| match error {
                    ExecutedException::VM(VMError::Evm(EVMError::OutOfGas)) => {
                        Some(ReceiptError::OutOfQuota)
                    }
                    ExecutedException::VM(VMError::Evm(EVMError::InvalidJumpDestination)) => {
                        Some(ReceiptError::BadJumpDestination)
                    }
                    ExecutedException::VM(VMError::Evm(EVMError::InvalidOpcode)) => {
                        Some(ReceiptError::BadInstruction)
                    }
                    ExecutedException::VM(VMError::Evm(EVMError::OutOfStack)) => {
                        Some(ReceiptError::OutOfStack)
                    }
                    ExecutedException::VM(VMError::Evm(EVMError::MutableCallInStaticContext)) => {
                        Some(ReceiptError::MutableCallInStaticContext)
                    }
                    ExecutedException::VM(VMError::Evm(EVMError::StackUnderflow)) => {
                        Some(ReceiptError::StackUnderflow)
                    }
                    ExecutedException::VM(VMError::Evm(EVMError::OutOfBounds)) => {
                        Some(ReceiptError::OutOfBounds)
                    }
                    ExecutedException::Reverted => Some(ReceiptError::Reverted),
                    _ => Some(ReceiptError::Internal),
                });

                let tx_quota_used = if receipt_error.is_some()
                    && receipt_error != Some(ReceiptError::Internal)
                    && receipt_error != Some(ReceiptError::Reverted)
                {
                    t.gas
                } else {
                    ret.quota_used
                };

                // Note: quota_used in Receipt is self.current_quota_used, this will be
                // handled by get_rich_receipt() while getting a single transaction receipt.
                let cumulative_quota_used = context.quota_used + tx_quota_used;
                let receipt = Receipt::new(
                    None,
                    cumulative_quota_used,
                    ret.logs,
                    receipt_error,
                    ret.account_nonce,
                    t.get_transaction_hash(),
                );

                self.receipts.push(receipt);
                ret.quota_used
            }
            Err(err) => {
                // FIXME: hasn't handle some errors.
                let receipt_error = match err {
                    ExecutionError::NotEnoughBaseGas => Some(ReceiptError::NotEnoughBaseQuota),
                    // FIXME: need to handle this two situation.
                    ExecutionError::BlockQuotaLimitReached => {
                        Some(ReceiptError::BlockQuotaLimitReached)
                    }
                    ExecutionError::AccountQuotaLimitReached => {
                        Some(ReceiptError::AccountQuotaLimitReached)
                    }
                    ExecutionError::InvalidNonce => Some(ReceiptError::InvalidNonce),
                    ExecutionError::NotEnoughBalance => Some(ReceiptError::NotEnoughCash),
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
                    ExecutionError::InvalidTransaction => Some(ReceiptError::TransactionMalformed),
                    _ => Some(ReceiptError::Internal),
                };

                let schedule = TxGasSchedule::default();
                // Bellow has a error, need gas*price before compare with balance
                let tx_quota_used = match err {
                    ExecutionError::Internal(_) => t.gas,
                    _ => cmp::min(
                        self.state
                            .borrow_mut()
                            .balance(t.sender())
                            .unwrap_or_else(|_| U256::from(0)),
                        U256::from(schedule.tx_gas),
                    ),
                };

                if conf.economical_model == EconomicalModel::Charge {
                    // When charge model, set the min(account.balance,gas_used)
                    let _ = self.deal_err_quota_cost(
                        t.sender(),
                        &context.coin_base,
                        tx_quota_used,
                        t.gas_price(),
                    );
                }

                let cumulative_quota_used = context.quota_used + tx_quota_used;
                trace!(
                    "context quota used: {:?}, tx quota used： {:?}",
                    context.quota_used,
                    tx_quota_used
                );
                let receipt = Receipt::new(
                    None,
                    cumulative_quota_used,
                    Vec::new(),
                    receipt_error,
                    0.into(),
                    t.get_transaction_hash(),
                );

                self.receipts.push(receipt);
                tx_quota_used
            }
        };

        // Note: current_quota_used: Whole quota used for the ExecutedBlock.
        self.current_quota_used += tx_quota_used;
        if conf.check_options.quota {
            if let Some(value) = self.account_gas.get_mut(t.sender()) {
                *value -= tx_quota_used;
            }
        }
    }

    fn deal_err_quota_cost(
        &self,
        sender: &Address,
        coin_base: &Address,
        quota: U256,
        quota_price: U256,
    ) -> U256 {
        if quota_price == U256::zero() {
            return quota;
        }
        let sender_balance = self.state.borrow_mut().balance(sender).unwrap();
        let tx_fee = quota * quota_price;
        trace!("fee -{:?}, sender balance-{:?}", tx_fee, sender_balance);
        let real_fee = cmp::min(sender_balance, tx_fee);

        if self
            .state
            .borrow_mut()
            .sub_balance(sender, real_fee)
            .is_err()
        {
            error!("Sub balance failed. tx_fee: {:?}", real_fee);
        } else {
            let _ = self.state.borrow_mut().add_balance(&coin_base, real_fee);
        }
        if real_fee == sender_balance {
            sender_balance.checked_div(quota_price).unwrap()
        } else {
            quota
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(self, conf: &BlockSysConfig) -> ClosedBlock {
        let mut context = self.get_context();
        // In protocol version 0, 1:
        // Auto Execution's env info author is default address
        // In protocol version > 1:
        // Auto Execution's env info author is block author
        if conf.chain_version < 2 {
            context.coin_base = Address::default();
        }

        if conf.auto_exec {
            auto_exec(
                Arc::clone(&self.state),
                self.contracts_db,
                conf.auto_exec_quota_limit,
                context,
            );
            self.state.borrow_mut().commit().expect("commit trie error");
        }

        // Rebuild block
        let mut block = Block::new(self.block);
        let state_root = self.state.borrow().root;
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

        // Note: It is ok to new a state, because no cache and checkpoint used.
        let mut state = CitaState::from_existing(
            Arc::<CitaTrieDB>::clone(&self.state.borrow().db),
            self.state.borrow().root,
        )
        .expect("Get state from trie db");

        state.cache = RefCell::new(self.state.borrow_mut().cache.to_owned().into_inner());

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

    pub fn clear_cache(&mut self) {
        self.state.clear();
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
    context: Context,
}

impl EVMBlockDataProvider {
    pub fn new(context: Context) -> Self {
        EVMBlockDataProvider { context }
    }
}

impl BlockDataProvider for EVMBlockDataProvider {
    fn get_block_hash(&self, number: &U256) -> H256 {
        // TODO: comment out what this function expects from context, since it will produce panics if the latter is inconsistent
        if *number < U256::from(self.context.block_number)
            && number.low_u64() >= cmp::max(256, self.context.block_number) - 256
        {
            let index = self.context.block_number - number.low_u64() - 1;
            assert!(
                index < self.context.last_hashes.len() as u64,
                format!(
                    "Inconsistent context, should contain at least {:?} last hashes",
                    index + 1
                )
            );
            let r = self.context.last_hashes[index as usize];
            trace!(
                "ext: blockhash({}) -> {} self.context.block_number={}\n",
                number,
                r,
                self.context.block_number
            );
            r
        } else {
            trace!(
                "ext: blockhash({}) -> null self.context.block_number={}\n",
                number,
                self.context.block_number
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

    #[test]
    fn test_encode_and_decode_null() {
        let transactions = vec![];
        let body = BlockBody { transactions };
        let body_rlp = rlp::encode(&body);
        let body: BlockBody = rlp::decode(&body_rlp);
        let body_encoded = rlp::encode(&body).into_vec();

        assert_eq!(body_rlp, body_encoded);
    }
}
