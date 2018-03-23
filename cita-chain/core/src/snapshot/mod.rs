// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Snapshot format and creation.

extern crate num_cpus;

// chunks around 4MB before compression
const PREFERRED_CHUNK_SIZE: usize = 4 * 1024 * 1024;

//use account_db::{AccountDB, AccountDBMut};
//use libexecutor::executor::Executor;
use libchain::chain::Chain;
use rlp::{DecoderError, RlpStream, UntrustedRlp};
//use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
//use std::time::Duration;
use util::{snappy, Bytes};
use util::{H256, Mutex, sha3};
//use util::{Trie, TrieMut};
//use util::HASH_NULL_RLP;
//use util::journaldb::{self, Algorithm};
//use util::journaldb::JournalDB;
//use util::kvdb::Database;

//pub mod service;
pub mod io;
mod error;
use self::error::Error;
//use self::io::SnapshotReader;
use self::io::SnapshotWriter;
//use self::service::Service;
//use snapshot::service::SnapshotService;
use std::collections::VecDeque;
use types::ids::BlockId;

//#[cfg(test)]
//mod tests;

/// A sink for produced chunks.
pub type ChunkSink<'a> = FnMut(&[u8]) -> Result<(), Error> + 'a;

/// A progress indicator for snapshots.
#[derive(Debug, Default)]
pub struct Progress {
    blocks: AtomicUsize,
    size: AtomicUsize,
    done: AtomicBool,
}

impl Progress {
    /// Get the number of accounts snapshotted thus far.
    pub fn blocks(&self) -> usize {
        self.blocks.load(Ordering::Relaxed)
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
        block_chunks: u32,
        /// Number of state chunks completed.
        block_chunks_done: u32,
    },
    /// Failed restoration.
    Failed,
}

/// Snapshot manifest type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestData {
    /// List of state chunk hashes.
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
    pub fn to_rlp(self) -> Bytes {
        let mut stream = RlpStream::new_list(4);
        stream.append_list(&self.block_hashes);
        stream.append(&self.state_root);
        stream.append(&self.block_number);
        stream.append(&self.block_hash);

        stream.out()
    }

    /// Restore manifest data from raw bytes.
    pub fn from_rlp(raw: &[u8]) -> Result<Self, DecoderError> {
        let decoder = UntrustedRlp::new(raw);

        let block_hashes: Vec<H256> = decoder.list_at(0)?;
        let state_root: H256 = decoder.val_at(1)?;
        let block_number: u64 = decoder.val_at(2)?;
        let block_hash: H256 = decoder.val_at(3)?;

        Ok(ManifestData {
            block_hashes: block_hashes,
            state_root: state_root,
            block_number: block_number,
            block_hash: block_hash,
        })
    }
}

/// snapshot using: given Executor+ starting block hash + database; writing into the given writer.
pub fn take_snapshot<W: SnapshotWriter + Send>(
    chain: &Arc<Chain>,
    block_at: H256,
    writer: W,
    p: &Progress,
) -> Result<(), Error> {
    let start_header = chain
        .block_header_by_hash(block_at)
        .ok_or(Error::InvalidStartingBlock(BlockId::Hash(block_at)))?;
    let state_root = start_header.state_root();
    let number = start_header.number();

    info!("Taking snapshot starting at block {}", number);

    let writer = Mutex::new(writer);
    let block_hashes = chunk_block(chain, block_at, &writer, p)?;

    //info!("produced {} state chunks.", state_hashes.len());

    let manifest_data = ManifestData {
        block_hashes: block_hashes,
        state_root: *state_root,
        block_number: number,
        block_hash: block_at,
    };

    writer.into_inner().finish(manifest_data)?;

    p.done.store(true, Ordering::SeqCst);

    Ok(())
}

/// Returns a list of chunk hashes, with the first having the blocks furthest from the genesis.
pub fn chunk_block<'a>(
    chain: &'a Chain,
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
            let hash = sha3(&compressed_data);

            writer.lock().write_block_chunk(hash, &compressed_data)?;
            trace!(target: "snapshot", "wrote block chunk. hash: {:?}, size: {}, uncompressed size: {}",
                   hash, compressed_data.len(), raw_data.len());

            progress
                .size
                .fetch_add(compressed_data.len(), Ordering::SeqCst);
            chunk_hashes.push(hash);
            Ok(())
        };

        chunk_all(chain, start_hash, &mut chunk_sink, PREFERRED_CHUNK_SIZE)?;
    }

    Ok(chunk_hashes)
}

pub fn chunk_all(
    chain: &Chain,
    block_at: H256,
    chunk_sink: &mut ChunkSink,
    preferred_size: usize,
) -> Result<(), Error> {
    BlockChunker {
        chain: chain,
        rlps: VecDeque::new(),
        current_hash: block_at,
        writer: chunk_sink,
        preferred_size: preferred_size,
    }.chunk_all()
}

/// Used to build block chunks.
struct BlockChunker<'a> {
    chain: &'a Chain,
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

        let genesis_hash = self.chain
            .block_hash_by_height(0)
            .expect("Genesis hash should always exist");

        loop {
            if self.current_hash == genesis_hash {
                break;
            }

            info!("[chunk_all] current_hash: {:?}", self.current_hash);

            let block = self.chain
                .block_by_hash(self.current_hash)
                .ok_or(Error::BlockNotFound(self.current_hash))?;
            info!("[chunk_all] block");
            let receipts = match self.chain.block_receipts(self.current_hash) {
                Some(r) => r.receipts,
                _ => Vec::new(),
            };
            info!("[chunk_all] get block receipts end");
            //let abridged_rlp = AbridgedBlock::from_block_view(&block.view()).into_inner();
            let block_rlp = {
                let mut block_stream = RlpStream::new_list(2);
                block_stream.append(&block.header);
                block_stream.append(&block.body);
                block_stream.out()
            };

            let pair = {
                let mut pair_stream = RlpStream::new_list(2);
                pair_stream.append_raw(&block_rlp, 1).append_list(&receipts);
                pair_stream.out()
            };

            let new_loaded_size = loaded_size + pair.len();

            // cut off the chunk if too large.

            if new_loaded_size > self.preferred_size && !self.rlps.is_empty() {
                self.write_chunk(last)?;
                loaded_size = pair.len();
            } else {
                loaded_size = new_loaded_size;
            }

            self.rlps.push_front(pair);

            last = self.current_hash;
            self.current_hash = *block.parent_hash();
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
        trace!(target: "snapshot", "prepared block chunk with {} blocks", self.rlps.len());

        let last_header = self.chain
            .block_header_by_hash(last)
            .ok_or(Error::BlockNotFound(last))?;

        let parent_number = last_header.number() - 1;
        let parent_hash = last_header.parent_hash();

        trace!(target: "snapshot", "parent last written block: {}", parent_hash);

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

/*
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
*/
