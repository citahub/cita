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
use db;
use db::*;

use filters::{PollFilter, PollManager};
use header::*;
pub use libchain::block::*;
use libchain::cache::CacheSize;

use libchain::extras::*;
use libchain::status::Status;
pub use libchain::transaction::*;

use libproto::*;
use libproto::blockchain::{AccountGasLimit as ProtoAccountGasLimit, Proof as ProtoProof, ProofType,
                           RichStatus as ProtoRichStatus};

use libproto::executer::ExecutedResult;
use proof::TendermintProof;
use protobuf::Message;
use protobuf::RepeatedField;
use receipt::{LocalizedReceipt, Receipt};
use serde_json;
use state::State;
use state_db::StateDB;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use types::filter::Filter;
use types::ids::{BlockId, TransactionId};
use types::log_entry::{LocalizedLogEntry, LogEntry};
use types::transaction::{Action, SignedTransaction};
use util::{journaldb, Address, H2048, H256, U256};
use util::{Mutex, RwLock};
use util::Hashable;
use util::HeapSizeOf;
use util::kvdb::*;

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
    /*    BlockHeaders(H256),
    BlockBodies(H256),
    BlockHashes(BlockNumber),
*/
    TransactionAddresses(H256),
    BlocksBlooms(LogGroupPosition),
    BlockReceipts(H256),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub check_permission: bool,
    pub check_quota: bool,
    pub check_prooftype: u8,
}

impl Config {
    pub fn default() -> Self {
        Config {
            check_permission: false,
            check_quota: false,
            check_prooftype: 2,
        }
    }
}

impl bc::group::BloomGroupDatabase for Chain {
    fn blooms_at(&self, position: &bc::group::GroupPosition) -> Option<bc::group::BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self.db
            .read_with_cache(db::COL_EXTRA, &self.blocks_blooms, &position)
            .map(Into::into);
        self.cache_man
            .lock()
            .note_used(CacheId::BlocksBlooms(position));
        result
    }
}

#[derive(Debug, Clone)]
pub enum BlockInQueue {
    Proposal(Block),
    ConsensusBlock(Block, ProtoProof),
    SyncBlock((Block, Option<ProtoProof>)),
}

// TODO: chain对外开放的方法，是保证能正确解析结构，即类似于Result<Block,Err>
// 所有直接unwrap的地方都可能会报错！
pub struct Chain {
    blooms_config: bc::Config,
    pub current_header: RwLock<Header>,
    pub is_sync: AtomicBool,
    // Max height in block map
    pub max_height: AtomicUsize,
    pub max_store_height: AtomicUsize,
    pub block_map: RwLock<BTreeMap<u64, BlockInQueue>>,
    pub db: Arc<KeyValueDB>,
    pub state_db: StateDB,

    // block cache
    pub block_headers: RwLock<HashMap<BlockNumber, Header>>,
    pub block_bodies: RwLock<HashMap<BlockNumber, BlockBody>>,

    // extra caches
    pub block_hashes: RwLock<HashMap<H256, BlockNumber>>,
    pub transaction_addresses: RwLock<HashMap<TransactionId, TransactionAddress>>,
    pub blocks_blooms: RwLock<HashMap<LogGroupPosition, BloomGroup>>,
    pub block_receipts: RwLock<HashMap<H256, BlockReceipts>>,
    pub nodes: RwLock<Vec<Address>>,

    pub block_gas_limit: AtomicUsize,
    pub account_gas_limit: RwLock<ProtoAccountGasLimit>,

    cache_man: Mutex<CacheManager<CacheId>>,
    polls_filter: Arc<Mutex<PollManager<PollFilter>>>,

    /// Switch, check proof type for add_sync_block
    pub check_prooftype: u8,
}

