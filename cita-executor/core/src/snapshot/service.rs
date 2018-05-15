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
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

//use super::{ManifestData, StateRebuilder, RestorationStatus, SnapshotService};
use super::{ManifestData, RestorationStatus, StateRebuilder};
use super::io::{LooseReader, LooseWriter, SnapshotReader, SnapshotWriter};

//use engines::EthEngine;
//use consensus::snapshot_components;
//use consensus::Rebuilder;
use error::Error;
//use types::ids::BlockId;

use cita_types::H256;
use journaldb::Algorithm;
use util::{Mutex, RwLock, RwLockReadGuard};
use util::Bytes;
use util::UtilError;
use util::kvdb::{Database, DatabaseConfig};
use util::snappy;

/// External database restoration handler
pub trait DatabaseRestore: Send + Sync {
    /// Restart with a new backend. Takes ownership of passed database and moves it to a new location.
    fn restore_db(&self, new_db: &str) -> Result<(), Error>;
}

/// State restoration manager.
struct Restoration {
    //manifest: ManifestData,
    state_chunks_left: HashSet<H256>,
    state: StateRebuilder,
    writer: Option<LooseWriter>,
    //final_state_root: H256,
    //guard: Guard,
    db: Arc<Database>,
}

struct RestorationParams<'a> {
    manifest: ManifestData,        // manifest to base restoration on.
    pruning: Algorithm,            // pruning algorithm for the database.
    db_path: PathBuf,              // database path
    db_config: &'a DatabaseConfig, // configuration for the database.
    writer: Option<LooseWriter>,   // writer for recovered snapshot.
                                   //guard: Guard,                  // guard for the restoration directory.
}

impl Restoration {
    // make a new restoration using the given parameters.
    fn new(params: RestorationParams) -> Result<Self, Error> {
        let manifest = params.manifest;

        let state_chunks = manifest.state_hashes.iter().cloned().collect();

        let raw_db =
            Arc::new(Database::open(params.db_config, &*params.db_path.to_string_lossy()).map_err(UtilError::from)?);

        //let secondary = components.rebuilder(raw_db.clone(), &manifest)?;

        //let root = manifest.state_root.clone();

        Ok(Restoration {
            //manifest: manifest,
            state_chunks_left: state_chunks,
            state: StateRebuilder::new(raw_db.clone(), params.pruning),
            writer: params.writer,
            //final_state_root: root,
            //guard: params.guard,
            db: raw_db,
        })
    }

    // feeds a state chunk, aborts early if `flag` becomes false.
    fn feed_state(&mut self, hash: H256, chunk: &[u8] /*, flag: &AtomicBool*/) -> Result<(), Error> {
        if self.state_chunks_left.contains(&hash) {
            let mut decompressed_data = Vec::new();
            snappy::decompress_to(chunk, &mut decompressed_data)?;

            let restoring_snapshot: AtomicBool = AtomicBool::new(false);
            restoring_snapshot.store(true, Ordering::SeqCst);
            self.state.feed(&decompressed_data, &restoring_snapshot)?;

            if let Some(ref mut writer) = self.writer.as_mut() {
                writer.write_state_chunk(hash, chunk)?;
            }

            self.state_chunks_left.remove(&hash);
        }

        Ok(())
    }
    /*
    // finish up restoration.
    fn finalize(self) -> Result<(), Error> {
        use util::TrieError;

        if !self.is_done() {
            return Ok(());
        }

        // verify final state root.
        let root = self.state.state_root();
        if root != self.final_state_root {
            warn!(
                "Final restored state has wrong state root: expected {:?}, got {:?}",
                root, self.final_state_root
            );
            return Err(TrieError::InvalidStateRoot(root).into());
        }

        // check for missing code.
        //self.state.finalize(self.manifest.block_number, self.manifest.block_hash)?;

        // connect out-of-order chunks and verify chain integrity.
        //self.secondary.finalize(engine)?;

        if let Some(writer) = self.writer {
            writer.finish(self.manifest)?;
        }

        self.guard.disarm();
        Ok(())
    }
    */

    // is everything done?
    fn is_done(&self) -> bool {
        self.state_chunks_left.is_empty()
    }
}

/// Snapshot service parameters.
pub struct ServiceParams {
    /// Database configuration options.
    pub db_config: DatabaseConfig,
    /// State pruning algorithm.
    //pub pruning: Algorithm,
    /// Usually "<chain hash>/snapshot"
    pub snapshot_root: PathBuf,
    /// A handle for database restoration.
    pub db_restore: Arc<DatabaseRestore>,
}

