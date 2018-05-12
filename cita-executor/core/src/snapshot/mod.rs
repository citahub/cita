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

//! Snapshot format and creation.

extern crate num_cpus;

// chunks around 4MB before compression
const PREFERRED_CHUNK_SIZE: usize = 4 * 1024 * 1024;

use account_db::{AccountDB, AccountDBMut};
use cita_types::{Address, H256, U256};
use db;
use libexecutor::executor::Executor;
use rlp::{DecoderError, RlpStream, UntrustedRlp};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;
use util::{snappy, Bytes, HashDB};
use util::{Trie, TrieDB, TrieDBMut, TrieMut, HASH_EMPTY};
use util::{Mutex, sha3};
use util::HASH_NULL_RLP;
use util::hashdb::DBValue;
use util::journaldb::{self, Algorithm};
use util::journaldb::JournalDB;
use util::kvdb::Database;

pub mod service;
pub mod io;
pub mod account;
mod error;
use self::error::Error;
use self::io::SnapshotReader;
use self::io::SnapshotWriter;
use self::service::Service;
use snapshot::service::SnapshotService;
pub use types::basic_account::BasicAccount as Account;
use types::ids::BlockId;

use super::state::Account as StateAccount;

//#[cfg(test)]
//mod tests;

/// A progress indicator for snapshots.
#[derive(Debug, Default)]
pub struct Progress {
    accounts: AtomicUsize,
    size: AtomicUsize,
    done: AtomicBool,
}

impl Progress {
    /// Get the number of accounts snapshotted thus far.
    pub fn accounts(&self) -> usize {
        self.accounts.load(Ordering::Relaxed)
    }

    /// Get the written size of the snapshot in bytes.
    pub fn size(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Whether the snapshot is complete.
    pub fn done(&self) -> bool {
        self.done.load(Ordering::SeqCst)
    }
}

/// Statuses for restorations.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RestorationStatus {
    ///	No restoration.
    Inactive,
    /// Ongoing restoration.
    Ongoing {
        /// Total number of state chunks.
        state_chunks: u32,
        /// Number of state chunks completed.
        state_chunks_done: u32,
    },
    /// Failed restoration.
    Failed,
}

/// Snapshot manifest type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestData {
    /// List of state chunk hashes.
    pub state_hashes: Vec<H256>,
    /// The final, expected state root.
    pub state_root: H256,
    // Block number this snapshot was taken at.
    pub block_number: u64,
    // Block hash this snapshot was taken at.
    pub block_hash: H256,
}

/// Snapshot manifest encode/decode.
impl ManifestData {
    /// Encode the manifest data to rlp.
    pub fn to_rlp(self) -> Bytes {
        let mut stream = RlpStream::new_list(4);
        stream.append_list(&self.state_hashes);
        stream.append(&self.state_root);
        stream.append(&self.block_number);
        stream.append(&self.block_hash);

        stream.out()
    }

    /// Restore manifest data from raw bytes.
    pub fn from_rlp(raw: &[u8]) -> Result<Self, DecoderError> {
        let decoder = UntrustedRlp::new(raw);

        let state_hashes: Vec<H256> = decoder.list_at(0)?;
        let state_root: H256 = decoder.val_at(1)?;
        let block_number: u64 = decoder.val_at(2)?;
        let block_hash: H256 = decoder.val_at(3)?;

        Ok(ManifestData {
            state_hashes: state_hashes,
            state_root: state_root,
            block_number: block_number,
            block_hash: block_hash,
        })
    }
}

/// snapshot using: given Executor+ starting block hash + database; writing into the given writer.
pub fn take_snapshot<W: SnapshotWriter + Send>(
    executor: &Executor,
    block_at: H256,
    state_db: &HashDB,
    writer: W,
    p: &Progress,
) -> Result<(), Error> {
    let start_header = executor
        .block_header_by_hash(block_at)
        .ok_or(Error::InvalidStartingBlock(BlockId::Hash(block_at)))?;
    let state_root = start_header.state_root();
    let number = start_header.number();

    info!("Taking snapshot starting at block {}", number);

    let writer = Mutex::new(writer);
    let state_hashes = chunk_state(state_db, state_root, &writer, p)?;

    info!("produced {} state chunks.", state_hashes.len());

    let manifest_data = ManifestData {
        state_hashes: state_hashes,
        state_root: *state_root,
        block_number: number,
        block_hash: block_at,
    };

    writer.into_inner().finish(manifest_data)?;

    p.done.store(true, Ordering::SeqCst);

    Ok(())
}

