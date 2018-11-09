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

//! A mutable state representation suitable to execute transactions.
//! Generic over a `Backend`. Deals with `Account`s.
//! Unconfirmed sub-states are managed with `checkpoint`s which may be canonicalized
//! or rolled back.

use cita_types::{Address, H256, U256};
use contracts::solc::Resource;
use engines::Engine;
use error::{Error, ExecutionError};
use evm::env_info::EnvInfo;
use evm::Error as EvmError;
use evm::Schedule;
use executive::{Executive, TransactOptions};
use factory::Factories;
use libexecutor::executor::{CheckOptions, EconomicalModel};
use receipt::{Receipt, ReceiptError};
use rlp::{self, Encodable};
use std::cell::{Ref, RefCell, RefMut};
use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;
use trace::FlatTrace;
use types::transaction::SignedTransaction;
use util::trie;
use util::*;

pub mod account;
pub mod backend;

pub use self::account::Account;
use self::backend::*;
use state_db::*;
pub use substate::Substate;

/// Used to return information about an `State::apply` operation.
pub struct ApplyOutcome {
    /// The receipt for the applied transaction.
    pub receipt: Receipt,
    /// The trace for the applied transaction, if None if tracing is disabled.
    pub trace: Vec<FlatTrace>,
}

/// Result type for the execution ("application") of a transaction.
pub type ApplyResult = Result<ApplyOutcome, Error>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Account modification state. Used to check if the account was
/// Modified in between commits and overall.
enum AccountState {
    /// Account was loaded from disk and never modified in this state object.
    CleanFresh,
    /// Account was loaded from the global cache and never modified.
    CleanCached,
    /// Account has been modified and is not committed to the trie yet.
    /// This is set if any of the account data is changed, including
    /// storage, code and ABI.
    Dirty,
    /// Account was modified and committed to the trie.
    Committed,
}

#[derive(Debug)]
/// In-memory copy of the account data. Holds the optional account
/// and the modification status.
/// Account entry can contain existing (`Some`) or non-existing
/// account (`None`)
pub struct AccountEntry {
    /// Account entry. `None` if account known to be non-existant.
    account: Option<Account>,
    /// Unmodified account balance.
    old_balance: Option<U256>,
    /// Entry state.
    state: AccountState,
}

// Account cache item. Contains account data and
// modification state
impl AccountEntry {
    pub fn is_dirty(&self) -> bool {
        self.state == AccountState::Dirty
    }

    pub fn is_commited(&self) -> bool {
        self.state == AccountState::Committed
    }

    fn exists_and_is_null(&self) -> bool {
        self.account.as_ref().map_or(false, |a| a.is_null())
    }

    /// Clone dirty data into new `AccountEntry`. This includes
    /// basic account data and modified storage keys.
    /// Returns None if clean.
    fn clone_if_dirty(&self) -> Option<AccountEntry> {
        if self.is_dirty() {
            Some(self.clone_dirty())
        } else {
            None
        }
    }

    /// Clone dirty data into new `AccountEntry`. This includes
    /// basic account data and modified storage keys.
    fn clone_dirty(&self) -> AccountEntry {
        AccountEntry {
            old_balance: self.old_balance,
            account: self.account.as_ref().map(Account::clone_dirty),
            state: self.state,
        }
    }

    // Create a new account entry and mark it as dirty.
    fn new_dirty(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            old_balance: account.as_ref().map(|a| *a.balance()),
            account,
            state: AccountState::Dirty,
        }
    }

    // Create a new account entry and mark it as clean.
    fn new_clean(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            old_balance: account.as_ref().map(|a| *a.balance()),
            account,
            state: AccountState::CleanFresh,
        }
    }

    // Create a new account entry and mark it as clean and cached.
    fn new_clean_cached(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            old_balance: account.as_ref().map(|a| *a.balance()),
            account,
            state: AccountState::CleanCached,
        }
    }

    // Replace data with another entry but preserve storage cache.
    fn overwrite_with(&mut self, other: AccountEntry) {
        self.state = other.state;
        match other.account {
            Some(acc) => {
                if let Some(ref mut ours) = self.account {
                    ours.overwrite_with(acc);
                }
            }
            None => self.account = None,
        }
    }

    pub fn account(&self) -> Option<&Account> {
        self.account.as_ref()
    }
}

#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct StateProof {
    address: Address,
    account_proof: Vec<Bytes>,
    key: H256,
    value_proof: Vec<Bytes>,
}

impl StateProof {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        rlp::decode(bytes)
    }

    pub fn verify(&self, state_root: H256) -> Option<H256> {
        trie::triedb::verify_value_proof(
            &self.address,
            state_root,
            &self.account_proof,
            Account::from_rlp,
        )
        .and_then(|a| a.verify_value_proof(&self.key, &self.value_proof))
    }

    /// Get the address field of the StateProof.
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Get the key field of the StateProof.
    pub fn key(&self) -> &H256 {
        &self.key
    }

    #[cfg(test)]
    pub fn set_address(&mut self, new_address: Address) {
        self.address = new_address;
    }
}

/// Representation of the entire state of all accounts in the system.
///
/// `State` can work together with `StateDB` to share account cache.
///
/// Local cache contains changes made locally and changes accumulated
/// locally from previous commits. Global cache reflects the database
/// state and never contains any changes.
///
/// Cache items contains account data, or the flag that account does not exist
/// and modification state (see `AccountState`)
///
/// Account data can be in the following cache states:
/// * In global but not local - something that was queried from the database,
/// but never modified
/// * In local but not global - something that was just added (e.g. new account)
/// * In both with the same value - something that was changed to a new value,
/// but changed back to a previous block in the same block (same State instance)
/// * In both with different values - something that was overwritten with a
/// new value.
///
/// All read-only state queries check local cache/modifications first,
/// then global state cache. If data is not found in any of the caches
/// it is loaded from the DB to the local cache.
///
/// **** IMPORTANT *************************************************************
/// All the modifications to the account data must set the `Dirty` state in the
/// `AccountEntry`. This is done in `require` and `require_or_from`. So just
/// use that.
/// ****************************************************************************
///
/// Upon destruction all the local cache data propagated into the global cache.
/// Propagated items might be rejected if current state is non-canonical.
///
/// State checkpointing.
///
/// A new checkpoint can be created with `checkpoint()`. checkpoints can be
/// created in a hierarchy.
/// When a checkpoint is active all changes are applied directly into
/// `cache` and the original value is copied into an active checkpoint.
/// Reverting a checkpoint with `revert_to_checkpoint` involves copying
/// original values from the latest checkpoint back into `cache`. The code
/// takes care not to overwrite cached storage while doing that.
/// checkpoint can be discarded with `discard_checkpoint`. All of the orignal
/// backed-up values are moved into a parent checkpoint (if any).
///
pub struct State<B: Backend> {
    db: B,
    root: H256,
    cache: RefCell<HashMap<Address, AccountEntry>>,
    // The original account is preserved in
    checkpoints: RefCell<Vec<HashMap<Address, Option<AccountEntry>>>>,
    account_start_nonce: U256,
    factories: Factories,
    pub account_permissions: HashMap<Address, Vec<Resource>>,
    pub group_accounts: HashMap<Address, Vec<Address>>,
    pub super_admin_account: Option<Address>,
}

