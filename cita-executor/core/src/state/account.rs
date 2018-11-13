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

// CITA, Copyright 2016-2017 Cryptape Technologies LLC.
// Add abi

//! Single account in the system.

use cita_types::traits::LowerHex;
use cita_types::{Address, H256, U256};
use lru_cache::LruCache;
use pod_account::*;
use rlp::*;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use types::basic_account::BasicAccount;
use util::*;

const STORAGE_CACHE_ITEMS: usize = 8192;

/// Single account in the system.
/// Keeps track of changes to the code and storage.
/// The changes are applied in `commit_storage` and `commit_code`
pub struct Account {
    // Balance of the account.
    balance: U256,
    /// Nonce of the account.
    nonce: U256,
    /// Trie-backed storage.
    storage_root: H256,
    /// LRU Cache of the trie-backed storage.
    /// This is limited to `STORAGE_CACHE_ITEMS` recent queries
    storage_cache: RefCell<LruCache<H256, H256>>,
    /// Modified storage. Accumulates changes to storage made in `set_storage`
    /// Takes precedence over `storage_cache`.
    storage_changes: HashMap<H256, H256>,
    /// Code hash of the account.
    code_hash: H256,
    /// Size of the account code.
    code_size: Option<usize>,
    /// Code cache of the account.
    code_cache: Arc<Bytes>,
    /// Account code new or has been modified.
    code_filth: Filth,
    /// ABI hash of the account.
    abi_hash: H256,
    /// Size of the account ABI.
    abi_size: Option<usize>,
    /// ABI cache of the account.
    abi_cache: Arc<Bytes>,
    /// Account ABI new or has been modified.
    abi_filth: Filth,
    /// Cached address hash.
    address_hash: Cell<Option<H256>>,
}

impl From<BasicAccount> for Account {
    fn from(basic: BasicAccount) -> Self {
        Account {
            balance: basic.balance,
            nonce: basic.nonce,
            storage_root: basic.storage_root,
            storage_cache: Self::empty_storage_cache(),
            storage_changes: HashMap::new(),
            code_hash: basic.code_hash,
            code_size: None,
            code_cache: Arc::new(vec![]),
            code_filth: Filth::Clean,
            abi_hash: basic.abi_hash,
            abi_size: None,
            abi_cache: Arc::new(vec![]),
            abi_filth: Filth::Clean,
            address_hash: Cell::new(None),
        }
    }
}

impl Account {
    #[cfg(test)]
    /// General constructor.
    pub fn new(
        balance: U256,
        nonce: U256,
        storage: HashMap<H256, H256>,
        code: Bytes,
        abi: Bytes,
    ) -> Account {
        Account {
            balance,
            nonce,
            storage_root: HASH_NULL_RLP,
            storage_cache: Self::empty_storage_cache(),
            storage_changes: storage,
            code_hash: code.crypt_hash(),
            code_size: Some(code.len()),
            code_cache: Arc::new(code),
            code_filth: Filth::Dirty,
            abi_hash: abi.crypt_hash(),
            abi_size: Some(abi.len()),
            abi_cache: Arc::new(abi),
            abi_filth: Filth::Dirty,
            address_hash: Cell::new(None),
        }
    }

    fn empty_storage_cache() -> RefCell<LruCache<H256, H256>> {
        RefCell::new(LruCache::new(STORAGE_CACHE_ITEMS))
    }

    /// General constructor.
    pub fn from_pod(pod: PodAccount) -> Account {
        Account {
            balance: pod.balance,
            nonce: pod.nonce,
            storage_root: HASH_NULL_RLP,
            storage_cache: Self::empty_storage_cache(),
            storage_changes: pod.storage.into_iter().collect(),
            code_hash: pod.code.as_ref().map_or(HASH_EMPTY, |c| c.crypt_hash()),
            code_filth: Filth::Dirty,
            code_size: Some(pod.code.as_ref().map_or(0, |c| c.len())),
            code_cache: Arc::new(pod.code.map_or_else(
                || {
                    warn!("POD account with unknown code is being created! Assuming no code.");
                    vec![]
                },
                |c| c,
            )),
            abi_hash: pod.abi.as_ref().map_or(HASH_EMPTY, |c| c.crypt_hash()),
            abi_filth: Filth::Dirty,
            abi_size: Some(pod.abi.as_ref().map_or(0, |c| c.len())),
            abi_cache: Arc::new(pod.abi.map_or_else(
                || {
                    warn!("POD account with unknown ABI is being created! Assuming no abi.");
                    vec![]
                },
                |c| c,
            )),
            address_hash: Cell::new(None),
        }
    }

