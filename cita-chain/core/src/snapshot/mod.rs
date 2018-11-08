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

//pub mod service;
pub mod error;
pub mod io;
pub mod service;

// chunks around 4MB before compression
const PREFERRED_CHUNK_SIZE: usize = 4 * 1024 * 1024;

// Maximal chunk size (decompressed)
// Snappy::decompressed_len estimation may sometimes yield results greater
// than PREFERRED_CHUNK_SIZE so allow some threshold here.
//const MAX_CHUNK_SIZE: usize = PREFERRED_CHUNK_SIZE / 4 * 5;
use header::Header;

use cita_types::H256;
use rlp::{DecoderError, Encodable, RlpStream, UntrustedRlp};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use util::kvdb::{DBTransaction, KeyValueDB};
use util::{snappy, Bytes, Hashable, Mutex, BLOCKLIMIT};

use basic_types::{LogBloom, LogBloomGroup};
use bloomchain::group::BloomGroupChain;
use bloomchain::{Bloom, Number as BloomChainNumber};

pub use self::error::Error;
use self::io::SnapshotReader;
use self::io::SnapshotWriter;
use self::service::{Service, SnapshotService};
use super::header::BlockNumber;
use db::{CacheUpdatePolicy, Writable, COL_BODIES, COL_EXTRA, COL_HEADERS};

use types::ids::BlockId;

use libchain::chain::Chain;
use types::block::{Block, BlockBody};
use types::extras::{BlockReceipts, CurrentHash, CurrentHeight, CurrentProof, LogGroupPosition};

use libproto::Proof;

use receipt::Receipt;

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
        /// Total number of block chunks.
        block_chunks: u32,
        /// Number of block chunks completed.
        block_chunks_done: u32,
    },
    /// Failed restoration.
    Failed,
}

/// Snapshot manifest type definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestData {
    /// List of block chunk hashes.
    pub block_hashes: Vec<H256>,
    /// The final, expected state root.
    pub state_root: H256,
    // Block number this snapshot was taken at.
    pub block_number: u64,
    // Block hash this snapshot was taken at.
    pub block_hash: H256,
    // Last Block proof
    pub last_proof: Proof,
}

/// Snapshot manifest encode/decode.
impl ManifestData {
    /// Encode the manifest data to rlp.
    pub fn to_rlp(&self) -> Bytes {
        let mut stream = RlpStream::new_list(4);
        stream.append_list(&self.block_hashes);
        stream.append(&self.state_root);
        stream.append(&self.block_number);
        stream.append(&self.block_hash);
        stream.append(&self.last_proof);

        stream.out()
    }

    /// Restore manifest data from raw bytes.
    pub fn from_rlp(raw: &[u8]) -> Result<Self, DecoderError> {
        let decoder = UntrustedRlp::new(raw);

        let block_hashes: Vec<H256> = decoder.list_at(0)?;
        let state_root: H256 = decoder.val_at(1)?;
        let block_number: u64 = decoder.val_at(2)?;
        let block_hash: H256 = decoder.val_at(3)?;
        let last_proof: Proof = decoder.val_at(4)?;

        Ok(ManifestData {
            block_hashes,
            state_root,
            block_number,
            block_hash,
            last_proof,
        })
    }
}

/// snapshot using: given Executor+ starting block hash + database; writing into the given writer.
pub fn take_snapshot<W: SnapshotWriter + Send>(
    chain: &Arc<Chain>,
    block_at: u64,
    writer: W,
    p: &Progress,
) -> Result<(), Error> {
    let block_hash = chain.block_hash_by_height(block_at).unwrap();

    let start_header = chain
        .block_header_by_hash(block_hash)
        .ok_or_else(|| Error::InvalidStartingBlock(BlockId::Hash(block_hash)))?;
    let state_root = start_header.state_root();

    info!("Taking snapshot starting at block {}", block_at);

    let writer = Mutex::new(writer);
    let block_hashes = chunk_secondary(chain, block_hash, &writer, p)?;

    info!("produced {} block chunks.", block_hashes.len());

    // get last_proof from chain, it will be used by cita-bft when restoring.
    let last_proof = if block_at == chain.get_current_height() {
        chain.current_block_poof()
    } else {
        chain.get_block_proof_by_height(block_at)
    }
    .unwrap();

    let manifest_data = ManifestData {
        block_hashes,
        state_root: *state_root,
        block_number: block_at,
        block_hash,
        last_proof,
    };

    writer.into_inner().finish(manifest_data)?;

    p.done.store(true, Ordering::SeqCst);

    Ok(())
}