#[derive(Copy, Clone)]
enum RequireCache {
    None,
    CodeSize,
    Code,
    AbiSize,
    Abi,
}

/// Mode of dealing with null accounts.
#[derive(PartialEq)]
pub enum CleanupMode<'a> {
    /// Create accounts which would be null.
    ForceCreate,
    /// Don't delete null accounts upon touching, but also don't create them.
    NoEmpty,
    /// Add encountered null accounts to the provided kill-set, to be deleted later.
    KillEmpty(&'a mut HashSet<Address>),
}

const SEC_TRIE_DB_UNWRAP_STR: &str =
    "A state can only be created with valid root.\
     Creating a SecTrieDB with a valid root will not fail.\
     Therefore creating a SecTrieDB with this state's root will not fail.";

impl<B: Backend> State<B> {
    /// Creates new state with empty state root
    #[cfg(test)]
    pub fn new(mut db: B, account_start_nonce: U256, factories: Factories) -> State<B> {
        let mut root = H256::new();
        {
            // init trie and reset root too null
            let _ = factories.trie.create(db.as_hashdb_mut(), &mut root);
        }

        State {
            db,
            root,
            cache: RefCell::new(HashMap::new()),
            checkpoints: RefCell::new(Vec::new()),
            account_start_nonce,
            factories,
            account_permissions: HashMap::new(),
            group_accounts: HashMap::new(),
            super_admin_account: None,
        }
    }

    pub fn cache(&self) -> Ref<HashMap<Address, AccountEntry>> {
        self.cache.borrow()
    }

    /// Creates new state with existing state root
    pub fn from_existing(
        db: B,
        root: H256,
        account_start_nonce: U256,
        factories: Factories,
    ) -> Result<State<B>, TrieError> {
        if !db.as_hashdb().contains(&root) {
            return Err(TrieError::InvalidStateRoot(root));
        }

        let state = State {
            db,
            root,
            cache: RefCell::new(HashMap::new()),
            checkpoints: RefCell::new(Vec::new()),
            account_start_nonce,
            factories,
            account_permissions: HashMap::new(),
            group_accounts: HashMap::new(),
            super_admin_account: None,
        };

        Ok(state)
    }

    /// Create a recoverable checkpoint of this state.
    pub fn checkpoint(&mut self) {
        self.checkpoints.get_mut().push(HashMap::new());
    }

    /// Merge last checkpoint with previous.
    pub fn discard_checkpoint(&mut self) {
        // merge with previous checkpoint
        let last = self.checkpoints.get_mut().pop();
        if let Some(mut checkpoint) = last {
            if let Some(ref mut prev) = self.checkpoints.get_mut().last_mut() {
                if prev.is_empty() {
                    **prev = checkpoint;
                } else {
                    for (k, v) in checkpoint.drain() {
                        prev.entry(k).or_insert(v);
                    }
                }
            }
        }
    }

    /// Revert to the last checkpoint and discard it.
    pub fn revert_to_checkpoint(&mut self) {
        if let Some(mut checkpoint) = self.checkpoints.get_mut().pop() {
            for (k, v) in checkpoint.drain() {
                match v {
                    Some(v) => {
                        match self.cache.get_mut().entry(k) {
                            Entry::Occupied(mut e) => {
                                // Merge checkpointed changes back into the main account
                                // storage preserving the cache.
                                e.get_mut().overwrite_with(v);
                            }
                            Entry::Vacant(e) => {
                                e.insert(v);
                            }
                        }
                    }
                    None => {
                        if let Entry::Occupied(e) = self.cache.get_mut().entry(k) {
                            if e.get().is_dirty() {
                                e.remove();
                            }
                        }
                    }
                }
            }
        }
    }

    fn insert_cache(&self, address: &Address, account: AccountEntry) {
        // Dirty account which is not in the cache means this is a new account.
        // It goes directly into the checkpoint as there's nothing to rever to.
        //
        // In all other cases account is read as clean first, and after that made
        // dirty in and added to the checkpoint with `note_cache`.
        let is_dirty = account.is_dirty();
        let old_value = self.cache.borrow_mut().insert(*address, account);
        if is_dirty {
            if let Some(ref mut checkpoint) = self.checkpoints.borrow_mut().last_mut() {
                checkpoint.entry(*address).or_insert(old_value);
            }
        }
    }

    fn note_cache(&self, address: &Address) {
        if let Some(ref mut checkpoint) = self.checkpoints.borrow_mut().last_mut() {
            checkpoint.entry(*address).or_insert_with(|| {
                self.cache
                    .borrow()
                    .get(address)
                    .map(AccountEntry::clone_dirty)
            });
        }
    }

    /// Destroy the current object and return root and database.
    pub fn drop(mut self) -> (H256, B) {
        self.propagate_to_global_cache();
        (self.root, self.db)
    }

    pub fn db(self) -> B {
        self.db
    }

    /// Return reference to root
    pub fn root(&self) -> &H256 {
        &self.root
    }

    /// Create a new contract at address `contract`. If there is already an account at the address
    /// it will have its code reset, ready for `init_code()`.
    pub fn new_contract(&mut self, contract: &Address, balance: U256, nonce_offset: U256) {
        self.insert_cache(
            contract,
            AccountEntry::new_dirty(Some(Account::new_contract(
                balance,
                self.account_start_nonce + nonce_offset,
            ))),
        );
    }

    /// Remove an existing account.
    pub fn kill_account(&mut self, account: &Address) {
        self.insert_cache(account, AccountEntry::new_dirty(None));
    }

    /// Determine whether an account exists.
    pub fn exists(&self, a: &Address) -> trie::Result<bool> {
        // Bloom filter does not contain empty accounts, so it is important here to
        // check if account exists in the database directly before EIP-161 is in effect.
        self.ensure_cached(a, RequireCache::None, false, |a| a.is_some())
    }

