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

use basic_types::*;
use bloomchain as bc;
use blooms::*;
pub use byteorder::{BigEndian, ByteOrder};
use cache_manager::CacheManager;
use call_analytics::CallAnalytics;
use cita_crypto::pubkey_to_address;
use cita_transaction::eth_transaction::{SignedTransaction, VMTransaction, Action};
use db;
use db::*;

use engines::NullEngine;
use env_info::{LastHashes, EnvInfo};
use error::CallError;
use evm::Factory as EvmFactory;
use executive::{Executive, Executed, TransactOptions, contract_address};
use factory::*;
use filters::{PollManager, PollFilter};
use header::*;
pub use libchain::block::*;
use libchain::cache::CacheSize;
use libchain::call_request::CallRequest;
use libchain::extras::*;

use libchain::genesis::Genesis;
pub use libchain::transaction::*;
use libproto::blockchain::{ProofType, BlockBody};
use libproto::request::FullTransaction;
use proof::TendermintProof;
pub use protobuf::{Message, RepeatedField};
use protobuf::core::parse_from_bytes;
use receipt::{Receipt, LocalizedReceipt};
use state::State;
use state_db::StateDB;

use std::collections::{BTreeMap, VecDeque};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::sync::mpsc::Sender;
use types::filter::Filter;
use types::ids::{BlockId, TransactionId};
use types::log_entry::{LogEntry, LocalizedLogEntry};
use util::{journaldb, H256, H512, U256, H2048, Address, Bytes};
use util::{RwLock, Mutex};
use util::HeapSizeOf;
use util::kvdb::*;
use util::trie::{TrieFactory, TrieSpec};

pub const VERSION: u32 = 0;
const LOG_BLOOMS_LEVELS: usize = 3;
const LOG_BLOOMS_ELEMENTS_PER_INDEX: usize = 16;

#[derive(PartialEq, Clone, Debug)]
pub enum BlockSource {
    CONSENSUS = 0,
    NET = 1,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum CacheId {
    Blocks(H256),
    BlockHashes(BlockNumber),
    TransactionAddresses(H256),
    BlocksBlooms(LogGroupPosition),
    BlockReceipts(H256),
}

impl bc::group::BloomGroupDatabase for Chain {
    fn blooms_at(&self, position: &bc::group::GroupPosition) -> Option<bc::group::BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.blocks_blooms, &position).map(Into::into);
        self.cache_man.lock().note_used(CacheId::BlocksBlooms(position));
        result
    }
}

// TODO: Chain Errors
pub trait TransactionHash {
    fn transaction_hashes(&self) -> Vec<H256>;
}

// TODO: chain对外开放的方法，是保证能正确解析结构，即类似于Result<Block,Err>
// 所有直接unwrap的地方都可能会报错！
// TODO: should keep current header
// TODO: refactor: add header struct
// TODO: refactor: add chain cache
pub struct Chain {
    blooms_config: bc::Config,
    pub current_hash: RwLock<H256>,
    pub current_height: AtomicUsize,
    pub is_sync: AtomicBool,
    pub max_height: AtomicUsize,
    pub block_map: RwLock<BTreeMap<u64, (BlockSource, Block)>>,
    pub db: Arc<KeyValueDB>,
    pub sync_sender: Mutex<Sender<u64>>,
    pub state_db: StateDB,
    pub factories: Factories,
    // Hash of the given block - only works for 256 most recent blocks excluding current
    pub last_hashes: RwLock<VecDeque<H256>>,

    // block cache
    blocks: RwLock<HashMap<H256, Block>>,

    // extra caches
    block_hashes: RwLock<HashMap<BlockNumber, H256>>,
    transaction_addresses: RwLock<HashMap<TransactionId, DBList<TransactionAddress>>>,
    blocks_blooms: RwLock<HashMap<LogGroupPosition, BloomGroup>>,
    block_receipts: RwLock<HashMap<H256, BlockReceipts>>,

    cache_man: Mutex<CacheManager<CacheId>>,
    polls_filter: Arc<Mutex<PollManager<PollFilter>>>,
}

pub fn save_genesis(db: &KeyValueDB, genesis: &Genesis) -> Result<(), String> {
    let mut batch = db.transaction();
    let hash = genesis.hash.into();
    let height: BlockNumber = 0;
    batch.put_vec(db::COL_BLOCKS, &genesis.hash.0, genesis.block.write_to_bytes().unwrap());
    batch.write(db::COL_BLOCKS, &ConstKey::CurrentHash, &hash);
    batch.write(db::COL_BLOCKS, &ConstKey::CurrentHeight, &height);
    batch.write(db::COL_EXTRA, &height, &hash);
    db.write(batch)
}

/// Get latest status
pub fn get_chain(db: &KeyValueDB) -> Option<(H256, u64)> {
    let current_hash = db.read(db::COL_BLOCKS, &ConstKey::CurrentHash);
    if let Some(hash) = current_hash {
        let current_height = db.read(db::COL_BLOCKS, &ConstKey::CurrentHeight);
        if let Some(height) = current_height {
            Some((hash, height))
        } else {
            warn!("not expected get_chain.");
            None
        }
    } else {
        None
    }
}

