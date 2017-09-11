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

use bloomchain as bc;
use blooms::*;
pub use byteorder::{BigEndian, ByteOrder};
use cache_manager::CacheManager;
use call_analytics::CallAnalytics;
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
use libproto::blockchain::{ProofType, Status as ProtoStatus};
use libproto::request::FullTransaction;
use proof::TendermintProof;
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
use types::transaction::{SignedTransaction, Transaction, Action};
use util::{journaldb, H256, U256, H2048, Address, Bytes};
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
    BlockHeaders(H256),
    BlockBodies(H256),
    BlockHashes(BlockNumber),
    TransactionAddresses(H256),
    BlocksBlooms(LogGroupPosition),
    BlockReceipts(H256),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Status {
    number: u64,
    hash: H256,
}

impl Status {
    fn new() -> Status {
        Status { number: 0, hash: H256::default() }
    }

    fn hash(&self) -> &H256 {
        &self.hash
    }

    fn number(&self) -> u64 {
        self.number
    }

    fn set_hash(&mut self, h: H256) {
        self.hash = h;
    }

    fn set_number(&mut self, n: u64) {
        self.number = n;
    }

    fn protobuf(&self) -> ProtoStatus {
        let mut ps = ProtoStatus::new();
        ps.set_height(self.number());
        ps.set_hash(self.hash().to_vec());
        ps
    }
}

impl bc::group::BloomGroupDatabase for Chain {
    fn blooms_at(&self, position: &bc::group::GroupPosition) -> Option<bc::group::BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.blocks_blooms, &position).map(Into::into);
        self.cache_man.lock().note_used(CacheId::BlocksBlooms(position));
        result
    }
}

pub trait TransactionHash {
    fn transaction_hashes(&self) -> Vec<H256>;
}

// TODO: chain对外开放的方法，是保证能正确解析结构，即类似于Result<Block,Err>
// 所有直接unwrap的地方都可能会报错！
pub struct Chain {
    blooms_config: bc::Config,
    pub current_header: RwLock<Header>,
    pub is_sync: AtomicBool,
    pub max_height: AtomicUsize,
    pub block_map: RwLock<BTreeMap<u64, (BlockSource, Block, bool)>>,
    pub db: Arc<KeyValueDB>,
    pub sync_sender: Mutex<Sender<u64>>,
    pub state_db: StateDB,
    pub factories: Factories,
    // Hash of the given block - only works for 256 most recent blocks excluding current
    pub last_hashes: RwLock<VecDeque<H256>>,

    // block cache
    block_headers: RwLock<HashMap<H256, Header>>,
    block_bodies: RwLock<HashMap<H256, BlockBody>>,

    // extra caches
    block_hashes: RwLock<HashMap<BlockNumber, H256>>,
    transaction_addresses: RwLock<HashMap<TransactionId, DBList<TransactionAddress>>>,
    blocks_blooms: RwLock<HashMap<LogGroupPosition, BloomGroup>>,
    block_receipts: RwLock<HashMap<H256, BlockReceipts>>,

    cache_man: Mutex<CacheManager<CacheId>>,
    polls_filter: Arc<Mutex<PollManager<PollFilter>>>,
}

/// Get latest status
pub fn get_chain(db: &KeyValueDB) -> Option<Header> {
    let h: Option<H256> = db.read(db::COL_EXTRA, &ConstKey::CurrentHash);
    if let Some(hash) = h {
        db.read(db::COL_HEADERS, &hash)
    } else {
        warn!("not expected get_chain.");
        None
    }
}

impl Chain {
    fn save_status(&self, batch: &mut DBTransaction) -> Status {
        let current_height = self.get_current_height();
        let current_hash = self.get_current_hash();

        batch.write(db::COL_EXTRA, &ConstKey::CurrentHash, &current_hash);
        //return status
        let mut status = Status::new();
        status.set_hash(current_hash);
        status.set_number(current_height);
        status
    }