    /// Determine whether an account exists and if not empty.
    pub fn exists_and_not_null(&self, a: &Address) -> trie::Result<bool> {
        self.ensure_cached(a, RequireCache::None, false, |a| {
            a.map_or(false, |a| !a.is_null())
        })
    }

    /// Determine whether an account exists and has code or non-zero nonce.
    pub fn exists_and_has_code_or_nonce(&self, a: &Address) -> trie::Result<bool> {
        self.ensure_cached(a, RequireCache::CodeSize, false, |a| {
            a.map_or(false, |a| {
                a.code_hash() != HASH_EMPTY || *a.nonce() != self.account_start_nonce
            })
        })
    }

    /// Get the balance of account `a`.
    pub fn balance(&self, a: &Address) -> trie::Result<U256> {
        self.ensure_cached(a, RequireCache::None, true, |a| {
            a.as_ref()
                .map_or(U256::zero(), |account| *account.balance())
        })
    }

    /// Get the nonce of account `a`.
    pub fn nonce(&self, a: &Address) -> trie::Result<U256> {
        self.ensure_cached(a, RequireCache::None, true, |a| {
            a.as_ref()
                .map_or(self.account_start_nonce, |account| *account.nonce())
        })
    }

    /// Get the storage root of account `a`.
    pub fn storage_root(&self, a: &Address) -> trie::Result<Option<H256>> {
        self.ensure_cached(a, RequireCache::None, true, |a| {
            a.as_ref()
                .and_then(|account| account.storage_root().cloned())
        })
    }

    /// Mutate storage of account `address` so that it is `value` for `key`.
    pub fn storage_at(&self, address: &Address, key: &H256) -> trie::Result<H256> {
        // Storage key search and update works like this:
        // 1. If there's an entry for the account in the local cache check for the key and return it if found.
        // 2. If there's an entry for the account in the global cache check for the key or load it into that account.
        // 3. If account is missing in the global cache load it into the local cache and cache the key there.

        {
            // check local cache first without updating
            let local_cache = self.cache.borrow_mut();
            let mut local_account = None;
            if let Some(maybe_acc) = local_cache.get(address) {
                match maybe_acc.account {
                    Some(ref account) => {
                        if let Some(value) = account.cached_storage_at(key) {
                            return Ok(value);
                        } else {
                            local_account = Some(maybe_acc);
                        }
                    }
                    _ => return Ok(H256::new()),
                }
            }

            // check the global cache and and cache storage key there if found,
            let trie_res = self.db.get_cached(address, |acc| match acc {
                None => Ok(H256::new()),
                Some(a) => {
                    let account_db = self
                        .factories
                        .accountdb
                        .readonly(self.db.as_hashdb(), a.address_hash(address));
                    a.storage_at(&self.factories.trie, account_db.as_hashdb(), key)
                }
            });

            if let Some(res) = trie_res {
                return res;
            }

            // otherwise cache the account localy and cache storage key there.
            if let Some(ref mut acc) = local_account {
                if let Some(ref account) = acc.account {
                    let account_db = self
                        .factories
                        .accountdb
                        .readonly(self.db.as_hashdb(), account.address_hash(address));
                    return account.storage_at(&self.factories.trie, account_db.as_hashdb(), key);
                } else {
                    return Ok(H256::new());
                }
            }
        }

        // check if the account could exist before any requests to trie
        if self.db.is_known_null(address) {
            return Ok(H256::zero());
        }

        // account is not found in the global cache, get from the DB and insert into local
        let db = self
            .factories
            .trie
            .readonly(self.db.as_hashdb(), &self.root)
            .expect(SEC_TRIE_DB_UNWRAP_STR);
        let maybe_acc = db.get_with(address, Account::from_rlp)?;
        let r = maybe_acc.as_ref().map_or(Ok(H256::new()), |a| {
            let account_db = self
                .factories
                .accountdb
                .readonly(self.db.as_hashdb(), a.address_hash(address));
            a.storage_at(&self.factories.trie, account_db.as_hashdb(), key)
        });
        self.insert_cache(address, AccountEntry::new_clean(maybe_acc));
        r
    }

    /// Get `value` proof for `key` of account `address`.
    pub fn get_state_proof(&self, address: &Address, key: &H256) -> Option<Vec<u8>> {
        // check if the account could exist before any requests to trie
        if self.db.is_known_null(address) {
            return None;
        }

        // get proof from the DB
        self.factories
            .trie
            .readonly(self.db.as_hashdb(), &self.root)
            .ok()
            .and_then(|db| {
                db.get_value_proof(address).and_then(|account_proof| {
                    db.get_with(address, Account::from_rlp)
                        .ok()
                        .and_then(|maybe_acc| {
                            maybe_acc.as_ref().and_then(|a| {
                                let account_db = self
                                    .factories
                                    .accountdb
                                    .readonly(self.db.as_hashdb(), a.address_hash(address));
                                a.get_value_proof(&self.factories.trie, account_db.as_hashdb(), key)
                                    .map(|value_proof| {
                                        StateProof {
                                            address: *address,
                                            account_proof,
                                            key: *key,
                                            value_proof,
                                        }
                                        .rlp_bytes()
                                        .into_vec()
                                    })
                            })
                        })
                })
            })
    }

    /// Get accounts' code.
    pub fn code(&self, a: &Address) -> trie::Result<Option<Arc<Bytes>>> {
        self.ensure_cached(a, RequireCache::Code, true, |a| {
            a.as_ref().and_then(|a| a.code().clone())
        })
    }

    /// Get an account's code hash.
    pub fn code_hash(&self, a: &Address) -> trie::Result<H256> {
        self.ensure_cached(a, RequireCache::None, true, |a| {
            a.as_ref().map_or(HASH_EMPTY, |a| a.code_hash())
        })
    }

    /// Get accounts' code size.
    pub fn code_size(&self, a: &Address) -> trie::Result<Option<usize>> {
        self.ensure_cached(a, RequireCache::CodeSize, true, |a| {
            a.as_ref().and_then(|a| a.code_size())
        })
    }

    /// Get accounts' ABI.
    pub fn abi(&self, a: &Address) -> trie::Result<Option<Arc<Bytes>>> {
        self.ensure_cached(a, RequireCache::Abi, true, |a| {
            a.as_ref().and_then(|a| a.abi().clone())
        })
    }

    /// Get an account's ABI hash.
    pub fn abi_hash(&self, a: &Address) -> trie::Result<H256> {
        self.ensure_cached(a, RequireCache::None, true, |a| {
            a.as_ref().map_or(HASH_EMPTY, |a| a.abi_hash())
        })
    }