    /// Create a new account.
    pub fn new_basic(balance: U256, nonce: U256) -> Account {
        Account {
            balance,
            nonce,
            storage_root: HASH_NULL_RLP,
            storage_cache: Self::empty_storage_cache(),
            storage_changes: HashMap::new(),
            code_hash: HASH_EMPTY,
            code_cache: Arc::new(vec![]),
            code_size: Some(0),
            code_filth: Filth::Clean,
            abi_hash: HASH_EMPTY,
            abi_cache: Arc::new(vec![]),
            abi_size: Some(0),
            abi_filth: Filth::Clean,
            address_hash: Cell::new(None),
        }
    }

    /// Create a new account from RLP.
    pub fn from_rlp(rlp: &[u8]) -> Account {
        let basic: BasicAccount = decode(rlp);
        basic.into()
    }

    /// Create a new contract account.
    /// NOTE: make sure you use `init_code` on this before `commit`ing.
    pub fn new_contract(balance: U256, nonce: U256) -> Account {
        Account {
            balance,
            nonce,
            storage_root: HASH_NULL_RLP,
            storage_cache: Self::empty_storage_cache(),
            storage_changes: HashMap::new(),
            code_hash: HASH_EMPTY,
            code_cache: Arc::new(vec![]),
            code_size: None,
            code_filth: Filth::Clean,
            abi_hash: HASH_EMPTY,
            abi_cache: Arc::new(vec![]),
            abi_size: None,
            abi_filth: Filth::Clean,
            address_hash: Cell::new(None),
        }
    }

    /// Set this account's code to the given code.
    /// NOTE: Account should have been created with `new_contract()`
    pub fn init_code(&mut self, code: Bytes) {
        self.code_hash = code.crypt_hash();
        self.code_cache = Arc::new(code);
        self.code_size = Some(self.code_cache.len());
        self.code_filth = Filth::Dirty;
    }

    /// Set this account's ABI to the given ABI.
    pub fn init_abi(&mut self, abi: Bytes) {
        self.abi_hash = abi.crypt_hash();
        self.abi_cache = Arc::new(abi);
        self.abi_size = Some(self.abi_cache.len());
        self.abi_filth = Filth::Dirty;
    }

    /// Reset this account's code to the given code.
    pub fn reset_code(&mut self, code: Bytes) {
        self.init_code(code);
    }

    /// Reset this account's ABI to the given ABI.
    pub fn reset_abi(&mut self, abi: Bytes) {
        self.init_abi(abi);
    }

    /// Set (and cache) the contents of the trie's storage at `key` to `value`.
    pub fn set_storage(&mut self, key: H256, value: H256) {
        self.storage_changes.insert(key, value);
    }

    /// Get (and cache) the contents of the trie's storage at `key`.
    /// Takes modifed storage into account.
    pub fn storage_at(
        &self,
        trie_factory: &TrieFactory,
        db: &HashDB,
        key: &H256,
    ) -> trie::Result<H256> {
        if let Some(value) = self.cached_storage_at(key) {
            return Ok(value);
        }

        let t = trie_factory.readonly(db, &self.storage_root)?;

        let item: U256 = t.get_with(key, ::rlp::decode)?.unwrap_or_else(U256::zero);
        let value: H256 = item.into();
        self.storage_cache.borrow_mut().insert(*key, value);
        Ok(value)
    }

    /// Get value proof of the trie's storage at `key`.
    pub fn get_value_proof(
        &self,
        trie_factory: &TrieFactory,
        db: &HashDB,
        key: &H256,
    ) -> Option<Vec<Bytes>> {
        trie_factory
            .readonly(db, &self.storage_root)
            .ok()
            .and_then(|t| t.get_value_proof(key))
    }

    /// Verify value proof of the trie's storage at `key`.
    pub fn verify_value_proof(&self, key: &H256, proof: &[Bytes]) -> Option<H256> {
        trie::triedb::verify_value_proof(key, self.storage_root, proof, ::rlp::decode)
            .map(|v: U256| v.into())
    }

    /// Get cached storage value if any. Returns `None` if the
    /// key is not in the cache.
    pub fn cached_storage_at(&self, key: &H256) -> Option<H256> {
        if let Some(value) = self.storage_changes.get(key) {
            return Some(*value);
        }
        if let Some(value) = self.storage_cache.borrow_mut().get_mut(key) {
            return Some(*value);
        }
        None
    }

    /// return the balance associated with this account.
    pub fn balance(&self) -> &U256 {
        &self.balance
    }

    /// return the nonce associated with this account.
    pub fn nonce(&self) -> &U256 {
        &self.nonce
    }

    /// return the code hash associated with this account.
    pub fn code_hash(&self) -> H256 {
        self.code_hash
    }

    /// return the abi hash associated with this account.
    pub fn abi_hash(&self) -> H256 {
        self.abi_hash
    }

    /// return the code hash associated with this account.
    pub fn address_hash(&self, address: &Address) -> H256 {
        let hash = self.address_hash.get();
        hash.unwrap_or_else(|| {
            let hash = address.crypt_hash();
            self.address_hash.set(Some(hash));
            hash
        })
    }