impl Chain {
    fn save_status(&self, batch: &mut DBTransaction) -> Status {
        let current_height = self.current_height.load(Ordering::SeqCst) as BlockNumber;
        let current_hash = *self.current_hash.read();

        batch.write(db::COL_BLOCKS, &ConstKey::CurrentHash, &current_hash);
        batch.write(db::COL_BLOCKS, &ConstKey::CurrentHeight, &current_height);

        //return status
        let mut status = Status::new();
        status.set_hash(current_hash.to_vec());
        status.set_height(current_height);
        status
    }

    pub fn init_chain(db: Arc<KeyValueDB>, mut genesis: Genesis, sync_sender: Sender<u64>) -> (Arc<Chain>, Status) {
        // 400 is the avarage size of the key
        let cache_man = CacheManager::new(1 << 14, 1 << 20, 400);

        let trie_factory = TrieFactory::new(TrieSpec::Generic);
        let factories = Factories {
            vm: EvmFactory::default(),
            trie: trie_factory,
            accountdb: Default::default(),
        };

        let journal_db = journaldb::new(db.clone(), Default::default(), COL_STATE);
        let state_db = StateDB::new(journal_db);
        let blooms_config = bc::Config {
            levels: LOG_BLOOMS_LEVELS,
            elements_per_index: LOG_BLOOMS_ELEMENTS_PER_INDEX,
        };

        if let Some((hash, height)) = get_chain(&*db) {
            let mut status = Status::new();
            status.set_hash(hash.0.to_vec());
            status.set_height(height);
            let chain = Arc::new(Chain {
                                     blooms_config: blooms_config,
                                     current_hash: RwLock::new(hash),
                                     current_height: AtomicUsize::new(height as usize),
                                     is_sync: AtomicBool::new(false),
                                     max_height: AtomicUsize::new(0),
                                     block_map: RwLock::new(BTreeMap::new()),
                                     blocks: RwLock::new(HashMap::new()),
                                     block_hashes: RwLock::new(HashMap::new()),
                                     transaction_addresses: RwLock::new(HashMap::new()),
                                     blocks_blooms: RwLock::new(HashMap::new()),
                                     block_receipts: RwLock::new(HashMap::new()),
                                     cache_man: Mutex::new(cache_man),
                                     db: db,
                                     state_db: state_db,
                                     factories: factories,
                                     sync_sender: Mutex::new(sync_sender),
                                     last_hashes: RwLock::new(VecDeque::new()),
                                     polls_filter: Arc::new(Mutex::new(PollManager::new())),
                                 });

            chain.build_last_hashes(Some(hash.into()), height);
            (chain, status)
        } else {
            let _ = genesis.lazy_execute();
            save_genesis(&*db, &genesis).expect("Failed to save genesis.");
            info!("init genesis {:?}", genesis);
            let mut status = Status::new();
            let genesis_hash: H256 = genesis.hash.into();
            status.set_hash(genesis_hash.to_vec());
            status.set_height(0);

            let chain = Arc::new(Chain {
                                     blooms_config: blooms_config,
                                     current_hash: RwLock::new(genesis_hash),
                                     current_height: AtomicUsize::new(0),
                                     is_sync: AtomicBool::new(false),
                                     max_height: AtomicUsize::new(0),
                                     block_map: RwLock::new(BTreeMap::new()),
                                     blocks: RwLock::new(HashMap::new()),
                                     block_hashes: RwLock::new(HashMap::new()),
                                     transaction_addresses: RwLock::new(HashMap::new()),
                                     blocks_blooms: RwLock::new(HashMap::new()),
                                     block_receipts: RwLock::new(HashMap::new()),
                                     cache_man: Mutex::new(cache_man),
                                     db: db,
                                     state_db: state_db,
                                     factories: factories,
                                     sync_sender: Mutex::new(sync_sender),
                                     last_hashes: RwLock::new(VecDeque::new()),
                                     polls_filter: Arc::new(Mutex::new(PollManager::new())),
                                 });
            chain.build_last_hashes(Some(genesis.hash.into()), 0);
            (chain, status)
        }
    }

    /// Get raw block by height
    fn block_by_height(&self, number: BlockNumber) -> Option<Block> {
        let result = self.block_hash(number);
        trace!("block_by_height by number {:?}, hash is {:?}", number, result);
        match result {
            Some(h) => self.block_by_hash(&h),
            None => None,
        }
    }