    /// Get accounts' ABI size.
    pub fn abi_size(&self, a: &Address) -> trie::Result<Option<usize>> {
        self.ensure_cached(a, RequireCache::AbiSize, true, |a| {
            a.as_ref().and_then(|a| a.abi_size())
        })
    }

    /// Add `incr` to the balance of account `a`.
    pub fn add_balance(&mut self, a: &Address, incr: &U256) -> trie::Result<()> {
        trace!(target: "state", "add_balance({}, {}): {}", a, incr, self.balance(a)?);
        let is_value_transfer = !incr.is_zero();
        if is_value_transfer {
            self.require(a, false, false)?.add_balance(incr);
        } else if self.exists(a)? {
            self.touch(a)?;
        }
        Ok(())
    }

    /// Subtract `decr` from the balance of account `a`.
    pub fn sub_balance(&mut self, a: &Address, decr: &U256) -> trie::Result<()> {
        trace!(target: "state", "sub_balance({}, {}): {}", a, decr, self.balance(a)?);
        if !decr.is_zero() || !self.exists(a)? {
            self.require(a, false, false)?.sub_balance(decr);
        }

        Ok(())
    }

    /// Subtracts `by` from the balance of `from` and adds it to that of `to`.
    pub fn transfer_balance(
        &mut self,
        from: &Address,
        to: &Address,
        by: &U256,
    ) -> trie::Result<()> {
        self.sub_balance(from, by)?;
        self.add_balance(to, by)?;
        Ok(())
    }

    /// Increment the nonce of account `a` by 1.
    pub fn inc_nonce(&mut self, a: &Address) -> trie::Result<()> {
        self.require(a, false, false).map(|mut x| x.inc_nonce())
    }

    /// Mutate storage of account `a` so that it is `value` for `key`.
    pub fn set_storage(&mut self, a: &Address, key: H256, value: H256) -> trie::Result<()> {
        if self.storage_at(a, &key)? != value {
            self.require(a, false, false)?.set_storage(key, value)
        }

        Ok(())
    }

    /// Initialise the code of account `a` so that it is `code`.
    /// NOTE: Account should have been created with `new_contract`.
    pub fn init_code(&mut self, a: &Address, code: Bytes) -> trie::Result<()> {
        self.require_or_from(
            a,
            true,
            false,
            || Account::new_contract(0.into(), self.account_start_nonce),
            |_| {},
        )?
        .init_code(code);
        Ok(())
    }

    /// Reset the code of account `a` so that it is `code`.
    pub fn reset_code(&mut self, a: &Address, code: Bytes) -> trie::Result<()> {
        self.require_or_from(
            a,
            true,
            false,
            || Account::new_contract(0.into(), self.account_start_nonce),
            |_| {},
        )?
        .reset_code(code);
        Ok(())
    }

    /// Initialise the ABI of account `a` so that it is `abi`.
    /// NOTE: Account should have been created with `new_contract`.
    pub fn init_abi(&mut self, a: &Address, abi: Bytes) -> trie::Result<()> {
        self.require_or_from(
            a,
            false,
            true,
            || Account::new_contract(0.into(), self.account_start_nonce),
            |_| {},
        )?
        .init_abi(abi);
        Ok(())
    }

    /// Reset the abi of account `a` so that it is `abi`.
    pub fn reset_abi(&mut self, a: &Address, abi: Bytes) -> trie::Result<()> {
        self.require_or_from(
            a,
            false,
            true,
            || Account::new_contract(0.into(), self.account_start_nonce),
            |_| {},
        )?
        .reset_abi(abi);
        Ok(())
    }