/// Returns a list of chunk hashes, with the first having the blocks furthest from the genesis.
pub fn chunk_secondary<'a>(
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
            let hash = compressed_data.crypt_hash();
            let size = compressed_data.len();

            writer.lock().write_block_chunk(hash, &compressed_data)?;
            info!(
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
            chain,
            rlps: VecDeque::new(),
            current_hash: start_hash,
            writer: &mut chunk_sink,
            preferred_size: PREFERRED_CHUNK_SIZE,
        }
        .chunk_all()?
    }

    Ok(chunk_hashes)
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

        let start_header = self
            .chain
            .block_header_by_hash(self.current_hash)
            .ok_or_else(|| Error::BlockNotFound(self.current_hash))?;
        let step = if start_header.number() < 100 {
            1
        } else {
            start_header.number() / 100
        };

        let genesis_hash = self
            .chain
            .block_hash_by_height(0)
            .expect("Genesis hash should always exist");
        info!("Genesis_hash: {:?}", genesis_hash);
        let mut blocks_num = 0;

        loop {
            if self.current_hash == genesis_hash {
                break;
            }

            let mut s = RlpStream::new();
            /*let block = self.chain
                .block_by_hash(self.current_hash)
                .ok_or(Error::BlockNotFound(self.current_hash))?;
            block.rlp_append(&mut s);
            let block_rlp = s.out();*/
            let header = self
                .chain
                .block_header_by_hash(self.current_hash)
                .ok_or_else(|| Error::BlockNotFound(self.current_hash))?;
            header.clone().rlp_append(&mut s);
            let header_rlp = s.out();

            if blocks_num % step == 0 {
                info!("current height: {:?}", header.number());
            }

            let pair: Vec<u8> = if blocks_num < BLOCKLIMIT {
                let body_rlp = {
                    let body: BlockBody = self
                        .chain
                        .block_body(BlockId::Hash(self.current_hash))
                        .ok_or_else(|| Error::BlockNotFound(self.current_hash))?;
                    let mut s = RlpStream::new();
                    body.rlp_append(&mut s);
                    s.out()
                };

                let receipts = match self.chain.block_receipts(self.current_hash) {
                    Some(r) => r.receipts,
                    _ => Vec::new(),
                };

                let mut pair_stream = RlpStream::new_list(3);
                pair_stream
                    .append_raw(&header_rlp, 1)
                    .append_list(&receipts)
                    .append_raw(&body_rlp, 1);
                pair_stream.out()
            } else {
                let mut pair_stream = RlpStream::new_list(1);
                pair_stream.append_raw(&header_rlp, 1);
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
            .chain
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

/// Brief info about inserted block.
#[derive(Clone)]
pub struct BlockInfo {
    /// Block hash.
    pub hash: H256,
    /// Block number.
    pub number: BlockNumber,
}

/// Block extras update info.
pub struct ExtrasUpdate {
    /// Block info.
    pub info: BlockInfo,
    /// Current block uncompressed rlp bytes
    pub block: Block,
    /// Modified block hashes.
    pub block_hashes: HashMap<H256, BlockNumber>,
    /// Modified block receipts.
    pub block_receipts: HashMap<H256, BlockReceipts>,
    /// Modified blocks blooms.
    pub blocks_blooms: HashMap<LogGroupPosition, LogBloomGroup>,
    // Modified transaction addresses (None signifies removed transactions).
    //pub transactions_addresses: HashMap<H256, TransactionAddress>,
}

/// Used to rebuild the state trie piece by piece.
pub struct BlockRebuilder {
    chain: Arc<Chain>,
    db: Arc<KeyValueDB>,
    //_disconnected: Vec<(u64, H256)>,
    best_number: u64,
    best_hash: H256,
    best_root: H256,
    cur_proof: Proof,
    fed_blocks: u64,
    snapshot_blocks: u64,
}

impl BlockRebuilder {
    /// Create a new state rebuilder to write into the given backing DB.
    pub fn new(
        chain: Arc<Chain>,
        db: Arc<KeyValueDB>,
        manifest: &ManifestData,
        snapshot_blocks: u64,
    ) -> Self {
        BlockRebuilder {
            chain,
            db,
            //_disconnected: Vec::new(),
            best_number: manifest.block_number,
            best_hash: manifest.block_hash,
            best_root: manifest.state_root,
            cur_proof: manifest.last_proof.clone(),
            fed_blocks: 0,
            snapshot_blocks,
        }
    }

    /// Feed an uncompressed state chunk into the rebuilder.
    pub fn feed(&mut self, chunk: &[u8], abort_flag: &AtomicBool) -> Result<(), ::error::Error> {
        let rlp = UntrustedRlp::new(chunk);
        let item_count = rlp.item_count()?;
        let num_blocks = (item_count - 2) as u64;
        info!("restoring block chunk with {} blocks.", num_blocks);

        if self.fed_blocks + num_blocks > self.snapshot_blocks {
            info!(
                "already {}, now {}, total {}",
                self.fed_blocks, num_blocks, self.snapshot_blocks
            );
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

            let mut block = Block::default();
            let pair = rlp.at(idx)?;
            //let block_rlp = pair.at(0)?.as_raw().to_owned();
            //let block: Block = ::rlp::decode(block_rlp.as_slice());
            let header_rlp = pair.at(0)?.as_raw().to_owned();
            let header: Header = ::rlp::decode(header_rlp.as_slice());
            {
                block.set_header(header.clone());
            }

            // if height + 100 < max_height, there will be bodies and receipts.
            let mut have_body: bool = false;

            let receipts: Vec<Receipt> = pair.list_at(1).unwrap_or_default();

            if let Ok(b) = pair.at(2) {
                have_body = true;
                let body_rlp = b.as_raw().to_owned();
                let body = ::rlp::decode(body_rlp.as_slice());
                block.set_body(body);
            };

            // TODO: abridged_block
            /*let receipts_root = ordered_trie_root(pair.at(1)?.iter().map(|r| r.as_raw()));
            
            let block = abridged_block.to_block(parent_hash, cur_number, receipts_root)?;
            let block_bytes = block.rlp_bytes();*/

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
            //verify_old_block(&mut self.rng, &block.header, engine, &self.chain, is_best)?;

            let mut batch = self.db.transaction();

            self.insert_unordered_block(&mut batch, &block, receipts, have_body, is_best);

            self.db.write_buffered(batch);

            // TODO: update current Chain.
            //self.chain.commit();

            //parent_hash = view!(BlockView, &block_bytes).hash();
            cur_number += 1;
        }

        self.fed_blocks += num_blocks;

        Ok(())
    }

    fn insert_unordered_block(
        &mut self,
        batch: &mut DBTransaction,
        block: &Block,
        receipts: Vec<Receipt>,
        have_body: bool,
        is_best: bool,
    ) {
        let header = block.header();
        let hash = header.hash().unwrap();
        let height = header.number();

        // store block in db
        batch.write(COL_HEADERS, &height, &header.clone());
        if have_body {
            let body = block.body();
            batch.write(COL_BODIES, &height, &body.clone());
        }

        // COL_EXTRA
        let info = BlockInfo {
            hash,
            number: height,
        };

        self.prepare_update(
            batch,
            ExtrasUpdate {
                block_hashes: self.prepare_block_hashes_update(block, &info),
                block_receipts: self.prepare_block_receipts_update(receipts, &info, have_body),
                blocks_blooms: self.prepare_block_blooms_update(block, &info),
                //transactions_addresses: self.prepare_transaction_addresses_update(block, &info),
                info,
                block: block.clone(),
            },
            is_best,
        );
    }

    /// This function returns modified block hashes.
    fn prepare_block_hashes_update(
        &self,
        _block: &Block,
        info: &BlockInfo,
    ) -> HashMap<H256, BlockNumber> {
        let mut block_hashes = HashMap::new();

        block_hashes.insert(info.hash, info.number);

        block_hashes
    }

    /// This function returns modified block receipts.
    fn prepare_block_receipts_update(
        &self,
        receipts: Vec<Receipt>,
        info: &BlockInfo,
        have_body: bool,
    ) -> HashMap<H256, BlockReceipts> {
        let mut block_receipts = HashMap::new();

        if have_body {
            block_receipts.insert(info.hash, BlockReceipts::new(receipts));
        }

        block_receipts
    }

    /// This functions returns modified blocks blooms.
    ///
    /// To accelerate blooms lookups, blooms are stored in multiple
    /// layers (BLOOM_LEVELS, currently 3).
    /// ChainFilter is responsible for building and rebuilding these layers.
    /// It returns them in HashMap, where values are Blooms and
    /// keys are BloomIndexes. BloomIndex represents bloom location on one
    /// of these layers.
    ///
    /// To reduce number of queries to database, block blooms are stored
    /// in BlocksBlooms structure which contains info about several
    /// (BLOOM_INDEX_SIZE, currently 16) consecutive blocks blooms.
    ///
    /// Later, BloomIndexer is used to map bloom location on filter layer (BloomIndex)
    /// to bloom location in database (BlocksBloomLocation).
    ///
    fn prepare_block_blooms_update(
        &self,
        block: &Block,
        info: &BlockInfo,
    ) -> HashMap<LogGroupPosition, LogBloomGroup> {
        let header = block.header();

        let log_bloom = LogBloom::from(header.log_bloom().to_vec().as_slice());
        let log_blooms: HashMap<LogGroupPosition, LogBloomGroup> = if log_bloom.is_zero() {
            HashMap::new()
        } else {
            let bgroup = BloomGroupChain::new(self.chain.blooms_config, &*self.chain);
            bgroup
                .insert(
                    info.number as BloomChainNumber,
                    Bloom::from(Into::<[u8; 256]>::into(log_bloom)),
                )
                .into_iter()
                .map(|p| (From::from(p.0), From::from(p.1)))
                .collect()
        };
        log_blooms
    }

    /// This function returns modified transaction addresses.
    /*fn prepare_transaction_addresses_update(
        &self,
        block: &Block,
        info: &BlockInfo,
    ) -> HashMap<H256, TransactionAddress> {
        let transaction_hashes = block.body().transaction_hashes();
    
        transaction_hashes
            .into_iter()
            .enumerate()
            .map(|(i, tx_hash)| {
                (
                    tx_hash,
                    TransactionAddress {
                        block_hash: info.hash,
                        index: i,
                    },
                )
            })
            .collect()
    }*/

    fn prepare_update(&self, batch: &mut DBTransaction, update: ExtrasUpdate, is_best: bool) {
        {
            let mut write_receipts = self.chain.block_receipts.write();
            batch.extend_with_cache(
                COL_EXTRA,
                &mut *write_receipts,
                update.block_receipts,
                CacheUpdatePolicy::Remove,
            );
        }

        {
            let mut write_blocks_blooms = self.chain.blocks_blooms.write();
            // update best block
            // update all existing blooms groups
            /*for (key, value) in update.blocks_blooms {
                match write_blocks_blooms.entry(key) {
                    hash_map::Entry::Occupied(mut entry) => {
                        entry.get_mut().accrue_bloom_group(&value);
                        batch.write(COL_EXTRA, entry.key(), entry.get());
                    },
                    hash_map::Entry::Vacant(entry) => {
                        batch.write(COL_EXTRA, entry.key(), &value);
                        entry.insert(value);
                    },
                }
            }*/
            batch.extend_with_cache(
                COL_EXTRA,
                &mut *write_blocks_blooms,
                update.blocks_blooms,
                CacheUpdatePolicy::Overwrite,
            );
        }

        // These cached values must be updated last with all four locks taken to avoid
        // cache decoherence
        {
            if is_best {
                batch.write(COL_EXTRA, &CurrentHash, &update.info.hash);
                //batch.write(COL_EXTRA, &CurrentProof, &update.info.hash);
                batch.write(COL_EXTRA, &CurrentHeight, &update.info.number);
            }

            let mut write_hashes = self.chain.block_hashes.write();
            //let mut write_txs = self.chain.transaction_addresses.write();
            //let mut write_hashes = HashMap::new();
            //let mut write_txs = HashMap::new();

            /*batch.extend_with_cache(
                COL_EXTRA,
                &mut *write_hashes,
                update.block_hashes,
                CacheUpdatePolicy::Overwrite,
            );*/
            batch.write_with_cache(
                COL_EXTRA,
                &mut *write_hashes,
                update.info.hash,
                update.info.number as BlockNumber,
                CacheUpdatePolicy::Overwrite,
            );
            /*batch.extend_with_cache(
                COL_EXTRA,
                &mut *write_txs,
                update.transactions_addresses,
                CacheUpdatePolicy::Overwrite,
            );*/
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
        let genesis_block = self
            .chain
            .block_by_height(0)
            .expect("Get genesis block failed");
        batch.write(COL_HEADERS, &0, &genesis_block.header.clone());

        batch.write(COL_BODIES, &0, &genesis_block.body.clone());
        /*let mut write_bodies: HashMap<BlockNumber, BlockBody> = HashMap::new();
        batch.write_with_cache(
            COL_BODIES,
            &mut write_bodies,
            0 as BlockNumber,
            genesis_block.body.clone(),
            CacheUpdatePolicy::Overwrite,
        );*/
        batch.write(COL_EXTRA, &CurrentProof, &self.cur_proof);

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

    let num_block = manifest.block_hashes.len();

    let informant_handle = snapshot.clone();
    ::std::thread::spawn(move || {
        while let RestorationStatus::Ongoing {
            block_chunks_done, ..
        } = informant_handle.status()
        {
            info!(
                "Processed {}/{} block chunks.",
                block_chunks_done, num_block
            );
            ::std::thread::sleep(Duration::from_secs(5));
        }
    });

    info!("Restoring block");
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