/// Get latest status
pub fn get_chain(db: &KeyValueDB) -> Option<Header> {
    let h: Option<H256> = db.read(db::COL_EXTRA, &CurrentHash);
    if let Some(hash) = h {
        let hi: Option<BlockNumber> = db.read(db::COL_EXTRA, &hash);
        if let Some(h) = hi {
            warn!("get_chain hash {:?}  bn{:?}  CurrentHash", hash, h);
            db.read(db::COL_HEADERS, &h)
        } else {
            warn!("not expected get_chain_current_head height");
            None
        }
    } else {
        warn!("not expected get_chain_current_head hash.");
        None
    }
}

pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
}

impl Chain {
    pub fn init_chain<R>(db: Arc<KeyValueDB>, sconfig: R) -> Chain
    where
        R: Read,
    {
        // 400 is the avarage size of the key
        let cache_man = CacheManager::new(1 << 14, 1 << 20, 400);

        let journal_db = journaldb::new(Arc::clone(&db), journaldb::Algorithm::Archive, COL_STATE);
        let state_db = StateDB::new(journal_db);
        let blooms_config = bc::Config {
            levels: LOG_BLOOMS_LEVELS,
            elements_per_index: LOG_BLOOMS_ELEMENTS_PER_INDEX,
        };

        let sc: Config = serde_json::from_reader(sconfig).expect("Failed to load json file.");
        info!("config check: {:?}", sc);

        //        let mut is_genesis_ok = false;
        let header = get_chain(&*db).unwrap_or(Header::default());
        info!("get chain head is : {:?}", header);
        let max_height = AtomicUsize::new(0);
        max_height.store(header.number() as usize, Ordering::SeqCst);
        let max_store_height = AtomicUsize::new(0);
        //        max_store_height.store(header.number() as usize, Ordering::SeqCst);
        max_store_height.store(::std::u64::MAX as usize, Ordering::SeqCst);

        let chain = Chain {
            blooms_config: blooms_config,
            current_header: RwLock::new(header.clone()),
            is_sync: AtomicBool::new(false),
            max_height: max_height,
            //TODO need to get saved body
            max_store_height: max_store_height,
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
            polls_filter: Arc::new(Mutex::new(PollManager::default())),
            nodes: RwLock::new(Vec::new()),
            block_gas_limit: AtomicUsize::new(18_446_744_073_709_551_615),
            account_gas_limit: RwLock::new(ProtoAccountGasLimit::new()),
            check_prooftype: sc.check_prooftype,
        };

        chain
    }

    /// Get block number by BlockId
    fn block_number(&self, id: BlockId) -> Option<BlockNumber> {
        match id {
            BlockId::Number(number) => Some(number),
            BlockId::Hash(hash) => self.block_height_by_hash(hash),
            BlockId::Earliest => Some(0),
            BlockId::Latest => Some(self.get_current_height()),
        }
    }

    /*fn write_cache<K, T>(&mut self, cache: &mut HashMap<K, T>, key: K, value: T)
    {
        cache.insert(key, value);
    }*/

    pub fn block_height_by_hash(&self, hash: H256) -> Option<BlockNumber> {
        let result = self.db
            .read_with_cache(db::COL_EXTRA, &self.block_hashes, &hash);
        //self.cache_man.lock().note_used(CacheId::BlockHashes(index));
        result
    }

    pub fn set_excuted_result_genesis(&self, ret: &ExecutedResult) {
        let blk = Block::default();
        self.set_db_result(ret, &blk);
    }

    pub fn set_db_config(&self, ret: &ExecutedResult) {
        let conf = ret.get_config();
        let nodes = conf.get_nodes();
        let nodes: Vec<Address> = nodes
            .into_iter()
            .map(|vecaddr| Address::from_slice(&vecaddr[..]))
            .collect();
        info!("consensus nodes {:?}", nodes);
        self.set_excuted_config(
            conf.get_block_gas_limit(),
            conf.get_account_gas_limit(),
            &nodes,
        );
    }

