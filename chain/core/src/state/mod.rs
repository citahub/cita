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

//! A mutable state representation suitable to execute transactions.
//! Generic over a `Backend`. Deals with `Account`s.
//! Unconfirmed sub-states are managed with `checkpoint`s which may be canonicalized
//! or rolled back.

use engines::NullEngine;
use env_info::EnvInfo;
use error::Error;
use executive::{Executive, TransactOptions};
use factory::Factories;
use receipt::Receipt;
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fmt;
use std::sync::Arc;
use trace::FlatTrace;
use types::transaction::SignedTransaction;
use util::*;
use util::trie;

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
    // CleanCached,
    /// Account has been modified and is not committed to the trie yet.
    /// This is set if any of the account data is changed, including
    /// storage and code.
    Dirty,
    /// Account was modified and committed to the trie.
    Committed,
}

#[derive(Debug)]
/// In-memory copy of the account data. Holds the optional account
/// and the modification status.
/// Account entry can contain existing (`Some`) or non-existing
/// account (`None`)
struct AccountEntry {
    account: Option<Account>,
    state: AccountState,
}

// Account cache item. Contains account data and
// modification state
impl AccountEntry {
    fn is_dirty(&self) -> bool {
        self.state == AccountState::Dirty
    }

    /// Clone dirty data into new `AccountEntry`. This includes
    /// basic account data and modified storage keys.
    /// Returns None if clean.
    fn clone_if_dirty(&self) -> Option<AccountEntry> {
        match self.is_dirty() {
            true => Some(self.clone_dirty()),
            false => None,
        }
    }

    /// Clone dirty data into new `AccountEntry`. This includes
    /// basic account data and modified storage keys.
    fn clone_dirty(&self) -> AccountEntry {
        AccountEntry {
            account: self.account.as_ref().map(Account::clone_dirty),
            state: self.state,
        }
    }

    // Create a new account entry and mark it as dirty.
    fn new_dirty(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            account: account,
            state: AccountState::Dirty,
        }
    }

    // Create a new account entry and mark it as clean.
    fn new_clean(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            account: account,
            state: AccountState::CleanFresh,
        }
    }

    // Create a new account entry and mark it as clean and cached.
    // fn new_clean_cached(account: Option<Account>) -> AccountEntry {
    //     AccountEntry {
    //         account: account,
    //         state: AccountState::CleanCached,
    //     }
    // }

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
/// checkpoint can be discateded with `discard_checkpoint`. All of the orignal
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
}