/// State trie chunker.
struct StateChunker<'a> {
    hashes: Vec<H256>,
    rlps: Vec<Bytes>,
    cur_size: usize,
    writer: &'a Mutex<SnapshotWriter + 'a>,
    progress: &'a Progress,
}

impl<'a> StateChunker<'a> {
    // Push a key, value pair to be encoded.
    //
    // If the buffer is greater than the desired chunk size,
    // this will write out the data to disk.
    fn push(&mut self, data: Bytes) -> Result<(), Error> {
        self.cur_size += data.len();
        self.rlps.push(data);
        Ok(())
    }

    // Write out the buffer to disk, pushing the created chunk's hash to
    // the list.
    fn write_chunk(&mut self) -> Result<(), Error> {
        let num_entries = self.rlps.len();
        let mut stream = RlpStream::new_list(num_entries);
        for rlp in self.rlps.drain(..) {
            stream.append_raw(&rlp, 1);
        }

        let raw_data = stream.out();

        let mut compressed_data = Vec::new();
        snappy::compress_to(&raw_data, &mut compressed_data)?;
        let hash = sha3(&compressed_data);

        self.writer
            .lock()
            .write_state_chunk(hash, &compressed_data)?;
        info!(target: "snapshot", "wrote state chunk.compressed size: {}, uncompressed size: {}",
              compressed_data.len(), raw_data.len());

        self.progress
            .accounts
            .fetch_add(num_entries, Ordering::SeqCst);
        self.progress
            .size
            .fetch_add(compressed_data.len(), Ordering::SeqCst);

        self.hashes.push(hash);
        self.cur_size = 0;

        Ok(())
    }
}

/// Walk the given state database starting from the given root,
/// creating chunks and writing them out.
///
/// Returns a list of hashes of chunks created, or any error it may
/// have encountered.
pub fn chunk_state<'a>(
    db: &HashDB,
    root: &H256,
    writer: &Mutex<SnapshotWriter + 'a>,
    progress: &'a Progress,
) -> Result<Vec<H256>, Error> {
    use util::Hashable;
    info!("[chunk_state] start state_root:{:?}", root);
    let account_trie = TrieDB::new(db, &root)?;
    info!("account_trie:{:?}", account_trie);

    let mut chunker = StateChunker {
        hashes: Vec::new(),
        rlps: Vec::new(),
        cur_size: 0,
        writer: writer,
        progress: progress,
    };

    let mut used_code = HashSet::new();

    // account_key here is the address' hash.
    for item in account_trie.iter()? {
        let (account_key, account_data) = item?;
        info!(
            "foreach account_trie, account_key:{:?}, account_data:{:?}",
            account_key, account_data
        );

        //for debug
        let account_state = StateAccount::from_rlp(&*account_data);
        info!("state account data:{:?}", account_state);

        let account = ::rlp::decode(&*account_data);
        let account_key = H256::from_slice(&account_key);
        let account_key_hash = Address::from_slice(&account_key);
        info!(
            "foreach account_trie, decode---account_key_hash:{:?}, account data:{:?}",
            account_key_hash, account
        );

        //let account_db = AccountDB::from_hash(db, account_key);
        let account_db = AccountDB::new(db, &account_key_hash);

        let fat_rlps = account::to_fat_rlps(
            &account_key_hash.crypt_hash(),
            &account,
            &account_db,
            &mut used_code,
            PREFERRED_CHUNK_SIZE - chunker.cur_size,
            PREFERRED_CHUNK_SIZE,
        )?;
        info!("fat_rlps account fat data: {:?}", fat_rlps);
        for (i, fat_rlp) in fat_rlps.into_iter().enumerate() {
            info!("fat_rlp index: {:?}, data: {:?}", i, fat_rlp);
            if i > 0 {
                chunker.write_chunk()?;
            }
            chunker.push(fat_rlp)?;
        }
    }

    info!("chunker left cur_size:{:?}", chunker.cur_size);
    if chunker.cur_size != 0 {
        chunker.write_chunk()?;
    }

    Ok(chunker.hashes)
}

