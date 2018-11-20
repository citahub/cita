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

//! Snapshot network service implementation.

use std::collections::HashSet;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::usize::MAX;

//use super::{SnapshotService};
use super::io::{LooseReader, LooseWriter, SnapshotReader, SnapshotWriter};
use super::{BlockRebuilder, ManifestData, RestorationStatus};

use cita_types::H256;
use error::Error;

use util::kvdb::{Database, DatabaseConfig};
use util::snappy;
use util::Bytes;
use util::UtilError;
use util::{Mutex, RwLock, RwLockReadGuard};

use libchain::chain::{get_chain, get_chain_body_height, Chain};

use filters::PollManager;

/// Number of blocks in an ethash snapshot.
// make dependent on difficulty incrment divisor?
//const SNAPSHOT_BLOCKS: u64 = 5000;
/// Maximum number of blocks allowed in an ethash snapshot.
//const MAX_SNAPSHOT_BLOCKS: u64 = 30000;

/// External database restoration handler
pub trait DatabaseRestore: Send + Sync {
    /// Restart with a new backend. Takes ownership of passed database and moves it to a new location.
    fn restore_db(&self, new_db: &str) -> Result<(), Error>;
}

impl DatabaseRestore for Chain {
    /// Restart the client with a new backend
    fn restore_db(&self, new_db: &str) -> Result<(), Error> {
        info!("Replacing client database with {:?}", new_db);

        let db = self.db.write();
        db.restore(new_db)?;

        // replace chain
        //*chain = Arc::new(BlockChain::new(self.config.blockchain.clone(), &[], db.clone()));
        let header = get_chain(&*db.clone()).expect("Get chain failed");

        let current_height = header.number();

        // header
        *self.current_header.write() = header.clone();

        self.current_height
            .store(current_height as usize, Ordering::SeqCst);

        if let Some(height) = get_chain_body_height(&*db.clone()) {
            self.max_store_height
                .store(height as usize, Ordering::SeqCst);
        }

        let mut block_map = self.block_map.write();
        block_map.clear();

        let mut block_header = self.block_headers.write();
        block_header.clear();
        block_header.shrink_to_fit();

        let mut block_bodies = self.block_bodies.write();
        block_bodies.clear();
        block_bodies.shrink_to_fit();

        let mut block_hashes = self.block_hashes.write();
        block_hashes.clear();
        block_hashes.shrink_to_fit();

        let mut transaction_addresses = self.transaction_addresses.write();
        transaction_addresses.clear();
        transaction_addresses.shrink_to_fit();

        let mut blocks_blooms = self.blocks_blooms.write();
        blocks_blooms.clear();
        blocks_blooms.shrink_to_fit();

        let mut block_receipts = self.block_receipts.write();
        block_receipts.clear();
        block_receipts.shrink_to_fit();

        let mut cache_man = self.cache_man.lock();
        cache_man.collect_garbage(MAX, |_| MAX);

        *self.polls_filter.lock() = PollManager::default();

        Ok(())
    }
}

/// State restoration manager.
struct Restoration {
    manifest: ManifestData,
    block_chunks_left: HashSet<H256>,
    secondary: Box<BlockRebuilder>,
    writer: Option<LooseWriter>,
    //guard: Guard,
    db: Arc<Database>,
}

struct RestorationParams<'a> {
    manifest: ManifestData, // manifest to base restoration on.
    //pruning: Algorithm,            // pruning algorithm for the database.
    db_path: PathBuf,              // database path
    db_config: &'a DatabaseConfig, // configuration for the database.
    writer: Option<LooseWriter>,   // writer for recovered snapshot..
    chain: Arc<Chain>,
    //guard: Guard,                  // guard for the restoration directory.
}