    /// Execute a given transaction.
    /// This will change the state accordingly.
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn apply(
        &mut self,
        env_info: &EnvInfo,
        engine: &Engine,
        t: &SignedTransaction,
        tracing: bool,
        economical_model: EconomicalModel,
        chain_owner: Address,
        check_options: &CheckOptions,
    ) -> ApplyResult {
        let options = TransactOptions {
            tracing,
            vm_tracing: false,
            check_permission: check_options.permission,
            check_quota: check_options.quota,
            check_send_tx_permission: check_options.send_tx_permission,
            check_create_contract_permission: check_options.create_contract_permission,
        };
        let vm_factory = self.factories.vm.clone();
        let native_factory = self.factories.native.clone();

        match Executive::new(
            self,
            env_info,
            engine,
            &vm_factory,
            &native_factory,
            false,
            economical_model,
            check_options.fee_back_platform,
            chain_owner,
        )
        .transact(t, options)
        {
            Ok(e) => {
                // trace!("Applied transaction. Diff:\n{}\n", state_diff::diff_pod(&old, &self.to_pod()));
                let receipt_error = e.exception.and_then(|evm_error| match evm_error {
                    EvmError::OutOfGas => Some(ReceiptError::OutOfQuota),
                    EvmError::BadJumpDestination { .. } => Some(ReceiptError::BadJumpDestination),
                    EvmError::BadInstruction { .. } => Some(ReceiptError::BadInstruction),
                    EvmError::StackUnderflow { .. } => Some(ReceiptError::StackUnderflow),
                    EvmError::OutOfStack { .. } => Some(ReceiptError::OutOfStack),
                    EvmError::MutableCallInStaticContext => {
                        Some(ReceiptError::MutableCallInStaticContext)
                    }
                    EvmError::Internal(_) => Some(ReceiptError::Internal),
                    EvmError::OutOfBounds => Some(ReceiptError::OutOfBounds),
                    EvmError::Reverted => Some(ReceiptError::Reverted),
                });
                let receipt = Receipt::new(
                    None,
                    e.cumulative_gas_used,
                    e.logs,
                    receipt_error,
                    e.account_nonce,
                    t.get_transaction_hash(),
                );
                trace!(target: "state", "Transaction receipt: {:?}", receipt);
                Ok(ApplyOutcome {
                    receipt,
                    trace: e.trace,
                })
            }
            Err(err) => {
                let receipt_error = match err {
                    ExecutionError::NotEnoughBaseGas { .. } => {
                        Some(ReceiptError::NotEnoughBaseQuota)
                    }
                    ExecutionError::BlockGasLimitReached { .. } => {
                        Some(ReceiptError::BlockQuotaLimitReached)
                    }
                    ExecutionError::AccountGasLimitReached { .. } => {
                        Some(ReceiptError::AccountQuotaLimitReached)
                    }
                    ExecutionError::InvalidNonce { .. } => Some(ReceiptError::InvalidNonce),
                    ExecutionError::NotEnoughCash { .. } => Some(ReceiptError::NotEnoughCash),
                    ExecutionError::NoTransactionPermission => {
                        Some(ReceiptError::NoTransactionPermission)
                    }
                    ExecutionError::NoContractPermission => {
                        Some(ReceiptError::NoContractPermission)
                    }
                    ExecutionError::NoCallPermission => Some(ReceiptError::NoCallPermission),
                    ExecutionError::ExecutionInternal { .. } => {
                        Some(ReceiptError::ExecutionInternal)
                    }
                    ExecutionError::TransactionMalformed { .. } => {
                        Some(ReceiptError::TransactionMalformed)
                    }
                };
                let schedule = Schedule::new_v1();
                let sender = *t.sender();
                let tx_gas_used = match err {
                    ExecutionError::ExecutionInternal { .. } => t.gas,
                    _ => cmp::min(
                        self.balance(&sender).unwrap_or_else(|_| U256::from(0)),
                        U256::from(schedule.tx_gas),
                    ),
                };

                if economical_model == EconomicalModel::Charge {
                    let fee_value = tx_gas_used * t.gas_price();
                    let sender_balance = self.balance(&sender).unwrap();

                    let tx_fee_value = if fee_value > sender_balance {
                        sender_balance
                    } else {
                        fee_value
                    };
                    if let Err(err) = self.sub_balance(&sender, &tx_fee_value) {
                        error!("Sub balance from error transaction sender failed, tx_fee_value={}, error={:?}", tx_fee_value, err);
                    }

                    if check_options.fee_back_platform && chain_owner != Address::from(0) {
                        self.add_balance(&chain_owner, &tx_fee_value)
                            .expect("Add balance to chain owner must success");
                    } else {
                        self.add_balance(&env_info.author, &tx_fee_value)
                            .expect("Add balance to author(miner) must success");
                    }
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
                Ok(ApplyOutcome {
                    receipt,
                    trace: Vec::new(),
                })
            }
        }
    }

    fn touch(&mut self, a: &Address) -> trie::Result<()> {
        self.require(a, false, false)?;
        Ok(())
    }

    /// Commits our cached account changes into the trie.
    pub fn commit(&mut self) -> Result<(), Error> {
        assert!(self.checkpoints.borrow().is_empty());

        // first, commit the sub trees.
        let mut accounts = self.cache.borrow_mut();
        for (address, ref mut a) in accounts.iter_mut().filter(|&(_, ref a)| a.is_dirty()) {
            if let Some(ref mut account) = a.account {
                let addr_hash = account.address_hash(address);
                {
                    let mut account_db = self
                        .factories
                        .accountdb
                        .create(self.db.as_hashdb_mut(), addr_hash);
                    account
                        .commit_storage(&self.factories.trie, account_db.as_hashdb_mut())
                        .map_err(|err| *err)?;

                    account.commit_code(account_db.as_hashdb_mut());
                    account.commit_abi(account_db.as_hashdb_mut())
                }
                if !account.is_empty() {
                    self.db.note_non_null_account(address);
                }
            }
        }

        {
            let mut trie = self
                .factories
                .trie
                .from_existing(self.db.as_hashdb_mut(), &mut self.root)
                .map_err(|err| *err)?;
            for (address, ref mut a) in accounts.iter_mut().filter(|&(_, ref a)| a.is_dirty()) {
                a.state = AccountState::Committed;
                match a.account {
                    Some(ref mut account) => {
                        trie.insert(address, &account.rlp()).map_err(|err| *err)?;
                    }
                    None => {
                        trie.remove(address).map_err(|err| *err)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Propagate local cache into shared canonical state cache.
    fn propagate_to_global_cache(&mut self) {
        let mut addresses = self.cache.borrow_mut();
        trace!("Committing cache {:?} entries", addresses.len());
        for (address, a) in addresses.drain().filter(|&(_, ref a)| {
            a.state == AccountState::Committed || a.state == AccountState::CleanFresh
        }) {
            self.db
                .add_to_account_cache(address, a.account, a.state == AccountState::Committed);
        }
    }

    /// Clear state cache
    pub fn clear(&mut self) {
        self.cache.borrow_mut().clear();
    }

    /// Remove any touched empty or dust accounts.
    pub fn kill_garbage(
        &mut self,
        touched: &HashSet<Address>,
        remove_empty_touched: bool,
        min_balance: &Option<U256>,
        kill_contracts: bool,
    ) -> trie::Result<()> {
        let to_kill: HashSet<_> = {
            self.cache
                .borrow()
                .iter()
                .filter_map(|(address, ref entry)| {
                    let should_kill = touched.contains(address)  // Check all touched accounts
                        && ((remove_empty_touched && entry.exists_and_is_null()) // Remove all empty touched accounts.
                            || min_balance.map_or(false, |ref balance| {
                                entry.account.as_ref().map_or(false, |account| {
                                    // Remove all basic and optionally contract accounts
                                    // where balance has been decreased.
                                    (account.is_basic() || kill_contracts)
                                        && account.balance() < balance
                                        && entry.old_balance.as_ref().map_or(false, |b| account.balance() < b)
                                })
                            }));
                    if should_kill {
                        Some(*address)
                    } else {
                        None
                    }
                })
                .collect()
        };
        for address in to_kill {
            self.kill_account(&address);
        }
        Ok(())
    }

    // load required account data from the databases.
    fn update_account_cache(
        require: RequireCache,
        account: &mut Account,
        state_db: &B,
        db: &HashDB,
    ) {
        match (account.is_cached(), require) {
            (false, RequireCache::Code) | (false, RequireCache::CodeSize) => {
                // if there's already code in the global cache, always cache it locally.
                let hash = account.code_hash();
                match state_db.get_cached_code(&hash) {
                    Some(code) => account.cache_given_code(code),
                    None => {
                        match require {
                            RequireCache::None => {}
                            RequireCache::Code => {
                                if let Some(code) = account.cache_code(db) {
                                    // propagate code loaded from the database to
                                    // the global code cache.
                                    state_db.cache_code(hash, code)
                                }
                            }
                            RequireCache::CodeSize => {
                                account.cache_code_size(db);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        };

        match (account.is_abi_cached(), require) {
            (false, RequireCache::Abi) | (false, RequireCache::AbiSize) => {
                account.cache_abi(db);
            }
            _ => {}
        }
    }

    /// Check caches for required data
    /// First searches for account in the local, then the shared cache.
    /// Populates local cache if nothing found.
    fn ensure_cached<F, U>(
        &self,
        a: &Address,
        require: RequireCache,
        check_null: bool,
        f: F,
    ) -> trie::Result<U>
    where
        F: Fn(Option<&Account>) -> U,
    {
        // check local cache first
        if let Some(ref mut maybe_acc) = self.cache.borrow_mut().get_mut(a) {
            if let Some(ref mut account) = maybe_acc.account {
                let accountdb = self
                    .factories
                    .accountdb
                    .readonly(self.db.as_hashdb(), account.address_hash(a));
                Self::update_account_cache(require, account, &self.db, accountdb.as_hashdb());
                return Ok(f(Some(account)));
            }
            return Ok(f(None));
        }
        // check global cache
        let result = self.db.get_cached(a, |mut acc| {
            if let Some(ref mut account) = acc {
                let accountdb = self
                    .factories
                    .accountdb
                    .readonly(self.db.as_hashdb(), account.address_hash(a));
                Self::update_account_cache(require, account, &self.db, accountdb.as_hashdb());
            }
            f(acc.map(|a| &*a))
        });
        match result {
            Some(r) => Ok(r),
            None => {
                // first check if it is not in database for sure
                if check_null && self.db.is_known_null(a) {
                    return Ok(f(None));
                }

                // not found in the global cache, get from the DB and insert into local
                let db = self
                    .factories
                    .trie
                    .readonly(self.db.as_hashdb(), &self.root)?;
                let mut maybe_acc = db.get_with(a, Account::from_rlp)?;
                if let Some(ref mut account) = maybe_acc.as_mut() {
                    let accountdb = self
                        .factories
                        .accountdb
                        .readonly(self.db.as_hashdb(), account.address_hash(a));
                    Self::update_account_cache(require, account, &self.db, accountdb.as_hashdb());
                }
                let r = f(maybe_acc.as_ref());
                self.insert_cache(a, AccountEntry::new_clean(maybe_acc));
                Ok(r)
            }
        }
    }

    /// Pull account `a` in our cache from the trie DB.
    /// `require_code` requires that the code be cached, too.
    /// `require_abi` requires that the abi be cached, too.
    fn require<'a>(
        &'a self,
        a: &Address,
        require_code: bool,
        require_abi: bool,
    ) -> trie::Result<RefMut<'a, Account>> {
        self.require_or_from(
            a,
            require_code,
            require_abi,
            || Account::new_basic(0u8.into(), self.account_start_nonce),
            |_| {},
        )
    }

    /// Pull account `a` in our cache from the trie DB.
    /// `require_code` requires that the code be cached, too.
    /// `require_abi` requires that the abi be cached, too.
    /// If it doesn't exist, make account equal the evaluation of `default`.
    fn require_or_from<'a, F, G>(
        &'a self,
        a: &Address,
        require_code: bool,
        require_abi: bool,
        default: F,
        not_default: G,
    ) -> trie::Result<RefMut<'a, Account>>
    where
        F: FnOnce() -> Account,
        G: FnOnce(&mut Account),
    {
        let contains_key = self.cache.borrow().contains_key(a);
        if !contains_key {
            match self.db.get_cached_account(a) {
                Some(acc) => self.insert_cache(a, AccountEntry::new_clean_cached(Some(acc))),
                None => {
                    let maybe_acc = if !self.db.is_known_null(a) {
                        let db = self
                            .factories
                            .trie
                            .readonly(self.db.as_hashdb(), &self.root)?;
                        AccountEntry::new_clean(db.get_with(a, Account::from_rlp)?)
                    } else {
                        AccountEntry::new_clean(None)
                    };
                    self.insert_cache(a, maybe_acc);
                }
            }
        }
        self.note_cache(a);

        // at this point the entry is guaranteed to be in the cache.
        Ok(RefMut::map(self.cache.borrow_mut(), |c| {
            let entry = c
                .get_mut(a)
                .expect("entry known to exist in the cache; qed");

            match &mut entry.account {
                &mut Some(ref mut acc) => not_default(acc),
                slot => *slot = Some(default()),
            }

            // set the dirty flag after changing account data.
            entry.state = AccountState::Dirty;
            match entry.account {
                Some(ref mut account) => {
                    if require_code || require_abi {
                        let addr_hash = account.address_hash(a);
                        let accountdb = self
                            .factories
                            .accountdb
                            .readonly(self.db.as_hashdb(), addr_hash);

                        if require_code {
                            Self::update_account_cache(
                                RequireCache::Code,
                                account,
                                &self.db,
                                accountdb.as_hashdb(),
                            );
                        }

                        if require_abi {
                            Self::update_account_cache(
                                RequireCache::Abi,
                                account,
                                &self.db,
                                accountdb.as_hashdb(),
                            );
                        }
                    }

                    account
                }
                _ => panic!("Required account must always exist; qed"),
            }
        }))
    }
}

impl<B: Backend> fmt::Debug for State<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cache.borrow())
    }
}

// TODO: cloning for `State` shouldn't be possible in general; Remove this and use
// checkpoints where possible.
impl Clone for State<StateDB> {
    fn clone(&self) -> State<StateDB> {
        let cache = {
            let mut cache: HashMap<Address, AccountEntry> = HashMap::new();
            for (key, val) in self.cache.borrow().iter() {
                if let Some(entry) = val.clone_if_dirty() {
                    cache.insert(*key, entry);
                }
            }
            cache
        };

        State {
            db: self.db.boxed_clone(),
            root: self.root,
            cache: RefCell::new(cache),
            checkpoints: RefCell::new(Vec::new()),
            account_start_nonce: self.account_start_nonce,
            factories: self.factories.clone(),
            account_permissions: self.account_permissions.clone(),
            group_accounts: self.group_accounts.clone(),
            super_admin_account: self.super_admin_account,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate libproto;
    extern crate logger;
    extern crate rustc_hex;
    ////////////////////////////////////////////////////////////////////////////////

    use self::libproto::blockchain;
    use self::rustc_hex::FromHex;
    use super::*;
    use cita_crypto::KeyPair;
    use cita_types::traits::LowerHex;
    use cita_types::{Address, H256};
    use engines::NullEngine;
    use evm::env_info::EnvInfo;
    use std::sync::Arc;
    use tests::helpers::*;
    use util::crypto::CreateKey;

    #[test]
    #[ignore]
    fn should_apply_create_transaction() {
        /*
        pragma solidity ^0.4.8;
        contract AbiTest {
          uint balance;
          function AbiTest() {}
          function setValue(uint value) {
            balance = value;
          }
        }
        */
        logger::silent();

        // 1) tx = (to, data(code), nonce, valid_until_block)
        let mut tx = blockchain::Transaction::new();
        tx.set_to(String::from(""));
        let nonce = "haha".to_owned();
        tx.set_nonce(nonce.clone());
        let block_limit = 100;
        tx.set_valid_until_block(block_limit);
        tx.set_quota(1844673);
        tx.set_data(
            "6060604052341561000f57600080fd5b60646000819055507f8fb1356be\
             6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3336064604051808373\
             ffffffffffffffffffffffffffffffffffffffff1673fffffffffffffffffffffffffffff\
             fffffffffff1681526020018281526020019250505060405180910390a161017580610092\
             6000396000f30060606040526000357c01000000000000000000000000000000000000000\
             00000000000000000900463ffffffff16806360fe47b1146100485780636d4ce63c146100\
             6b57600080fd5b341561005357600080fd5b6100696004808035906020019091905050610\
             094565b005b341561007657600080fd5b61007e610140565b604051808281526020019150\
             5060405180910390f35b7fc6d8c0af6d21f291e7c359603aa97e0ed500f04db6e983b9fce\
             75a91c6b8da6b816040518082815260200191505060405180910390a1806000819055507f\
             fd28ec3ec2555238d8ad6f9faf3e4cd10e574ce7e7ef28b73caa53f9512f65b9338260405\
             1808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffff\
             ffffffffffffffffff1681526020018281526020019250505060405180910390a150565b6\
             00080549050905600a165627a7a723058208777d774164b22030e359c5220ad3599f2a294\
             b4a0ae14b78c4f6a3246525c180029"
                .from_hex()
                .unwrap(),
        );

        // 2) stx = (from, content(code, nonce, signature))
        // TODO: Should get or generate private key that have send transation permission.
        //       Should work both for ed25519 and secp256k1.
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let stx = tx.sign(*privkey);

        // 4) signed
        let signed = SignedTransaction::new(&stx).unwrap();

        // 5)
        let mut state = get_temp_state();
        let info = EnvInfo {
            number: 0,
            author: Address::default(),
            timestamp: 0,
            difficulty: 0.into(),
            gas_limit: U256::from(u64::max_value()),
            last_hashes: Arc::new(vec![]),
            gas_used: 0.into(),
            account_gas_limit: 1844674.into(),
        };
        let contract_address = ::executive::contract_address(&signed.sender(), &U256::from(1));
        let engine = NullEngine::cita();

        println!("contract_address {:?}", contract_address);

        let check_options = CheckOptions {
            permission: false,
            quota: false,
            fee_back_platform: false,
            send_tx_permission: false,
            create_contract_permission: false,
        };
        let result = state
            .apply(
                &info,
                &engine,
                &signed,
                true,
                Default::default(),
                Address::from(0),
                &check_options,
            )
            .unwrap();
        println!(
            "{:?}",
            state
                .code(&contract_address)
                .expect("result should unwrap.")
                .expect("option should unwrap")
        );
        println!("{:?}", result.trace);
        assert_eq!(
            state.code(&contract_address).unwrap().unwrap(),
            Arc::new(
                "60606040526000357c010000000000000000000000000000000000000000000000\
                 0000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b\
                 605560048080359060200190919050506057565b005b806000819055505b505600\
                 a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e37\
                 1628798c41e292ca0029"
                    .from_hex()
                    .unwrap()
            )
        );
        assert_eq!(
            state.abi(&contract_address).unwrap().unwrap(),
            Arc::new(vec![])
        );
    }

    #[test]
    fn should_work_when_cloned() {
        let a = Address::zero();

        let mut state = {
            let mut state = get_temp_state();
            assert_eq!(state.exists(&a).unwrap(), false);
            state.inc_nonce(&a).unwrap();
            state.commit().unwrap();
            state.clone()
        };

        state.inc_nonce(&a).unwrap();
        state.commit().unwrap();
    }

    #[test]
    fn code_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state
                .require_or_from(
                    &a,
                    false,
                    false,
                    || Account::new_contract(42.into(), 0.into()),
                    |_| {},
                )
                .unwrap();
            state.init_code(&a, vec![1, 2, 3]).unwrap();
            assert_eq!(
                state.code(&a).unwrap(),
                Some(Arc::new([1u8, 2, 3].to_vec()))
            );

            state.commit().unwrap();
            assert_eq!(
                state.code(&a).unwrap(),
                Some(Arc::new([1u8, 2, 3].to_vec()))
            );
            state.drop()
        };

        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();

        assert_eq!(
            state.code(&a).unwrap(),
            Some(Arc::new([1u8, 2, 3].to_vec()))
        );
    }

    #[test]
    fn abi_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state
                .require_or_from(
                    &a,
                    false,
                    false,
                    || Account::new_contract(42.into(), 0.into()),
                    |_| {},
                )
                .unwrap();
            state.init_abi(&a, vec![1, 2, 3]).unwrap();
            assert_eq!(state.abi(&a).unwrap(), Some(Arc::new([1u8, 2, 3].to_vec())));

            state.commit().unwrap();
            assert_eq!(state.abi(&a).unwrap(), Some(Arc::new([1u8, 2, 3].to_vec())));
            state.drop()
        };
        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();

        assert_eq!(state.abi(&a).unwrap(), Some(Arc::new([1u8, 2, 3].to_vec())));
    }

    #[test]
    fn storage_at_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state
                .set_storage(&a, H256::from(1u64), H256::from(69u64))
                .unwrap();
            state.commit().unwrap();
            state.drop()
        };

        let s = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
        assert_eq!(
            s.storage_at(&a, &H256::from(1u64)).unwrap(),
            H256::from(69u64)
        );
    }

    #[test]
    fn state_proof() {
        let a = Address::from(0x1234u64);
        let b = Address::from(0x123456u64);
        let c = Address::from(0x12345678u64);
        let d = Address::from(0x4321u64);
        let err_adr = Address::from(0x1u64);
        let (root, db) = {
            let mut state = get_temp_state();
            state
                .set_storage(&a, H256::from(0x12u64), H256::from(69u64))
                .unwrap();
            state
                .set_storage(&a, H256::from(0x1234u64), H256::from(70u64))
                .unwrap();
            state
                .set_storage(&a, H256::from(0x123456u64), H256::from(71u64))
                .unwrap();
            state
                .set_storage(&a, H256::from(0x4321u64), H256::from(72u64))
                .unwrap();

            state
                .set_storage(&b, H256::from(2u64), H256::from(73u64))
                .unwrap();

            state
                .set_storage(&c, H256::from(2u64), H256::from(73u64))
                .unwrap();

            state
                .set_storage(&d, H256::from(2u64), H256::from(73u64))
                .unwrap();

            state.commit().unwrap();
            state.drop()
        };

        let s =
            State::from_existing(db, root.clone(), U256::from(0u8), Default::default()).unwrap();
        let state_proof_bs = s.get_state_proof(&a, &H256::from(0x123456u64)).unwrap();
        let mut state_proof = StateProof::from_bytes(&state_proof_bs);
        assert_eq!(state_proof.verify(root).unwrap(), H256::from(71u64));
        // test for error key
        assert_eq!(s.get_state_proof(&a, &H256::from(0x12345678u64)), None);
        assert_eq!(
            s.get_state_proof(&err_adr, &H256::from(0x12345678u64)),
            None
        );
        state_proof.set_address(err_adr);
        assert_eq!(state_proof.verify(root), None);

        let err_state_proof_bs = s.get_state_proof(&a, &H256::from(0x1u64)).unwrap();
        let err_state_proof = StateProof::from_bytes(&err_state_proof_bs);
        assert_eq!(err_state_proof.verify(root), None);
    }

    #[test]
    fn get_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state.inc_nonce(&a).unwrap();
            state.commit().unwrap();
            assert_eq!(state.nonce(&a).unwrap(), U256::from(1));
            state.drop()
        };

        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(1u64));
    }

    #[test]
    fn remove() {
        let a = Address::zero();
        let mut state = get_temp_state();
        assert_eq!(state.exists(&a).unwrap(), false);
        assert_eq!(state.exists_and_not_null(&a).unwrap(), false);
        state.inc_nonce(&a).unwrap();
        assert_eq!(state.exists(&a).unwrap(), true);
        assert_eq!(state.exists_and_not_null(&a).unwrap(), true);
        assert_eq!(state.nonce(&a).unwrap(), U256::from(1u64));
        state.kill_account(&a);
        assert_eq!(state.exists(&a).unwrap(), false);
        assert_eq!(state.exists_and_not_null(&a).unwrap(), false);
        assert_eq!(state.nonce(&a).unwrap(), U256::from(0u64));
    }

    #[test]
    fn empty_account_is_not_created() {
        let a = Address::zero();
        let db = get_temp_state_db();
        let (root, db) = {
            let mut state = State::new(db, U256::from(0u8), Default::default());
            state.commit().unwrap();
            state.drop()
        };
        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
        assert!(!state.exists(&a).unwrap());
        assert!(!state.exists_and_not_null(&a).unwrap());
    }

    #[test]
    fn empty_account_exists_when_creation_forced() {
        let a = Address::zero();
        let db = get_temp_state_db();
        let (root, db) = {
            let mut state = State::new(db, U256::from(0u8), Default::default());
            state.require(&a, false, false).unwrap();
            state.commit().unwrap();
            state.drop()
        };
        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
        assert!(state.exists(&a).unwrap());
        assert!(!state.exists_and_not_null(&a).unwrap());
    }

    #[test]
    fn remove_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state.inc_nonce(&a).unwrap();
            state.commit().unwrap();
            assert_eq!(state.exists(&a).unwrap(), true);
            assert_eq!(state.nonce(&a).unwrap(), U256::from(1u64));
            state.drop()
        };

        let (root, db) = {
            let mut state =
                State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
            assert_eq!(state.exists(&a).unwrap(), true);
            assert_eq!(state.nonce(&a).unwrap(), U256::from(1u64));
            state.kill_account(&a);
            state.commit().unwrap();
            assert_eq!(state.exists(&a).unwrap(), false);
            assert_eq!(state.nonce(&a).unwrap(), U256::from(0u64));
            state.drop()
        };

        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
        assert_eq!(state.exists(&a).unwrap(), false);
        assert_eq!(state.nonce(&a).unwrap(), U256::from(0u64));
    }

    #[test]
    fn alter_nonce() {
        let mut state = get_temp_state();
        let a = Address::zero();
        state.inc_nonce(&a).unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(1u64));
        state.inc_nonce(&a).unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(2u64));
        state.commit().unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(2u64));
        state.inc_nonce(&a).unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(3u64));
        state.commit().unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(3u64));
    }

    #[test]
    fn nonce() {
        let mut state = get_temp_state();
        let a = Address::zero();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(0u64));
        state.commit().unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(0u64));
    }

    #[test]
    fn ensure_cached() {
        let mut state = get_temp_state();
        let a = Address::zero();
        state.require(&a, false, false).unwrap();
        state.commit().unwrap();

        #[cfg(feature = "sha3hash")]
        let expected = "530acecc6ec873396bb3e90b6578161f9688ed7eeeb93d6fba5684895a93b78a";
        #[cfg(feature = "blake2bhash")]
        let expected = "da6a27e8063dd144a208f56f6b8dd6b3536ce6adbea93a1bcba95ce7fedb802c";
        #[cfg(feature = "sm3hash")]
        let expected = "63b984566f02930a2a959cef805ab0b17f68653a1430a2422e62f21a9f878cde";

        assert_eq!(state.root().lower_hex(), expected);
    }

    #[test]
    fn checkpoint_basic() {
        let mut state = get_temp_state();
        let a = Address::zero();
        state.checkpoint();
        state.inc_nonce(&a).unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(1));
        state.discard_checkpoint();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(1));
        state.checkpoint();
        state.inc_nonce(&a).unwrap();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(2));
        state.revert_to_checkpoint();
        assert_eq!(state.nonce(&a).unwrap(), U256::from(1));
    }

    #[test]
    fn create_empty() {
        let mut state = get_temp_state();
        state.commit().unwrap();

        #[cfg(feature = "sha3hash")]
        let expected = "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421";
        #[cfg(feature = "blake2bhash")]
        let expected = "c14af59107ef14003e4697a40ea912d865eb1463086a4649977c13ea69b0d9af";
        #[cfg(feature = "sm3hash")]
        let expected = "995b949869f80fa1465a9d8b6fa759ec65c3020d59c2624662bdff059bdf19b3";

        assert_eq!(state.root().lower_hex(), expected);
    }

    #[test]
    fn should_not_panic_on_state_diff_with_storage() {
        let mut state = get_temp_state();

        let a: Address = 0xa.into();
        state.init_code(&a, b"abcdefg".to_vec()).unwrap();;
        state.set_storage(&a, 0xb.into(), 0xc.into()).unwrap();

        let mut new_state = state.clone();
        new_state.set_storage(&a, 0xb.into(), 0xd.into()).unwrap();

        // new_state.diff_from(state).unwrap();
    }

}