    pub fn set_db_result(&self, ret: &ExecutedResult, block: &Block) {
        //config set in memory
        self.set_db_config(ret);

        let info = ret.get_executed_info();
        let number = info.get_header().get_height();
        let mut hdr = Header::new();
        let log_bloom = H2048::from(info.get_header().get_log_bloom());
        hdr.set_gas_limit(U256::from(info.get_header().get_gas_limit()));
        hdr.set_gas_used(U256::from(info.get_header().get_gas_used()));
        hdr.set_number(number);
        //        hdr.set_parent_hash(*block.parent_hash());
        hdr.set_parent_hash(H256::from_slice(info.get_header().get_prevhash()));
        hdr.set_receipts_root(H256::from(info.get_header().get_receipts_root()));
        hdr.set_state_root(H256::from(info.get_header().get_state_root()));
        hdr.set_timestamp(info.get_header().get_timestamp());
        hdr.set_transactions_root(H256::from(info.get_header().get_transactions_root()));
        hdr.set_log_bloom(log_bloom.clone());
        hdr.set_proof(block.proof().clone());

        let hash = hdr.hash();
        let blocks_blooms: HashMap<LogGroupPosition, BloomGroup> = if log_bloom.is_zero() {
            HashMap::new()
        } else {
            let bgroup = bc::group::BloomGroupChain::new(self.blooms_config, self);
            bgroup
                .insert(number as bc::Number, Bloom::from(log_bloom).into())
                .into_iter()
                .map(|p| (From::from(p.0), From::from(p.1)))
                .collect()
        };

        let mut batch = DBTransaction::new();
        if info.get_receipts().len() > 0 {
            let receipts: Vec<Option<Receipt>> = info.get_receipts()
                .into_iter()
                .map(|receipt_with_option| {
                    let mut receipt = None;
                    if receipt_with_option.receipt.is_some() {
                        receipt = Some(Receipt::from(receipt_with_option.get_receipt().clone()));
                    }
                    receipt
                })
                .collect();

            let block_receipts = BlockReceipts::new(receipts.clone());
            let mut write_receipts = self.block_receipts.write();
            batch.write_with_cache(
                db::COL_EXTRA,
                &mut *write_receipts,
                hash,
                block_receipts,
                CacheUpdatePolicy::Overwrite,
            );
        }
        if info.get_transactions().len() > 0 {
            let transactions: HashMap<H256, TransactionAddress> = info.get_transactions()
                .into_iter()
                .map(|(k, v)| {
                    let block_hash = H256::from_slice(v.get_block_hash());
                    let address = TransactionAddress {
                        block_hash: block_hash,
                        index: v.index as usize,
                    };
                    let k = H256::from_str(k).unwrap();
                    (k, address)
                })
                .collect();

            let mut write_txs = self.transaction_addresses.write();
            batch.extend_with_cache(
                db::COL_EXTRA,
                &mut *write_txs,
                transactions.clone(),
                CacheUpdatePolicy::Overwrite,
            );
        }

        let mut write_headers = self.block_headers.write();
        let mut write_bodies = self.block_bodies.write();
        let mut write_blooms = self.blocks_blooms.write();
        let mut write_hashes = self.block_hashes.write();

        batch.write_with_cache(
            db::COL_HEADERS,
            &mut *write_headers,
            number as BlockNumber,
            hdr.clone(),
            CacheUpdatePolicy::Overwrite,
        );
        let mheight = self.max_store_height.load(Ordering::SeqCst) as u64;
        if mheight != ::std::u64::MAX && mheight < number {
            batch.write_with_cache(
                db::COL_BODIES,
                &mut *write_bodies,
                number,
                block.body().clone(),
                CacheUpdatePolicy::Overwrite,
            );
        }
        self.max_height.store(number as usize, Ordering::SeqCst);
        batch.write_with_cache(
            db::COL_EXTRA,
            &mut *write_hashes,
            hash,
            number as BlockNumber,
            CacheUpdatePolicy::Overwrite,
        );
        batch.extend_with_cache(
            db::COL_EXTRA,
            &mut *write_blooms,
            blocks_blooms.clone(),
            CacheUpdatePolicy::Overwrite,
        );
        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        self.db.write(batch).expect("DB write failed.");
        {
            *self.current_header.write() = hdr;
        }
    }