impl Restoration {
    // make a new restoration using the given parameters.
    fn new(params: RestorationParams) -> Result<Self, Error> {
        let manifest = params.manifest;

        let block_chunks = manifest.block_hashes.iter().cloned().collect();

        let raw_db = Arc::new(
            Database::open(params.db_config, &*params.db_path.to_string_lossy())
                .map_err(UtilError::from)?,
        );

        let block_rebuilder = BlockRebuilder::new(
            params.chain.clone(),
            raw_db.clone(),
            &manifest,
            manifest.block_number,
        );

        Ok(Restoration {
            manifest,
            block_chunks_left: block_chunks,
            secondary: Box::new(block_rebuilder),
            writer: params.writer,
            //guard: params.guard,
            db: raw_db,
        })
    }

    // feeds a block chunk
    fn feed_blocks(&mut self, hash: H256, chunk: &[u8], flag: &AtomicBool) -> Result<(), Error> {
        if self.block_chunks_left.contains(&hash) {
            let mut decompressed_data = Vec::new();
            snappy::decompress_to(chunk, &mut decompressed_data)?;

            self.secondary.feed(&decompressed_data, flag)?;

            if let Some(ref mut writer) = self.writer.as_mut() {
                writer.write_block_chunk(hash, chunk)?;
            }

            self.block_chunks_left.remove(&hash);
        }

        Ok(())
    }

    // finish up restoration.
    fn finalize(self) -> Result<(), Error> {
        if !self.is_done() {
            return Ok(());
        }

        // connect out-of-order chunks and verify chain integrity.
        self.secondary.finalize()?;

        if let Some(writer) = self.writer {
            writer.finish(self.manifest)?;
        }

        //self.guard.disarm();
        Ok(())
    }

    // is everything done?
    fn is_done(&self) -> bool {
        self.block_chunks_left.is_empty()
    }
}

/// Snapshot service parameters.
pub struct ServiceParams {
    /// Database configuration options.
    pub db_config: DatabaseConfig,
    /// Usually "<chain hash>/snapshot"
    pub snapshot_root: PathBuf,
    /// A handle for database restoration.
    pub db_restore: Arc<DatabaseRestore>,
    pub chain: Arc<Chain>,
}

/// `SnapshotService` implementation.
/// This controls taking snapshots and restoring from them.
pub struct Service {
    restoration: Mutex<Option<Restoration>>,
    snapshot_root: PathBuf,
    db_config: DatabaseConfig,
    status: Mutex<RestorationStatus>,
    reader: RwLock<Option<LooseReader>>,
    block_chunks: AtomicUsize,
    db_restore: Arc<DatabaseRestore>,
    progress: super::Progress,
    taking_snapshot: AtomicBool,
    restoring_snapshot: AtomicBool,
    chain: Arc<Chain>,
}

impl Service {
    /// Create a new snapshot service from the given parameters.
    pub fn new(params: ServiceParams) -> Result<Self, Error> {
        let mut service = Service {
            restoration: Mutex::new(None),
            snapshot_root: params.snapshot_root,
            db_config: params.db_config,
            //pruning: params.pruning,
            status: Mutex::new(RestorationStatus::Inactive),
            reader: RwLock::new(None),
            block_chunks: AtomicUsize::new(0),
            db_restore: params.db_restore,
            progress: Default::default(),
            taking_snapshot: AtomicBool::new(false),
            restoring_snapshot: AtomicBool::new(false),
            chain: params.chain.clone(),
        };

        // create the root snapshot dir if it doesn't exist.
        if let Err(e) = fs::create_dir_all(&service.snapshot_root) {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(e.into());
            }
        }

        // delete the temporary restoration dir if it does exist.
        if let Err(e) = fs::remove_dir_all(service.restoration_dir()) {
            if e.kind() != ErrorKind::NotFound {
                return Err(e.into());
            }
        }

        // delete the temporary snapshot dir if it does exist.
        if let Err(e) = fs::remove_dir_all(service.temp_snapshot_dir()) {
            if e.kind() != ErrorKind::NotFound {
                return Err(e.into());
            }
        }

        let reader = LooseReader::new(service.snapshot_dir()).ok();
        *service.reader.get_mut() = reader;