/// `SnapshotService` implementation.
/// This controls taking snapshots and restoring from them.
pub struct Service {
    restoration: Mutex<Option<Restoration>>,
    snapshot_root: PathBuf,
    db_config: DatabaseConfig,
    //pruning: Algorithm,
    status: Mutex<RestorationStatus>,
    reader: RwLock<Option<LooseReader>>,
    state_chunks: AtomicUsize,
    db_restore: Arc<DatabaseRestore>,
    progress: super::Progress,
    taking_snapshot: AtomicBool,
    restoring_snapshot: AtomicBool,
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
            state_chunks: AtomicUsize::new(0),
            db_restore: params.db_restore,
            progress: Default::default(),
            taking_snapshot: AtomicBool::new(false),
            restoring_snapshot: AtomicBool::new(false),
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

        let p = &self.progress;
        info!("Snapshot: {} accounts {} bytes", p.accounts(), p.size());
    }

    /// Initialize the restoration synchronously.
    /// The recover flag indicates whether to recover the restored snapshot.
    pub fn init_restore(&self, manifest: ManifestData, recover: bool) -> Result<(), Error> {
        let rest_dir = self.restoration_dir();

        let mut res = self.restoration.lock();

        self.state_chunks.store(0, Ordering::SeqCst);
        //self.block_chunks.store(0, Ordering::SeqCst);

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
        let writer = match recover {
            true => Some(LooseWriter::new(self.temp_recovery_dir())?),
            false => None,
        };

        let params = RestorationParams {
            manifest: manifest,
            pruning: Algorithm::Archive,
            db_path: self.restoration_db(),
            db_config: &self.db_config,
            writer: writer,
            //guard: Guard::new(rest_dir),
        };

        let state_chunks = params.manifest.state_hashes.len();
        //let block_chunks = params.manifest.block_hashes.len();

        *res = Some(Restoration::new(params)?);

        *self.status.lock() = RestorationStatus::Ongoing {
            state_chunks: state_chunks as u32,
            //block_chunks: block_chunks as u32,
            state_chunks_done: self.state_chunks.load(Ordering::SeqCst) as u32,
            //block_chunks_done: self.block_chunks.load(Ordering::SeqCst) as u32,
        };

        self.restoring_snapshot.store(true, Ordering::SeqCst);
        Ok(())
    }

    // finalize the restoration. this accepts an already-locked
    // restoration as an argument -- so acquiring it again _will_
    // lead to deadlock.
    fn finalize_restoration(&self, rest: &mut Option<Restoration>) -> Result<(), Error> {
        trace!(target: "snapshot", "finalizing restoration");

        let recover = rest.as_ref().map_or(false, |rest| rest.writer.is_some());

        // destroy the restoration before replacing databases and snapshot.
        //rest.take()
        //.map(|r| r.finalize(&*self.engine))
        //.unwrap_or(Ok(()))?;

        self.replace_client_db()?;

        if recover {
            let mut reader = self.reader.write();
            *reader = None; // destroy the old reader if it existed.

            let snapshot_dir = self.snapshot_dir();

            if snapshot_dir.exists() {
                trace!(target: "snapshot", "removing old snapshot dir at {}", snapshot_dir.to_string_lossy());
                fs::remove_dir_all(&snapshot_dir)?;
            }

            trace!(target: "snapshot", "copying restored snapshot files over");
            fs::rename(self.temp_recovery_dir(), &snapshot_dir)?;

            *reader = Some(LooseReader::new(snapshot_dir)?);
        }

        let _ = fs::remove_dir_all(self.restoration_dir());
        *self.status.lock() = RestorationStatus::Inactive;

        Ok(())
    }

    /// Feed a chunk of either kind. no-op if no restoration or status is wrong.
    fn feed_chunk(&self, hash: H256, chunk: &[u8] /*, is_state: bool*/) -> Result<(), Error> {
        // TODO: be able to process block chunks and state chunks at same time?
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
                            rest.feed_state(hash, chunk).map(|_| rest.is_done()),
                            rest.db.clone(),
                        )
                    };

                    let res = match res {
                        Ok(is_done) => {
                            self.state_chunks.fetch_add(1, Ordering::SeqCst);

                            match is_done {
                                true => {
                                    db.flush().map_err(UtilError::from)?;
                                    drop(db);
                                    return self.finalize_restoration(&mut *restoration);
                                }
                                false => Ok(()),
                            }
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
    pub fn feed_state_chunk(&self, hash: H256, chunk: &[u8]) {
        match self.feed_chunk(hash, chunk) {
            Ok(()) => (),
            Err(e) => {
                warn!("Encountered error during state restoration: {}", e);
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
            ref mut state_chunks_done,
            ..
        } = *cur_status
        {
            *state_chunks_done = self.state_chunks.load(Ordering::SeqCst) as u32;
            //*block_chunks_done = self.block_chunks.load(Ordering::SeqCst) as u32;
        }

        cur_status.clone()
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
