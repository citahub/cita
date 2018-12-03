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
// Remove some hf code.

use byteorder::{ByteOrder, LittleEndian};
use cita_types::{Address, H256};
use db::COL_ACCOUNT_BLOOM;
use ethcore_bloom_journal::*;
use header::BlockNumber;
use lru_cache::LruCache;
use state::backend::*;
use state::Account;
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use util::cache::MemoryLruCache;
use util::{DBTransaction, HashDB, JournalDB, KeyValueDB, Mutex, UtilError};

/// Value used to initialize bloom bitmap size.
///
/// Bitmap size is the size in bytes (not bits) that will be allocated in memory.
pub const ACCOUNT_BLOOM_SPACE: usize = 1_048_576;

/// Value used to initialize bloom items count.
///
/// Items count is an estimation of the maximum number of items to store.
pub const DEFAULT_ACCOUNT_PRESET: usize = 1_000_000;

/// Key for a value storing amount of hashes
pub const ACCOUNT_BLOOM_HASHCOUNT_KEY: &[u8] = b"account_hash_count";

const STATE_CACHE_BLOCKS: usize = 12;

// The percentage of supplied cache size to go to accounts.
const ACCOUNT_CACHE_RATIO: usize = 90;

/// Shared canonical state cache.
struct AccountCache {
    /// DB Account cache. `None` indicates that account is known to be missing.
    // When changing the type of the values here, be sure to update `mem_used` and
    // `new`.
    accounts: LruCache<Address, Option<Account>>,
    /// Information on the modifications in recently committed blocks; specifically which addresses
    /// changed in which block. Ordered by block number.
    modifications: VecDeque<BlockChanges>,
}

/// Buffered account cache item.
struct CacheQueueItem {
    /// Account address.
    address: Address,
    /// Acccount data or `None` if account does not exist.
    account: SyncAccount,
    /// Indicates that the account was modified before being
    /// added to the cache.
    modified: bool,
}

#[derive(Debug)]
/// Accumulates a list of accounts changed in a block.
struct BlockChanges {
    /// Block number.
    number: BlockNumber,
    /// Block hash.
    hash: H256,
    /// Parent block hash.
    parent: H256,
    /// A set of modified account addresses.
    accounts: HashSet<Address>,
    /// Block is part of the canonical chain.
    is_canon: bool,
}

pub struct StateDB {
    /// Backing database.
    db: Box<JournalDB>,
    /// Shared canonical state cache.
    account_cache: Arc<Mutex<AccountCache>>,
    /// DB Code cache. Maps code hashes to shared bytes.
    code_cache: Arc<Mutex<MemoryLruCache<H256, Arc<Vec<u8>>>>>,
    /// Local dirty account cache.
    local_account_cache: Vec<CacheQueueItem>,
    /// Shared account bloom. Does not handle chain reorganizations.
    account_bloom: Arc<Mutex<Bloom>>,
    cache_size: usize,
    /// Hash of the block on top of which this instance was created or
    /// `None` if cache is disabled
    pub parent_hash: Option<H256>,
    /// Hash of the committing block or `None` if not committed yet.
    commit_hash: Option<H256>,
    /// Number of the committing block or `None` if not committed yet.
    commit_number: Option<BlockNumber>,
}

impl StateDB {
    pub fn new(db: Box<JournalDB>, cache_size: usize) -> StateDB {
        let bloom = Self::load_bloom(&**db.backing());
        let acc_cache_size = cache_size * ACCOUNT_CACHE_RATIO / 100;
        let code_cache_size = cache_size - acc_cache_size;
        let cache_items = acc_cache_size / ::std::mem::size_of::<Option<Account>>();

        StateDB {
            db,
            account_cache: Arc::new(Mutex::new(AccountCache {
                accounts: LruCache::new(cache_items),
                modifications: VecDeque::new(),
            })),
            code_cache: Arc::new(Mutex::new(MemoryLruCache::new(code_cache_size))),
            local_account_cache: Vec::new(),
            account_bloom: Arc::new(Mutex::new(bloom)),
            cache_size,
            parent_hash: None,
            commit_hash: None,
            commit_number: None,
        }
    }