        Ok(service)
    }

    // get the current snapshot dir.
    fn snapshot_dir(&self) -> PathBuf {
        let mut dir = self.snapshot_root.clone();
        dir.push("current");
        dir
    }

    // get the temporary snapshot dir.
    fn temp_snapshot_dir(&self) -> PathBuf {
        let mut dir = self.snapshot_root.clone();
        dir.push("in_progress");
        dir
    }

    // get the restoration directory.
    fn restoration_dir(&self) -> PathBuf {
        let mut dir = self.snapshot_root.clone();
        dir.push("restoration");
        dir
    }

    // restoration db path.
    fn restoration_db(&self) -> PathBuf {
        let mut dir = self.restoration_dir();
        dir.push("db");
        dir
    }

    // temporary snapshot recovery path.
    fn temp_recovery_dir(&self) -> PathBuf {
        let mut dir = self.restoration_dir();
        dir.push("temp");
        dir
    }

    // replace one the client's database with our own.
    fn replace_client_db(&self) -> Result<(), Error> {
        let our_db = self.restoration_db();

        self.db_restore.restore_db(&*our_db.to_string_lossy())?;
        Ok(())
    }

    /// Get a reference to the snapshot reader.
    pub fn reader(&self) -> RwLockReadGuard<Option<LooseReader>> {
        self.reader.read()
    }

    /// Tick the snapshot service. This will log any active snapshot
    /// being taken.
    pub fn tick(&self) {
        if self.progress.done() || !self.taking_snapshot.load(Ordering::SeqCst) {
            return;
        }

        let _p = &self.progress;
        //info!("Snapshot: {} accounts {} bytes", p.accounts(), p.size());
    }

    /// Initialize the restoration synchronously.
    /// The recover flag indicates whether to recover the restored snapshot.
    pub fn init_restore(&self, manifest: ManifestData, recover: bool) -> Result<(), Error> {
        let rest_dir = self.restoration_dir();

        let mut res = self.restoration.lock();

        self.block_chunks.store(0, Ordering::SeqCst);

        // tear down existing restoration.
        *res = None;

        // delete and restore the restoration dir.
        if let Err(e) = fs::remove_dir_all(&rest_dir) {
            match e.kind() {
                ErrorKind::NotFound => {}
                _ => return Err(e.into()),
            }
        }

        fs::create_dir_all(&rest_dir)?;

        // make new restoration.
        let writer = if recover {
            Some(LooseWriter::new(self.temp_recovery_dir())?)
        } else {
            None
        };

        let params = RestorationParams {
            chain: self.chain.clone(),
            manifest,
            db_path: self.restoration_db(),
            db_config: &self.db_config,
            writer,
            //guard: Guard::new(rest_dir),
        };

        let block_chunks = params.manifest.block_hashes.len();

        *res = Some(Restoration::new(params)?);

        *self.status.lock() = RestorationStatus::Ongoing {
            block_chunks: block_chunks as u32,
            block_chunks_done: self.block_chunks.load(Ordering::SeqCst) as u32,
        };

        self.restoring_snapshot.store(true, Ordering::SeqCst);
        Ok(())
    }

    // finalize the restoration. this accepts an already-locked
    // restoration as an argument -- so acquiring it again _will_
    // lead to deadlock.
    fn finalize_restoration(&self, rest: &mut Option<Restoration>) -> Result<(), Error> {
        info!("finalizing restoration");

        let recover = rest.as_ref().map_or(false, |rest| rest.writer.is_some());

        // destroy the restoration before replacing databases and snapshot.
        rest.take().map(|r| r.finalize()).unwrap_or(Ok(()))?;

        self.replace_client_db()?;

        if recover {
            let mut reader = self.reader.write();
            *reader = None; // destroy the old reader if it existed.

            let snapshot_dir = self.snapshot_dir();

            if snapshot_dir.exists() {
                trace!(
                    "removing old snapshot dir at {}",
                    snapshot_dir.to_string_lossy(),
                );
                fs::remove_dir_all(&snapshot_dir)?;
            }

            trace!("copying restored snapshot files over");
            fs::rename(self.temp_recovery_dir(), &snapshot_dir)?;

            *reader = Some(LooseReader::new(snapshot_dir)?);
        }

        fs::remove_dir_all(&self.snapshot_root)?;
        *self.status.lock() = RestorationStatus::Inactive;

        Ok(())
    }

    /// Feed a chunk of either kind. no-op if no restoration or status is wrong.
    fn feed_chunk(&self, hash: H256, chunk: &[u8]) -> Result<(), Error> {
        let (result, db) = {
            let mut restoration = self.restoration.lock();

            match self.status() {
                RestorationStatus::Inactive | RestorationStatus::Failed => return Ok(()),
                RestorationStatus::Ongoing { .. } => {
                    let (res, db) = {
                        let rest = match *restoration {
                            Some(ref mut r) => r,
                            None => return Ok(()),
                        };

                        (
                            rest.feed_blocks(hash, chunk, &self.restoring_snapshot)
                                .map(|_| rest.is_done()),
                            rest.db.clone(),
                        )
                    };

                    let res = match res {
                        Ok(is_done) => {
                            self.block_chunks.fetch_add(1, Ordering::SeqCst);

                            if is_done {
                                db.flush().map_err(UtilError::from)?;
                                drop(db);
                                return self.finalize_restoration(&mut *restoration);
                            }
                            Ok(())
                        }
                        other => other.map(drop),
                    };
                    (res, db)
                }
            }
        };
        result.and_then(|_| db.flush().map_err(|e| UtilError::from(e).into()))
    }

    /// Feed a state chunk to be processed synchronously.
    pub fn feed_block_chunk(&self, hash: H256, chunk: &[u8]) {
        match self.feed_chunk(hash, chunk) {
            Ok(()) => (),
            Err(e) => {
                warn!("Encountered error during block restoration: {}", e);
                *self.restoration.lock() = None;
                *self.status.lock() = RestorationStatus::Failed;
                let _ = fs::remove_dir_all(self.restoration_dir());
            }
        }
    }
}