    // Get block hash by height
    pub fn block_hash(&self, index: BlockNumber) -> Option<H256> {
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.block_hashes, &index);
        self.cache_man.lock().note_used(CacheId::BlockHashes(index));
        result
    }

    /// Get raw block by hash
    fn block_by_hash(&self, hash: &H256) -> Option<Block> {
        trace!("block_by_hash {}", hash);
        // Check cache first
        {
            let read = self.blocks.read();
            if let Some(v) = read.get(hash) {
                trace!("block_by_hash from cache.");
                return Some(v.clone());
            }
        }

        // Read from DB and populate cache
        let opt = self.db.get(db::COL_BLOCKS, hash).expect("Low level database error. Some issue with disk?");

        let result = match opt {
            Some(b) => {
                let block = parse_from_bytes::<Block>(b.to_vec().as_slice()).unwrap();
                let mut write = self.blocks.write();
                write.insert(hash.clone(), block.clone());
                Some(block)
            }
            None => None,
        };

        self.cache_man.lock().note_used(CacheId::Blocks(hash.clone()));
        result

    }

    /// Get block by BlockId
    pub fn block(&self, id: BlockId) -> Option<Block> {
        match id {
            BlockId::Hash(hash) => self.block_by_hash(&hash),
            BlockId::Number(number) => self.block_by_height(number),
            BlockId::Earliest => self.block_by_height(0),
            BlockId::Latest => self.block_by_height(self.get_current_height()),
        }
    }

    /// Get transaction by hash
    pub fn transaction(&self, hash: TransactionId) -> Option<FullTransaction> {
        match self.transaction_address(&hash) {
            Some(transaction_address) => {
                let index = transaction_address.index;
                let block_hash = transaction_address.block_hash;
                self.block(BlockId::Hash(block_hash)).map(|mut blk| {
                    let transactions = blk.mut_body().take_transactions();
                    let tx = transactions.into_iter().as_slice()[index as usize].clone();
                    let mut full_ts = FullTransaction::new();
                    full_ts.set_transaction(tx);
                    full_ts.set_block_number(blk.mut_header().height);
                    full_ts.set_block_hash(block_hash.to_vec());
                    full_ts.set_index(index as u32);
                    full_ts
                })
            }
            None => None,
        }
    }

    pub fn localized_receipt(&self, id: TransactionId, tx_address: TransactionAddress) -> Option<LocalizedReceipt> {
        trace!("Get receipt id: {:?}, tx_address: {:?}", id, tx_address);
        let mut previous_receipts = (0..tx_address.index + 1)
            .map(|index| {
                     let mut address = tx_address.clone();
                     address.index = index;
                     self.transaction_receipt(&address)
                 })
            .collect::<Vec<Option<Receipt>>>();

        let last_receipt = previous_receipts.pop().expect("Current receipt is provided; qed");
        let prior_gas_used = match tx_address.index {
            0 => 0.into(),
            i => {
                previous_receipts.clone()
                                 .into_iter()
                                 .nth(i - 1)
                                 .map_or(0.into(), |receipt| receipt.map_or(0.into(), |r| r.gas_used))
            }
        };
        let no_of_logs = previous_receipts.into_iter().map(|receipt| receipt.map_or(0, |r| r.logs.len())).sum::<usize>();

        last_receipt.and_then(|last_receipt| {
            // Get sender
            let mut full_transaction = self.transaction(id).unwrap();
            let mut signed_tx = full_transaction.take_transaction();
            let raw_tx = signed_tx.mut_transaction_with_sig().take_transaction();
            let block_hash = tx_address.block_hash;
            let block_number = self.block_number(BlockId::Hash(block_hash.clone())).unwrap_or(0);

            let public = H512::from_slice(signed_tx.get_signer());
            let sender = pubkey_to_address(&public);
            info!("sender is {:?}", sender);
            let contract_address = match raw_tx.get_to().is_empty() {
                false => None,
                true => Some(contract_address(&sender, &raw_tx.nonce.parse::<U256>().unwrap_or_default())),
            };

            let receipt = LocalizedReceipt {
                transaction_hash: id,
                transaction_index: tx_address.index,
                block_hash: block_hash,
                block_number: block_number,
                cumulative_gas_used: last_receipt.gas_used,
                gas_used: last_receipt.gas_used - prior_gas_used,
                contract_address: contract_address,
                logs: last_receipt.logs
                                  .into_iter()
                                  .enumerate()
                                  .map(|(i, log)| {
                    LocalizedLogEntry {
                        entry: log,
                        block_hash: block_hash,
                        block_number: block_number,
                        transaction_hash: id,
                        transaction_index: tx_address.index,
                        transaction_log_index: i,
                        log_index: no_of_logs + i,
                    }
                })
                                  .collect(),
                log_bloom: last_receipt.log_bloom,
                state_root: last_receipt.state_root,
            };
            Some(receipt)
        })
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_height.load(Ordering::SeqCst) as u64
    }

    pub fn get_max_height(&self) -> u64 {
        self.max_height.load(Ordering::SeqCst) as u64
    }

    pub fn validate_hash(&self, block_hash: &[u8]) -> bool {
        let current_hash = *self.current_hash.read();
        let block_hash = H256::from_slice(block_hash);
        trace!("validate_hash current_hash {:?} block_hash {:?}", current_hash, block_hash);
        current_hash == block_hash
    }

    pub fn validate_height(&self, block_number: u64) -> bool {
        let current_height = self.current_height.load(Ordering::SeqCst) as u64;
        trace!("validate_height current_height {:?} block_number {:?}", current_height, block_number - 1);
        current_height == block_number - 1
    }

    /// Execute block in vm
    fn execute_block(&self, block: Block) -> OpenBlock {
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let mut open_block = OpenBlock::new(self.factories.clone(), false, block, self.state_db.boxed_clone(), current_state_root, last_hashes.into()).unwrap();
        open_block.apply_transactions();

        open_block
    }

    fn last_hashes(&self) -> LastHashes {
        LastHashes::from(self.last_hashes.read().clone())
    }

    fn block_body(&self, hash: &H256) -> Option<BlockBody> {
        self.block(BlockId::Hash(*hash)).map_or(None, |mut v| Some(v.take_body()))
    }

    pub fn logs<F>(&self, mut blocks: Vec<BlockNumber>, matches: F, limit: Option<usize>) -> Vec<LocalizedLogEntry>
    where
        F: Fn(&LogEntry) -> bool,
        Self: Sized,
    {
        // sort in reverse order
        blocks.sort_by(|a, b| b.cmp(a));

        let mut log_index = 0;
        let mut logs = blocks.into_iter()
                             .filter_map(|number| self.block_hash(number).map(|hash| (number, hash)))
                             .filter_map(|(number, hash)| self.block_receipts(&hash).map(|r| (number, hash, r.receipts)))
                             .filter_map(|(number, hash, receipts)| self.block_body(&hash).map(|ref b| (number, hash, receipts, b.transaction_hashes())))
                             .flat_map(|(number, hash, mut receipts, mut hashes)| {
            if receipts.len() != hashes.len() {
                warn!("Block {} ({}) has different number of receipts ({}) to transactions ({}). Database corrupt?", number, hash, receipts.len(), hashes.len());
                assert!(false);
            }
            log_index = receipts.iter().fold(0, |sum, receipt| sum + receipt.clone().map_or(0, |r| r.logs.len()));

            let receipts_len = receipts.len();
            hashes.reverse();
            receipts.reverse();
            receipts.into_iter()
                    .filter_map(|receipt| receipt.map(|r| r.logs))
                    .zip(hashes)
                    .enumerate()
                    .flat_map(move |(index, (mut logs, tx_hash))| {
                let current_log_index = log_index;
                let no_of_logs = logs.len();
                log_index -= no_of_logs;

                logs.reverse();
                logs.into_iter().enumerate().map(move |(i, log)| {
                    LocalizedLogEntry {
                        entry: log,
                        block_hash: hash,
                        block_number: number,
                        transaction_hash: tx_hash,
                        // iterating in reverse order
                        transaction_index: receipts_len - index - 1,
                        transaction_log_index: no_of_logs - i - 1,
                        log_index: current_log_index - i - 1,
                    }
                })
            })
        })
                             .filter(|log_entry| matches(&log_entry.entry))
                             .take(limit.unwrap_or(::std::usize::MAX))
                             .collect::<Vec<LocalizedLogEntry>>();
        logs.reverse();
        logs
    }

    /// Get the number of given block's hash.
    pub fn get_block_number(&self, hash: &H256) -> Option<BlockNumber> {
        self.block(BlockId::Hash(*hash)).map_or(None, |v| Some(v.get_header().get_height()))
    }

    /// Returns numbers of blocks containing given bloom.
    pub fn blocks_with_bloom(&self, bloom: &H2048, from_block: BlockNumber, to_block: BlockNumber) -> Vec<BlockNumber> {
        let range = from_block as bc::Number..to_block as bc::Number;
        let chain = bc::group::BloomGroupChain::new(self.blooms_config, self);
        chain.with_bloom(&range, &Bloom::from(bloom.clone()).into())
             .into_iter()
             .map(|b| b as BlockNumber)
             .collect()
    }

    fn block_number(&self, id: BlockId) -> Option<BlockNumber> {
        match id {
            BlockId::Number(number) => Some(number),
            BlockId::Hash(ref hash) => self.get_block_number(hash),
            BlockId::Earliest => Some(0),
            BlockId::Latest => Some(self.get_current_height()),
        }
    }

    /// Returns numbers of blocks containing given bloom by blockId.
    pub fn blocks_with_bloom_by_id(&self, bloom: &H2048, from_block: BlockId, to_block: BlockId) -> Option<Vec<BlockNumber>> {
        match (self.block_number(from_block), self.block_number(to_block)) {
            (Some(from), Some(to)) => Some(self.blocks_with_bloom(bloom, from, to)),
            _ => None,
        }
    }

    pub fn get_logs(&self, filter: Filter) -> Vec<LocalizedLogEntry> {
        let blocks = filter.bloom_possibilities().iter()
            .filter_map(|bloom| self.blocks_with_bloom_by_id(bloom, filter.from_block.clone(), filter.to_block.clone()))
            .flat_map(|m| m)
            // remove duplicate elements
            .collect::<HashSet<u64>>()
            .into_iter()
            .collect::<Vec<u64>>();

        self.logs(blocks, |entry| filter.matches(entry), filter.limit)
    }

    /// Build last 256 hashes.
    fn build_last_hashes(&self, prevhash: Option<H256>, parent_height: u64) -> Arc<LastHashes> {
        let parent_hash = prevhash.unwrap_or_else(|| self.block_hash(parent_height).expect("Block height always valid."));
        {
            let hashes = self.last_hashes.read();
            if hashes.front().map_or(false, |h| h == &parent_hash) {
                let mut res = Vec::from(hashes.clone());
                res.resize(256, H256::default());
                return Arc::new(res);
            }
        }
        let mut last_hashes = LastHashes::new();
        last_hashes.resize(256, H256::default());
        last_hashes[0] = parent_hash;
        for i in 0..255 {
            if parent_height < i + 1 {
                break;
            };
            let height = parent_height - i - 1;
            match self.block_hash(height) {
                Some(hash) => {
                    let index = (i + 1) as usize;
                    last_hashes[index] = hash.clone();
                }
                None => break,
            }
        }
        let mut cached_hashes = self.last_hashes.write();
        *cached_hashes = VecDeque::from(last_hashes.clone());
        Arc::new(last_hashes)
    }

    fn update_last_hashes(&self, hash: &H256) {
        let mut hashes = self.last_hashes.write();
        if hashes.len() > 255 {
            hashes.pop_back();
        }
        hashes.push_front(hash.clone());
    }

    fn current_state_root(&self) -> H256 {
        info!("current_hash: {:?}", self.current_hash.read().clone());
        let block = self.block_by_hash(&self.current_hash.read()).expect("Current hash always stores in db.");
        let header = block.get_header();
        H256::from(header.get_state_root())
    }

    /// Commit block in db, including:
    /// 1. Block including transactions
    /// 2. TransactionAddress
    /// 3. State
    /// 3. Receipts
    /// 4. Bloom
    //TODO: Separate commit and insert block
    pub fn commit_block(&self, batch: &mut DBTransaction, block: ClosedBlock) {
        let height = block.get_header().get_height();
        let hash = block.hash.clone();
        let block_data = block.write_to_bytes().unwrap();

        trace!("commit block in db {:?}, {:?}", hash, height);

        // blocks blooms
        let log_bloom = block.receipts.clone().into_iter().filter_map(|r| r).fold(LogBloom::zero(), |mut b, r| {
            b = &b | &r.log_bloom;
            b
        });
        let blocks_blooms: HashMap<LogGroupPosition, BloomGroup> = match log_bloom.is_zero() {
            true => HashMap::new(),
            false => {
                let chain = bc::group::BloomGroupChain::new(self.blooms_config, self);
                chain.insert(height as bc::Number, Bloom::from(log_bloom).into())
                     .into_iter()
                     .map(|p| (From::from(p.0), From::from(p.1)))
                     .collect()
            }
        };

        let block_receipts = BlockReceipts::new(block.receipts.clone());

        {
            let mut write_receipts = self.block_receipts.write();
            let mut write_blooms = self.blocks_blooms.write();
            let mut write_hashes = self.block_hashes.write();
            let mut write_txs = self.transaction_addresses.write();

            batch.write_with_cache(db::COL_EXTRA, &mut *write_hashes, height as BlockNumber, hash, CacheUpdatePolicy::Overwrite);
            batch.write_with_cache(db::COL_EXTRA, &mut *write_receipts, hash, block_receipts, CacheUpdatePolicy::Overwrite);
            batch.extend_with_cache(db::COL_EXTRA, &mut *write_blooms, blocks_blooms.clone(), CacheUpdatePolicy::Overwrite);
            batch.extend_with_cache_append(db::COL_EXTRA, &*self.db, &mut *write_txs, block.transactions_uni.clone(), AppendPolicy::Overwrite);
            batch.extend_with_cache_append(db::COL_EXTRA, &*self.db, &mut *write_txs, block.transactions_dup.clone(), AppendPolicy::Update);

        }

        // Store block in db
        batch.put_vec(db::COL_BLOCKS, &hash, block_data);

        //note used
        self.cache_man.lock().note_used(CacheId::BlockHashes(height as BlockNumber));
        self.cache_man.lock().note_used(CacheId::BlockReceipts(hash));

        for (key, _) in blocks_blooms {
            self.cache_man.lock().note_used(CacheId::BlocksBlooms(key));
        }

        for (key, _) in block.transactions_uni.clone() {
            self.cache_man.lock().note_used(CacheId::TransactionAddresses(key));
        }

        for (key, _) in block.transactions_dup.clone() {
            self.cache_man.lock().note_used(CacheId::TransactionAddresses(key));
        }


        let mut state = block.drain();
        // Store triedb changes in journal db
        state.journal_under(batch, height, &hash).expect("DB commit failed");

    }

    /// Get the address of transaction with given hash.
    pub fn transaction_address(&self, hash: &TransactionId) -> Option<TransactionAddress> {
        let result = self.db.read_list_with_cache(db::COL_EXTRA, &self.transaction_addresses, hash).map(|v| v[0].clone());
        self.cache_man.lock().note_used(CacheId::TransactionAddresses(hash.clone()));
        result
    }

    // TODO: cache it after transact
    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: &H256) -> Option<BlockReceipts> {
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.block_receipts, hash);
        self.cache_man.lock().note_used(CacheId::BlockReceipts(hash.clone()));
        result
    }

    pub fn cita_call(&self, request: CallRequest, id: BlockId) -> Result<Bytes, String> {
        let signed = self.sign_call(request);
        let result = self.call(&signed, id, Default::default());
        result.map(|b| b.output.into()).or_else(|_| Err(String::from("Call Error")))
    }

    fn sign_call(&self, request: CallRequest) -> SignedTransaction {
        let from = request.from.unwrap_or(Address::zero());
        VMTransaction {
            nonce: U256::zero(),
            action: Action::Call(request.to),
            gas: U256::from(50_000_000),
            gas_price: U256::zero(),
            value: U256::zero(),
            data: request.data.map_or_else(Vec::new, |d| d.to_vec()),
            hash: H256::default(),
        }
        .fake_sign(from)
    }

    /// Attempt to get a copy of a specific block's final state.
    pub fn state_at(&self, id: BlockId) -> Option<State<StateDB>> {
        self.block(id).and_then(|block| {
                                    let db = self.state_db.boxed_clone();
                                    let header = block.get_header();
                                    let root = header.get_state_root();
                                    State::from_existing(db, H256::from_slice(root), U256::from(0), self.factories.clone()).ok()
                                })
    }

    // TODO: cache state_root
    /// Get a copy of the best block's state.
    pub fn state(&self) -> State<StateDB> {
        let current_hash = *self.current_hash.read();
        let block_id = BlockId::Hash(current_hash);
        self.state_at(block_id).expect("State root of current block always valid.")
    }

    //get account
    pub fn code_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id).and_then(|s| s.code(address).ok()).map(|c| c.map(|c| (&*c).clone()))
    }

    //account  transaction count
    pub fn nonce(&self, address: &Address, id: BlockId) -> Option<U256> {
        self.state_at(id).and_then(|s| s.nonce(address).ok())
    }

    fn call(&self, t: &SignedTransaction, block_id: BlockId, analytics: CallAnalytics) -> Result<Executed, CallError> {
        let block = self.block(block_id).ok_or(CallError::StatePruned)?;
        let header = block.get_header();
        let last_hashes = self.build_last_hashes(None, header.get_height());
        let env_info = EnvInfo {
            number: header.get_height(),
            author: Address::default(),
            timestamp: header.get_timestamp(),
            difficulty: U256::default(),
            last_hashes: last_hashes,
            gas_used: U256::from(header.get_gas_used()),
            gas_limit: U256::max_value(),
        };
        // that's just a copy of the state.
        let mut state = self.state_at(block_id).ok_or(CallError::StatePruned)?;
        let engine = NullEngine::default();

        let options = TransactOptions {
            tracing: analytics.transaction_tracing,
            vm_tracing: analytics.vm_tracing,
            check_nonce: false,
        };
        let ret = Executive::new(&mut state, &env_info, &engine, &self.factories.vm).transact(t, options)?;

        Ok(ret)
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, address: &TransactionAddress) -> Option<Receipt> {
        self.block_receipts(&address.block_hash)
            .and_then(|br| br.receipts.into_iter().nth(address.index))
            .and_then(|option_receipt| option_receipt)
    }

    /// Add block to chain:
    /// 1. Execute block
    /// 2. Commit block
    /// 3. Update cache
    // TODO: Commit plain block type in db
    // TODO: move proof check to sync module
    pub fn add_block(&self, batch: &mut DBTransaction, blk: Block) -> Option<H256> {
        let height = blk.get_header().get_height() as usize;
        let mut proof_ok = true;
        if blk.get_header().has_proof() {
            let proof = blk.get_header().get_proof().clone();
            if proof.get_field_type() == ProofType::Tendermint {
                let proof = TendermintProof::from(proof);
                proof_ok = proof.simple_check(height - 1);
            }
        }

        if self.validate_hash(blk.get_header().get_prevhash()) && proof_ok {
            let open_block = self.execute_block(blk.clone());
            let closed_block = open_block.close();
            self.commit_block(batch, closed_block.clone());
            self.update_last_hashes(&closed_block.hash);
            Some(closed_block.hash)
        } else {
            None
        }
    }

    pub fn set_block(&self, block: Block) -> Option<Vec<u8>> {
        let blk_height = block.get_header().get_height();
        trace!("set_block height = {:?}, hash = {:?}", blk_height, block.crypt_hash());
        if self.validate_height(blk_height) {
            let mut batch = self.db.transaction();
            if let Some(current_hash) = self.add_block(&mut batch, block) {
                trace!("set_block current_hash!!!!!!{:?} {:?}", blk_height, H256::from(current_hash));
                {
                    *self.current_hash.write() = current_hash;
                }
                self.current_height.fetch_add(1, Ordering::SeqCst);

                let status = self.save_status(&mut batch);

                self.db.write(batch).expect("DB write failed.");

                info!("chain update {:?}", blk_height);
                Some(status.write_to_bytes().unwrap())
            } else {
                warn!("add block failed");
                None
            }
        } else {
            None
        }
    }

    pub fn compare_status(&self, st: Status) -> (u64, u64) {
        let current_height = self.current_height.load(Ordering::SeqCst) as u64;
        if st.get_height() > current_height {
            (current_height + 1, st.get_height() - current_height)
        } else {
            (0, 0)
        }
    }

    /// Get current cache size.
    pub fn cache_size(&self) -> CacheSize {
        CacheSize {
            transaction_addresses: self.transaction_addresses.read().heap_size_of_children(),
            blocks_blooms: self.blocks_blooms.read().heap_size_of_children(),
            block_receipts: self.block_receipts.read().heap_size_of_children(),
        }
    }

    /// Ticks our cache system and throws out any old data.
    pub fn collect_garbage(&self) {
        let current_size = self.cache_size().total();

        let mut blocks = self.blocks.write();
        let mut block_hashes = self.block_hashes.write();
        let mut transaction_addresses = self.transaction_addresses.write();
        let mut blocks_blooms = self.blocks_blooms.write();
        let mut block_receipts = self.block_receipts.write();

        let mut cache_man = self.cache_man.lock();
        cache_man.collect_garbage(current_size, |ids| {
            for id in &ids {
                match *id {
                    CacheId::Blocks(ref h) => {
                        blocks.remove(h);
                    }
                    CacheId::BlockHashes(ref h) => {
                        block_hashes.remove(h);
                    }
                    CacheId::TransactionAddresses(ref h) => {
                        transaction_addresses.remove(h);
                    }
                    CacheId::BlocksBlooms(ref h) => {
                        blocks_blooms.remove(h);
                    }
                    CacheId::BlockReceipts(ref h) => {
                        block_receipts.remove(h);
                    }
                }
            }

            blocks.shrink_to_fit();
            block_hashes.shrink_to_fit();
            transaction_addresses.shrink_to_fit();
            blocks_blooms.shrink_to_fit();
            block_receipts.shrink_to_fit();

            block_hashes.heap_size_of_children() + transaction_addresses.heap_size_of_children() + blocks_blooms.heap_size_of_children() + block_receipts.heap_size_of_children()
        });
    }

    pub fn poll_filter(&self) -> Arc<Mutex<PollManager<PollFilter>>> {
        self.polls_filter.clone()
    }
}