    /// Loads accounts bloom from the database
    /// This bloom is used to handle request for the non-existant account fast
    pub fn load_bloom(db: &KeyValueDB) -> Bloom {
        let hash_count_entry = db
            .get(COL_ACCOUNT_BLOOM, ACCOUNT_BLOOM_HASHCOUNT_KEY)
            .expect("Low-level database error");

        let hash_count_bytes = match hash_count_entry {
            Some(bytes) => bytes,
            None => return Bloom::new(ACCOUNT_BLOOM_SPACE, DEFAULT_ACCOUNT_PRESET),
        };

        assert_eq!(hash_count_bytes.len(), 1);
        let hash_count = hash_count_bytes[0];

        let mut bloom_parts = vec![0u64; ACCOUNT_BLOOM_SPACE / 8];
        let mut key = [0u8; 8];
        for (i, part) in bloom_parts
            .iter_mut()
            .enumerate()
            .take(ACCOUNT_BLOOM_SPACE / 8)
        {
            LittleEndian::write_u64(&mut key, i as u64);
            *part = db
                .get(COL_ACCOUNT_BLOOM, &key)
                .expect("low-level database error")
                .and_then(|val| Some(LittleEndian::read_u64(&val[..])))
                .unwrap_or(0u64);
        }

        let bloom = Bloom::from_parts(&bloom_parts, u32::from(hash_count));
        trace!(target: "account_bloom", "Bloom is {:?} full, hash functions count = {:?}",
               bloom.saturation(), hash_count);
        bloom
    }

    /// Commit blooms journal to the database transaction
    pub fn commit_bloom(batch: &mut DBTransaction, journal: BloomJournal) -> Result<(), UtilError> {
        assert!(journal.hash_functions <= 255);
        batch.put(
            COL_ACCOUNT_BLOOM,
            ACCOUNT_BLOOM_HASHCOUNT_KEY,
            &[journal.hash_functions as u8],
        );
        let mut key = [0u8; 8];
        let mut val = [0u8; 8];

        for (bloom_part_index, bloom_part_value) in journal.entries {
            LittleEndian::write_u64(&mut key, bloom_part_index as u64);
            LittleEndian::write_u64(&mut val, bloom_part_value);
            batch.put(COL_ACCOUNT_BLOOM, &key, &val);
        }
        Ok(())
    }

    /// Journal all recent operations under the given era and ID.
    pub fn journal_under(
        &mut self,
        batch: &mut DBTransaction,
        now: u64,
        id: &H256,
    ) -> Result<u32, UtilError> {
        {
            let mut bloom_lock = self.account_bloom.lock();
            Self::commit_bloom(batch, bloom_lock.drain_journal())?;
        }
        let records = self.db.journal_under(batch, now, id)?;
        self.commit_hash = Some(*id);
        self.commit_number = Some(now);
        Ok(records)
    }

    /// Mark a given candidate from an ancient era as canonical, enacting its removals from the
    /// backing database and reverting any non-canonical historical commit's insertions.
    pub fn mark_canonical(
        &mut self,
        batch: &mut DBTransaction,
        now: u64,
        id: &H256,
    ) -> Result<u32, UtilError> {
        self.db.mark_canonical(batch, now, id)
    }

    /// Clone the database for a canonical state.
    pub fn boxed_clone_canon(&self, parent: &H256) -> StateDB {
        StateDB {
            db: self.db.boxed_clone(),
            account_cache: self.account_cache.clone(),
            code_cache: self.code_cache.clone(),
            local_account_cache: Vec::new(),
            account_bloom: self.account_bloom.clone(),
            cache_size: self.cache_size,
            parent_hash: Some(*parent),
            commit_hash: None,
            commit_number: None,
        }
    }

    /// Heap size used.
    pub fn mem_used(&self) -> usize {
        // TODO: account for LRU-cache overhead; this is a close approximation.
        self.db.mem_used() + {
            let accounts = self.account_cache.lock().accounts.len();
            let code_size = self.code_cache.lock().current_size();
            code_size + accounts * ::std::mem::size_of::<Option<Account>>()
        }
    }

    /// Returns underlying `JournalDB`.
    pub fn journal_db(&self) -> &JournalDB {
        &*self.db
    }

    /// Query how much memory is set aside for the accounts cache (in bytes).
    pub fn cache_size(&self) -> usize {
        self.cache_size
    }

    /// Check if the account can be returned from cache by matching current block parent hash against canonical
    /// state and filtering out account modified in later blocks.
    fn is_allowed(
        addr: &Address,
        parent_hash: &Option<H256>,
        modifications: &VecDeque<BlockChanges>,
    ) -> bool {
        let mut parent = match *parent_hash {
            None => {
                trace!("Cache lookup skipped for {:?}: no parent hash", addr);
                return false;
            }
            Some(ref parent) => parent,
        };
        if modifications.is_empty() {
            return true;
        }
        // Ignore all accounts modified in later blocks
        // Modifications contains block ordered by the number
        // We search for our parent in that list first and then for
        // all its parent until we hit the canonical block,
        // checking against all the intermediate modifications.
        for m in modifications {
            if &m.hash == parent {
                if m.is_canon {
                    return true;
                }
                parent = &m.parent;
            }
            if m.accounts.contains(addr) {
                trace!(
                    "Cache lookup skipped for {:?}: modified in a later block",
                    addr
                );
                return false;
            }
        }
        trace!(
            "Cache lookup skipped for {:?}: parent hash is unknown",
            addr
        );
        false
    }
}