impl SnapshotService for Service {
    //fn manifest(&self) -> Option<ManifestData> {
    //self.reader.read().as_ref().map(|r| r.manifest().clone())
    //}

    fn chunk(&self, hash: H256) -> Option<Bytes> {
        self.reader.read().as_ref().and_then(|r| r.chunk(hash).ok())
    }

    fn status(&self) -> RestorationStatus {
        let mut cur_status = self.status.lock();
        if let RestorationStatus::Ongoing {
            ref mut block_chunks_done,
            ..
        } = *cur_status
        {
            *block_chunks_done = self.block_chunks.load(Ordering::SeqCst) as u32;
            //*block_chunks_done = self.block_chunks.load(Ordering::SeqCst) as u32;
        }

        *cur_status
    }
    /*
    fn begin_restore(&self, manifest: ManifestData) {
        if let Err(e) = self.io_channel.lock().send(ClientIoMessage::BeginRestoration(manifest)) {
            trace!("Error sending snapshot service message: {:?}", e);
        }
    }
    */

    fn abort_restore(&self) {
        self.restoring_snapshot.store(false, Ordering::SeqCst);
        *self.restoration.lock() = None;
        *self.status.lock() = RestorationStatus::Inactive;
    }
    /*
    fn restore_state_chunk(&self, hash: H256, chunk: Bytes) {
        if let Err(e) = self.io_channel.lock().send(ClientIoMessage::FeedStateChunk(hash, chunk)) {
            trace!("Error sending snapshot service message: {:?}", e);
        }
    }
    */
}
/// The interface for a snapshot network service.
/// This handles:
///    - restoration of snapshots to temporary databases.
///    - responding to queries for snapshot manifests and chunks
pub trait SnapshotService: Sync + Send {
    /// Query the most recent manifest data.
    //fn manifest(&self) -> Option<&ManifestData>;

    /// Get raw chunk for a given hash.
    fn chunk(&self, hash: H256) -> Option<Bytes>;

    /// Ask the snapshot service for the restoration status.
    fn status(&self) -> RestorationStatus;

    /// Abort an in-progress restoration if there is one.
    fn abort_restore(&self);
}

impl Drop for Service {
    fn drop(&mut self) {
        self.abort_restore();
    }
}