    /// returns the account's code. If `None` then the code cache isn't available -
    /// get someone who knows to call `note_code`.
    pub fn code(&self) -> Option<Arc<Bytes>> {
        if self.code_hash != HASH_EMPTY && self.code_cache.is_empty() {
            return None;
        }
        Some(Arc::clone(&self.code_cache))
    }

    /// returns the account's abi. If `None` then the abi cache isn't available -
    /// get someone who knows to call `abi_code`.
    pub fn abi(&self) -> Option<Arc<Bytes>> {
        if self.abi_hash != HASH_EMPTY && self.abi_cache.is_empty() {
            return None;
        }
        Some(Arc::clone(&self.abi_cache))
    }

    /// returns the account's code size. If `None` then the code cache or code size cache isn't available -
    /// get someone who knows to call `note_code`.
    pub fn code_size(&self) -> Option<usize> {
        self.code_size
    }

    /// returns the account's ABI size. If `None` then the ABI cache or ABI size cache isn't available -
    /// get someone who knows to call `note_abi`.
    pub fn abi_size(&self) -> Option<usize> {
        self.abi_size
    }

    #[cfg(test)]
    /// Provide a byte array which hashes to the `code_hash`. returns the hash as a result.
    pub fn note_code(&mut self, code: Bytes) -> Result<(), H256> {
        let h = code.crypt_hash();
        if self.code_hash == h {
            self.code_cache = Arc::new(code);
            self.code_size = Some(self.code_cache.len());
            Ok(())
        } else {
            Err(h)
        }
    }

    #[cfg(test)]
    /// Provide a byte array which hashes to the `abi_hash`. returns the hash as a result.
    pub fn note_abi(&mut self, abi: Bytes) -> Result<(), H256> {
        let h = abi.crypt_hash();
        if self.abi_hash == h {
            self.abi_cache = Arc::new(abi);
            self.abi_size = Some(self.abi_cache.len());
            Ok(())
        } else {
            Err(h)
        }
    }

    /// Is `code_cache` valid; such that code is going to return Some?
    pub fn is_cached(&self) -> bool {
        !self.code_cache.is_empty() || (self.code_cache.is_empty() && self.code_hash == HASH_EMPTY)
    }

    /// Is `abi_cache` valid; such that abi is going to return Some?
    pub fn is_abi_cached(&self) -> bool {
        !self.abi_cache.is_empty() || (self.abi_cache.is_empty() && self.abi_hash == HASH_EMPTY)
    }

    /// Provide a database to get `code_hash`. Should not be called if it is a contract without code.
    pub fn cache_code(&mut self, db: &HashDB) -> Option<Arc<Bytes>> {
        // TODO: fill out self.code_cache;
        trace!(
            "Account::cache_code: ic={}; self.code_hash={:?}, self.code_cache={}",
            self.is_cached(),
            self.code_hash,
            self.code_cache.lower_hex()
        );

        if self.is_cached() {
            return Some(Arc::clone(&self.code_cache));
        }

        match db.get(&self.code_hash) {
            Some(x) => {
                self.code_size = Some(x.len());
                self.code_cache = Arc::new(x.to_vec());
                Some(Arc::clone(&self.code_cache))
            }
            _ => {
                warn!("Failed get code of {}", self.code_hash);
                None
            }
        }
    }

    /// Provide a database to get `abi_hash`. Should not be called if it is a contract without abi.
    pub fn cache_abi(&mut self, db: &HashDB) -> Option<Arc<Bytes>> {
        // TODO: fill out self.abi_cache;
        trace!(
            "Account::cache_abi: ic={}; self.abi_hash={:?}, self.abi_cache={}",
            self.is_abi_cached(),
            self.abi_hash,
            self.abi_cache.lower_hex()
        );

        if self.is_abi_cached() {
            return Some(Arc::clone(&self.abi_cache));
        }

        match db.get(&self.abi_hash) {
            Some(x) => {
                self.abi_size = Some(x.len());
                self.abi_cache = Arc::new(x.to_vec());
                Some(Arc::clone(&self.abi_cache))
            }
            _ => {
                warn!("Failed get abi of {}", self.abi_hash);
                None
            }
        }
    }

    /// Provide code to cache. For correctness, should be the correct code for the
    /// account.
    pub fn cache_given_code(&mut self, code: Arc<Bytes>) {
        trace!(
            "Account::cache_given_code: ic={}; self.code_hash={:?}, self.code_cache={}",
            self.is_cached(),
            self.code_hash,
            self.code_cache.lower_hex()
        );

        self.code_size = Some(code.len());
        self.code_cache = code;
    }

    /// Provide ABI to cache. For correctness, should be the correct ABI for the
    /// account.
    pub fn cache_given_abi(&mut self, abi: Arc<Bytes>) {
        trace!(
            "Account::cache_given_abi: ic={}; self.abi_hash={:?}, self.abi_cache={}",
            self.is_abi_cached(),
            self.abi_hash,
            self.abi_cache.lower_hex()
        );

        self.abi_size = Some(abi.len());
        self.abi_cache = abi;
    }

