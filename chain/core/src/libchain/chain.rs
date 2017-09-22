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
use contracts::{NodeManager, AccountManager, QuotaManager, AccountGasLimit};
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
use libproto::blockchain::{ProofType, Status as ProtoStatus, RichStatus as ProtoRichStatus};
use libproto::request::FullTransaction;
use native::Factory as NativeFactory;
use proof::TendermintProof;
use protobuf::RepeatedField;
use receipt::{Receipt, LocalizedReceipt};
use serde_json;
use state::State;
use state_db::StateDB;

use std::collections::{BTreeMap, VecDeque};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
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
use util::UtilError;
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

#[derive(Debug, PartialEq, Deserialize)]
pub struct Switch {
    pub nonce: bool,
    pub permission: bool,
}

impl Status {
    fn new() -> Self {
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

    #[allow(dead_code)]
    fn protobuf(&self) -> ProtoStatus {
        let mut ps = ProtoStatus::new();
        ps.set_height(self.number());
        ps.set_hash(self.hash().to_vec());
        ps
    }
}

impl Switch {
    pub fn new() -> Self {
        Switch { nonce: false, permission: false }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct RichStatus {
    number: u64,
    hash: H256,
    block_gas_limit: u64,
    account_gas_limit: AccountGasLimit,
    nodes: Vec<Address>,
}

impl RichStatus {
    fn new() -> Self {
        RichStatus {
            number: 0,
            hash: H256::default(),
            block_gas_limit: 0,
            nodes: vec![],
            account_gas_limit: AccountGasLimit::new(),
        }
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

    fn set_nodes(&mut self, nodes: Vec<Address>) {
        self.nodes = nodes
    }

    fn set_block_gas_limit(&mut self, block_gas_limit: u64) {
        self.block_gas_limit = block_gas_limit;
    }

    fn set_account_gas_limit(&mut self, account_gas_limit: AccountGasLimit) {
        self.account_gas_limit = account_gas_limit;
    }

    fn protobuf(&self) -> ProtoRichStatus {
        let mut ps = ProtoRichStatus::new();
        let mut account_gas_limit = AccountGasLimit::new();
        ps.set_height(self.number());
        ps.set_hash(self.hash().to_vec());
        let node_list = self.nodes.clone().into_iter().map(|address| address.to_vec()).collect();
        ps.set_nodes(RepeatedField::from_vec(node_list));
        ps.set_block_gas_limit(self.block_gas_limit);
        account_gas_limit.set_common_gas_limit(self.account_gas_limit.get_common_gas_limit());
        account_gas_limit.set_specific_gas_limit(self.account_gas_limit.get_specific_gas_limit().clone());
        ps.set_account_gas_limit(account_gas_limit.into());
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
    pub block_map: RwLock<BTreeMap<u64, (BlockSource, Block)>>,
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
    // System contract config cache
    senders: RwLock<HashMap<Address, bool>>,
    creators: RwLock<HashMap<Address, bool>>,
    pub nodes: RwLock<Vec<Address>>,
    pub block_gas_limit: AtomicUsize,
    pub account_gas_limit: RwLock<AccountGasLimit>,
    cache_man: Mutex<CacheManager<CacheId>>,
    polls_filter: Arc<Mutex<PollManager<PollFilter>>>,

    // switch, check them or not
    pub switch: Switch,
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

    pub fn init_chain(db: Arc<KeyValueDB>, mut genesis: Genesis, sync_sender: Sender<u64>, path: &str) -> (Arc<Chain>, ProtoRichStatus) {
        // 400 is the avarage size of the key
        let cache_man = CacheManager::new(1 << 14, 1 << 20, 400);

        let trie_factory = TrieFactory::new(TrieSpec::Generic);
        let factories = Factories {
            vm: EvmFactory::default(),
            native: NativeFactory::default(),
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

        let switch_file = File::open(path).unwrap();
        let sconfig = BufReader::new(switch_file);
        let sw: Switch = serde_json::from_reader(sconfig).expect("Failed to load json file.");
        info!("config check: {:?}", sw);

        let max_height = AtomicUsize::new(0);
        max_height.store(header.number() as usize, Ordering::SeqCst);
        let raw_chain = Chain {
            blooms_config: blooms_config,
            current_header: RwLock::new(header.clone()),
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
            senders: RwLock::new(HashMap::new()),
            creators: RwLock::new(HashMap::new()),
            nodes: RwLock::new(Vec::new()),
            block_gas_limit: AtomicUsize::new(18446744073709551615),
            account_gas_limit: RwLock::new(AccountGasLimit::new()),
            switch: sw,
        };

        // Build chain config
        let chain = Arc::new(raw_chain);
        chain.build_last_hashes(Some(header.hash()), header.number());
        chain.reload_config();

        // Generate status
        let mut status = RichStatus::new();
        status.set_hash(header.hash());
        status.set_number(header.number());
        let nodes: Vec<Address> = chain.nodes.read().to_vec();
        status.set_nodes(nodes);
        // Set BlockGasLimit
        let block_gas_limit: u64 = chain.block_gas_limit.load(Ordering::SeqCst) as u64;
        status.set_block_gas_limit(block_gas_limit);
        // Set AccountGasLimit
        let account_gas_limit = chain.account_gas_limit.read().clone();
        status.set_account_gas_limit(account_gas_limit);

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
        self.transaction_address(hash)
            .map_or(None, |addr| {
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
        self.transaction_address(hash)
            .map_or(None, |addr| {
            let index = addr.index;
            let hash = addr.block_hash;
            self.block_by_hash(hash)
                .map(|block| {
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
            Some(&Some(ref r)) => r.gas_used,
            _ => 0.into(),
        };

        let no_of_logs = receipts.iter().fold(0, |acc, r| acc + r.as_ref().unwrap().logs.len());

        last_receipt.and_then(|last_receipt| {
            // Get sender
            let stx = self.transaction_by_address(hash, index).unwrap();
            let number = self.block_number_by_hash(hash).unwrap_or(0);

            let mut contract_addr = None;
            if last_receipt.error.is_none() {
                contract_addr = match stx.action() {
                    &Action::Create => Some(contract_address(&stx.sender(), stx.nonce())),
                    _ => None,
                };
            }

            let receipt = LocalizedReceipt {
                transaction_hash: id,
                transaction_index: index,
                block_hash: hash,
                block_number: number,
                cumulative_gas_used: last_receipt.gas_used,
                gas_used: last_receipt.gas_used - prior_gas_used,
                contract_address: contract_addr,
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
                error: last_receipt.error,
            };
            Some(receipt)
        })
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_header.read().number()
    }

    pub fn get_current_hash(&self) -> H256 {
        self.current_header.read().hash()
    }

    pub fn get_max_height(&self) -> u64 {
        self.max_height.load(Ordering::SeqCst) as u64
    }

    pub fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
    }

    pub fn logs<F>(&self, mut blocks: Vec<BlockNumber>, matches: F, limit: Option<usize>) -> Vec<LocalizedLogEntry>
        where F: Fn(&LogEntry) -> bool,
              Self: Sized
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
                logs.into_iter()
                    .enumerate()
                    .map(move |(i, log)| {
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
        self.prune_ancient(state).expect("mark_canonical failed");
    }

    fn prune_ancient(&self, mut state_db: StateDB) -> Result<(), UtilError> {
        let number = match state_db.journal_db().latest_era() {
            Some(n) => n,
            None => return Ok(()),
        };
        let history = 2;
        // prune all ancient eras until we're below the memory target,
        // but have at least the minimum number of states.
        loop {
            match state_db.journal_db().earliest_era() {
                Some(era) if era + history <= number => {
                    trace!(target: "client", "Pruning state for ancient era {}", era);
                    match self.block_hash(era) {
                        Some(ancient_hash) => {
                            let mut batch = DBTransaction::new();
                            state_db.mark_canonical(&mut batch, era, &ancient_hash)?;
                            self.db.write_buffered(batch);
                            state_db.journal_db().flush();
                        }
                        None => debug!(target: "client", "Missing expected hash for block {}", era),
                    }
                }
                _ => break, // means that every era is kept, no pruning necessary.
            }
        }
        Ok(())
    }

    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: H256) -> Option<BlockReceipts> {
        let result = self.db.read_with_cache(db::COL_EXTRA, &self.block_receipts, &hash);
        self.cache_man.lock().note_used(CacheId::BlockReceipts(hash));
        result
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, address: &TransactionAddress) -> Option<Receipt> {
        self.block_receipts(address.block_hash.clone())
            .map_or(None, |r| r.receipts[address.index].clone())
    }

    /// Attempt to get a copy of a specific block's final state.
    // TODO: Add senders and creators?
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
        let mut state = self.gen_state(self.current_state_root()).unwrap();
        state.senders = self.senders.read().clone();
        state.creators = self.creators.read().clone();
        state
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
        result.map(|b| b.output.into()).or_else(|e| Err(format!("Call Error {}", e)))
    }

    fn sign_call(&self, request: CallRequest) -> SignedTransaction {
        let from = request.from.unwrap_or(Address::zero());
        trace!("sign_call with from {:?}", from);
        Transaction {
            nonce: U256::zero(),
            action: Action::Call(request.to),
            // May be failed
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
            account_gas_limit: u64::max_value().into(),
        };
        // that's just a copy of the state.
        let mut state = self.state_at(block_id).ok_or(CallError::StatePruned)?;
        // TODO: Should be not check permission when eth_call or use history permission
        state.senders = self.senders.read().clone();
        state.creators = self.creators.read().clone();
        let engine = NullEngine::default();

        let options = TransactOptions {
            tracing: analytics.transaction_tracing,
            vm_tracing: analytics.vm_tracing,
            check_nonce: false,
            check_permission: false,
        };
        let ret = Executive::new(&mut state, &env_info, &engine, &self.factories.vm, &self.factories.native)
            .transact(t, options)?;

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
        let senders = self.senders.read().clone();
        let creators = self.creators.read().clone();
        let switch = &self.switch;

        let mut open_block = OpenBlock::new(self.factories.clone(), senders, creators, false, block, self.state_db.boxed_clone(), current_state_root, last_hashes.into(), &self.account_gas_limit.read().clone()).unwrap();
        open_block.apply_transactions(&switch);

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

    /// Reload system config from system contract
    pub fn reload_config(&self) {
        {
            // Reload senders and creators cache
            *self.senders.write() = AccountManager::load_senders(self);
            *self.creators.write() = AccountManager::load_creators(self);
        }
        {
            // Reload consensus nodes cache
            *self.nodes.write() = NodeManager::read(self);
        }
        {
            // Reload BlockGasLimit cache
            let block_gas_limit = QuotaManager::block_gas_limit(self);
            self.block_gas_limit.swap(block_gas_limit as usize, Ordering::SeqCst);
        }

        {
            // Reload AccountGasLimit cache
            let common_gas_limit = QuotaManager::account_gas_limit(self);
            let specific = QuotaManager::specific(self);
            let mut account_gas_limit = self.account_gas_limit.write();
            account_gas_limit.set_common_gas_limit(common_gas_limit);
            account_gas_limit.set_specific_gas_limit(specific);
        }
    }

    pub fn set_block(&self, block: Block) -> Option<ProtoRichStatus> {
        let height = block.number();
        trace!("set_block height = {:?}, hash = {:?}", height, block.hash());
        if self.validate_height(height) {
            let mut batch = self.db.transaction();
            if let Some(header) = self.add_block(&mut batch, block) {

                trace!("set_block current_hash!!!!!! height={:?}, header={:?}, header={:?}", height, header.hash(), header);

                {
                    *self.current_header.write() = header;
                }

                // DB write
                let status = self.save_status(&mut batch);
                self.db.write(batch).expect("DB write failed.");

                // reload_config
                self.reload_config();

                let mut rich_status = RichStatus::new();
                rich_status.set_hash(*status.hash());
                rich_status.set_number(status.number());
                rich_status.set_nodes(self.nodes.read().clone());
                rich_status.set_block_gas_limit(self.block_gas_limit.load(Ordering::SeqCst) as u64);
                rich_status.set_account_gas_limit(self.account_gas_limit.read().clone().into());

                info!("chain update {:?}", height);
                Some(rich_status.protobuf())
            } else {
                let mut guard = self.block_map.write();
                let _ = guard.remove(&height);
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
    extern crate rustc_serialize;

    use self::rustc_serialize::hex::FromHex;
    use super::*;
    use std::env;
    use test::Bencher;
    use tests::helpers::{init_chain, bench_chain, solc, create_block};
    use util::{H256, Address};
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

    #[test]
    fn test_code_at() {
        let chain = init_chain();
        let source = r#"
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
"#;
        let (data, _) = solc("mortal", source);
        println!("data: {:?}", data);

        let block = create_block(&chain, Address::from(0), &data, (0, 1));
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
        let chain = init_chain();
        let source = r#"
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
        "#;
        let (data, _) = solc("ConstructSol", source);
        let block = create_block(&chain, Address::from(0), &data, (0, 1));
        chain.set_block(block.clone());

        let txhash = block.body().transactions()[0].hash();
        let receipt = chain.localized_receipt(txhash).unwrap();
        let contract_address = receipt.contract_address.unwrap();

        let log = &receipt.logs[0];
        assert_eq!(contract_address, log.address);
        let data = "60fe47b1000000000000000000000000000000000000000000000000000000000000000a".from_hex().unwrap();
        let block = create_block(&chain, contract_address, &data, (1, 2));
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

    fn bench_simple_storage(name: &str, data: &Vec<u8>) {
        let bench_mode = env::args().find(|x| x.contains("--bench")).is_some();
        let source = r#"
pragma solidity ^0.4.0;

contract SimpleStorage {
  uint uint_value;             // 0
  string string_value;// 1
  uint[] array_value;//2
  mapping (uint => uint) public map_value;//3

  function SimpleStorage() {
    uint_value = 0;
    string_value='string';
    array_value= new uint[](3);
    map_value[0]= 0;
  }

  /* 0) uint */
  function uint_set(uint value) {
    uint_value =  value;
  }

  function uint_get() returns (uint) {
    return uint_value;
  }

  /* 1) string */
  function string_set(string  value) {
    string_value =  value;
  }

  function string_get() returns (string) {
    return string_value;
  }

  /* 2 array*/
  function array_set(uint index, uint value) {
    array_value[index]=  value;
  }

  function array_get(uint  index) returns (uint) {
    return array_value[index];
  }

  /* 3) map */
  function map_set(uint key, uint value) {
    map_value[key] = value;
  }

  function map_get(uint key) returns (uint) {
    return map_value[key];
  }
}"#;
        let (code, _) = solc("SimpleStorage", source);
        let tpb = if bench_mode { 10000 } else { 1 };
        println!("pass");
        let evm = bench_chain(&code, &data, tpb, Address::zero());
        let native = bench_chain(&code, &data, tpb, Address::from(0x400));
        println!("test {:20} ... bench: {:5} tpb {:10} tps(evm) {:10} tps(native) {:3.2}% ", name, tpb, evm, native, native as f32 / evm as f32 * 100.0);
    }

    #[bench]
    fn bench_uint_set(b: &mut Bencher) {
        let name = "bench_uint_set";
        let data = "aa91543e000000000000000000000000000000000000000000000000000000000000000a".from_hex().unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }

    #[bench]
    fn bench_uint_get(b: &mut Bencher) {
        let name = "bench_uint_get";
        let data = "aa91543e000000000000000000000000000000000000000000000000000000000000000a".from_hex().unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }
    #[bench]
    fn bench_string_set(b: &mut Bencher) {
        let name = "bench_string_set";
        let data = "c9615770000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000033132330000000000000000000000000000000000000000000000000000000000"
            .from_hex()
            .unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }

    #[bench]
    fn bench_string_get(b: &mut Bencher) {
        let name = "bench_string_get";
        let data = "e3135d14".from_hex().unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }
    #[bench]
    fn bench_array_set(b: &mut Bencher) {
        let name = "bench_array_set";
        let data = "118b229c0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000b"
            .from_hex()
            .unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }

    #[bench]
    fn bench_array_get(b: &mut Bencher) {
        let name = "bench_array_get";
        let data = "180a4bbf0000000000000000000000000000000000000000000000000000000000000001".from_hex().unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }
    #[bench]
    fn bench_map_set(b: &mut Bencher) {
        let name = "bench_map_set";
        let data = "118b229c0000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000c"
            .from_hex()
            .unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }

    #[bench]
    fn bench_map_get(b: &mut Bencher) {
        let name = "bench_map_get";
        let data = "180a4bbf0000000000000000000000000000000000000000000000000000000000000001".from_hex().unwrap();
        bench_simple_storage(name, &data);
        b.iter(|| {});
    }
}