#[cfg(test)]
mod tests {
    extern crate cita_crypto;
    extern crate env_logger;
    extern crate mktemp;
    extern crate test;
    use self::Chain;
    use self::cita_crypto::*;
    use super::*;
    use db;
    use libchain::block::{Block, BlockHeader, BlockBody};
    use libchain::genesis::{Spec, Admin};
    use libproto::blockchain;
    use protobuf::RepeatedField;
    use rustc_serialize::hex::FromHex;
    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use std::time::{UNIX_EPOCH, Instant};
    use test::{Bencher, black_box};
    use util::{U256, H256, Address};
    use util::kvdb::{Database, DatabaseConfig};

    #[test]
    fn test_heapsizeof() {
        let test: Vec<String> = Vec::new();
        assert_eq!(test.heap_size_of_children(), 0);
    }
    #[test]
    fn test_cache_size() {
        let transaction_addresses: HashMap<TransactionId, TransactionAddress> = HashMap::new();
        let blocks_blooms: HashMap<LogGroupPosition, BloomGroup> = HashMap::new();
        let mut block_receipts: HashMap<H256, BlockReceipts> = HashMap::new();

        assert_eq!(transaction_addresses.heap_size_of_children(), 0);
        assert_eq!(blocks_blooms.heap_size_of_children(), 0);
        assert_eq!(block_receipts.heap_size_of_children(), 0);

        block_receipts.insert(H256::from("000000000000000000000000000000000000000000000000123456789abcdef0"), BlockReceipts::new(vec![]));
        assert_eq!(block_receipts.heap_size_of_children(), 1856);

    }