    /// Provide a database to get `code_size`. Should not be called if it is a contract without code.
    pub fn cache_code_size(&mut self, db: &HashDB) -> bool {
        // TODO: fill out self.code_cache;
        trace!(
            "Account::cache_code_size: ic={}; self.code_hash={:?}, self.code_cache={}",
            self.is_cached(),
            self.code_hash,
            self.code_cache.lower_hex()
        );
        self.code_size.is_some() || if self.code_hash != HASH_EMPTY {
            match db.get(&self.code_hash) {
                Some(x) => {
                    self.code_size = Some(x.len());
                    true
                }
                _ => {
                    warn!("Failed get code of {}", self.code_hash);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Provide a database to get `abi_size`. Should not be called if it is a contract without abi.
    pub fn cache_abi_size(&mut self, db: &HashDB) -> bool {
        // TODO: fill out self.abi_cache;
        trace!(
            "Account::cache_abi_size: ic={}; self.abi_hash={:?}, self.abi_cache={}",
            self.is_abi_cached(),
            self.abi_hash,
            self.abi_cache.lower_hex()
        );
        self.abi_size.is_some() || if self.abi_hash != HASH_EMPTY {
            match db.get(&self.abi_hash) {
                Some(x) => {
                    self.abi_size = Some(x.len());
                    true
                }
                _ => {
                    warn!("Failed get abi of {}", self.abi_hash);
                    false
                }
            }
        } else {
            false
        }
    }

    /// Determine whether there are any un-`commit()`-ed storage-setting operations.
    pub fn storage_is_clean(&self) -> bool {
        self.storage_changes.is_empty()
    }

    /// Check if account has zero nonce, balance, no code and no storage.
    ///
    /// NOTE: Will panic if `!self.storage_is_clean()`
    pub fn is_empty(&self) -> bool {
        assert!(
            self.storage_is_clean(),
            "Account::is_empty() may only legally be called when storage is clean."
        );
        self.is_null() && self.storage_root == HASH_NULL_RLP
    }

    /// Check if account has zero nonce, balance, no code, no abi.
    pub fn is_null(&self) -> bool {
        self.balance.is_zero()
            && self.nonce.is_zero()
            && self.code_hash == HASH_EMPTY
            && self.abi_hash == HASH_EMPTY
    }

    /// Check if account is basic (Has no code).
    pub fn is_basic(&self) -> bool {
        self.code_hash == HASH_EMPTY
    }

    /// Return the storage root associated with this account or None if it has been altered via the overlay.
    pub fn storage_root(&self) -> Option<&H256> {
        if self.storage_is_clean() {
            Some(&self.storage_root)
        } else {
            None
        }
    }

    /// Return the storage overlay.
    pub fn storage_changes(&self) -> &HashMap<H256, H256> {
        &self.storage_changes
    }

    /// Increment the nonce of the account by one.
    pub fn inc_nonce(&mut self) {
        self.nonce = self.nonce + U256::from(1u8);
    }

    /// Increase account balance.
    pub fn add_balance(&mut self, x: &U256) {
        self.balance = self.balance.saturating_add(*x);
    }

    /// Decrease account balance.
    pub fn sub_balance(&mut self, x: &U256) {
        self.balance = self.balance.saturating_sub(*x);
    }

    /// Commit the `storage_changes` to the backing DB and update `storage_root`.
    pub fn commit_storage(
        &mut self,
        trie_factory: &TrieFactory,
        db: &mut HashDB,
    ) -> trie::Result<()> {
        let mut t = trie_factory.from_existing(db, &mut self.storage_root)?;
        for (k, v) in self.storage_changes.drain() {
            // cast key and value to trait type,
            // so we can call overloaded `to_bytes` method
            if v.is_zero() {
                t.remove(&k)?
            } else {
                t.insert(&k, &encode(&U256::from(&*v)))?
            };

            self.storage_cache.borrow_mut().insert(k, v);
        }
        Ok(())
    }

    /// Commit any unsaved code. `code_hash` will always return the hash of the `code_cache` after this.
    pub fn commit_code(&mut self, db: &mut HashDB) {
        match (self.code_filth == Filth::Dirty, self.code_cache.is_empty()) {
            (true, true) => {
                self.code_size = Some(0);
                self.code_filth = Filth::Clean;
            }
            (true, false) => {
                db.emplace(self.code_hash, DBValue::from_slice(&*self.code_cache));
                self.code_size = Some(self.code_cache.len());
                self.code_filth = Filth::Clean;
            }
            (false, _) => {}
        }
    }

    /// Commit any unsaved abi. `abi_hash` will always return the hash of the `abi_cache` after this.
    pub fn commit_abi(&mut self, db: &mut HashDB) {
        match (self.abi_filth == Filth::Dirty, self.abi_cache.is_empty()) {
            (true, true) => {
                self.abi_size = Some(0);
                self.abi_filth = Filth::Clean;
            }
            (true, false) => {
                db.emplace(self.abi_hash, DBValue::from_slice(&*self.abi_cache));
                self.abi_size = Some(self.abi_cache.len());
                self.abi_filth = Filth::Clean;
            }
            (false, _) => {}
        }
    }

    /// Export to RLP.
    pub fn rlp(&self) -> Bytes {
        let mut stream = RlpStream::new_list(5);
        stream.append(&self.nonce);
        stream.append(&self.balance);
        stream.append(&self.storage_root);
        stream.append(&self.code_hash);
        stream.append(&self.abi_hash);
        stream.out()
    }

    /// Clone basic account data
    pub fn clone_basic(&self) -> Account {
        Account {
            balance: self.balance,
            nonce: self.nonce,
            storage_root: self.storage_root,
            storage_cache: Self::empty_storage_cache(),
            storage_changes: HashMap::new(),
            code_hash: self.code_hash,
            code_size: self.code_size,
            code_cache: Arc::clone(&self.code_cache),
            code_filth: self.code_filth,
            abi_hash: self.abi_hash,
            abi_size: self.abi_size,
            abi_cache: Arc::clone(&self.abi_cache),
            abi_filth: self.abi_filth,
            address_hash: self.address_hash.clone(),
        }
    }

    /// Clone account data and dirty storage keys
    pub fn clone_dirty(&self) -> Account {
        let mut account = self.clone_basic();
        account.storage_changes = self.storage_changes.clone();
        account.code_cache = Arc::clone(&self.code_cache);
        account.abi_cache = Arc::clone(&self.abi_cache);
        account
    }

    /// Clone account data, dirty storage keys and cached storage keys.
    pub fn clone_all(&self) -> Account {
        let mut account = self.clone_dirty();
        account.storage_cache = self.storage_cache.clone();
        account
    }

    /// Replace self with the data from other account merging storage cache.
    /// Basic account data and all modifications are overwritten
    /// with new values.
    pub fn overwrite_with(&mut self, other: Account) {
        self.balance = other.balance;
        self.nonce = other.nonce;
        self.storage_root = other.storage_root;
        self.code_hash = other.code_hash;
        self.code_filth = other.code_filth;
        self.code_cache = other.code_cache;
        self.code_size = other.code_size;
        self.abi_hash = other.abi_hash;
        self.abi_filth = other.abi_filth;
        self.abi_cache = other.abi_cache;
        self.abi_size = other.abi_size;
        self.address_hash = other.address_hash;
        let mut cache = self.storage_cache.borrow_mut();
        for (k, v) in other.storage_cache.into_inner() {
            cache.insert(k, v); //TODO: cloning should not be required here
        }
        self.storage_changes = other.storage_changes;
    }
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", PodAccount::from_account(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use account_db::*;
    use rlp::{Compressible, RlpType, UntrustedRlp};

    #[test]
    fn account_compress() {
        let raw = Account::new_basic(0.into(), 4.into()).rlp();
        let rlp = UntrustedRlp::new(&raw);
        let compact_vec = rlp.compress(RlpType::Snapshot).to_vec();
        //assert!(raw.len() > compact_vec.len());
        let again_raw = UntrustedRlp::new(&compact_vec).decompress(RlpType::Snapshot);
        assert_eq!(raw, again_raw.to_vec());
    }

    #[test]
    fn storage_at() {
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        let rlp = {
            let mut a = Account::new_contract(0.into(), 0.into());
            a.set_storage(
                H256::from(&U256::from(0x00u64)),
                H256::from(&U256::from(0x1234u64)),
            );
            a.commit_storage(&Default::default(), &mut db).unwrap();
            a.init_code(vec![]);
            a.commit_code(&mut db);
            a.init_abi(vec![]);
            a.commit_abi(&mut db);
            a.rlp()
        };

        let a = Account::from_rlp(&rlp);

        #[cfg(feature = "sha3hash")]
        let expected = "71623f5ec821de33ad5aa81f8c82f0916c6f60de0a536f8c466d440c56715bd5";
        #[cfg(feature = "blake2bhash")]
        let expected = "01d418c29a2942a1257a3be24134e125d9ef52ca2c0e9174969cd86fb9bf74e9";
        #[cfg(feature = "sm3hash")]
        let expected = "293a5287836ae79f19f88d45b5ac5b2c2fd2f60b7c4250ff595cb228582f49c5";

        assert_eq!(a.storage_root().unwrap().lower_hex(), expected);

        assert_eq!(
            a.storage_at(
                &Default::default(),
                &db.immutable(),
                &H256::from(&U256::from(0x00u64))
            )
            .unwrap(),
            H256::from(&U256::from(0x1234u64))
        );
        assert_eq!(
            a.storage_at(
                &Default::default(),
                &db.immutable(),
                &H256::from(&U256::from(0x01u64))
            )
            .unwrap(),
            H256::new()
        );
    }

    #[test]
    fn value_proof() {
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        let rlp = {
            let mut a = Account::new_contract(0.into(), 0.into());
            a.set_storage(
                H256::from(&U256::from(0x1234u64)),
                H256::from(&U256::from(0x4321u64)),
            );
            a.set_storage(
                H256::from(&U256::from(0x123456u64)),
                H256::from(&U256::from(0x654321u64)),
            );
            a.set_storage(
                H256::from(&U256::from(0x12345678u64)),
                H256::from(&U256::from(0x87654321u64)),
            );
            a.set_storage(
                H256::from(&U256::from(0x654321u64)),
                H256::from(&U256::from(0x123456u64)),
            );

            a.commit_storage(&Default::default(), &mut db).unwrap();
            a.init_code(vec![]);
            a.commit_code(&mut db);
            a.init_abi(vec![]);
            a.commit_abi(&mut db);
            a.rlp()
        };

        let a = Account::from_rlp(&rlp);

        {
            let value_proof = a
                .get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x1234u64)),
                )
                .unwrap();
            let val = a
                .verify_value_proof(&H256::from(&U256::from(0x1234u64)), &value_proof)
                .unwrap();

            assert_eq!(val, H256::from(&U256::from(0x4321u64)));
        }

        {
            let value_proof = a
                .get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x123456u64)),
                )
                .unwrap();
            let val = a
                .verify_value_proof(&H256::from(&U256::from(0x123456u64)), &value_proof)
                .unwrap();