impl Backend for StateDB {
    fn as_hashdb(&self) -> &HashDB {
        self.db.as_hashdb()
    }

    fn as_hashdb_mut(&mut self) -> &mut HashDB {
        self.db.as_hashdb_mut()
    }

    fn add_to_account_cache(&mut self, address: Address, data: Option<Account>, modified: bool) {
        self.local_account_cache.push(CacheQueueItem {
            address,
            account: SyncAccount(data),
            modified,
        })
    }

    fn cache_code(&self, hash: H256, code: Arc<Vec<u8>>) {
        let mut cache = self.code_cache.lock();

        cache.insert(hash, code);
    }

    fn get_cached_account(&self, addr: &Address) -> Option<Account> {
        let mut cache = self.account_cache.lock();
        if !Self::is_allowed(addr, &self.parent_hash, &cache.modifications) {
            return None;
        }
        cache
            .accounts
            .get_mut(addr)
            .and_then(|a| a.as_ref().map(|a| a.clone_basic()))
    }

    fn get_cached<F, U>(&self, a: &Address, f: F) -> Option<U>
    where
        F: FnOnce(Option<&mut Account>) -> U,
    {
        let mut cache = self.account_cache.lock();
        if !Self::is_allowed(a, &self.parent_hash, &cache.modifications) {
            return None;
        }
        cache.accounts.get_mut(a).map(|c| f(c.as_mut()))
    }

    fn get_cached_code(&self, hash: &H256) -> Option<Arc<Vec<u8>>> {
        let mut cache = self.code_cache.lock();

        cache.get_mut(hash).cloned()
    }

    fn note_non_null_account(&self, address: &Address) {
        trace!(target: "account_bloom", "Note account bloom: {:?}", address);
        let mut bloom = self.account_bloom.lock();
        bloom.set(address);
    }

    fn is_known_null(&self, address: &Address) -> bool {
        trace!(target: "account_bloom", "Check account bloom: {:?}", address);
        !self.account_bloom.lock().check(address)
    }

    /// Propagate local cache into the global cache.
    /// `sync_cache` should be called after the block has been committed.
    fn sync_account_cache(&mut self) {
        trace!(
            "sync_cache id = (#{:?}, {:?}), parent={:?}",
            self.commit_number,
            self.commit_hash,
            self.parent_hash,
        );
        let mut cache = self.account_cache.lock();
        let cache = &mut *cache;

        // Propagate cache only if committing on top of the latest canonical state
        // blocks are ordered by number and only one block with a given number is marked as canonical
        // (contributed to canonical state cache)
        if let (Some(ref number), Some(ref hash), Some(ref parent)) =
            (self.commit_number, self.commit_hash, self.parent_hash)
        {
            if cache.modifications.len() == STATE_CACHE_BLOCKS {
                cache.modifications.pop_back();
            }
            let mut modifications = HashSet::new();
            trace!(
                "committing {} cache entries",
                self.local_account_cache.len()
            );
            for account in self.local_account_cache.drain(..) {
                if account.modified {
                    modifications.insert(account.address);
                }

                let acc = account.account.0;
                if let Some(&mut Some(ref mut existing)) = cache.accounts.get_mut(&account.address)
                {
                    if let Some(new) = acc {
                        if account.modified {
                            existing.overwrite_with(new);
                        }
                        continue;
                    }
                }
                cache.accounts.insert(account.address, acc);
            }

            // Save modified accounts. These are ordered by the block number.
            let block_changes = BlockChanges {
                accounts: modifications,
                number: *number,
                hash: *hash,
                is_canon: true,
                parent: *parent,
            };
            let insert_at = cache
                .modifications
                .iter()
                .enumerate()
                .find(|&(_, m)| m.number < *number)
                .map(|(i, _)| i);
            trace!("inserting modifications at {:?}", insert_at);
            if let Some(insert_at) = insert_at {
                cache.modifications.insert(insert_at, block_changes);
            } else {
                cache.modifications.push_back(block_changes);
            }
        }
    }
}

/// Sync wrapper for the account.
struct SyncAccount(Option<Account>);
/// That implementation is safe because account is never modified or accessed in any way.
/// We only need `Sync` here to allow `StateDb` to be kept in a `RwLock`.
/// `Account` is `!Sync` by default because of `RefCell`s inside it.
unsafe impl Sync for SyncAccount {}