/// Used to rebuild the state trie piece by piece.
pub struct StateRebuilder {
    db: Box<JournalDB>,
    state_root: H256,
    known_code: HashMap<H256, H256>, // code hashes mapped to first account with this code.
    missing_code: HashMap<H256, Vec<H256>>, // maps code hashes to lists of accounts missing that code.
    //bloom: Bloom,
    known_storage_roots: HashMap<H256, H256>, // maps account hashes to last known storage root.
                                              //Only filled for last account per chunk.
}

impl StateRebuilder {
    /// Create a new state rebuilder to write into the given backing DB.
    pub fn new(db: Arc<Database>, pruning: Algorithm) -> Self {
        StateRebuilder {
            db: journaldb::new(db.clone(), pruning, db::COL_STATE),
            state_root: HASH_NULL_RLP,
            known_code: HashMap::new(),
            missing_code: HashMap::new(),
            //bloom: StateDB::load_bloom(&*db),
            known_storage_roots: HashMap::new(),
        }
    }

    /// Feed an uncompressed state chunk into the rebuilder.
    pub fn feed(&mut self, chunk: &[u8], flag: &AtomicBool) -> Result<(), ::error::Error> {
        let rlp = UntrustedRlp::new(chunk);
        let empty_rlp = StateAccount::new_basic(U256::zero(), U256::zero()).rlp();
        let mut pairs = Vec::with_capacity(rlp.item_count()?);

        // initialize the pairs vector with empty values so we have slots to write into.
        pairs.resize(rlp.item_count()?, (H256::new(), Vec::new()));

        let status = rebuild_accounts(
            self.db.as_hashdb_mut(),
            rlp,
            &mut pairs,
            &self.known_code,
            &mut self.known_storage_roots,
            flag,
        )?;

        for (addr_hash, code_hash) in status.missing_code {
            self.missing_code
                .entry(code_hash)
                .or_insert_with(Vec::new)
                .push(addr_hash);
        }

        // patch up all missing code. must be done after collecting all new missing code entries.
        for (code_hash, code, first_with) in status.new_code {
            for addr_hash in self.missing_code
                .remove(&code_hash)
                .unwrap_or_else(Vec::new)
            {
                let mut db = AccountDBMut::from_hash(self.db.as_hashdb_mut(), addr_hash);
                db.emplace(code_hash, DBValue::from_slice(&code));
            }

            self.known_code.insert(code_hash, first_with);
        }

        let backing = self.db.backing().clone();

        // batch trie writes
        {
            let mut account_trie = if self.state_root != HASH_NULL_RLP {
                TrieDBMut::from_existing(self.db.as_hashdb_mut(), &mut self.state_root)?
            } else {
                TrieDBMut::new(self.db.as_hashdb_mut(), &mut self.state_root)
            };

            for (hash, thin_rlp) in pairs {
                //if !flag.load(Ordering::SeqCst) { return Err(  }

                if &thin_rlp[..] != &empty_rlp[..] {
                    //self.bloom.set(&*hash);
                }
                account_trie.insert(&hash, &thin_rlp)?;
            }
        }

        //let bloom_journal = self.bloom.drain_journal();
        let mut batch = backing.transaction();
        //StateDB::commit_bloom(&mut batch, bloom_journal)?;
        self.db.inject(&mut batch)?;
        backing.write_buffered(batch);
        trace!(target: "snapshot", "current state root: {:?}", self.state_root);
        Ok(())
    }

    /// Get the state root of the rebuilder.
    pub fn state_root(&self) -> H256 {
        self.state_root
    }
}

#[derive(Default)]
struct RebuiltStatus {
    // new code that's become available. (code_hash, code, addr_hash)
    new_code: Vec<(H256, Bytes, H256)>,
    missing_code: Vec<(H256, H256)>, // accounts that are missing code.
}