            assert_eq!(val, H256::from(&U256::from(0x654321u64)));
        }

        {
            let value_proof = a
                .get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x12345678u64)),
                )
                .unwrap();
            let val = a
                .verify_value_proof(&H256::from(&U256::from(0x12345678u64)), &value_proof)
                .unwrap();

            assert_eq!(val, H256::from(&U256::from(0x87654321u64)));
        }

        {
            let value_proof = a
                .get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x654321u64)),
                )
                .unwrap();
            let val = a
                .verify_value_proof(&H256::from(&U256::from(0x654321u64)), &value_proof)
                .unwrap();

            assert_eq!(val, H256::from(&U256::from(0x123456u64)));
        }

        // bad case
        {
            assert_eq!(
                a.get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x65432100u64)),
                ),
                None
            );
            let value_proof = a
                .get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x654321u64)),
                )
                .unwrap();
            assert_eq!(
                a.verify_value_proof(&H256::from(&U256::from(0x65u64)), &value_proof),
                None
            )
        }
        // bad case
        {
            let value_proof = a
                .get_value_proof(
                    &Default::default(),
                    &db.immutable(),
                    &H256::from(&U256::from(0x65u64)),
                )
                .unwrap();
            assert_eq!(
                a.verify_value_proof(&H256::from(&U256::from(0x65u64)), &value_proof),
                None
            )
        }
    }

    #[test]
    fn note_code() {
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());

        let rlp = {
            let mut a = Account::new_contract(0.into(), 0.into());
            a.init_code(vec![0x55, 0x44, 0xffu8]);
            a.commit_code(&mut db);
            a.rlp()
        };

        let mut a = Account::from_rlp(&rlp);
        assert!(a.cache_code(&db.immutable()).is_some());

        let mut a = Account::from_rlp(&rlp);
        assert_eq!(a.note_code(vec![0x55, 0x44, 0xffu8]), Ok(()));
    }

    #[test]
    fn note_abi() {
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());

        let rlp = {
            let mut a = Account::new_contract(0.into(), 0.into());
            a.init_abi(vec![0x55, 0x44, 0xffu8]);
            a.commit_abi(&mut db);
            a.rlp()
        };

        let mut a = Account::from_rlp(&rlp);
        assert!(a.cache_abi(&db.immutable()).is_some());

        let mut a = Account::from_rlp(&rlp);
        assert_eq!(a.note_abi(vec![0x55, 0x44, 0xffu8]), Ok(()));
    }

    #[test]
    fn commit_storage() {
        let mut a = Account::new_contract(0.into(), 0.into());
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        a.set_storage(0.into(), 0x1234.into());
        assert_eq!(a.storage_root(), None);

        #[cfg(feature = "sha3hash")]
        let expected = "71623f5ec821de33ad5aa81f8c82f0916c6f60de0a536f8c466d440c56715bd5";
        #[cfg(feature = "blake2bhash")]
        let expected = "01d418c29a2942a1257a3be24134e125d9ef52ca2c0e9174969cd86fb9bf74e9";
        #[cfg(feature = "sm3hash")]
        let expected = "293a5287836ae79f19f88d45b5ac5b2c2fd2f60b7c4250ff595cb228582f49c5";

        a.commit_storage(&Default::default(), &mut db).unwrap();
        assert_eq!(a.storage_root().unwrap().lower_hex(), expected);
    }

    #[test]
    fn commit_remove_commit_storage() {
        let mut a = Account::new_contract(0.into(), 0.into());
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        a.set_storage(0.into(), 0x1234.into());
        a.commit_storage(&Default::default(), &mut db).unwrap();
        a.set_storage(1.into(), 0x1234.into());
        a.commit_storage(&Default::default(), &mut db).unwrap();
        a.set_storage(1.into(), 0.into());

        #[cfg(feature = "sha3hash")]
        let expected = "71623f5ec821de33ad5aa81f8c82f0916c6f60de0a536f8c466d440c56715bd5";
        #[cfg(feature = "blake2bhash")]
        let expected = "01d418c29a2942a1257a3be24134e125d9ef52ca2c0e9174969cd86fb9bf74e9";
        #[cfg(feature = "sm3hash")]
        let expected = "293a5287836ae79f19f88d45b5ac5b2c2fd2f60b7c4250ff595cb228582f49c5";

        a.commit_storage(&Default::default(), &mut db).unwrap();
        assert_eq!(a.storage_root().unwrap().lower_hex(), expected);
    }

    #[test]
    fn commit_code() {
        let mut a = Account::new_contract(0.into(), 0.into());
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        a.init_code(vec![0x55, 0x44, 0xffu8]);
        assert_eq!(a.code_filth, Filth::Dirty);
        assert_eq!(a.code_size(), Some(3));
        a.commit_code(&mut db);

        #[cfg(feature = "sha3hash")]
        let expected = "af231e631776a517ca23125370d542873eca1fb4d613ed9b5d5335a46ae5b7eb";
        #[cfg(feature = "blake2bhash")]
        let expected = "d9c3b9ce5f61497874544e3c8a111295256705ed0c32730db01ed36a1cef9845";
        #[cfg(feature = "sm3hash")]
        let expected = "a812a243d71509028ca423a5f1d74514468898389d18c4342e1f2d8683b21d73";

        assert_eq!(a.code_hash().lower_hex(), expected);
    }

    #[test]
    fn commit_abi() {
        let mut a = Account::new_contract(0.into(), 0.into());
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        a.init_abi(vec![0x55, 0x44, 0xffu8]);
        assert_eq!(a.abi_filth, Filth::Dirty);
        assert_eq!(a.abi_size(), Some(3));
        a.commit_abi(&mut db);

        #[cfg(feature = "sha3hash")]
        let expected = "af231e631776a517ca23125370d542873eca1fb4d613ed9b5d5335a46ae5b7eb";
        #[cfg(feature = "blake2bhash")]
        let expected = "d9c3b9ce5f61497874544e3c8a111295256705ed0c32730db01ed36a1cef9845";
        #[cfg(feature = "sm3hash")]
        let expected = "a812a243d71509028ca423a5f1d74514468898389d18c4342e1f2d8683b21d73";

        assert_eq!(a.abi_hash().lower_hex(), expected);
    }

    #[test]
    fn reset_code() {
        let mut a = Account::new_contract(0.into(), 0.into());
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        a.init_code(vec![0x55, 0x44, 0xffu8]);
        assert_eq!(a.code_filth, Filth::Dirty);
        a.commit_code(&mut db);
        assert_eq!(a.code_filth, Filth::Clean);

        #[cfg(feature = "sha3hash")]
        let expected = "af231e631776a517ca23125370d542873eca1fb4d613ed9b5d5335a46ae5b7eb";
        #[cfg(feature = "blake2bhash")]
        let expected = "d9c3b9ce5f61497874544e3c8a111295256705ed0c32730db01ed36a1cef9845";
        #[cfg(feature = "sm3hash")]
        let expected = "a812a243d71509028ca423a5f1d74514468898389d18c4342e1f2d8683b21d73";

        assert_eq!(a.code_hash().lower_hex(), expected);

        a.reset_code(vec![0x55]);
        assert_eq!(a.code_filth, Filth::Dirty);
        a.commit_code(&mut db);

        #[cfg(feature = "sha3hash")]
        let expected = "37bf2238b11b68cdc8382cece82651b59d3c3988873b6e0f33d79694aa45f1be";
        #[cfg(feature = "blake2bhash")]
        let expected = "32df85a4ebfe3725d6e19352057c4755aa0f2a4c01ba0c94c18dd5813ce43a01";
        #[cfg(feature = "sm3hash")]
        let expected = "657a94d4f15695633005ec909ac5655111599bcd8278b75d183b86feeb778bff";

        assert_eq!(a.code_hash().lower_hex(), expected);
    }

    #[test]
    fn reset_abi() {
        let mut a = Account::new_contract(0.into(), 0.into());
        let mut db = MemoryDB::new();
        let mut db = AccountDBMut::new(&mut db, &Address::new());
        a.init_abi(vec![0x55, 0x44, 0xffu8]);
        assert_eq!(a.abi_filth, Filth::Dirty);
        a.commit_abi(&mut db);
        assert_eq!(a.abi_filth, Filth::Clean);

        #[cfg(feature = "sha3hash")]
        let expected = "af231e631776a517ca23125370d542873eca1fb4d613ed9b5d5335a46ae5b7eb";
        #[cfg(feature = "blake2bhash")]
        let expected = "d9c3b9ce5f61497874544e3c8a111295256705ed0c32730db01ed36a1cef9845";
        #[cfg(feature = "sm3hash")]
        let expected = "a812a243d71509028ca423a5f1d74514468898389d18c4342e1f2d8683b21d73";

        assert_eq!(a.abi_hash().lower_hex(), expected);

        a.reset_abi(vec![0x55]);
        assert_eq!(a.abi_filth, Filth::Dirty);
        a.commit_abi(&mut db);

        #[cfg(feature = "sha3hash")]
        let expected = "37bf2238b11b68cdc8382cece82651b59d3c3988873b6e0f33d79694aa45f1be";
        #[cfg(feature = "blake2bhash")]
        let expected = "32df85a4ebfe3725d6e19352057c4755aa0f2a4c01ba0c94c18dd5813ce43a01";
        #[cfg(feature = "sm3hash")]
        let expected = "657a94d4f15695633005ec909ac5655111599bcd8278b75d183b86feeb778bff";

        assert_eq!(a.abi_hash().lower_hex(), expected);
    }

    #[test]
    fn rlpio() {
        let a = Account::new(
            U256::from(0u8),
            U256::from(0u8),
            HashMap::new(),
            Bytes::new(),
            Bytes::new(),
        );
        let b = Account::from_rlp(&a.rlp());
        assert_eq!(a.nonce(), b.nonce());
        assert_eq!(a.code_hash(), b.code_hash());
        assert_eq!(a.abi_hash(), b.abi_hash());
        assert_eq!(a.storage_root(), b.storage_root());
    }

    #[test]
    fn new_account() {
        let a = Account::new(
            U256::from(0u8),
            U256::from(0u8),
            HashMap::new(),
            Bytes::new(),
            Bytes::new(),
        );

        #[cfg(feature = "sha3hash")]
        let expected = "f8658080a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622f\
                        b5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfa\
                        d8045d85a470a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7b\
                        fad8045d85a470";
        #[cfg(feature = "blake2bhash")]
        let expected = "f8658080a0c14af59107ef14003e4697a40ea912d865eb1463086a4649977c13\
                        ea69b0d9afa0d67f729f8d19ed2e92f817cf5c31c7812dd39ed35b0b1aae41c7\
                        665f46c36b9fa0d67f729f8d19ed2e92f817cf5c31c7812dd39ed35b0b1aae41\
                        c7665f46c36b9f";
        #[cfg(feature = "sm3hash")]
        let expected = "f8658080a0995b949869f80fa1465a9d8b6fa759ec65c3020d59c2624662bdff\
                        059bdf19b3a01ab21d8355cfa17f8e61194831e81a8f22bec8c728fefb747ed0\
                        35eb5082aa2ba01ab21d8355cfa17f8e61194831e81a8f22bec8c728fefb747e\
                        d035eb5082aa2b";

        assert_eq!(a.rlp().to_hex(), expected);

        assert_eq!(a.nonce(), &U256::from(0u8));
        assert_eq!(a.code_hash(), HASH_EMPTY);
        assert_eq!(a.abi_hash(), HASH_EMPTY);
        assert_eq!(a.storage_root().unwrap(), &HASH_NULL_RLP);
    }

    #[test]
    fn create_account() {
        let a = Account::new(
            U256::from(0u8),
            U256::from(0u8),
            HashMap::new(),
            Bytes::new(),
            Bytes::new(),
        );

        #[cfg(feature = "sha3hash")]
        let expected = "f8658080a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622f\
                        b5e363b421a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfa\
                        d8045d85a470a0c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7b\
                        fad8045d85a470";
        #[cfg(feature = "blake2bhash")]
        let expected = "f8658080a0c14af59107ef14003e4697a40ea912d865eb1463086a4649977c13\
                        ea69b0d9afa0d67f729f8d19ed2e92f817cf5c31c7812dd39ed35b0b1aae41c7\
                        665f46c36b9fa0d67f729f8d19ed2e92f817cf5c31c7812dd39ed35b0b1aae41\
                        c7665f46c36b9f";
        #[cfg(feature = "sm3hash")]
        let expected = "f8658080a0995b949869f80fa1465a9d8b6fa759ec65c3020d59c2624662bdff\
                        059bdf19b3a01ab21d8355cfa17f8e61194831e81a8f22bec8c728fefb747ed0\
                        35eb5082aa2ba01ab21d8355cfa17f8e61194831e81a8f22bec8c728fefb747e\
                        d035eb5082aa2b";

        assert_eq!(a.rlp().to_hex(), expected);
    }

}
