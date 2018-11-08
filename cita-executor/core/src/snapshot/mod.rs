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

extern crate ethcore_bloom_journal;
extern crate num_cpus;

// chunks around 4MB before compression
const PREFERRED_CHUNK_SIZE: usize = 4 * 1024 * 1024;

use account_db::{AccountDB, AccountDBMut};

use cita_types::{Address, H256, U256};
use db::{Writable, COL_EXTRA, COL_HEADERS, COL_STATE};
use libexecutor::executor::Executor;
use rlp::{DecoderError, Encodable, RlpStream, UntrustedRlp};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use util::hashdb::DBValue;
use util::journaldb::JournalDB;
use util::journaldb::{self, Algorithm};
use util::kvdb::{DBTransaction, Database, KeyValueDB};
use util::{snappy, Bytes, HashDB, Hashable, Mutex, HASH_NULL_RLP};
use util::{Trie, TrieDB, TrieDBMut, TrieMut, HASH_EMPTY};

pub mod account;
pub mod error;
pub mod io;
pub mod service;
pub use self::error::Error;
use self::io::SnapshotReader;
use self::io::SnapshotWriter;
use self::service::Service;
use snapshot::service::SnapshotService;
pub use types::basic_account::BasicAccount;
use types::ids::BlockId;

use super::state::Account as StateAccount;
use super::state_db::StateDB;
use ethcore_bloom_journal::Bloom;
use header::Header;

use types::extras::CurrentHash;

/// A sink for produced chunks.
pub type ChunkSink<'a> = FnMut(&[u8]) -> Result<(), Error> + 'a;

/// A progress indicator for snapshots.
#[derive(Debug, Default)]
pub struct Progress {
    accounts: AtomicUsize,
    blocks: AtomicUsize,
    size: AtomicUsize,
    done: AtomicBool,
}

impl Progress {
    /// Reset the progress.
    pub fn reset(&self) {
        self.accounts.store(0, Ordering::Release);
        self.blocks.store(0, Ordering::Release);
        self.size.store(0, Ordering::Release);

        // atomic fence here to ensure the others are written first?
        // logs might very rarely get polluted if not.
        self.done.store(false, Ordering::Release);
    }

    /// Get the number of accounts snapshotted thus far.
    pub fn accounts(&self) -> usize {
        self.accounts.load(Ordering::Acquire)
    }

    /// Get the number of blocks snapshotted thus far.
    pub fn blocks(&self) -> usize {
        self.blocks.load(Ordering::Acquire)
    }

    /// Get the written size of the snapshot in bytes.
    pub fn size(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }

    /// Whether the snapshot is complete.
    pub fn done(&self) -> bool {
        self.done.load(Ordering::Acquire)
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
        /// Total number of block chunks.
        block_chunks: u32,
        /// Number of state chunks completed.
        state_chunks_done: u32,
        /// Number of block chunks completed.
        block_chunks_done: u32,
    },
    /// Failed restoration.
    Failed,
}