// rebuild a set of accounts and their storage.
// returns a status detailing newly-loaded code and accounts missing code.
fn rebuild_accounts(
    db: &mut HashDB,
    account_fat_rlps: UntrustedRlp,
    out_chunk: &mut [(H256, Bytes)],
    known_code: &HashMap<H256, H256>,
    known_storage_roots: &mut HashMap<H256, H256>,
    _abort_flag: &AtomicBool,
) -> Result<RebuiltStatus, ::error::Error> {
    let mut status = RebuiltStatus::default();
    for (account_rlp, out) in account_fat_rlps.into_iter().zip(out_chunk.iter_mut()) {
        //if !abort_flag.load(Ordering::SeqCst) { return Err(Error::RestorationAborted.into()) }

        let hash: H256 = account_rlp.val_at(0)?;
        let fat_rlp = account_rlp.at(1)?;

        let thin_rlp = {
            // fill out the storage trie and code while decoding.
            let (acc, maybe_code) = {
                let mut acct_db = AccountDBMut::from_hash(db, hash);
                let storage_root = known_storage_roots
                    .get(&hash)
                    .cloned()
                    .unwrap_or(H256::zero());
                account::from_fat_rlp(&mut acct_db, fat_rlp, storage_root).unwrap()
            };

            let code_hash = acc.code_hash.clone();
            match maybe_code {
                // new inline code
                Some(code) => status.new_code.push((code_hash, code, hash)),
                None => {
                    if code_hash != HASH_EMPTY {
                        // see if this code has already been included inline
                        match known_code.get(&code_hash) {
                            Some(&first_with) => {
                                // if so, load it from the database.
                                let code = AccountDB::from_hash(db, first_with)
                                    .get(&code_hash)
                                    .ok_or_else(|| Error::MissingCode(vec![first_with]))
                                    .unwrap();

                                // and write it again under a different mangled key
                                AccountDBMut::from_hash(db, hash).emplace(code_hash, code);
                            }
                            // if not, queue it up to be filled later
                            None => status.missing_code.push((hash, code_hash)),
                        }
                    }
                }
            }

            ::rlp::encode(&acc).into_vec()
        };

        *out = (hash, thin_rlp);
    }
    if let Some(&(ref hash, ref rlp)) = out_chunk.iter().last() {
        known_storage_roots.insert(*hash, ::rlp::decode::<Account>(rlp).storage_root);
    }
    if let Some(&(ref hash, ref rlp)) = out_chunk.iter().next() {
        known_storage_roots.insert(*hash, ::rlp::decode::<Account>(rlp).storage_root);
    }
    Ok(status)
}

// helper for reading chunks from arbitrary reader and feeding them into the
// service.
pub fn restore_using<R: SnapshotReader>(snapshot: Arc<Service>, reader: &R, recover: bool) -> Result<(), String> {
    let manifest = reader.manifest();

    info!(
        "Restoring to block #{} (0x{:?})",
        manifest.block_number, manifest.block_hash
    );

    snapshot
        .init_restore(manifest.clone(), recover)
        .map_err(|e| format!("Failed to begin restoration: {}", e))?;

    let num_state = manifest.state_hashes.len();

    let informant_handle = snapshot.clone();
    ::std::thread::spawn(move || {
        while let RestorationStatus::Ongoing {
            state_chunks_done, ..
        } = informant_handle.status()
        {
            info!(
                "Processed {}/{} state chunks.",
                state_chunks_done, num_state
            );
            ::std::thread::sleep(Duration::from_secs(5));
        }
    });

    info!("Restoring state");
    for &state_hash in &manifest.state_hashes {
        if snapshot.status() == RestorationStatus::Failed {
            return Err("Restoration failed".into());
        }

        let chunk = reader.chunk(state_hash).map_err(|e| {
            format!(
                "Encountered error while reading chunk {:?}: {}",
                state_hash, e
            )
        })?;

        let hash = sha3(&chunk);
        if hash != state_hash {
            return Err(format!(
                "Mismatched chunk hash. Expected {:?}, got {:?}",
                state_hash, hash
            ));
        }

        snapshot.feed_state_chunk(state_hash, &chunk);
    }

    match snapshot.status() {
        RestorationStatus::Ongoing { .. } => Err("Snapshot file is incomplete and missing chunks.".into()),
        RestorationStatus::Failed => Err("Snapshot restoration failed.".into()),
        RestorationStatus::Inactive => {
            info!("Restoration complete.");
            Ok(())
        }
    }
}