    pub fn init_chain(db: Arc<KeyValueDB>, mut genesis: Genesis, sync_sender: Sender<u64>) -> (Arc<Chain>, ProtoStatus) {
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

        let header = match get_chain(&*db) {
            Some(header) => {
                header
            }
            _ => {
                genesis.lazy_execute(&state_db, &factories).expect("Failed to save genesis.");
                info!("init genesis {:?}", genesis);
                genesis.block.header().clone()
            }
        };

        let mut status = Status::new();
        status.set_hash(header.hash().clone());
        status.set_number(header.number());
        let max_height = AtomicUsize::new(0);
        max_height.store(header.number() as usize, Ordering::SeqCst);

        let chain = Arc::new(Chain {
                                 blooms_config: blooms_config,
                                 current_header: RwLock::new(header),
                                 is_sync: AtomicBool::new(false),
                                 max_height: max_height,
                                 block_map: RwLock::new(BTreeMap::new()),
                                 block_headers: RwLock::new(HashMap::new()),
                                 block_bodies: RwLock::new(HashMap::new()),
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


        chain.build_last_hashes(Some(status.hash().clone()), status.number());
        (chain, status.protobuf())
    }

    /// Get block number by BlockId
    fn block_number(&self, id: BlockId) -> Option<BlockNumber> {
        match id {
            BlockId::Number(number) => Some(number),
            BlockId::Hash(hash) => self.block_number_by_hash(hash),
            BlockId::Earliest => Some(0),
            BlockId::Latest => Some(self.get_current_height()),
        }
    }

    // Get block hash by number
    pub fn block_hash(&self, index: BlockNumber) -> Option<H256> {
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.block_hashes, &index);
        self.cache_man.lock().note_used(CacheId::BlockHashes(index));
        result
    }

    /// Get block number by hash.
    fn block_number_by_hash(&self, hash: H256) -> Option<BlockNumber> {
        self.block_header_by_hash(hash).map_or(None, |h| Some(h.number()))
    }

    /// Get block by BlockId
    pub fn block(&self, id: BlockId) -> Option<Block> {
        match id {
            BlockId::Hash(hash) => self.block_by_hash(hash),
            BlockId::Number(number) => self.block_by_height(number),
            BlockId::Earliest => self.block_by_height(0),
            BlockId::Latest => self.block_by_height(self.get_current_height()),
        }
    }

    // Get block by hash
    pub fn block_by_hash(&self, hash: H256) -> Option<Block> {
        match (self.block_header_by_hash(hash), self.block_body_by_hash(hash)) {
            (Some(h), Some(b)) => Some(Block { header: h, body: b }),
            _ => None,
        }
    }

    /// Get block by height
    pub fn block_by_height(&self, number: BlockNumber) -> Option<Block> {
        self.block_hash(number).map_or(None, |h| self.block_by_hash(h))
    }

    /// Get block header by BlockId
    fn block_header(&self, id: BlockId) -> Option<Header> {
        match id {
            BlockId::Hash(hash) => self.block_header_by_hash(hash),
            BlockId::Number(number) => self.block_header_by_height(number),
            BlockId::Earliest => self.block_header_by_height(0),
            BlockId::Latest => self.block_header_by_height(self.get_current_height()),
        }
    }

    // Get block header by hash
    fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.hash() == hash {
                return Some(header.clone());
            }
        }
        let result = self.db.read_with_cache(db::COL_HEADERS, &self.block_headers, &hash);
        self.cache_man.lock().note_used(CacheId::BlockHeaders(hash));
        result
    }

    /// Get block header by height
    fn block_header_by_height(&self, number: BlockNumber) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.number() == number {
                return Some(header.clone());
            }
        }
        self.block_hash(number).map_or(None, |h| self.block_header_by_hash(h))
    }

    /// Get block body by BlockId
    fn block_body(&self, id: BlockId) -> Option<BlockBody> {
        match id {
            BlockId::Hash(hash) => self.block_body_by_hash(hash),
            BlockId::Number(number) => self.block_body_by_height(number),
            BlockId::Earliest => self.block_body_by_height(0),
            BlockId::Latest => self.block_body_by_height(self.get_current_height()),
        }
    }

    // Get block body by hash
    fn block_body_by_hash(&self, hash: H256) -> Option<BlockBody> {
        let result = self.db.read_with_cache(db::COL_BODIES, &self.block_bodies, &hash);
        self.cache_man.lock().note_used(CacheId::BlockHeaders(hash));
        result
    }

    /// Get block body by height
    fn block_body_by_height(&self, number: BlockNumber) -> Option<BlockBody> {
        self.block_hash(number).map_or(None, |h| self.block_body_by_hash(h))
    }

    /// Get transaction by hash
    pub fn transaction(&self, hash: TransactionId) -> Option<SignedTransaction> {
        self.transaction_address(hash).map_or(None, |addr| {
            let index = addr.index;
            let hash = addr.block_hash;
            self.transaction_by_address(hash, index)
        })
    }

    /// Get address of transaction by hash.
    fn transaction_address(&self, hash: TransactionId) -> Option<TransactionAddress> {
        let result = self.db
                         .read_list_with_cache(db::COL_EXTRA, &self.transaction_addresses, &hash)
                         .map(|v| v[0].clone());
        self.cache_man.lock().note_used(CacheId::TransactionAddresses(hash));
        result
    }

    /// Get transaction by address
    fn transaction_by_address(&self, hash: H256, index: usize) -> Option<SignedTransaction> {
        self.block_body_by_hash(hash).map(|body| body.transactions()[index].clone())
    }

    /// Get transaction hashes by block hash
    pub fn transaction_hashes(&self, id: BlockId) -> Option<Vec<H256>> {
        self.block_body(id).map(|body| body.transaction_hashes())
    }

    /// Get full transaction by hash
    pub fn full_transaction(&self, hash: TransactionId) -> Option<FullTransaction> {
        self.transaction_address(hash).map_or(None, |addr| {
            let index = addr.index;
            let hash = addr.block_hash;
            self.block_by_hash(hash).map(|block| {
                let transactions = block.body().transactions();
                let tx = transactions[index].protobuf();
                let mut full_ts = FullTransaction::new();
                full_ts.set_transaction(tx);
                full_ts.set_block_number(block.number());
                full_ts.set_block_hash(hash.to_vec());
                full_ts.set_index(index as u32);
                full_ts
            })
        })
    }

    pub fn localized_receipt(&self, id: TransactionId) -> Option<LocalizedReceipt> {
        trace!("Get receipt id: {:?}", id);

        let address = match self.transaction_address(id) {
            Some(addr) => addr,
            _ => return None,
        };
        let hash = address.block_hash;
        let index = address.index;

        let mut receipts = match self.block_receipts(hash) {
            Some(r) => r.receipts,
            _ => return None,
        };

        receipts.truncate(index + 1);
        let last_receipt = receipts.pop().expect("Current receipt is provided; qed");

        receipts.retain(|ref r| r.is_some());

        let prior_gas_used = match receipts.last() {
            Some(&Some(ref r)) => r.gas_used.clone(),
            _ => 0.into(),
        };

        let no_of_logs = receipts.iter().fold(0, |acc, r| acc + r.as_ref().unwrap().logs.len());

        last_receipt.and_then(|last_receipt| {
            // Get sender
            let stx = self.transaction_by_address(hash, index).unwrap();
            let number = self.block_number_by_hash(hash).unwrap_or(0);

            let contract_address = match stx.action() {
                &Action::Create => Some(contract_address(&stx.sender(), stx.nonce())),
                &Action::Store => {
                    let store_addr: Address = STORE_ADDRESS.into();
                    Some(store_addr)
                }
                _ => None,
            };

            let receipt = LocalizedReceipt {
                transaction_hash: id,
                transaction_index: index,
                block_hash: hash,
                block_number: number,
                cumulative_gas_used: last_receipt.gas_used,
                gas_used: last_receipt.gas_used - prior_gas_used,
                contract_address: contract_address,
                logs: last_receipt.logs
                                  .into_iter()
                                  .enumerate()
                                  .map(|(i, log)| {
                    LocalizedLogEntry {
                        entry: log,
                        block_hash: hash,
                        block_number: number,
                        transaction_hash: id,
                        transaction_index: index,
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
        self.current_header.read().number()
    }

    pub fn get_current_hash(&self) -> H256 {
        self.current_header.read().hash().clone()
    }

    pub fn get_max_height(&self) -> u64 {
        self.max_height.load(Ordering::SeqCst) as u64
    }

    pub fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
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
                             .filter_map(|(number, hash)| self.block_receipts(hash).map(|r| (number, hash, r.receipts)))
                             .filter_map(|(number, hash, receipts)| self.block_body_by_hash(hash).map(|ref b| (number, hash, receipts, b.transaction_hashes())))
                             .flat_map(|(number, hash, mut receipts, mut hashes)| {
            if receipts.len() != hashes.len() {
                warn!("Block {} ({}) has different number of receipts ({}) to transactions ({}). Database corrupt?", number, hash, receipts.len(), hashes.len());
                assert!(false);
            }
            log_index = receipts.iter().fold(0, |sum, receipt| sum + receipt.as_ref().map_or(0, |r| r.logs.len()));

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

    /// Returns numbers of blocks containing given bloom.
    pub fn blocks_with_bloom(&self, bloom: &H2048, from_block: BlockNumber, to_block: BlockNumber) -> Vec<BlockNumber> {
        let range = from_block as bc::Number..to_block as bc::Number;
        let chain = bc::group::BloomGroupChain::new(self.blooms_config, self);
        chain.with_bloom(&range, &Bloom::from(bloom.clone()).into())
             .into_iter()
             .map(|b| b as BlockNumber)
             .collect()
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

    fn last_hashes(&self) -> LastHashes {
        LastHashes::from(self.last_hashes.read().clone())
    }

    /// Build last 256 block hashes.
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

    /// Commit block in db, including:
    /// 1. Block including transactions
    /// 2. TransactionAddress
    /// 3. State
    /// 3. Receipts
    /// 4. Bloom
    pub fn commit_block(&self, batch: &mut DBTransaction, block: ClosedBlock) {

        let height = block.number();
        let hash = block.hash().clone();
        trace!("commit block in db {:?}, {:?}", hash, height);

        let log_bloom = *block.log_bloom();

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
            let mut write_headers = self.block_headers.write();
            let mut write_bodies = self.block_bodies.write();
            let mut write_receipts = self.block_receipts.write();
            let mut write_blooms = self.blocks_blooms.write();
            let mut write_hashes = self.block_hashes.write();
            let mut write_txs = self.transaction_addresses.write();

            batch.write_with_cache(db::COL_HEADERS, &mut *write_headers, hash, block.header().clone(), CacheUpdatePolicy::Overwrite);
            batch.write_with_cache(db::COL_BODIES, &mut *write_bodies, hash, block.body().clone(), CacheUpdatePolicy::Overwrite);
            batch.write_with_cache(db::COL_EXTRA, &mut *write_hashes, height as BlockNumber, hash, CacheUpdatePolicy::Overwrite);
            batch.write_with_cache(db::COL_EXTRA, &mut *write_receipts, hash, block_receipts, CacheUpdatePolicy::Overwrite);
            batch.extend_with_cache(db::COL_EXTRA, &mut *write_blooms, blocks_blooms.clone(), CacheUpdatePolicy::Overwrite);
            batch.extend_with_cache_append(db::COL_EXTRA, &*self.db, &mut *write_txs, block.transactions_uni.clone(), AppendPolicy::Overwrite);
            batch.extend_with_cache_append(db::COL_EXTRA, &*self.db, &mut *write_txs, block.transactions_dup.clone(), AppendPolicy::Update);

        }

        //note used
        self.cache_man.lock().note_used(CacheId::BlockHashes(height as BlockNumber));
        self.cache_man.lock().note_used(CacheId::BlockReceipts(hash));
        self.cache_man.lock().note_used(CacheId::BlockHeaders(hash));
        self.cache_man.lock().note_used(CacheId::BlockBodies(hash));

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

    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: H256) -> Option<BlockReceipts> {
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.block_receipts, &hash);
        self.cache_man.lock().note_used(CacheId::BlockReceipts(hash));
        result
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, address: &TransactionAddress) -> Option<Receipt> {
        self.block_receipts(address.block_hash.clone()).map_or(None, |r| r.receipts[address.index].clone())
    }

    /// Attempt to get a copy of a specific block's final state.
    pub fn state_at(&self, id: BlockId) -> Option<State<StateDB>> {
        self.block_header(id).map_or(None, |h| self.gen_state(*h.state_root()))
    }

    /// generate block's final state.
    pub fn gen_state(&self, root: H256) -> Option<State<StateDB>> {
        let db = self.state_db.boxed_clone();
        State::from_existing(db, root, U256::from(0), self.factories.clone()).ok()
    }

    /// Get a copy of the best block's state.
    pub fn state(&self) -> State<StateDB> {
        self.gen_state(self.current_state_root()).expect("State root of current block is invalid.")
    }

    /// Get code by address
    pub fn code_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id).and_then(|s| s.code(address).ok()).map(|c| c.map(|c| (&*c).clone()))
    }

    /// Get transaction count by address
    pub fn nonce(&self, address: &Address, id: BlockId) -> Option<U256> {
        self.state_at(id).and_then(|s| s.nonce(address).ok())
    }

    pub fn eth_call(&self, request: CallRequest, id: BlockId) -> Result<Bytes, String> {
        let signed = self.sign_call(request);
        let result = self.call(&signed, id, Default::default());
        result.map(|b| b.output.into()).or_else(|_| Err(String::from("Call Error")))
    }

    fn sign_call(&self, request: CallRequest) -> SignedTransaction {
        let from = request.from.unwrap_or(Address::zero());
        Transaction {
            nonce: U256::zero(),
            action: Action::Call(request.to),
            gas: U256::from(50_000_000),
            gas_price: U256::zero(),
            value: U256::zero(),
            data: request.data.map_or_else(Vec::new, |d| d.to_vec()),
            block_limit: u64::max_value(),
        }
        .fake_sign(from)
    }

    fn call(&self, t: &SignedTransaction, block_id: BlockId, analytics: CallAnalytics) -> Result<Executed, CallError> {
        let header = self.block_header(block_id).ok_or(CallError::StatePruned)?;
        let last_hashes = self.build_last_hashes(None, header.number());
        let env_info = EnvInfo {
            number: header.number(),
            author: Address::default(),
            timestamp: header.timestamp(),
            difficulty: U256::default(),
            last_hashes: last_hashes,
            gas_used: *header.gas_used(),
            gas_limit: *header.gas_limit(),
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

    pub fn validate_hash(&self, block_hash: &H256) -> bool {
        let current_hash = self.get_current_hash();
        trace!("validate_hash current_hash {:?} block_hash {:?}", current_hash, block_hash);
        current_hash == *block_hash
    }

    pub fn validate_height(&self, block_number: u64) -> bool {
        let current_height = self.get_current_height();
        trace!("validate_height current_height {:?} block_number {:?}", current_height, block_number - 1);
        current_height + 1 == block_number
    }

    /// Execute block in vm
    fn execute_block(&self, block: Block) -> OpenBlock {
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let mut open_block = OpenBlock::new(self.factories.clone(), false, block, self.state_db.boxed_clone(), current_state_root, last_hashes.into()).unwrap();
        open_block.apply_transactions();

        open_block
    }

    /// Add block to chain:
    /// 1. Execute block
    /// 2. Commit block
    /// 3. Update cache
    pub fn add_block(&self, batch: &mut DBTransaction, block: Block) -> Option<Header> {
        let height = block.number();
        match block.proof_type() {
            Some(ProofType::Tendermint) => {
                let proof = TendermintProof::from(block.proof().clone());
                if !proof.simple_check(height as usize - 1) {
                    return None;
                }
            }
            _ => {}
        }

        if self.validate_hash(block.parent_hash()) {
            let mut open_block = self.execute_block(block);
            let closed_block = open_block.close();
            let header = closed_block.header().clone();
            self.commit_block(batch, closed_block);
            self.update_last_hashes(&header.hash());
            Some(header)
        } else {
            None
        }
    }

    pub fn set_block(&self, block: Block) -> Option<ProtoStatus> {
        let height = block.number();
        trace!("set_block height = {:?}, hash = {:?}", height, block.hash());
        if self.validate_height(height) {
            let mut batch = self.db.transaction();
            if let Some(header) = self.add_block(&mut batch, block) {

                trace!("set_block current_hash!!!!!!{:?} {:?}", height, header.hash());

                {
                    *self.current_header.write() = header;
                }

                let status = self.save_status(&mut batch);

                self.db.write(batch).expect("DB write failed.");
                info!("chain update {:?}", status.number);
                Some(status.protobuf())
            } else {
                warn!("add block failed");
                None
            }
        } else {
            None
        }
    }

    pub fn compare_status(&self, st: Status) -> (u64, u64) {
        let current_height = self.get_current_height();
        if st.number() > current_height {
            (current_height + 1, st.number() - current_height)
        } else {
            (0, 0)
        }
    }

    /// Get current cache size.
    pub fn cache_size(&self) -> CacheSize {
        CacheSize {
            blocks: self.block_headers.read().heap_size_of_children() + self.block_bodies.read().heap_size_of_children(),
            transaction_addresses: self.transaction_addresses.read().heap_size_of_children(),
            blocks_blooms: self.blocks_blooms.read().heap_size_of_children(),
            block_receipts: self.block_receipts.read().heap_size_of_children(),
        }
    }

    /// Ticks our cache system and throws out any old data.
    pub fn collect_garbage(&self) {
        let current_size = self.cache_size().total();

        let mut block_headers = self.block_headers.write();
        let mut block_bodies = self.block_bodies.write();
        let mut block_hashes = self.block_hashes.write();
        let mut transaction_addresses = self.transaction_addresses.write();
        let mut blocks_blooms = self.blocks_blooms.write();
        let mut block_receipts = self.block_receipts.write();

        let mut cache_man = self.cache_man.lock();
        cache_man.collect_garbage(current_size, |ids| {
            for id in &ids {
                match *id {
                    CacheId::BlockHeaders(ref h) => {
                        block_headers.remove(h);
                    }
                    CacheId::BlockBodies(ref h) => {
                        block_bodies.remove(h);
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

            block_headers.shrink_to_fit();
            block_bodies.shrink_to_fit();
            block_hashes.shrink_to_fit();
            transaction_addresses.shrink_to_fit();
            blocks_blooms.shrink_to_fit();
            block_receipts.shrink_to_fit();

            block_headers.heap_size_of_children() + block_bodies.heap_size_of_children() + block_hashes.heap_size_of_children() + transaction_addresses.heap_size_of_children() + blocks_blooms.heap_size_of_children() + block_receipts.heap_size_of_children()
        });
    }

    pub fn poll_filter(&self) -> Arc<Mutex<PollManager<PollFilter>>> {
        self.polls_filter.clone()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_must_use, deprecated, unused_extern_crates)]
    extern crate cita_crypto;
    extern crate env_logger;
    extern crate mktemp;
    extern crate rustc_serialize;
    use self::Chain;
    use super::*;
    use cita_crypto::{KeyPair, PrivKey, SIGNATURE_NAME};
    use db;
    use libchain::block::{Block, BlockBody};
    use libchain::genesis::Spec;
    use libproto::blockchain;
    use self::rustc_serialize::hex::FromHex;
    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use std::time::{UNIX_EPOCH, Instant};
    use test::{Bencher, black_box};
    use types::transaction::SignedTransaction;
    use util::{U256, H256, Address};
    use util::crypto::CreateKey;
    use util::kvdb::{Database, DatabaseConfig};
    //use util::hashable::HASH_NAME;

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

    fn init_chain() -> Arc<Chain> {
        let _ = env_logger::init();
        let tempdir = mktemp::Temp::new_dir().unwrap().to_path_buf();
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
        let genesis = Genesis {
            spec: Spec {
                alloc: HashMap::new(),
                prevhash: H256::from(0),
                timestamp: 0,
            },
            block: Block::default(),
        };
        let (sync_tx, _) = channel();
        let (chain, _) = Chain::init_chain(Arc::new(db), genesis, sync_tx);
        chain
    }

    fn create_block(chain: &Chain, privkey: &PrivKey, to: Address, data: Vec<u8>, nonce: (u32, u32)) -> Block {
        let mut block = Block::new();

        block.set_parent_hash(chain.get_current_hash());
        block.set_timestamp(UNIX_EPOCH.elapsed().unwrap().as_secs());
        block.set_number(chain.get_current_height() + 1);
        // header.proof= ?;

        let mut body = BlockBody::new();
        let mut txs = Vec::new();
        for i in nonce.0..nonce.1 {
            let mut tx = blockchain::Transaction::new();
            if to == Address::from(0) {
                tx.set_to(String::from(""));
            } else {
                tx.set_to(to.hex());
            }
            tx.set_nonce(U256::from(i).to_hex());
            tx.set_data(data.clone());
            tx.set_valid_until_block(100);
            tx.set_quota(184467440737095);

            let stx = tx.sign(*privkey);
            let new_tx = SignedTransaction::new(&stx).unwrap();
            txs.push(new_tx);
        }
        body.set_transactions(txs);
        block.set_body(body);
        block
    }

    #[bench]
    fn bench_execute_block(b: &mut Bencher) {
        let chain = init_chain();
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let data = "60606040523415600b57fe5b5b5b5b608e8061001c6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603a575bfe5b3415604157fe5b605560048080359060200190919050506057565b005b806000819055505b505600a165627a7a7230582079b763be08c24124c9fa25c78b9d221bdee3e981ca0b2e371628798c41e292ca0029"
            .from_hex()
            .unwrap();

        let block = create_block(&chain, privkey, Address::from(0), data, (0, 1));
        chain.set_block(block.clone());

        let txhash = block.body().transactions()[0].hash();
        let receipt = chain.localized_receipt(txhash).expect("no receipt found");
        let to = receipt.contract_address.unwrap();
        let data = format!("{}{}", "55241077", "0000000000000000000000000000000000000000000000000000000012345678")
            .from_hex()
            .unwrap();
        println!("passsss");
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
    fn test_code_at() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let chain = init_chain();
        /*
            pragma solidity ^0.4.8;

            contract mortal {
                /* Define variable owner of the type address*/
                address owner;

                /* this function is executed at initialization and sets the owner of the contract */
                function mortal() { owner = msg.sender; }

                /* Function to recover the funds on the contract */
                function kill() { if (msg.sender == owner) selfdestruct(owner); }
            }

            contract greeter is mortal {
                /* define variable greeting of the type string */
                string greeting;

                /* this runs when the contract is executed */
                function greeter(string _greeting) public {
                    greeting = _greeting;
                }

                /* main function */
                function greet() constant returns (string) {
                    return greeting;
                }
            }
        */
        let data = "6060604052341561000f57600080fd5b5b336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505b5b61010c806100616000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806341c0e1b514603d575b600080fd5b3415604757600080fd5b604d604f565b005b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141560dd576000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16ff5b5b5600a165627a7a72305820de567cec1777627b898689638799169aacaf87d3ea313a0d8dab5758bac937670029"
            .from_hex()
            .unwrap();
        println!("data: {:?}", data);

        let block = create_block(&chain, privkey, Address::from(0), data, (0, 1));
        chain.set_block(block.clone());

        let tx = &block.body.transactions[0];
        let txhash = tx.hash();
        let receipt = chain.localized_receipt(txhash).unwrap();

        let contract_address = receipt.contract_address.unwrap();
        println!("contract address: {}", contract_address);
        let code = chain.code_at(&contract_address, BlockId::Latest);
        assert!(code.is_some());
        assert!(code.unwrap().is_some());
    }

    #[test]
    fn test_contract() {
        //let keypair = KeyPair::gen_keypair();
        //let privkey = keypair.privkey();
        //let pubkey = keypair.pubkey();
        let privkey = if SIGNATURE_NAME == "ed25519" {
            PrivKey::from("fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcfd978661983b8af4bc115d2d66")
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("35593bd681b8fc0737c2fdbef6e3c89a975dde47176dbd9724091e84fbf305b0")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let chain = init_chain();

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

        let block = create_block(&chain, &privkey, Address::from(0), data, (0, 1));
        chain.set_block(block.clone());

        let txhash = block.body().transactions()[0].hash();
        let receipt = chain.localized_receipt(txhash).unwrap();

        println!("{:?}", receipt);
        let contract_address = receipt.contract_address.unwrap();
        println!("contract address: {}", contract_address);
        let log = &receipt.logs[0];
        assert_eq!(contract_address, log.address);
        if SIGNATURE_NAME == "ed25519" {
            assert_eq!(contract_address, Address::from("b2f0aa00c6bc02a2b07646a1a213e1bed6fefff6"));
        } else if SIGNATURE_NAME == "secp256k1" {
            assert_eq!(contract_address, Address::from("893ed563bbe983e04441792e7ae866d4134adfd7"));
        };
        println!("contract_address as slice {:?}", contract_address.to_vec().as_slice());
        if SIGNATURE_NAME == "ed25519" {
            // log data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 111, 59, 43, 53, 88, 72, 145, 132, 114, 215, 155, 118, 248, 179, 151, 41, 8, 138, 13, 0]
            assert!(log.data.as_slice().ends_with(contract_address.to_vec().as_slice()));
            assert_eq!(
                log.data,
                Bytes::from(vec![
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    178,
                    240,
                    170,
                    0,
                    198,
                    188,
                    2,
                    162,
                    176,
                    118,
                    70,
                    161,
                    162,
                    19,
                    225,
                    190,
                    214,
                    254,
                    255,
                    246,
                ])
            );
        } else if SIGNATURE_NAME == "secp256k1" {
            // log data: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137, 62, 213, 99, 187, 233, 131, 224, 68, 65, 121, 46, 122, 232, 102, 212, 19, 74, 223, 215]
            assert!(log.data.as_slice().ends_with(contract_address.to_vec().as_slice()));
            assert_eq!(
                log.data,
                Bytes::from(vec![
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    137,
                    62,
                    213,
                    99,
                    187,
                    233,
                    131,
                    224,
                    68,
                    65,
                    121,
                    46,
                    122,
                    232,
                    102,
                    212,
                    19,
                    74,
                    223,
                    215,
                ])
            );
        };

        // set a=10
        let data = "60fe47b1000000000000000000000000000000000000000000000000000000000000000a".from_hex().unwrap();
        let block = create_block(&chain, &privkey, contract_address, data, (1, 2));
        chain.set_block(block.clone());
        let txhash = block.body().transactions()[0].hash();
        let receipt = chain.localized_receipt(txhash).unwrap();
        println!("{:?}", receipt);

        // get a is 10
        let data = "6d4ce63c".from_hex().unwrap();
        let call_request = CallRequest {
            from: None,
            to: contract_address,
            data: Some(data.into()),
        };
        let call_result = chain.eth_call(call_request, BlockId::Latest);
        assert_eq!(call_result, Ok(Bytes::from(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10])));
        println!("call_result: {:?}", call_result);
    }
}