#[derive(Copy, Clone)]
enum RequireCache {
    None,
    CodeSize,
    Code,
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

const SEC_TRIE_DB_UNWRAP_STR: &'static str = "A state can only be created with valid root. Creating a SecTrieDB with a valid root will not fail. Therefore creating a SecTrieDB with this state's root will not fail.";

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
            db: db,
            root: root,
            cache: RefCell::new(HashMap::new()),
            checkpoints: RefCell::new(Vec::new()),
            account_start_nonce: account_start_nonce,
            factories: factories,
        }
    }

    /// Creates new state with existing state root
    pub fn from_existing(db: B, root: H256, account_start_nonce: U256, factories: Factories) -> Result<State<B>, TrieError> {
        if !db.as_hashdb().contains(&root) {
            return Err(TrieError::InvalidStateRoot(root));
        }

        let state = State {
            db: db,
            root: root,
            cache: RefCell::new(HashMap::new()),
            checkpoints: RefCell::new(Vec::new()),
            account_start_nonce: account_start_nonce,
            factories: factories,
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
        if account.is_dirty() {
            if let Some(ref mut checkpoint) = self.checkpoints.borrow_mut().last_mut() {
                if !checkpoint.contains_key(address) {
                    checkpoint.insert(address.clone(), self.cache.borrow_mut().insert(address.clone(), account));
                    return;
                }
            }
        }
        self.cache.borrow_mut().insert(address.clone(), account);
    }

    fn note_cache(&self, address: &Address) {
        if let Some(ref mut checkpoint) = self.checkpoints.borrow_mut().last_mut() {
            if !checkpoint.contains_key(address) {
                checkpoint.insert(address.clone(), self.cache.borrow().get(address).map(AccountEntry::clone_dirty));
            }
        }
    }

    /// Destroy the current object and return root and database.
    pub fn drop(self) -> (H256, B) {
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
    pub fn new_contract(&mut self, contract: &Address, nonce_offset: U256) {
        self.insert_cache(contract, AccountEntry::new_dirty(Some(Account::new_contract(self.account_start_nonce + nonce_offset))));
    }

    /// Remove an existing account.
    pub fn kill_account(&mut self, account: &Address) {
        self.insert_cache(account, AccountEntry::new_dirty(None));
    }

    // TODO: Check it later.
    /// Determine whether an account exists.
    pub fn exists(&self, a: &Address) -> trie::Result<bool> {
        // Bloom filter does not contain empty accounts, so it is important here to
        // check if account exists in the database directly before EIP-161 is in effect.
        self.ensure_cached(a, RequireCache::None, false, |a| a.is_some())
    }

    /// Determine whether an account exists and if not empty.
    pub fn exists_and_not_null(&self, a: &Address) -> trie::Result<bool> {
        self.ensure_cached(a, RequireCache::None, false, |a| a.map_or(false, |a| !a.is_null()))
    }

    /// Get the nonce of account `a`.
    pub fn nonce(&self, a: &Address) -> trie::Result<U256> {
        self.ensure_cached(a, RequireCache::None, true, |a| a.as_ref().map_or(self.account_start_nonce, |account| *account.nonce()))
    }

    /// Get the storage root of account `a`.
    pub fn storage_root(&self, a: &Address) -> trie::Result<Option<H256>> {
        self.ensure_cached(a, RequireCache::None, true, |a| a.as_ref().and_then(|account| account.storage_root().cloned()))
    }

    // TODO: Add global cache.
    /// Mutate storage of account `address` so that it is `value` for `key`.
    pub fn storage_at(&self, address: &Address, key: &H256) -> trie::Result<H256> {
        // Storage key search and update works like this:
        // 1. If there's an entry for the account in the local cache check for the key and return it if found.
        // 2. If there's an entry for the account in the global cache check for the key or load it into that account.
        // 3. If account is missing in the global cache load it into the local cache and cache the key there.

        // check local cache first without updating
        {
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

            // otherwise cache the account localy and cache storage key there.
            if let Some(ref mut acc) = local_account {
                if let Some(ref account) = acc.account {
                    let account_db = self.factories.accountdb.readonly(self.db.as_hashdb(), account.address_hash(address));
                    return account.storage_at(&self.factories.trie, account_db.as_hashdb(), key);
                } else {
                    return Ok(H256::new());
                }
            }
        }

        // TODO: add account bloom. check if the account could exist before any requests to trie

        // account is not found in the global cache, get from the DB and insert into local
        let db = self.factories.trie.readonly(self.db.as_hashdb(), &self.root).expect(SEC_TRIE_DB_UNWRAP_STR);
        let maybe_acc = db.get_with(address, Account::from_rlp)?;
        let r = maybe_acc.as_ref().map_or(Ok(H256::new()), |a| {
            let account_db = self.factories.accountdb.readonly(self.db.as_hashdb(), a.address_hash(address));
            a.storage_at(&self.factories.trie, account_db.as_hashdb(), key)
        });
        self.insert_cache(address, AccountEntry::new_clean(maybe_acc));
        r
    }

    /// Get accounts' code.
    pub fn code(&self, a: &Address) -> trie::Result<Option<Arc<Bytes>>> {
        self.ensure_cached(a, RequireCache::Code, true, |a| a.as_ref().map_or(None, |a| a.code().clone()))
    }

    /// Get an account's code hash.
    pub fn code_hash(&self, a: &Address) -> trie::Result<H256> {
        self.ensure_cached(a, RequireCache::None, true, |a| a.as_ref().map_or(HASH_EMPTY, |a| a.code_hash()))
    }

    /// Get accounts' code size.
    pub fn code_size(&self, a: &Address) -> trie::Result<Option<usize>> {
        self.ensure_cached(a, RequireCache::CodeSize, true, |a| a.as_ref().and_then(|a| a.code_size()))
    }

    /// Increment the nonce of account `a` by 1.
    pub fn inc_nonce(&mut self, a: &Address) -> trie::Result<()> {
        self.require(a, false).map(|mut x| x.inc_nonce())
    }

    /// Mutate storage of account `a` so that it is `value` for `key`.
    pub fn set_storage(&mut self, a: &Address, key: H256, value: H256) -> trie::Result<()> {
        if self.storage_at(a, &key)? != value {
            self.require(a, false)?.set_storage(key, value)
        }

        Ok(())
    }

    /// Initialise the code of account `a` so that it is `code`.
    /// NOTE: Account should have been created with `new_contract`.
    pub fn init_code(&mut self, a: &Address, code: Bytes) -> trie::Result<()> {
        self.require_or_from(a, true, || Account::new_contract(self.account_start_nonce), |_| {})?
            .init_code(code);
        Ok(())
    }

    /// Reset the code of account `a` so that it is `code`.
    pub fn reset_code(&mut self, a: &Address, code: Bytes) -> trie::Result<()> {
        self.require_or_from(a, true, || Account::new_contract(self.account_start_nonce), |_| {})?
            .reset_code(code);
        Ok(())
    }

    /// Execute a given transaction.
    /// This will change the state accordingly.
    pub fn apply(&mut self, env_info: &EnvInfo, t: &SignedTransaction, tracing: bool) -> ApplyResult {
        //        let old = self.to_pod();
        let engine = &NullEngine::default();
        let options = TransactOptions {
            tracing: tracing,
            vm_tracing: false,
            check_nonce: false,
        };
        let vm_factory = self.factories.vm.clone();
        let native_factory = self.factories.native.clone();
        let e = Executive::new(self, env_info, engine, &vm_factory, &native_factory).transact(t, options)?;

        // TODO uncomment once to_pod() works correctly.
        //        trace!("Applied transaction. Diff:\n{}\n", state_diff::diff_pod(&old, &self.to_pod()));

        let receipt = Receipt::new(None, e.cumulative_gas_used, e.logs);
        trace!(target: "state", "Transaction receipt: {:?}", receipt);
        Ok(ApplyOutcome { receipt: receipt, trace: e.trace })
    }

    /// Commit accounts to SecTrieDBMut. This is similar to cpp-ethereum's dev::eth::commit.
    /// `accounts` is mutable because we may need to commit the code or storage and record that.
    #[cfg_attr(feature = "dev", allow(match_ref_pats))]
    #[cfg_attr(feature = "dev", allow(needless_borrow))]
    fn commit_into(factories: &Factories, db: &mut B, root: &mut H256, accounts: &mut HashMap<Address, AccountEntry>) -> Result<(), Error> {
        // first, commit the sub trees.
        for (address, ref mut a) in accounts.iter_mut().filter(|&(_, ref a)| a.is_dirty()) {
            if let Some(ref mut account) = a.account {
                let addr_hash = account.address_hash(address);
                {
                    let mut account_db = factories.accountdb.create(db.as_hashdb_mut(), addr_hash);
                    account.commit_storage(&factories.trie, account_db.as_hashdb_mut())?;

                    account.commit_code(account_db.as_hashdb_mut());
                }
            }
        }

        {
            let mut trie = factories.trie.from_existing(db.as_hashdb_mut(), root)?;
            for (address, ref mut a) in accounts.iter_mut().filter(|&(_, ref a)| a.is_dirty()) {
                a.state = AccountState::Committed;
                match a.account {
                    Some(ref mut account) => {
                        trie.insert(address, &account.rlp())?;
                    }
                    None => {
                        trie.remove(address)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Commits our cached account changes into the trie.
    pub fn commit(&mut self) -> Result<(), Error> {
        assert!(self.checkpoints.borrow().is_empty());
        Self::commit_into(&self.factories, &mut self.db, &mut self.root, &mut *self.cache.borrow_mut())
    }

    /// Clear state cache
    pub fn clear(&mut self) {
        self.cache.borrow_mut().clear();
    }

    // TODO
    // load required account data from the databases.
    fn update_account_cache(
        require: RequireCache,
        account: &mut Account,
        //state_db: &B,
        db: &HashDB,
    ) {
        match (account.is_cached(), require) {
            (true, _) |
            (false, RequireCache::None) => {}
            (false, _) => {
                // if there's already code in the global cache, always cache it
                // locally.
                // let hash = account.code_hash();
                // match state_db.get_cached_code(&hash) {
                //     Some(code) => account.cache_given_code(code),
                //     None => {
                //         match require {
                //             RequireCache::None => {}
                //             RequireCache::Code => {
                //                 if let Some(code) = account.cache_code(db) {
                //                     // propagate code loaded from the database to
                //                     // the global code cache.
                //                     state_db.cache_code(hash, code)
                //                 }
                //             }
                //             RequireCache::CodeSize => {
                //                 account.cache_code_size(db);
                //             }
                //         }
                //     }
                // }
                account.cache_code(db);
            }
        }
    }

    /// Check caches for required data
    /// First searches for account in the local, then the shared cache.
    /// Populates local cache if nothing found.
    fn ensure_cached<F, U>(&self, a: &Address, require: RequireCache, _: bool, f: F) -> trie::Result<U>
    where
        F: Fn(Option<&Account>) -> U,
    {
        // check local cache first
        if let Some(ref mut maybe_acc) = self.cache.borrow_mut().get_mut(a) {
            if let Some(ref mut account) = maybe_acc.account {
                let accountdb = self.factories.accountdb.readonly(self.db.as_hashdb(), account.address_hash(a));
                Self::update_account_cache(require,
                                           account,
                                           /* &self.db, */
                                           accountdb.as_hashdb());
                return Ok(f(Some(account)));
            }
            return Ok(f(None));
        }
        // TODO: check global cache

        // first check if it is not in database for sure

        // not found in the global cache, get from the DB and insert into local
        let db = self.factories.trie.readonly(self.db.as_hashdb(), &self.root)?;
        let mut maybe_acc = db.get_with(a, Account::from_rlp)?;
        if let Some(ref mut account) = maybe_acc.as_mut() {
            let accountdb = self.factories.accountdb.readonly(self.db.as_hashdb(), account.address_hash(a));
            Self::update_account_cache(require,
                                       account,
                                       /* &self.db, */
                                       accountdb.as_hashdb());
        }
        let r = f(maybe_acc.as_ref());
        self.insert_cache(a, AccountEntry::new_clean(maybe_acc));
        Ok(r)
    }

    /// Pull account `a` in our cache from the trie DB. `require_code` requires that the code be cached, too.
    fn require<'a>(&'a self, a: &Address, require_code: bool) -> trie::Result<RefMut<'a, Account>> {
        self.require_or_from(a, require_code, || Account::new_basic(self.account_start_nonce), |_| {})
    }

    /// Pull account `a` in our cache from the trie DB. `require_code` requires that the code be cached, too.
    /// If it doesn't exist, make account equal the evaluation of `default`.
    fn require_or_from<'a, F, G>(&'a self, a: &Address, require_code: bool, default: F, not_default: G) -> trie::Result<RefMut<'a, Account>>
    where
        F: FnOnce() -> Account,
        G: FnOnce(&mut Account),
    {
        let contains_key = self.cache.borrow().contains_key(a);
        if !contains_key {
            let db = self.factories.trie.readonly(self.db.as_hashdb(), &self.root)?;
            let maybe_acc = AccountEntry::new_clean(db.get_with(a, Account::from_rlp)?);
            self.insert_cache(a, maybe_acc);
        }
        self.note_cache(a);

        // at this point the entry is guaranteed to be in the cache.
        Ok(RefMut::map(self.cache.borrow_mut(), |c| {
            let entry = c.get_mut(a).expect("entry known to exist in the cache; qed");

            match &mut entry.account {
                &mut Some(ref mut acc) => not_default(acc),
                slot => *slot = Some(default()),
            }

            // set the dirty flag after changing account data.
            entry.state = AccountState::Dirty;
            match entry.account {
                Some(ref mut account) => {
                    if require_code {
                        let addr_hash = account.address_hash(a);
                        let accountdb = self.factories.accountdb.readonly(self.db.as_hashdb(), addr_hash);
                        Self::update_account_cache(RequireCache::Code,
                                                   account,
                                                   /* &self.db, */
                                                   accountdb.as_hashdb());
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
                    cache.insert(key.clone(), entry);
                }
            }
            cache
        };

        State {
            db: self.db.boxed_clone(),
            root: self.root.clone(),
            cache: RefCell::new(cache),
            checkpoints: RefCell::new(Vec::new()),
            account_start_nonce: self.account_start_nonce.clone(),
            factories: self.factories.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate libproto;
    extern crate cita_crypto;
    extern crate protobuf;
    extern crate env_logger;
    extern crate rustc_hex;
    ////////////////////////////////////////////////////////////////////////////////

    use self::libproto::blockchain;
    use self::rustc_hex::FromHex;
    use super::*;
    use cita_crypto::KeyPair;
    use env_info::EnvInfo;
    use std::sync::Arc;
    use tests::helpers::*;
    use util::{H256, Address};
    use util::crypto::CreateKey;
    use util::hashable::HASH_NAME;

    #[test]
    fn should_apply_create_transaction() {
        /*
        ~/codes/parity-contract-demo $ cat contracts/AbiTest.sol
        pragma solidity ^0.4.8;
        contract AbiTest {
          uint balance;
          function AbiTest() {}
          function setValue(uint value) {
            balance = value;
          }
        }
        ~/codes/parity-contract-demo $ solc contracts/AbiTest.sol  --bin-runtime --bin --hash
        Warning: This is a pre-release compiler version, please do not use it in production.

        ======= contracts/AbiTest.sol:AbiTest =======
        Binary:
        60606040523415600b57fe5b5b5b5b608e8061001c6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b605560048080359060200190919050506057565b005b806000819055505b505600a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e371628798c41e292ca0029
        Binary of the runtime part:
        60606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b605560048080359060200190919050506057565b005b806000819055505b505600a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e371628798c41e292ca0029
        Function signatures:
        55241077: setValue(uint256)
         */
        let _ = env_logger::init();

        // 1) tx = (to, data(code), nonce, valid_until_block)
        let mut tx = blockchain::Transaction::new();
        tx.set_to(String::from(""));
        let nonce = "haha".to_owned();
        tx.set_nonce(nonce.clone());
        let block_limit = 100;
        tx.set_valid_until_block(block_limit);
        tx.set_quota(184467440737095);
        tx.set_data("60606040523415600b57fe5b5b5b5b608e8061001c6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b605560048080359060200190919050506057565b005b806000819055505b505600a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e371628798c41e292ca0029"
                        .from_hex()
                        .unwrap());

        // 2) stx = (from, content(code, nonce, signature))
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let stx = tx.sign(*privkey);

        // 4) signed
        let signed = SignedTransaction::new(&stx).unwrap();

        // 5)
        let mut state = get_temp_state();
        let info = EnvInfo::default();
        //info.gas_limit = U256::from(100_000);
        let contract_address = ::executive::contract_address(&signed.sender(), &nonce, block_limit);
        let result = state.apply(&info, &signed, true).unwrap();
        println!("{:?}", state.code(&contract_address).unwrap().unwrap());
        println!("{:?}", result.trace);
        assert_eq!(state.code(&contract_address).unwrap().unwrap(),
                   Arc::new("60606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b605560048080359060200190919050506057565b005b806000819055505b505600a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e371628798c41e292ca0029"
                                .from_hex()
                                .unwrap()));
    }

    #[test]
    fn should_work_when_cloned() {
        // init_log();

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

    // #[test]
    // fn should_trace_failed_create_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Create,
    //             value: 100.into(),
    //             data: FromHex::from_hex("5b600056").unwrap(),
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         action: trace::Action::Create(trace::Create {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             value: 100.into(),
    //             gas: 78792.into(),
    //             init: vec![91, 96, 0, 86],
    //         }),
    //         result: trace::Res::FailedCreate(TraceError::OutOfGas),
    //         subtraces: 0
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_call_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(), FromHex::from_hex("6000").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(3),
    //             output: vec![]
    //         }),
    //         subtraces: 0,
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_basic_call_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(0),
    //             output: vec![]
    //         }),
    //         subtraces: 0,
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_call_transaction_to_builtin() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = &*Spec::new_test().engine;

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0x1.into()),
    //             value: 0.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     let result = state.apply(&info, engine, &t, true).unwrap();

    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: "0000000000000000000000000000000000000001".into(),
    //             value: 0.into(),
    //             gas: 79_000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(3000),
    //             output: vec![]
    //         }),
    //         subtraces: 0,
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_not_trace_subcall_transaction_to_builtin() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = &*Spec::new_test().engine;

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 0.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("600060006000600060006001610be0f1").unwrap())
    //         .unwrap();
    //     let result = state.apply(&info, engine, &t, true).unwrap();

    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 0.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(28_061),
    //             output: vec![]
    //         }),
    //         subtraces: 0,
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_not_trace_callcode() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = &*Spec::new_test().engine;

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 0.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("60006000600060006000600b611000f2").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xb.into(), FromHex::from_hex("6000").unwrap())
    //         .unwrap();
    //     let result = state.apply(&info, engine, &t, true).unwrap();

    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 0.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: 64.into(),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xa.into(),
    //             to: 0xa.into(),
    //             value: 0.into(),
    //             gas: 4096.into(),
    //             input: vec![],
    //             call_type: CallType::CallCode,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: 3.into(),
    //             output: vec![],
    //         }),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_not_trace_delegatecall() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     info.number = 0x789b0;
    //     let engine = &*Spec::new_test().engine;

    //     println!("schedule.have_delegate_call: {:?}",
    //              engine.schedule(&info).have_delegate_call);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 0.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("6000600060006000600b618000f4").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xb.into(), FromHex::from_hex("6000").unwrap())
    //         .unwrap();
    //     let result = state.apply(&info, engine, &t, true).unwrap();

    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 0.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(61),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 0.into(),
    //             gas: 32768.into(),
    //             input: vec![],
    //             call_type: CallType::DelegateCall,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: 3.into(),
    //             output: vec![],
    //         }),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_failed_call_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(), FromHex::from_hex("5b600056").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::FailedCall(TraceError::OutOfGas),
    //         subtraces: 0,
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_call_with_subcall_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("60006000600060006000600b602b5a03f1").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xb.into(), FromHex::from_hex("6000").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();

    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(69),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xa.into(),
    //             to: 0xb.into(),
    //             value: 0.into(),
    //             gas: 78934.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(3),
    //             output: vec![]
    //         }),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_call_with_basic_subcall_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("60006000600060006045600b6000f1").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(31761),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xa.into(),
    //             to: 0xb.into(),
    //             value: 69.into(),
    //             gas: 2300.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult::default()),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_not_trace_call_with_invalid_basic_subcall_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("600060006000600060ff600b6000f1").unwrap())
    //         .unwrap(); // not enough funds.
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(31761),
    //             output: vec![]
    //         }),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_failed_subcall_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![], //600480600b6000396000f35b600056
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("60006000600060006000600b602b5a03f1").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xb.into(), FromHex::from_hex("5b600056").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(79_000),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xa.into(),
    //             to: 0xb.into(),
    //             value: 0.into(),
    //             gas: 78934.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::FailedCall(TraceError::OutOfGas),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_call_with_subcall_with_subcall_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("60006000600060006000600b602b5a03f1").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xb.into(),
    //                    FromHex::from_hex("60006000600060006000600c602b5a03f1").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xc.into(), FromHex::from_hex("6000").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(135),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xa.into(),
    //             to: 0xb.into(),
    //             value: 0.into(),
    //             gas: 78934.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(69),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0, 0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xb.into(),
    //             to: 0xc.into(),
    //             value: 0.into(),
    //             gas: 78868.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(3),
    //             output: vec![]
    //         }),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_failed_subcall_with_subcall_transaction() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![], //600480600b6000396000f35b600056
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("60006000600060006000600b602b5a03f1").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xb.into(),
    //                    FromHex::from_hex("60006000600060006000600c602b5a03f1505b601256").unwrap())
    //         .unwrap();
    //     state
    //         .init_code(&0xc.into(), FromHex::from_hex("6000").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &(100.into()), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();

    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(79_000),
    //             output: vec![]
    //         })
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 1,
    //             action: trace::Action::Call(trace::Call {
    //             from: 0xa.into(),
    //             to: 0xb.into(),
    //             value: 0.into(),
    //             gas: 78934.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::FailedCall(TraceError::OutOfGas),
    //     }, FlatTrace {
    //         trace_address: vec![0, 0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Call(trace::Call {
    //             from: 0xb.into(),
    //             to: 0xc.into(),
    //             value: 0.into(),
    //             gas: 78868.into(),
    //             call_type: CallType::Call,
    //             input: vec![],
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: U256::from(3),
    //             output: vec![]
    //         }),
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    // #[test]
    // fn should_trace_suicide() {
    //     init_log();

    //     let mut state = get_temp_state();

    //     let mut info = EnvInfo::default();
    //     info.gas_limit = 1_000_000.into();
    //     let engine = TestEngine::new(5);

    //     let t = Transaction {
    //             nonce: 0.into(),
    //             gas_price: 0.into(),
    //             gas: 100_000.into(),
    //             action: Action::Call(0xa.into()),
    //             value: 100.into(),
    //             data: vec![],
    //         }
    //         .sign(&secret(), None);

    //     state
    //         .init_code(&0xa.into(),
    //                    FromHex::from_hex("73000000000000000000000000000000000000000bff").unwrap())
    //         .unwrap();
    //     state
    //         .add_balance(&0xa.into(), &50.into(), CleanupMode::NoEmpty)
    //         .unwrap();
    //     state
    //         .add_balance(&t.sender(), &100.into(), CleanupMode::NoEmpty)
    //         .unwrap();
    //     let result = state.apply(&info, &engine, &t, true).unwrap();
    //     let expected_trace = vec![FlatTrace {
    //         trace_address: Default::default(),
    //         subtraces: 1,
    //         action: trace::Action::Call(trace::Call {
    //             from: "9cce34f7ab185c7aba1b7c8140d620b4bda941d6".into(),
    //             to: 0xa.into(),
    //             value: 100.into(),
    //             gas: 79000.into(),
    //             input: vec![],
    //             call_type: CallType::Call,
    //         }),
    //         result: trace::Res::Call(trace::CallResult {
    //             gas_used: 3.into(),
    //             output: vec![]
    //         }),
    //     }, FlatTrace {
    //         trace_address: vec![0].into_iter().collect(),
    //         subtraces: 0,
    //         action: trace::Action::Suicide(trace::Suicide {
    //             address: 0xa.into(),
    //             refund_address: 0xb.into(),
    //             balance: 150.into(),
    //         }),
    //         result: trace::Res::None,
    //     }];

    //     assert_eq!(result.trace, expected_trace);
    // }

    #[test]
    fn code_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state.require_or_from(&a, false, || Account::new_contract(0.into()), |_| {}).unwrap();
            state.init_code(&a, vec![1, 2, 3]).unwrap();
            assert_eq!(state.code(&a).unwrap(), Some(Arc::new([1u8, 2, 3].to_vec())));

            state.commit().unwrap();
            assert_eq!(state.code(&a).unwrap(), Some(Arc::new([1u8, 2, 3].to_vec())));
            state.drop()
        };

        let state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();

        assert_eq!(state.code(&a).unwrap(), Some(Arc::new([1u8, 2, 3].to_vec())));
    }

    #[test]
    fn storage_at_from_database() {
        let a = Address::zero();
        let (root, db) = {
            let mut state = get_temp_state();
            state.set_storage(&a, H256::from(1u64), H256::from(69u64)).unwrap();
            state.commit().unwrap();
            state.drop()
        };

        let s = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
        assert_eq!(s.storage_at(&a, &H256::from(1u64)).unwrap(), H256::from(69u64));
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
            state.require(&a, false).unwrap();
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
            let mut state = State::from_existing(db, root, U256::from(0u8), Default::default()).unwrap();
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
        state.require(&a, false).unwrap();
        state.commit().unwrap();
        if HASH_NAME == "sha3" {
            assert_eq!(state.root().hex(), "42d8434e6d43bbdfa67dee7c7ef5c17159056e7727ac1ad5ba9928aeb9eb0112");
        } else if HASH_NAME == "blake2b" {
            assert_eq!(state.root().hex(), "a989e73cbcbb961ac9777ca453449c42a0c008c70ef16326b7d4c96681f5d90d");
        }
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

    // #[test]
    // fn checkpoint_nested() {
    //     let mut state = get_temp_state();
    //     let a = Address::zero();
    //     state.checkpoint();
    //     state.checkpoint();
    //     state
    //         .add_balance(&a, &U256::from(69u64), CleanupMode::NoEmpty)
    //         .unwrap();
    //     assert_eq!(state.balance(&a).unwrap(), U256::from(69u64));
    //     state.discard_checkpoint();
    //     assert_eq!(state.balance(&a).unwrap(), U256::from(69u64));
    //     state.revert_to_checkpoint();
    //     assert_eq!(state.balance(&a).unwrap(), U256::from(0));
    // }

    #[test]
    fn create_empty() {
        let mut state = get_temp_state();
        if HASH_NAME == "sha3" {
            state.commit().unwrap();
            assert_eq!(state.root().hex(), "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
        } else if HASH_NAME == "blake2b" {
            state.commit().unwrap();
            assert_eq!(state.root().hex(), "c14af59107ef14003e4697a40ea912d865eb1463086a4649977c13ea69b0d9af");
        }
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