    fn init_chain() -> (Arc<Chain>, PrivKey) {
        let _ = env_logger::init();
        let privkey = PrivKey::from(H256::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0"));
        let keypair = KeyPair::from_privkey(privkey).unwrap();
        let pubkey = keypair.pubkey();
        let tempdir = mktemp::Temp::new_dir().unwrap().to_path_buf();
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
        let genesis = Genesis {
            spec: Spec {
                prevhash: H256::from(0),
                timestamp: 0,
                admin: Admin {
                    pubkey: *pubkey,
                    crypto: privkey.hex(),
                    identifier: String::from(""),
                },
            },
            block: Block::default(),
            hash: H256::default(),
        };
        let (sync_tx, _) = channel();
        let (chain, _) = Chain::init_chain(Arc::new(db), genesis, sync_tx);
        (chain, privkey)
    }

    fn create_block(chain: &Chain, privkey: PrivKey, to: Address, data: Vec<u8>, nonce: (u32, u32)) -> Block {
        let mut block = Block::new();

        let mut header = BlockHeader::new();
        header.set_prevhash(chain.current_hash.read().to_vec());
        header.timestamp = UNIX_EPOCH.elapsed().unwrap().as_secs();
        header.height = chain.get_current_height() + 1;
        // header.proof= ?;
        // header.commit= ?;


        block.set_header(header);
        let mut body = BlockBody::new();
        let mut txs = Vec::new();
        for i in nonce.0..nonce.1 {
            // 1) tx = (to, data(code), nonce)
            let mut tx = blockchain::Transaction::new();
            if to == Address::from(0) {
                tx.set_to(String::from(""));
            } else {
                tx.set_to(to.hex());
            }
            tx.set_nonce(U256::from(i).to_hex());
            tx.set_data(data.clone());
            tx.set_valid_until_block(0);

            let mut uv_tx = blockchain::UnverifiedTransaction::new();
            uv_tx.set_transaction(tx);

            // 2) stx = (from, content(code, nonce, signature))
            let mut stx = blockchain::SignedTransaction::new();
            stx.set_transaction_with_sig(uv_tx);
            stx.sign(privkey);

            txs.push(stx);
        }
        body.set_transactions(RepeatedField::from_vec(txs));
        block.set_body(body);
        block
    }

    #[bench]
    fn bench_execute_block(b: &mut Bencher) {
        let (chain, privkey) = init_chain();
        let data = "60606040523415600b57fe5b5b5b5b608e8061001c6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b605560048080359060200190919050506057565b005b806000819055505b505600a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e371628798c41e292ca0029"
            .from_hex()
            .unwrap();

        let block = create_block(&chain, privkey, Address::from(0), data, (0, 1));
        chain.set_block(block.clone());

        let txhash = H256::from_slice(block.get_body().get_transactions()[0].get_tx_hash());
        let receipt = chain.transaction_address(&txhash)
                           .and_then(|tx_address| chain.localized_receipt(txhash, tx_address))
                           .unwrap();
        let to = receipt.contract_address.unwrap();
        let data = format!("{}{}", "55241077", "0000000000000000000000000000000000000000000000000000000012345678")
            .from_hex()
            .unwrap();
        println!();
        let bench = |tpb: u32| {
            let start = Instant::now();
            let block = create_block(&chain, privkey, to, data.clone(), (1, tpb + 1));
            black_box(chain.execute_block(block));
            let elapsed = start.elapsed();
            let tps = u64::from(tpb) * 1_000_000_000 / (elapsed.as_secs() * 1_000_000_000 + u64::from(elapsed.subsec_nanos()));
            println!("tpb: {:>6}, tps: {:>6}", tpb, tps);
        };
        bench(3000);
        bench(5000);
        bench(10000);
        bench(20000);
        b.iter(|| {});
    }

    #[test]
    fn test_contract() {
        let (chain, privkey) = init_chain();
        /*
            pragma solidity ^0.4.8;
            contract ConstructSol {
                uint a;
                event LogCreate(address contractAddr);
                event A(uint);
                function ConstructSol(){
                    LogCreate(this);
                }

                function set(uint _a) {
                    a = _a;
                    A(a);
                }

                function get() returns (uint) {
                    return a;
                }
            }
		*/
        let data = "6060604052341561000f57600080fd5b5b7fb8f132fb6526e0405f3ce4f3bab301f1d4409b1e7f2c01c2037d6cf845c831cb30604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a15b5b610107806100846000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b11460475780636d4ce63c146067575b600080fd5b3415605157600080fd5b60656004808035906020019091905050608d565b005b3415607157600080fd5b607760d1565b6040518082815260200191505060405180910390f35b806000819055507fa17a9e66f0c355e3aa3b9ea969991204d6b1d2e62a47877f612cb2371d79e06a6000546040518082815260200191505060405180910390a15b50565b6000805490505b905600a165627a7a72305820bb7224faec63935671f0b4722064773ccae237bec4f6fbb252c362f2192dca900029"
            .from_hex()
            .unwrap();

        println!("data: {:?}", data);

        let block = create_block(&chain, privkey, Address::from(0), data, (0, 1));
        chain.set_block(block.clone());

        let txhash = H256::from_slice(block.get_body().get_transactions()[0].get_tx_hash());
        let receipt = chain.transaction_address(&txhash)
                           .and_then(|tx_address| chain.localized_receipt(txhash, tx_address))
                           .unwrap();

        println!("{:?}", receipt);
        let contract_address = receipt.contract_address.unwrap();
        println!("contract address: {}", contract_address);
        let log = &receipt.logs[0];
        assert_eq!(contract_address, log.address);
        println!("contract_address as slice {:?}", contract_address.to_vec().as_slice());
        assert!(log.data.as_slice().ends_with(contract_address.to_vec().as_slice()));
        let code = chain.code_at(&contract_address, BlockId::Latest);
        assert!(code.is_some());
        assert!(code.unwrap().is_some());

        // set a=10
        let data = "60fe47b1000000000000000000000000000000000000000000000000000000000000000a".from_hex().unwrap();
        let block = create_block(&chain, privkey, contract_address, data, (1, 2));
        chain.set_block(block.clone());
        let txhash = H256::from_slice(block.get_body().get_transactions()[0].get_tx_hash());
        let receipt = chain.transaction_address(&txhash)
                           .and_then(|tx_address| chain.localized_receipt(txhash, tx_address))
                           .unwrap();
        println!("{:?}", receipt);

        // get a is 10
        let data = "6d4ce63c".from_hex().unwrap();
        let call_request = CallRequest {
            from: None,
            to: contract_address,
            data: Some(data.into()),
        };
        let call_result = chain.cita_call(call_request, BlockId::Latest);
        assert_eq!(call_result, Ok(Bytes::from(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10])));
        println!("call_result: {:?}", call_result);
    }
}