/// Snapshot manifest type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestData {
    /// List of state chunk hashes.
    pub state_hashes: Vec<H256>,
    /// List of block chunk hashes.
    pub block_hashes: Vec<H256>,
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
    pub fn to_rlp(&self) -> Bytes {
        let mut stream = RlpStream::new_list(5);
        stream.append_list(&self.state_hashes);
        stream.append_list(&self.block_hashes);
        stream.append(&self.state_root);
        stream.append(&self.block_number);
        stream.append(&self.block_hash);

        stream.out()
    }

    /// Restore manifest data from raw bytes.
    pub fn from_rlp(raw: &[u8]) -> Result<Self, DecoderError> {
        let decoder = UntrustedRlp::new(raw);

        let state_hashes: Vec<H256> = decoder.list_at(0)?;
        let block_hashes: Vec<H256> = decoder.list_at(1)?;
        let state_root: H256 = decoder.val_at(2)?;
        let block_number: u64 = decoder.val_at(3)?;
        let block_hash: H256 = decoder.val_at(4)?;

        Ok(ManifestData {
            state_hashes,
            block_hashes,
            state_root,
            block_number,
            block_hash,
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
        .ok_or_else(|| Error::InvalidStartingBlock(BlockId::Hash(block_at)))?;
    let state_root = start_header.state_root();
    let number = start_header.number();

    info!(
        "Taking snapshot starting at block {}, state_root {:?}",
        number, state_root
    );

    let writer = Mutex::new(writer);
    let state_hashes = chunk_state(state_db, state_root, &writer, p)?;
    let block_hashes = chunk_secondary(executor, block_at, &writer, p)?;

    let manifest_data = ManifestData {
        state_hashes,
        block_hashes,
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
        let hash = compressed_data.crypt_hash();

        self.writer
            .lock()
            .write_state_chunk(hash, &compressed_data)?;
        trace!(
            "wrote state chunk.compressed size: {}, uncompressed size: {}",
            compressed_data.len(),
            raw_data.len(),
        );

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
    let account_trie = TrieDB::new(db, &root).map_err(|err| *err)?;

    let mut chunker = StateChunker {
        hashes: Vec::new(),
        rlps: Vec::new(),
        cur_size: 0,
        writer,
        progress,
    };

    let mut used_code = HashSet::new();
    let mut used_abi = HashSet::new();

    // account_key here is the address' hash.
    for item in account_trie.iter().map_err(|err| *err)? {
        let (account_key, account_data) = item.map_err(|err| *err)?;

        let basic_account = ::rlp::decode(&*account_data);
        let account_key = H256::from_slice(&account_key);
        let account_address = Address::from_slice(&account_key);
        trace!("Account: {:?}", account_address);

        let account_db = AccountDB::new(db, &account_address);

        let fat_rlps = account::to_fat_rlps(
            &account_address,
            &basic_account,
            &account_db,
            &mut used_code,
            &mut used_abi,
            PREFERRED_CHUNK_SIZE - chunker.cur_size,
            PREFERRED_CHUNK_SIZE,
        )?;

        for (i, fat_rlp) in fat_rlps.into_iter().enumerate() {
            if i > 0 {
                chunker.write_chunk()?;
            }
            chunker.push(fat_rlp)?;
        }
    }

    if chunker.cur_size != 0 {
        chunker.write_chunk()?;
    }

    Ok(chunker.hashes)
}

/// Used to build block chunks.
struct BlockChunker<'a> {
    executor: &'a Executor,
    // block, receipt rlp pairs.
    rlps: VecDeque<Bytes>,
    current_hash: H256,
    writer: &'a mut ChunkSink<'a>,
    preferred_size: usize,
}

impl<'a> BlockChunker<'a> {
    // Repeatedly fill the buffers and writes out chunks, moving backwards from starting block hash.
    // Loops until we reach the first desired block, and writes out the remainder.
    fn chunk_all(&mut self) -> Result<(), Error> {
        let mut loaded_size = 0;
        let mut last = self.current_hash;

        let start_header = self
            .executor
            .block_header_by_hash(self.current_hash)
            .ok_or_else(|| Error::BlockNotFound(self.current_hash))?;
        let step = if start_header.number() < 100 {
            1
        } else {
            start_header.number() / 100
        };

        let genesis_block = self
            .executor
            .block_header_by_height(0)
            .expect("Get genesis block failed");
        let genesis_hash = genesis_block.hash().unwrap();
        info!("genesis_hash: {:?}", genesis_hash);
        let mut blocks_num = 0;

        loop {
            if self.current_hash == genesis_hash {
                break;
            }

            let header = self
                .executor
                .block_header_by_hash(self.current_hash)
                .ok_or_else(|| Error::BlockNotFound(self.current_hash))?;

            let mut s = RlpStream::new();
            header.rlp_append(&mut s);
            let header_rlp = s.out();

            if blocks_num % step == 0 {
                info!("current height: {:?}", header.number());
            }

            let new_loaded_size = loaded_size + header_rlp.len();

            // cut off the chunk if too large.
            if new_loaded_size > self.preferred_size && !self.rlps.is_empty() {
                self.write_chunk(last)?;
                loaded_size = header_rlp.len();
            } else {
                loaded_size = new_loaded_size;
            }

            self.rlps.push_front(header_rlp);

            last = self.current_hash;
            self.current_hash = *header.parent_hash();

            blocks_num += 1;
        }

        if loaded_size != 0 {
            self.write_chunk(last)?;
        }

        Ok(())
    }

    // write out the data in the buffers to a chunk on disk
    //
    // we preface each chunk with the parent of the first block's details,
    // obtained from the details of the last block written.
    fn write_chunk(&mut self, last: H256) -> Result<(), Error> {
        trace!("prepared block chunk with {} blocks", self.rlps.len());

        let last_header = self
            .executor
            .block_header_by_hash(last)
            .ok_or_else(|| Error::BlockNotFound(last))?;

        let parent_number = last_header.number() - 1;
        let parent_hash = last_header.parent_hash();

        trace!("parent last written block: {}", parent_hash);

        let num_entries = self.rlps.len();
        let mut rlp_stream = RlpStream::new_list(2 + num_entries);
        rlp_stream.append(&parent_number).append(parent_hash);

        for pair in self.rlps.drain(..) {
            rlp_stream.append_raw(&pair, 1);
        }

        let raw_data = rlp_stream.out();

        (self.writer)(&raw_data)?;

        Ok(())
    }
}

/// Returns a list of chunk hashes, with the first having the blocks furthest from the genesis.
pub fn chunk_secondary<'a>(
    executor: &'a Executor,
    start_hash: H256,
    writer: &Mutex<SnapshotWriter + 'a>,
    progress: &'a Progress,
) -> Result<Vec<H256>, Error> {
    let mut chunk_hashes = Vec::new();
    let mut compressed_data = Vec::new();

    {
        let mut chunk_sink = |raw_data: &[u8]| {
            compressed_data.clear();
            snappy::compress_to(raw_data, &mut compressed_data)?;
            let hash = compressed_data.crypt_hash();
            let size = compressed_data.len();

            writer.lock().write_block_chunk(hash, &compressed_data)?;
            trace!(
                "wrote secondary chunk. hash: {:?}, size: {}, uncompressed size: {}",
                hash,
                size,
                raw_data.len(),
            );

            progress.size.fetch_add(size, Ordering::SeqCst);
            chunk_hashes.push(hash);
            Ok(())
        };

        BlockChunker {
            executor,
            rlps: VecDeque::new(),
            current_hash: start_hash,
            writer: &mut chunk_sink,
            preferred_size: PREFERRED_CHUNK_SIZE,
        }
        .chunk_all()?
    }

    Ok(chunk_hashes)
}

/// Used to rebuild the state trie piece by piece.
pub struct StateRebuilder {
    db: Box<JournalDB>,
    state_root: H256,
    known_code: HashMap<H256, Address>, // code hashes mapped to first account with this code.
    missing_code: HashMap<H256, Vec<Address>>, // maps code hashes to lists of accounts missing that code.
    known_abi: HashMap<H256, Address>,         // abi hashes mapped to first account with this abi.
    missing_abi: HashMap<H256, Vec<Address>>, // maps abi hashes to lists of accounts missing that abi.
    bloom: Bloom,
    known_storage_roots: HashMap<Address, H256>, // maps account hashes to last known storage root. Only
                                                 // filled for last account per chunk.
}

impl StateRebuilder {
    /// Create a new state rebuilder to write into the given backing DB.
    pub fn new(db: Arc<Database>, pruning: Algorithm) -> Self {
        let bloom = StateDB::load_bloom(&*db);
        StateRebuilder {
            db: journaldb::new(db, pruning, COL_STATE),
            state_root: HASH_NULL_RLP,
            known_code: HashMap::new(),
            missing_code: HashMap::new(),
            known_abi: HashMap::new(),
            missing_abi: HashMap::new(),
            bloom,
            known_storage_roots: HashMap::new(),
        }
    }

    /// Feed an uncompressed state chunk into the rebuilder.
    pub fn feed(&mut self, chunk: &[u8], flag: &AtomicBool) -> Result<(), ::error::Error> {
        let rlp = UntrustedRlp::new(chunk);
        let empty_rlp = StateAccount::new_basic(U256::zero(), U256::zero()).rlp();
        let mut pairs = Vec::with_capacity(rlp.item_count()?);

        // initialize the pairs vector with empty values so we have slots to write into.
        pairs.resize(rlp.item_count()?, (Address::new(), Vec::new()));

        let status = rebuild_accounts(
            self.db.as_hashdb_mut(),
            &rlp,
            &mut pairs,
            &self.known_code,
            &self.known_abi,
            &mut self.known_storage_roots,
            flag,
        )?;

        for (addr, code_hash) in status.missing_code {
            self.missing_code
                .entry(code_hash)
                .or_insert_with(Vec::new)
                .push(addr);
        }

        // patch up all missing code. must be done after collecting all new missing code entries.
        for (code_hash, code, first_with) in status.new_code {
            for addr in self
                .missing_code
                .remove(&code_hash)
                .unwrap_or_else(Vec::new)
            {
                let mut db = AccountDBMut::new(self.db.as_hashdb_mut(), &addr);
                db.emplace(code_hash, DBValue::from_slice(&code));
            }

            self.known_code.insert(code_hash, first_with);
        }

        for (addr, abi_hash) in status.missing_abi {
            self.missing_abi
                .entry(abi_hash)
                .or_insert_with(Vec::new)
                .push(addr);
        }

        // patch up all missing code. must be done after collecting all new missing code entries.
        for (abi_hash, abi, first_with) in status.new_abi {
            for addr in self.missing_abi.remove(&abi_hash).unwrap_or_else(Vec::new) {
                let mut db = AccountDBMut::new(self.db.as_hashdb_mut(), &addr);
                db.emplace(abi_hash, DBValue::from_slice(&abi));
            }

            self.known_abi.insert(abi_hash, first_with);
        }

        let backing = self.db.backing().clone();

        // batch trie writes
        {
            let mut account_trie =
                TrieDBMut::from_existing(self.db.as_hashdb_mut(), &mut self.state_root)
                    .map_err(|err| *err)?;

            for (addr, thin_rlp) in pairs {
                if !flag.load(Ordering::SeqCst) {
                    return Err(Error::RestorationAborted.into());
                }

                if thin_rlp[..] != empty_rlp[..] {
                    self.bloom.set(addr);
                }

                account_trie.insert(&addr, &thin_rlp).map_err(|err| *err)?;
            }
        }

        let bloom_journal = self.bloom.drain_journal();
        let mut batch = backing.transaction();
        StateDB::commit_bloom(&mut batch, bloom_journal)?;
        self.db.inject(&mut batch)?;
        backing.write_buffered(batch);

        trace!("current state root: {:?}", self.state_root);

        Ok(())
    }

    /// Finalize the restoration. Check for accounts missing code and make a dummy
    /// journal entry.
    /// Once all chunks have been fed, there should be nothing missing.
    pub fn finalize(mut self, era: u64, id: H256) -> Result<Box<JournalDB>, ::error::Error> {
        /*let missing = self.missing_code.values().cloned().collect::<Vec<_>>();
        if !missing.is_empty() { return Err(Error::MissingCode(missing).into()) }
        
        let missing = self.missing_abi.values().cloned().collect::<Vec<_>>();
        if !missing.is_empty() { return Err(Error::MissingAbi(missing).into()) }*/

        let mut batch = self.db.backing().transaction();
        self.db.journal_under(&mut batch, era, &id)?;
        self.db.backing().write_buffered(batch);

        Ok(self.db)
    }

    /// Get the state root of the rebuilder.
    pub fn state_root(&self) -> H256 {
        self.state_root
    }
}

#[derive(Default)]
struct RebuiltStatus {
    // new code that's become available. (code_hash, code, addr_hash)
    new_code: Vec<(H256, Bytes, Address)>,
    missing_code: Vec<(Address, H256)>, // accounts that are missing code.
    new_abi: Vec<(H256, Bytes, Address)>,
    missing_abi: Vec<(Address, H256)>, // accounts that are missing abi.
}

// rebuild a set of accounts and their storage.
// returns a status detailing newly-loaded code, accounts missing code,
// newly-loaded abi and accounts missing abi.
fn rebuild_accounts(
    db: &mut HashDB,
    account_fat_rlps: &UntrustedRlp,
    out_chunk: &mut [(Address, Bytes)],
    known_code: &HashMap<H256, Address>,
    known_abi: &HashMap<H256, Address>,
    known_storage_roots: &mut HashMap<Address, H256>,
    abort_flag: &AtomicBool,
) -> Result<RebuiltStatus, ::error::Error> {
    let mut status = RebuiltStatus::default();
    for (account_rlp, out) in account_fat_rlps.into_iter().zip(out_chunk.iter_mut()) {
        if !abort_flag.load(Ordering::SeqCst) {
            return Err(Error::RestorationAborted.into());
        }

        let address: Address = account_rlp.val_at(0)?;
        let fat_rlp = account_rlp.at(1)?;

        let thin_rlp = {
            // fill out the storage trie and code while decoding.
            let (acc, maybe_code, maybe_abi) = {
                let mut acct_db = AccountDBMut::new(db, &address);
                let storage_root = known_storage_roots
                    .get(&address)
                    .cloned()
                    .unwrap_or_else(H256::zero);
                account::from_fat_rlp(&mut acct_db, &fat_rlp, storage_root).unwrap()
            };

            let code_hash = acc.code_hash;
            match maybe_code {
                // new inline code
                Some(code) => status.new_code.push((code_hash, code, address)),
                None => {
                    if code_hash != HASH_EMPTY {
                        // see if this code has already been included inline
                        match known_code.get(&code_hash) {
                            Some(&first_with) => {
                                // if so, load it from the database.
                                let code = AccountDB::new(db, &first_with)
                                    .get(&code_hash)
                                    .ok_or_else(|| Error::MissingCode(vec![first_with]))
                                    .unwrap();

                                // and write it again under a different mangled key
                                AccountDBMut::new(db, &address).emplace(code_hash, code);
                            }
                            // if not, queue it up to be filled later
                            None => status.missing_code.push((address, code_hash)),
                        }
                    }
                }
            }
            let abi_hash = acc.abi_hash;
            match maybe_abi {
                // new inline abi
                Some(abi) => status.new_abi.push((abi_hash, abi, address)),
                None => {
                    if abi_hash != HASH_EMPTY {
                        // see if this abi has already been included inline
                        match known_abi.get(&abi_hash) {
                            Some(&first_with) => {
                                // if so, load it from the database.
                                let abi = AccountDB::new(db, &first_with)
                                    .get(&abi_hash)
                                    .ok_or_else(|| Error::MissingAbi(vec![first_with]))
                                    .unwrap();

                                // and write it again under a different mangled key
                                AccountDBMut::new(db, &address).emplace(abi_hash, abi);
                            }
                            // if not, queue it up to be filled later
                            None => status.missing_abi.push((address, abi_hash)),
                        }
                    }
                }
            }

            ::rlp::encode(&acc).into_vec()
        };

        *out = (address, thin_rlp);
    }
    if let Some(&(ref address, ref rlp)) = out_chunk.iter().last() {
        known_storage_roots.insert(*address, ::rlp::decode::<BasicAccount>(rlp).storage_root);
    }
    if let Some(&(ref address, ref rlp)) = out_chunk.iter().next() {
        known_storage_roots.insert(*address, ::rlp::decode::<BasicAccount>(rlp).storage_root);
    }
    Ok(status)
}

/// Used to rebuild the state trie piece by piece.
pub struct BlockRebuilder {
    executor: Arc<Executor>,
    db: Arc<KeyValueDB>,
    //_disconnected: Vec<(u64, H256)>,
    best_number: u64,
    best_hash: H256,
    best_root: H256,
    fed_blocks: u64,
    snapshot_blocks: u64,
}

impl BlockRebuilder {
    /// Create a new state rebuilder to write into the given backing DB.
    pub fn new(
        executor: Arc<Executor>,
        db: Arc<KeyValueDB>,
        manifest: &ManifestData,
        snapshot_blocks: u64,
    ) -> Self {
        BlockRebuilder {
            executor,
            db,
            //_disconnected: Vec::new(),
            best_number: manifest.block_number,
            best_hash: manifest.block_hash,
            best_root: manifest.state_root,
            fed_blocks: 0,
            snapshot_blocks,
        }
    }

    /// Feed an uncompressed state chunk into the rebuilder.
    pub fn feed(&mut self, chunk: &[u8], abort_flag: &AtomicBool) -> Result<(), ::error::Error> {
        let rlp = UntrustedRlp::new(chunk);
        let item_count = rlp.item_count()?;
        let num_blocks = (item_count - 2) as u64;
        trace!("restoring block chunk with {} blocks.", num_blocks);

        if self.fed_blocks + num_blocks > self.snapshot_blocks {
            return Err(
                Error::TooManyBlocks(self.snapshot_blocks, self.fed_blocks + num_blocks).into(),
            );
        }

        // todo: assert here that these values are consistent with chunks being in order.
        let mut cur_number = rlp.val_at::<u64>(0)? + 1;
        //let mut parent_hash = rlp.val_at::<H256>(1)?;

        for idx in 2..item_count {
            if !abort_flag.load(Ordering::SeqCst) {
                return Err(Error::RestorationAborted.into());
            }

            let pair = rlp.at(idx)?;

            let header_rlp = pair.as_raw().to_owned();
            let header: Header = ::rlp::decode(header_rlp.as_slice());

            let is_best = cur_number == self.best_number;

            if is_best {
                if header.hash().unwrap() != self.best_hash {
                    return Err(Error::WrongBlockHash(
                        cur_number,
                        self.best_hash,
                        header.hash().unwrap(),
                    )
                    .into());
                }

                if header.state_root() != &self.best_root {
                    return Err(Error::WrongStateRoot(self.best_root, *header.state_root()).into());
                }
            }

            // TODO: verify
            //verify_old_block(&mut self.rng, &header, engine, &self.chain, is_best)?;

            let mut batch = self.db.transaction();

            self.insert_unordered_block(&mut batch, &header, is_best);

            self.db.write_buffered(batch);

            // TODO: update current Chain.
            //self.chain.commit();

            //parent_hash = view!(BlockView, &block_bytes).hash();
            cur_number += 1;
        }

        self.fed_blocks += num_blocks;

        Ok(())
    }

    fn insert_unordered_block(&self, batch: &mut DBTransaction, header: &Header, is_best: bool) {
        let height = header.number();
        let hash = header.hash().unwrap();

        // store block in db
        batch.write(COL_HEADERS, &hash, &header.clone());

        batch.write(COL_EXTRA, &height, &hash);

        if is_best {
            batch.write(COL_EXTRA, &CurrentHash, &hash);
        }
    }

    /// Glue together any disconnected chunks and check that the chain is complete.
    fn finalize(&self) -> Result<(), ::error::Error> {
        let mut batch = self.db.transaction();

        /*for (first_num, first_hash) in self.disconnected.drain(..) {
            let parent_num = first_num - 1;
        
            // check if the parent is even in the chain.
            // since we don't restore every single block in the chain,
            // the first block of the first chunks has nothing to connect to.
            if let Some(parent_hash) = self.chain.block_hash(parent_num) {
                // if so, add the child to it.
                self.chain.add_child(&mut batch, parent_hash, first_hash);
            }
        }*/

        /*let genesis_hash = self.chain.genesis_hash();
        self.chain.insert_epoch_transition(&mut batch, 0, ::engines::EpochTransition {
            block_number: 0,
            block_hash: genesis_hash,
            proof: vec![],
        });*/
        let genesis_header = self
            .executor
            .block_header_by_height(0)
            .expect("Get genesis block failed");
        let hash = genesis_header.hash().unwrap();
        batch.write(COL_HEADERS, &hash, &genesis_header);
        batch.write(COL_EXTRA, &0, &hash);

        self.db.write_buffered(batch);
        Ok(())
    }
}

// helper for reading chunks from arbitrary reader and feeding them into the
// service.
pub fn restore_using<R: SnapshotReader>(
    snapshot: &Arc<Service>,
    reader: &R,
    recover: bool,
) -> Result<(), String> {
    let manifest = reader.manifest();

    info!(
        "Restoring to block #{} (0x{:?})",
        manifest.block_number, manifest.block_hash
    );

    snapshot
        .init_restore(manifest.clone(), recover)
        .map_err(|e| format!("Failed to begin restoration: {}", e))?;

    let (num_state, num_blocks) = (manifest.state_hashes.len(), manifest.block_hashes.len());

    let informant_handle = snapshot.clone();
    ::std::thread::spawn(move || {
        while let RestorationStatus::Ongoing {
            state_chunks_done,
            block_chunks_done,
            ..
        } = informant_handle.status()
        {
            info!(
                "Processed {}/{} state chunks and {}/{} block chunks.",
                state_chunks_done, num_state, block_chunks_done, num_blocks
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

        let hash = chunk.crypt_hash();
        if hash != state_hash {
            return Err(format!(
                "Mismatched chunk hash. Expected {:?}, got {:?}",
                state_hash, hash
            ));
        }

        snapshot.feed_state_chunk(state_hash, &chunk);
    }

    info!("Restoring blocks");
    for &block_hash in &manifest.block_hashes {
        if snapshot.status() == RestorationStatus::Failed {
            return Err("Restoration failed".into());
        }

        let chunk = reader.chunk(block_hash).map_err(|e| {
            format!(
                "Encountered error while reading chunk {:?}: {}",
                block_hash, e
            )
        })?;

        let hash = chunk.crypt_hash();
        if hash != block_hash {
            return Err(format!(
                "Mismatched chunk hash. Expected {:?}, got {:?}",
                block_hash, hash
            ));
        }
        snapshot.feed_block_chunk(block_hash, &chunk);
    }

    match snapshot.status() {
        RestorationStatus::Ongoing { .. } => {
            Err("Snapshot file is incomplete and missing chunks.".into())
        }
        RestorationStatus::Failed => Err("Snapshot restoration failed.".into()),
        RestorationStatus::Inactive => {
            info!("Restoration complete.");
            Ok(())
        }
    }
}