    pub fn broadcast_current_block(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let mheight = self.max_store_height.load(Ordering::SeqCst) as u64;
        let height = self.max_height.load(Ordering::SeqCst) as u64;

        if mheight > height {
            let body = self.db
                .read_with_cache(db::COL_BODIES, &self.block_bodies, &mheight);
            if let Some(blockbody) = body {
                let mut block = Block::new();
                block.set_body(blockbody);

                let mut blocks = vec![];
                blocks.push(block.protobuf());

                let mut sync_res = SyncResponse::new();
                sync_res.set_blocks(RepeatedField::from_vec(blocks));
                let msg = factory::create_msg(
                    submodules::CHAIN,
                    topics::NEW_BLK,
                    communication::MsgType::SYNC_RES,
                    sync_res.write_to_bytes().unwrap(),
                );
                ctx_pub
                    .clone()
                    .send(("net.blk".to_string(), msg.write_to_bytes().unwrap()))
                    .unwrap();
            }
        }
    }

    pub fn broadcast_current_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        self.delivery_current_rich_status(&ctx_pub);
        if !self.is_sync.load(Ordering::SeqCst) {
            self.broadcast_status(&ctx_pub);
        }
    }

    pub fn set_excuted_result(&self, ret: &ExecutedResult, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let info = ret.get_executed_info();
        let number = info.get_header().get_height();
        if number == 0 {
            if self.max_store_height.load(Ordering::SeqCst) as u64 == ::std::u64::MAX {
                self.set_excuted_result_genesis(ret);
            }
            let block_tx_hashes = Vec::new();
            self.delivery_block_tx_hashes(number, block_tx_hashes, &ctx_pub);
            self.broadcast_current_status(&ctx_pub);
            return;
        }

        if number == self.get_current_height() {
            self.set_db_config(ret);
            self.broadcast_current_status(&ctx_pub);
            return;
        }

        let block_in_queue: Option<BlockInQueue>;
        //get block saved
        {
            let block_map = self.block_map.read();
            block_in_queue = block_map.get(&number).cloned();
        }

        match block_in_queue {
            Some(BlockInQueue::ConsensusBlock(block, _)) => {
                if self.validate_height(block.number()) && self.validate_hash(block.parent_hash()) {
                    self.set_db_result(&ret, &block);
                    self.broadcast_current_status(&ctx_pub);
                    debug!("set consensus block-{}", number);
                }
            }
            Some(BlockInQueue::SyncBlock((block, op))) => {
                if let Some(_) = op {
                    debug!("SyncBlock has proof in  {} ", block.number());
                } else {
                    debug!("SyncBlock not has proof in  {}", block.number());
                }
                if number == self.get_current_height() + 1 {
                    self.set_db_result(&ret, &block);
                    self.is_sync.store(true, Ordering::SeqCst);
                    self.broadcast_current_status(&ctx_pub);
                    self.is_sync.store(false, Ordering::SeqCst);
                    debug!("finish sync blocks to {}", number);
                };
            }
            _ => {
                debug!("block-{} in queue is invalid", number);
            }
        }

        let mut guard = self.block_map.write();
        let new_map = guard.split_off(&self.get_current_height());
        *guard = new_map;
    }

    pub fn set_excuted_config(&self, bgas_limit: u64, agas_limit: &ProtoAccountGasLimit, nodes: &Vec<Address>) {
        self.block_gas_limit
            .store(bgas_limit as usize, Ordering::SeqCst);
        *self.account_gas_limit.write() = agas_limit.clone();
        *self.nodes.write() = nodes.clone();
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
        self.block_height_by_hash(hash)
            .map_or(None, |h| self.block_by_height(h))
    }

    /// Get block by height
    pub fn block_by_height(&self, number: BlockNumber) -> Option<Block> {
        match (
            self.block_header_by_height(number),
            self.block_body_by_height(number),
        ) {
            (Some(h), Some(b)) => Some(Block { header: h, body: b }),
            _ => None,
        }
    }

    /// Get block header by BlockId
    pub fn block_header(&self, id: BlockId) -> Option<Header> {
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
        self.block_height_by_hash(hash)
            .map_or(None, |h| self.block_header_by_height(h))
    }

    fn block_header_by_height(&self, idx: BlockNumber) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.number() == idx {
                return Some(header.clone());
            }
        }
        let result = self.db
            .read_with_cache(db::COL_HEADERS, &self.block_headers, &idx);
        //self.cache_man.lock().note_used(CacheId::BlockHeaders(hash));
        result
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

    pub fn block_hash_by_height(&self, height: BlockNumber) -> Option<H256> {
        self.block_header_by_height(height)
            .map_or(None, |hdr| Some(hdr.hash()))
    }
    // Get block body by hash
    fn block_body_by_hash(&self, hash: H256) -> Option<BlockBody> {
        self.block_height_by_hash(hash)
            .map_or(None, |h| self.block_body_by_height(h))
    }

    /// Get block body by height
    fn block_body_by_height(&self, number: BlockNumber) -> Option<BlockBody> {
        let result = self.db
            .read_with_cache(db::COL_BODIES, &self.block_bodies, &number);
        //self.cache_man.lock().note_used(CacheId::BlockHeaders(hash));
        result
    }

    /// Get block tx hashes
    pub fn block_tx_hashes(&self, number: BlockNumber) -> Option<Vec<H256>> {
        self.block_body_by_height(number)
            .map(|body| body.transaction_hashes())
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
            .read_with_cache(db::COL_EXTRA, &self.transaction_addresses, &hash);
        self.cache_man
            .lock()
            .note_used(CacheId::TransactionAddresses(hash));
        result
    }

    /// Get transaction by address
    fn transaction_by_address(&self, hash: H256, index: usize) -> Option<SignedTransaction> {
        self.block_body_by_hash(hash)
            .map(|body| body.transactions()[index].clone())
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

        receipts.retain(|r| r.is_some());

        let prior_gas_used = match receipts.last() {
            Some(&Some(ref r)) => r.gas_used,
            _ => 0.into(),
        };

        let no_of_logs = receipts
            .iter()
            .fold(0, |acc, r| acc + r.as_ref().unwrap().logs.len());

        last_receipt.and_then(|last_receipt| {
            // Get sender
            let stx = self.transaction_by_address(hash, index).unwrap();
            let number = self.block_height_by_hash(hash).unwrap_or(0);

            let contract_address = match *stx.action() {
                Action::Create => Some(contract_address(stx.sender(), stx.account_nonce())),
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
                logs: last_receipt
                    .logs
                    .into_iter()
                    .enumerate()
                    .map(|(i, log)| LocalizedLogEntry {
                        entry: log,
                        block_hash: hash,
                        block_number: number,
                        transaction_hash: id,
                        transaction_index: index,
                        transaction_log_index: i,
                        log_index: no_of_logs + i,
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

    pub fn get_max_store_height(&self) -> u64 {
        self.max_store_height.load(Ordering::SeqCst) as u64
    }

    pub fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
    }

    pub fn current_block_poof(&self) -> Option<ProtoProof> {
        self.db.read(db::COL_EXTRA, &CurrentProof)
    }

    pub fn save_current_block_poof(&self,proof: ProtoProof) {
        let mut batch = DBTransaction::new();
        batch.write(db::COL_EXTRA, &CurrentProof, &proof);
        self.db.write(batch).expect("save_current_block_poof DB write failed.");
    }

    pub fn get_chain_prooftype(&self) -> Option<ProofType> {
        match self.check_prooftype {
            0 => Some(ProofType::AuthorityRound),
            1 => Some(ProofType::Raft),
            2 => Some(ProofType::Tendermint),
            _ => None,
        }
    }

    pub fn logs<F>(&self, mut blocks: Vec<BlockNumber>, matches: F, limit: Option<usize>) -> Vec<LocalizedLogEntry>
    where
        F: Fn(&LogEntry) -> bool,
        Self: Sized,
    {
        // sort in reverse order
        blocks.sort_by(|a, b| b.cmp(a));

        let mut log_index = 0;
        let mut logs = blocks
            .into_iter()
            .filter_map(|number| self.block_hash_by_height(number).map(|hash| (number, hash)))
            .filter_map(|(number, hash)| {
                self.block_receipts(hash)
                    .map(|r| (number, hash, r.receipts))
            })
            .filter_map(|(number, hash, receipts)| {
                self.block_body_by_hash(hash)
                    .map(|ref b| (number, hash, receipts, b.transaction_hashes()))
            })
            .flat_map(|(number, hash, mut receipts, mut hashes)| {
                if receipts.len() != hashes.len() {
                    warn!(
                        "Block {} ({}) has different number of receipts ({}) to transactions ({}). Database corrupt?",
                        number,
                        hash,
                        receipts.len(),
                        hashes.len()
                    );
                    assert!(false);
                }
                log_index = receipts.iter().fold(0, |sum, receipt| {
                    sum + receipt.as_ref().map_or(0, |r| r.logs.len())
                });

                let receipts_len = receipts.len();
                hashes.reverse();
                receipts.reverse();
                receipts
                    .into_iter()
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
        chain
            .with_bloom(&range, &Bloom::from(*bloom).into())
            .into_iter()
            .map(|b| b as BlockNumber)
            .collect()
    }

    /// Returns numbers of blocks containing given bloom by blockId.
    pub fn blocks_with_bloom_by_id(
        &self,
        bloom: &H2048,
        from_block: BlockId,
        to_block: BlockId,
    ) -> Option<Vec<BlockNumber>> {
        match (self.block_number(from_block), self.block_number(to_block)) {
            (Some(from), Some(to)) => Some(self.blocks_with_bloom(bloom, from, to)),
            _ => None,
        }
    }

    pub fn get_logs(&self, filter: Filter) -> Vec<LocalizedLogEntry> {
        let blocks = filter.bloom_possibilities().iter()
            .filter_map(|bloom| self.blocks_with_bloom_by_id(bloom, filter.from_block, filter.to_block))
            .flat_map(|m| m)
            // remove duplicate elements
            .collect::<HashSet<u64>>()
            .into_iter()
            .collect::<Vec<u64>>();

        self.logs(blocks, |entry| filter.matches(entry), filter.limit)
    }

    /// Delivery block tx hashes to auth
    pub fn delivery_block_tx_hashes(
        &self,
        block_height: u64,
        tx_hashes: Vec<H256>,
        ctx_pub: &Sender<(String, Vec<u8>)>,
    ) {
        if block_height == ::std::u64::MAX {
            return;
        }

        let ctx_pub_clone = ctx_pub.clone();
        let mut block_tx_hashes = BlockTxHashes::new();
        block_tx_hashes.set_height(block_height);
        {
            //Need
            block_tx_hashes.set_block_gas_limit(self.block_gas_limit.load(Ordering::SeqCst) as u64);
            block_tx_hashes.set_account_gas_limit(self.account_gas_limit.read().clone().into());
        }
        thread::spawn(move || {
            let mut tx_hashes_in_u8 = Vec::new();
            for tx_hash_in_h256 in &tx_hashes {
                tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());
            }
            block_tx_hashes.set_tx_hashes(RepeatedField::from_slice(&tx_hashes_in_u8[..]));
            let msg = factory::create_msg(
                submodules::CHAIN,
                topics::BLOCK_TXHASHES,
                communication::MsgType::BLOCK_TXHASHES,
                block_tx_hashes.write_to_bytes().unwrap(),
            );

            ctx_pub_clone
                .send(("chain.txhashes".to_string(), msg.write_to_bytes().unwrap()))
                .unwrap();
            trace!("delivery block's tx hashes for height: {}", block_height);
        });
    }

    /// Delivery current rich status
    pub fn delivery_current_rich_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        if self.max_height.load(Ordering::SeqCst) as u64 == ::std::u64::MAX {
            return;
        }
        let header = &*self.current_header.read();
        self.delivery_rich_status(header, ctx_pub);
    }

    /// Delivery rich status to consensus
    /// Consensus should resend block if chain commit block failed.
    fn delivery_rich_status(&self, header: &Header, ctx_pub: &Sender<(String, Vec<u8>)>) {
        if self.nodes.read().is_empty() {
            return;
        }
        let current_hash = header.hash();
        let current_height = header.number();
        let nodes: Vec<Address> = self.nodes.read().clone();

        let mut rich_status = ProtoRichStatus::new();
        rich_status.set_hash(current_hash.0.to_vec());
        rich_status.set_height(current_height);
        let node_list = nodes.into_iter().map(|address| address.to_vec()).collect();
        rich_status.set_nodes(RepeatedField::from_vec(node_list));

        let msg = factory::create_msg(
            submodules::CHAIN,
            topics::RICH_STATUS,
            communication::MsgType::RICH_STATUS,
            rich_status.write_to_bytes().unwrap(),
        );
        ctx_pub
            .send((
                "chain.richstatus".to_string(),
                msg.write_to_bytes().unwrap(),
            ))
            .unwrap();
    }

    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: H256) -> Option<BlockReceipts> {
        let result = self.db
            .read_with_cache(db::COL_EXTRA, &self.block_receipts, &hash);
        self.cache_man
            .lock()
            .note_used(CacheId::BlockReceipts(hash));
        result
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, address: &TransactionAddress) -> Option<Receipt> {
        self.block_receipts(address.block_hash)
            .map_or(None, |r| r.receipts[address.index].clone())
    }

    /// Current status
    fn current_status(&self) -> Status {
        let mut status = Status::default();
        status.set_hash(self.get_current_hash());
        status.set_number(self.get_current_height());
        status
    }

    /// Attempt to get a copy of a specific block's final state.
    pub fn state_at(&self, id: BlockId) -> Option<State<StateDB>> {
        self.block_header(id)
            .map_or(None, |h| self.gen_state(*h.state_root()))
    }

    /// generate block's final state.
    pub fn gen_state(&self, root: H256) -> Option<State<StateDB>> {
        let db = self.state_db.boxed_clone();
        State::from_existing(db, root).ok()
    }

    pub fn validate_hash(&self, block_hash: &H256) -> bool {
        let current_hash = self.get_current_hash();
        trace!(
            "validate_hash current_hash {:?} block_hash {:?}",
            current_hash,
            block_hash
        );
        current_hash == *block_hash
    }

    pub fn validate_height(&self, block_number: u64) -> bool {
        let current_height = self.get_current_height();
        trace!(
            "validate_height current_height {:?} block_number {:?}",
            current_height,
            block_number - 1
        );
        current_height + 1 == block_number
    }

    // Get the height of proof.
    pub fn get_block_proof_height(block: &Block) -> usize {
        match block.proof_type() {
            Some(ProofType::Tendermint) => {
                let proof = TendermintProof::from(block.proof().clone());
                //block height 1's proof is height MAX
                if proof.height == ::std::usize::MAX {
                    return 0;
                }
                proof.height
            }
            _ => block.number() as usize,
        }
    }

    /// Broadcast new status
    pub fn broadcast_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        if self.max_height.load(Ordering::SeqCst) as u64 == ::std::u64::MAX {
            return;
        }
        let status = self.current_status().protobuf();
        let sync_msg = factory::create_msg(
            submodules::CHAIN,
            topics::NEW_STATUS,
            communication::MsgType::STATUS,
            status.write_to_bytes().unwrap(),
        );
        info!(
            "chain.status {:?}, {:?}",
            status.get_height(),
            status.get_hash()
        );
        ctx_pub
            .send((
                "chain.status".to_string(),
                sync_msg.write_to_bytes().unwrap(),
            ))
            .unwrap();
    }

    pub fn set_block_body(&self, height: u64, block: &Block) {
        let mut batch = DBTransaction::new();
        {
            let mut write_bodies = self.block_bodies.write();
            batch.write_with_cache(
                db::COL_BODIES,
                &mut *write_bodies,
                height as BlockNumber,
                block.body().clone(),
                CacheUpdatePolicy::Overwrite,
            );
        }
        let _ = self.db.write(batch);
    }

    pub fn set_block(&self, block: Block, ctx_pub: &Sender<(String, Vec<u8>)>) {
        // Delivery block tx hashes to auth
        let height = block.number();
        let tx_hashes = block.body().transaction_hashes();
        self.delivery_block_tx_hashes(height, tx_hashes, ctx_pub);

        //Need to send block to
        //let closed_block = self.execute_block(block);
        //self.finalize_block(closed_block, ctx_pub);
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
            blocks: self.block_headers.read().heap_size_of_children()
                + self.block_bodies.read().heap_size_of_children(),
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
                    /*CacheId::BlockHeaders(_) => {
                        //block_headers.remove(h);
                    }
                    CacheId::BlockBodies(_) => {
                        //block_bodies.remove(h);
                    }
                    CacheId::BlockHashes(_) => {
                        //block_hashes.remove(h);
                    }*/
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

            block_headers.heap_size_of_children() + block_bodies.heap_size_of_children()
                + block_hashes.heap_size_of_children() + transaction_addresses.heap_size_of_children()
                + blocks_blooms.heap_size_of_children() + block_receipts.heap_size_of_children()
        });
    }

    pub fn poll_filter(&self) -> Arc<Mutex<PollManager<PollFilter>>> {
        Arc::clone(&self.polls_filter)
    }
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize;

    use self::rustc_serialize::hex::FromHex;
    use super::*;
    use std::env;
    use std::sync::mpsc::channel;
    use std::thread;
    use test::Bencher;
    use tests::helpers::{bench_chain, create_block, init_chain, solc};
    use util::{Address, H256};

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

        block_receipts.insert(
            H256::from("000000000000000000000000000000000000000000000000123456789abcdef0"),
            BlockReceipts::new(vec![]),
        );
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

        let block = create_block(&chain, Address::from(0), &data, (0, 2));
        let (ctx_pub, recv) = channel();
        thread::spawn(move || loop {
            let _ = recv.recv();
        });
        chain.set_block(block.clone(), &ctx_pub);

        let tx = &block.body.transactions[0];
        let txhash = tx.hash();
        let receipt = chain.localized_receipt(txhash).unwrap();

        let contract_address = receipt.contract_address.unwrap();
        println!("contract address: {}", contract_address);
        let code = chain.code_at(&contract_address, BlockId::Latest);
        assert!(code.is_some());
        assert!(code.unwrap().is_some());

        let tx1 = &block.body.transactions[1];
        let tx1hash = tx1.hash();
        let receipt1 = chain.localized_receipt(tx1hash).unwrap();

        let contract_address1 = receipt1.contract_address.unwrap();
        assert!(contract_address != contract_address1);
    }
}
