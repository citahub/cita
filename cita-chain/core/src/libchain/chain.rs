// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use basic_types::{LogBloom, LogBloomGroup};
use bloomchain::group::{
    BloomGroup, BloomGroupChain, BloomGroupDatabase, GroupPosition as BloomGroupPosition,
};
use bloomchain::{Bloom, Config as BloomChainConfig, Number as BloomChainNumber};
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

use libproto::blockchain::{
    AccountGasLimit as ProtoAccountGasLimit, Proof as ProtoProof, ProofType,
    RichStatus as ProtoRichStatus, StateSignal,
};

use cita_types::traits::LowerHex;
use cita_types::{Address, H256, U256};
use header::Header;
use libproto::executor::ExecutedResult;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::{BlockTxHashes, FullTransaction, Message};
use proof::BftProof;
use receipt::{LocalizedReceipt, Receipt};
use rlp::{self, Encodable};
use state_db::StateDB;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{Into, TryInto};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use types::filter::Filter;
use types::ids::{BlockId, TransactionId};
use types::log_entry::{LocalizedLogEntry, LogEntry};
use types::transaction::{Action, SignedTransaction};
use util::journaldb;
use util::kvdb::*;
use util::merklehash;
use util::Hashable;
use util::HeapSizeOf;
use util::{Mutex, RwLock};

pub const VERSION: u32 = 0;
const LOG_BLOOMS_LEVELS: usize = 3;
const LOG_BLOOMS_ELEMENTS_PER_INDEX: usize = 16;

#[derive(Debug, Clone)]
pub struct RelayInfo {
    pub from_chain_id: u32,
    pub to_chain_id: u32,
    pub dest_contract: Address,
    pub dest_hasher: [u8; 4],
    pub cross_chain_nonce: u64,
}

#[derive(Debug, Clone, RlpEncodable, RlpDecodable)]
pub struct TxProof {
    tx: SignedTransaction,
    receipt: Receipt,
    receipt_proof: merklehash::MerkleProof,
    block_header: Header,
    next_proposal_header: Header,
    proposal_proof: ProtoProof,
}

impl TxProof {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        rlp::decode(bytes)
    }

    pub fn verify(&self, authorities: &[Address]) -> bool {
        // Calculate transaction hash, and it should be same as the transaction hash in receipt.
        if self.receipt.transaction_hash == self.tx.calc_transaction_hash() {
        } else {
            warn!("txproof verify transaction_hash failed");
            return false;
        };
        // Use receipt_proof and receipt_root to prove the receipt in the block.
        if merklehash::verify_proof(
            *self.block_header.receipts_root(),
            &self.receipt_proof,
            self.receipt.clone().rlp_bytes().into_vec().crypt_hash(),
        ) {
        } else {
            warn!("txproof verify receipt root merklehash failed");
            return false;
        };
        // Calculate block header hash, and is should be same as the parent_hash in next header
        if self.block_header.note_dirty().hash() == *self.next_proposal_header.parent_hash() {
        } else {
            warn!("txproof verify block header hash failed");
            return false;
        };
        let third_proof = BftProof::from(self.proposal_proof.clone());
        // Verify next block header, use proof.proposal
        if self.next_proposal_header.proposal_protobuf().crypt_hash() == third_proof.proposal {
        } else {
            warn!("txproof verify next block header failed");
            return false;
        };
        // Verify signatures in proposal proof.
        if third_proof.check(self.block_header.number() as usize + 1, authorities) {
        } else {
            warn!("txproof verify signatures for next block header failed");
            return false;
        };
        true
    }

    // extract info which relayer needed
    pub fn extract_relay_info(&self) -> Option<RelayInfo> {
        if self.receipt.logs.is_empty() {
            return None;
        }
        let data = &self.receipt.logs[0].data;
        // data must be:
        // uint256 from_chain_id,
        // uint256 to_chain_id,
        // address dest_contract,
        // bytes4 dest_hasher,
        // uint256 cross_chain_nonce
        if data.len() != 160 {
            None
        } else {
            let mut iter = data.chunks(32);
            let from_chain_id = U256::from(iter.next().unwrap()).low_u32();
            let to_chain_id = U256::from(iter.next().unwrap()).low_u32();
            let dest_contract = Address::from(H256::from(iter.next().unwrap()));
            let dest_hasher = iter.next().unwrap()[..4]
                .into_iter()
                .take(4)
                .enumerate()
                .fold([0u8; 4], |mut acc, (idx, val)| {
                    acc[idx] = *val;
                    acc
                });
            let cross_chain_nonce = U256::from(iter.next().unwrap()).low_u64();
            Some(RelayInfo {
                from_chain_id,
                to_chain_id,
                dest_contract,
                dest_hasher,
                cross_chain_nonce,
            })
        }
    }

    // verify proof
    // check as crosschain protocol
    // extract sender and tx data
    pub fn extract_crosschain_data(
        &self,
        my_contrac_addr: Address,
        my_hasher: [u8; 4],
        my_cross_chain_nonce: u64,
        my_chain_id: u32,
        authorities: &[Address],
    ) -> Option<(Address, Vec<u8>)> {
        if self.verify(authorities) {
            self.extract_relay_info().and_then(
                |RelayInfo {
                     to_chain_id,
                     dest_contract,
                     dest_hasher,
                     cross_chain_nonce,
                     ..
                 }| {
                    // from_chain_id: if we can got authorities, the from_chain_id must be right
                    // cross chain only between main chain and one sidechain
                    // check to_chain_id == my chain_id
                    // check dest_contract == this
                    // check hasher == RECV_FUNC_HASHER
                    // check cross_chain_nonce == cross_chain_nonce
                    // extract origin tx sender and origin tx data
                    if to_chain_id == my_chain_id
                        && dest_contract == my_contrac_addr
                        && dest_hasher == my_hasher
                        && cross_chain_nonce == my_cross_chain_nonce
                    {
                        // sendToSideChain(uint32 toChainId, address destContract, bytes txData)
                        // skip func hasher, uint32, address, bytes position and length
                        let (_, origin_tx_data) = self.tx.data.split_at(4 + 32 * 4);
                        Some((*self.tx.sender(), origin_tx_data.to_owned()))
                    } else {
                        None
                    }
                },
            )
        } else {
            None
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum BlockSource {
    CONSENSUS = 0,
    NET = 1,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum CacheId {
    BlockHeaders(BlockNumber),
    BlockBodies(BlockNumber),
    BlockHashes(H256),
    TransactionAddresses(H256),
    BlocksBlooms(LogGroupPosition),
    BlockReceipts(H256),
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Config {
    pub prooftype: u8,
    pub cache_size: Option<usize>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            prooftype: 2,
            cache_size: Some(1 << 20),
        }
    }

    pub fn new(path: &str) -> Self {
        let mut c: Config = parse_config!(Config, path);
        if c.cache_size.is_none() {
            c.cache_size = Some(1 << 20 as usize);
        }
        c
    }
}

impl BloomGroupDatabase for Chain {
    fn blooms_at(&self, position: &BloomGroupPosition) -> Option<BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self
            .db
            .read()
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

pub struct Chain {
    pub blooms_config: BloomChainConfig,
    pub current_header: RwLock<Header>,
    // Chain current height
    pub current_height: AtomicUsize,
    // Max height in block map
    pub max_store_height: AtomicUsize,
    pub block_map: RwLock<BTreeMap<u64, BlockInQueue>>,
    pub proof_map: RwLock<BTreeMap<u64, ProtoProof>>,
    pub db: RwLock<Arc<KeyValueDB>>,
    pub state_db: RwLock<StateDB>,

    // block cache
    pub block_headers: RwLock<HashMap<BlockNumber, Header>>,
    pub block_bodies: RwLock<HashMap<BlockNumber, BlockBody>>,

    // extra caches
    pub block_hashes: RwLock<HashMap<H256, BlockNumber>>,
    pub transaction_addresses: RwLock<HashMap<TransactionId, TransactionAddress>>,
    pub blocks_blooms: RwLock<HashMap<LogGroupPosition, LogBloomGroup>>,
    pub block_receipts: RwLock<HashMap<H256, BlockReceipts>>,
    pub nodes: RwLock<Vec<Address>>,
    pub block_interval: RwLock<u64>,

    pub block_quota_limit: AtomicUsize,
    pub account_quota_limit: RwLock<ProtoAccountGasLimit>,
    pub check_quota: AtomicBool,

    pub cache_man: Mutex<CacheManager<CacheId>>,
    pub polls_filter: Arc<Mutex<PollManager<PollFilter>>>,

    /// Proof type
    pub prooftype: u8,

    // snapshot flag
    pub is_snapshot: RwLock<bool>,

    admin_address: RwLock<Option<Address>>,

    pub version: RwLock<Option<u32>>,
}

/// Get latest status
pub fn get_chain(db: &KeyValueDB) -> Option<Header> {
    // CANNOT replace CurrentHash & hash with CurrentHeight to get current_height,
    // because CurrentHeight is set after BlockBody is stored, and CurrentHash is set after BlockHeader is stored.
    let h: Option<H256> = db.read(db::COL_EXTRA, &CurrentHash);
    if let Some(hash) = h {
        let hi: Option<BlockNumber> = db.read(db::COL_EXTRA, &hash);
        if let Some(h) = hi {
            trace!("get_chain hash {:?}  bn{:?}  CurrentHash", hash, h);
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

pub fn get_chain_body_height(db: &KeyValueDB) -> Option<BlockNumber> {
    db.read(db::COL_EXTRA, &CurrentHeight)
}

pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    use rlp::RlpStream;

    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
}

impl Chain {
    pub fn init_chain(db: Arc<KeyValueDB>, chain_config: &Config) -> Chain {
        info!("chain config: {:?}", chain_config);

        // 400 is the avarage size of the key
        let cache_man = CacheManager::new(
            chain_config.cache_size.unwrap() * 3 / 4,
            chain_config.cache_size.unwrap(),
            400,
        );

        let journal_db = journaldb::new(Arc::clone(&db), journaldb::Algorithm::Archive, COL_STATE);
        let state_db = StateDB::new(journal_db);
        let blooms_config = BloomChainConfig {
            levels: LOG_BLOOMS_LEVELS,
            elements_per_index: LOG_BLOOMS_ELEMENTS_PER_INDEX,
        };

        let header = get_chain(&*db).unwrap_or_default();
        debug!("get chain head is : {:?}", header);
        let current_height = AtomicUsize::new(header.number() as usize);
        let max_store_height = AtomicUsize::new(0);
        if let Some(height) = get_chain_body_height(&*db) {
            max_store_height.store(height as usize, Ordering::SeqCst);
        }

        info!(
            "get chain max_store_height : {:?}  current_height: {:?}",
            max_store_height, current_height
        );

        let chain = Chain {
            blooms_config,
            current_header: RwLock::new(header.clone()),
            current_height,
            max_store_height,
            block_map: RwLock::new(BTreeMap::new()),
            block_headers: RwLock::new(HashMap::new()),
            block_bodies: RwLock::new(HashMap::new()),
            block_hashes: RwLock::new(HashMap::new()),
            transaction_addresses: RwLock::new(HashMap::new()),
            blocks_blooms: RwLock::new(HashMap::new()),
            block_receipts: RwLock::new(HashMap::new()),
            cache_man: Mutex::new(cache_man),
            db: RwLock::new(db),
            state_db: RwLock::new(state_db),
            polls_filter: Arc::new(Mutex::new(PollManager::default())),
            nodes: RwLock::new(Vec::new()),
            // need to be cautious here
            // because it's not read from the config file
            block_interval: RwLock::new(3000),
            block_quota_limit: AtomicUsize::new(18_446_744_073_709_551_615),
            account_quota_limit: RwLock::new(ProtoAccountGasLimit::new()),
            check_quota: AtomicBool::new(false),
            prooftype: chain_config.prooftype,
            proof_map: RwLock::new(BTreeMap::new()),
            is_snapshot: RwLock::new(false),
            admin_address: RwLock::new(None),
            version: RwLock::new(None),
        };

        if let Some(proto_proof) = chain.current_block_poof() {
            if let Some(ProofType::Bft) = chain.get_chain_prooftype() {
                let proof = BftProof::from(proto_proof.clone());
                chain
                    .proof_map
                    .write()
                    .insert(proof.height as u64, proto_proof);
            }
        }
        chain
    }

    /// Get block number by BlockId
    fn block_number(&self, id: BlockId) -> Option<BlockNumber> {
        match id {
            BlockId::Number(number) => Some(number),
            BlockId::Hash(hash) => self.block_height_by_hash(hash),
            BlockId::Earliest => Some(0),
            BlockId::Latest => Some(self.get_latest_height()),
            BlockId::Pending => Some(self.get_pending_height()),
        }
    }

    pub fn get_proof_with_height(&self, height: u64) -> Option<ProtoProof> {
        self.proof_map.read().get(&height).cloned()
    }

    pub fn set_proof_with_height(&self, height: u64, proof: &ProtoProof) {
        self.proof_map.write().insert(height, proof.clone());
    }

    pub fn clean_proof_with_height(&self, height: u64) {
        let mut guard = self.proof_map.write();
        let new_map = guard.split_off(&height);
        *guard = new_map;
    }

    pub fn block_height_by_hash(&self, hash: H256) -> Option<BlockNumber> {
        let result = self
            .db
            .read()
            .read_with_cache(db::COL_EXTRA, &self.block_hashes, &hash);
        self.cache_man.lock().note_used(CacheId::BlockHashes(hash));
        result
    }

    fn set_config(&self, ret: &ExecutedResult) {
        let conf = ret.get_config();
        let nodes = conf.get_nodes();
        let nodes: Vec<Address> = nodes
            .into_iter()
            .map(|vecaddr| Address::from_slice(&vecaddr[..]))
            .collect();
        let block_interval = conf.get_block_interval();
        let version = conf.get_version();
        debug!(
            "consensus nodes {:?}, block_interval {:?}, version {}",
            nodes, block_interval, version
        );

        self.check_quota
            .store(conf.get_check_quota(), Ordering::Relaxed);
        self.block_quota_limit
            .store(conf.get_block_quota_limit() as usize, Ordering::SeqCst);
        *self.account_quota_limit.write() = conf.get_account_quota_limit().clone();
        *self.nodes.write() = nodes.clone();
        *self.block_interval.write() = block_interval;
        *self.admin_address.write() = if conf.get_admin_address().is_empty() {
            None
        } else {
            Some(Address::from(conf.get_admin_address()))
        };
        *self.version.write() = Some(version);
    }

    pub fn set_db_result(&self, ret: &ExecutedResult, block: &Block) {
        let info = ret.get_executed_info();
        let number = info.get_header().get_height();
        let version = block.version();
        let mut hdr = Header::new();
        let log_bloom = LogBloom::from(info.get_header().get_log_bloom());
        hdr.set_quota_limit(U256::from(info.get_header().get_quota_limit()));
        hdr.set_quota_used(U256::from(info.get_header().get_quota_used()));
        hdr.set_number(number);
        // hdr.set_parent_hash(*block.parent_hash());
        hdr.set_parent_hash(H256::from_slice(info.get_header().get_prevhash()));
        hdr.set_receipts_root(H256::from(info.get_header().get_receipts_root()));
        hdr.set_state_root(H256::from(info.get_header().get_state_root()));
        hdr.set_timestamp(info.get_header().get_timestamp());
        hdr.set_transactions_root(H256::from(info.get_header().get_transactions_root()));
        hdr.set_log_bloom(log_bloom);
        hdr.set_proof(block.proof().clone());
        hdr.set_proposer(Address::from(info.get_header().get_proposer()));
        hdr.set_version(version);

        let hash = hdr.hash();
        trace!(
            "commit block in db hash {:?}, height {:?}, version {}",
            hash,
            number,
            version
        );
        let block_transaction_addresses = block.transaction_addresses(hash);
        let blocks_blooms: HashMap<LogGroupPosition, LogBloomGroup> = if log_bloom.is_zero() {
            HashMap::new()
        } else {
            let bgroup = BloomGroupChain::new(self.blooms_config, self);
            bgroup
                .insert(
                    number as BloomChainNumber,
                    Bloom::from(Into::<[u8; 256]>::into(log_bloom)),
                )
                .into_iter()
                .map(|p| (From::from(p.0), From::from(p.1)))
                .collect()
        };

        let mut batch = DBTransaction::new();
        if !info.get_receipts().is_empty() {
            let receipts: Vec<Receipt> = info
                .get_receipts()
                .into_iter()
                .map(|receipt_with_option| Receipt::from(receipt_with_option.get_receipt().clone()))
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
            self.cache_man
                .lock()
                .note_used(CacheId::BlockReceipts(hash));
        }
        if !block_transaction_addresses.is_empty() {
            let mut write_txs = self.transaction_addresses.write();
            batch.extend_with_cache(
                db::COL_EXTRA,
                &mut *write_txs,
                block_transaction_addresses,
                CacheUpdatePolicy::Overwrite,
            );
            for key in block.body().transaction_hashes() {
                self.cache_man
                    .lock()
                    .note_used(CacheId::TransactionAddresses(key));
            }
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
        let mheight = self.get_max_store_height();
        if mheight < number || (number == 0 && mheight == 0) {
            batch.write_with_cache(
                db::COL_BODIES,
                &mut *write_bodies,
                number,
                block.body().clone(),
                CacheUpdatePolicy::Overwrite,
            );
        }
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

        //note used
        self.cache_man.lock().note_used(CacheId::BlockHashes(hash));
        self.cache_man
            .lock()
            .note_used(CacheId::BlockHeaders(number as BlockNumber));
        self.cache_man
            .lock()
            .note_used(CacheId::BlockBodies(number as BlockNumber));

        for (key, _) in blocks_blooms {
            self.cache_man.lock().note_used(CacheId::BlocksBlooms(key));
        }

        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        self.db.read().write(batch).expect("DB write failed.");
        {
            *self.current_header.write() = hdr;
        }
        self.current_height.store(number as usize, Ordering::SeqCst);
        self.clean_proof_with_height(number);
    }

    pub fn broadcast_current_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        self.delivery_current_rich_status(&ctx_pub);
        self.broadcast_status(&ctx_pub);
    }

    pub fn signal_to_executor(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let mut state_signal = StateSignal::new();
        state_signal.set_height(self.get_current_height());
        let msg: Message = state_signal.into();
        ctx_pub
            .send((
                routing_key!(Chain >> StateSignal).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    pub fn set_executed_result(&self, ret: &ExecutedResult, ctx_pub: &Sender<(String, Vec<u8>)>) {
        // Set config in memory
        self.set_config(ret);

        let info = ret.get_executed_info();
        let number = info.get_header().get_height();

        // Genesis block
        if number == 0 && self.get_current_height() == 0 {
            let blk = Block::default();
            self.set_db_result(ret, &blk);
            let block_tx_hashes = Vec::new();
            self.delivery_block_tx_hashes(number, &block_tx_hashes, &ctx_pub);
            self.broadcast_current_status(&ctx_pub);
            return;
        }

        // Duplicated block
        if number <= self.get_current_height() {
            self.broadcast_current_status(&ctx_pub);
            return;
        }

        // New block
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
                    let tx_hashes = block.body().transaction_hashes();
                    self.delivery_block_tx_hashes(number, &tx_hashes, &ctx_pub);
                    self.broadcast_current_status(&ctx_pub);
                    debug!("executed set consensus block-{}", number);
                }
            }
            Some(BlockInQueue::SyncBlock((block, op))) => {
                if op.is_some() {
                    debug!("SyncBlock has proof in  {} ", block.number());
                } else {
                    debug!("SyncBlock not has proof in  {}", block.number());
                }
                if number == self.get_current_height() + 1 {
                    if self.validate_hash(block.parent_hash()) {
                        self.set_db_result(&ret, &block);
                        let tx_hashes = block.body().transaction_hashes();
                        self.delivery_block_tx_hashes(number, &tx_hashes, &ctx_pub);
                        self.broadcast_current_status(&ctx_pub);
                        debug!("finish sync blocks to {}", number);
                    } else {
                        self.clear_block_map();
                        self.broadcast_current_status(&ctx_pub);
                    }
                } else {
                    self.signal_to_executor(&ctx_pub);
                    warn!(
                        "executor'ret is not continous,ret num {} current height {}",
                        number,
                        self.get_current_height()
                    );
                }
            }
            _ => {
                warn!("block-{} in queue is invalid", number);
            }
        }

        // Discard the blocks whose height is less than current height in 'block_map', reducing its' memory usage.
        let mut guard = self.block_map.write();
        let new_map = guard.split_off(&self.get_current_height());
        *guard = new_map;
    }

    /// Get block by BlockId
    pub fn block(&self, id: BlockId) -> Option<Block> {
        match id {
            BlockId::Hash(hash) => self.block_by_hash(hash),
            BlockId::Number(number) => self.block_by_height(number),
            BlockId::Earliest => self.block_by_height(0),
            BlockId::Latest => self.block_by_height(self.get_latest_height()),
            BlockId::Pending => self.block_by_height(self.get_pending_height()),
        }
    }

    /// Get block by hash
    pub fn block_by_hash(&self, hash: H256) -> Option<Block> {
        self.block_height_by_hash(hash)
            .and_then(|h| self.block_by_height(h))
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
            BlockId::Latest => self.block_header_by_height(self.get_latest_height()),
            BlockId::Pending => self.block_header_by_height(self.get_pending_height()),
        }
    }

    /// Get block header by hash
    pub fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.hash() == hash {
                return Some(header.clone());
            }
        }
        self.block_height_by_hash(hash)
            .and_then(|h| self.block_header_by_height(h))
    }

    fn block_header_by_height(&self, idx: BlockNumber) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.number() == idx {
                return Some(header.clone());
            }
        }
        let result = self
            .db
            .read()
            .read_with_cache(db::COL_HEADERS, &self.block_headers, &idx);
        self.cache_man.lock().note_used(CacheId::BlockHeaders(idx));
        result
    }

    /// Get block body by BlockId
    pub fn block_body(&self, id: BlockId) -> Option<BlockBody> {
        match id {
            BlockId::Hash(hash) => self.block_body_by_hash(hash),
            BlockId::Number(number) => self.block_body_by_height(number),
            BlockId::Earliest => self.block_body_by_height(0),
            BlockId::Latest => self.block_body_by_height(self.get_latest_height()),
            BlockId::Pending => self.block_body_by_height(self.get_pending_height()),
        }
    }

    pub fn block_hash_by_height(&self, height: BlockNumber) -> Option<H256> {
        self.block_header_by_height(height)
            .and_then(|hdr| Some(hdr.hash()))
    }

    /// Get block body by hash
    fn block_body_by_hash(&self, hash: H256) -> Option<BlockBody> {
        self.block_height_by_hash(hash)
            .and_then(|h| self.block_body_by_height(h))
    }

    /// Get block body by height
    fn block_body_by_height(&self, number: BlockNumber) -> Option<BlockBody> {
        let result = self
            .db
            .read()
            .read_with_cache(db::COL_BODIES, &self.block_bodies, &number);
        self.cache_man
            .lock()
            .note_used(CacheId::BlockBodies(number));
        result
    }

    /// Get block tx hashes
    pub fn block_tx_hashes(&self, number: BlockNumber) -> Option<Vec<H256>> {
        self.block_body_by_height(number)
            .map(|body| body.transaction_hashes())
    }

    /// Get transaction by hash
    pub fn transaction(&self, hash: TransactionId) -> Option<SignedTransaction> {
        self.transaction_address(hash).and_then(|addr| {
            let index = addr.index;
            let hash = addr.block_hash;
            self.transaction_by_address(hash, index)
        })
    }

    /// Get address of transaction by hash.
    fn transaction_address(&self, hash: TransactionId) -> Option<TransactionAddress> {
        let result =
            self.db
                .read()
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
        self.transaction_address(hash).and_then(|addr| {
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

    pub fn get_transaction_proof(&self, hash: TransactionId) -> Option<(Vec<u8>)> {
        self.transaction_address(hash)
            .and_then(|addr| {
                self.block_by_hash(addr.block_hash)
                    .map(|block| (addr, block))
            })
            .and_then(|(addr, block)| {
                self.block_receipts(addr.block_hash)
                    .map(|receipts| (addr.index, block, receipts))
            })
            .and_then(|(index, block, receipts)| {
                receipts
                    .receipts
                    .get(index)
                    .and_then(|receipt| {
                        if receipt.transaction_hash == hash {
                            Some(receipt)
                        } else {
                            None
                        }
                    })
                    .and_then(|receipt| {
                        merklehash::MerkleTree::from_bytes(
                            receipts.receipts.iter().map(|r| r.rlp_bytes().into_vec()),
                        )
                        .get_proof_by_input_index(index)
                        .map(|receipt_proof| (index, block, receipt.clone(), receipt_proof))
                    })
            })
            .and_then(|(index, block, receipt, receipt_proof)| {
                block
                    .body()
                    .transactions()
                    .get(index)
                    .map(|tx| (tx.clone(), receipt, receipt_proof, block.header().clone()))
            })
            .and_then(|(tx, receipt, receipt_proof, block_header)| {
                self.block_by_height(block_header.number() + 1)
                    .map(|next_block| {
                        (
                            tx,
                            receipt,
                            receipt_proof,
                            block_header,
                            next_block.header().proposal(),
                        )
                    })
            })
            .and_then(
                |(tx, receipt, receipt_proof, block_header, next_proposal_header)| {
                    self.block_by_height(next_proposal_header.number() + 1)
                        .map(|third_block| {
                            (
                                tx,
                                receipt,
                                receipt_proof,
                                block_header,
                                next_proposal_header,
                                third_block.header().proof().clone(),
                            )
                        })
                },
            )
            .map(
                |(
                    tx,
                    receipt,
                    receipt_proof,
                    block_header,
                    next_proposal_header,
                    proposal_proof,
                )| {
                    TxProof {
                        tx,
                        receipt,
                        receipt_proof,
                        block_header,
                        next_proposal_header,
                        proposal_proof,
                    }
                    .rlp_bytes()
                    .into_vec()
                },
            )
    }

    pub fn get_block_header_bytes(&self, id: BlockId) -> Option<Vec<u8>> {
        self.block_header(id).map(|x| x.rlp_bytes().into_vec())
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

        let prior_quota_used = match receipts.last() {
            Some(ref r) => r.quota_used,
            _ => 0.into(),
        };

        let no_of_logs = receipts.iter().fold(0, |acc, r| acc + r.logs.len());

        if last_receipt.transaction_hash == id {
            // Get sender
            let stx = self.transaction_by_address(hash, index).unwrap();
            let number = self.block_height_by_hash(hash).unwrap_or(0);

            let contract_address = match *stx.action() {
                Action::Create if last_receipt.error.is_none() => {
                    Some(contract_address(stx.sender(), &last_receipt.account_nonce))
                }
                _ => None,
            };

            let receipt = LocalizedReceipt {
                transaction_hash: id,
                transaction_index: index,
                block_hash: hash,
                block_number: number,
                cumulative_quota_used: last_receipt.quota_used,
                quota_used: last_receipt.quota_used - prior_quota_used,
                contract_address,
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
        } else {
            error!("The transaction_hash in receipt is not equal to transaction hash from input.");
            None
        }
    }

    #[inline]
    pub fn get_current_height(&self) -> u64 {
        self.current_height.load(Ordering::SeqCst) as u64
    }

    #[inline]
    pub fn get_pending_height(&self) -> u64 {
        self.current_header.read().number()
    }

    #[inline]
    pub fn get_latest_height(&self) -> u64 {
        self.current_header.read().number().saturating_sub(1)
    }

    #[inline]
    pub fn get_current_hash(&self) -> H256 {
        self.current_header.read().hash()
    }

    #[inline]
    pub fn get_max_store_height(&self) -> u64 {
        self.max_store_height.load(Ordering::SeqCst) as u64
    }

    #[inline]
    pub fn set_max_store_height(&self, height: u64) {
        self.max_store_height
            .store(height as usize, Ordering::SeqCst);
    }

    #[inline]
    pub fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
    }

    #[inline]
    pub fn current_block_poof(&self) -> Option<ProtoProof> {
        self.db.read().read(db::COL_EXTRA, &CurrentProof)
    }

    pub fn save_current_block_poof(&self, proof: &ProtoProof) {
        let mut batch = DBTransaction::new();
        batch.write(db::COL_EXTRA, &CurrentProof, proof);
        self.db
            .read()
            .write(batch)
            .expect("save_current_block_poof DB write failed.");
    }

    pub fn get_chain_prooftype(&self) -> Option<ProofType> {
        match self.prooftype {
            0 => Some(ProofType::AuthorityRound),
            1 => Some(ProofType::Raft),
            2 => Some(ProofType::Bft),
            _ => None,
        }
    }

    pub fn logs<F>(
        &self,
        mut blocks: Vec<BlockNumber>,
        matches: F,
        limit: Option<usize>,
    ) -> Vec<LocalizedLogEntry>
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
                log_index = receipts
                    .iter()
                    .fold(0, |sum, receipt| sum + receipt.logs.len());

                let receipts_len = receipts.len();
                hashes.reverse();
                receipts.reverse();
                receipts
                    .into_iter()
                    .map(|receipt| receipt.logs)
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
    pub fn blocks_with_bloom(
        &self,
        bloom: &LogBloom,
        from_block: BlockNumber,
        to_block: BlockNumber,
    ) -> Vec<BlockNumber> {
        let range = from_block as BloomChainNumber..to_block as BloomChainNumber;
        BloomGroupChain::new(self.blooms_config, self)
            .with_bloom(&range, &Bloom::from(Into::<[u8; 256]>::into(*bloom)))
            .into_iter()
            .map(|b| b as BlockNumber)
            .collect()
    }

    /// Returns numbers of blocks containing given bloom by blockId.
    pub fn blocks_with_bloom_by_id(
        &self,
        bloom: &LogBloom,
        from_block: BlockId,
        to_block: BlockId,
    ) -> Option<Vec<BlockNumber>> {
        match (self.block_number(from_block), self.block_number(to_block)) {
            (Some(from), Some(to)) => Some(self.blocks_with_bloom(bloom, from, to)),
            _ => None,
        }
    }

    pub fn get_logs(&self, filter: &Filter) -> Vec<LocalizedLogEntry> {
        let blocks = filter
            .bloom_possibilities()
            .iter()
            .filter_map(|bloom| {
                self.blocks_with_bloom_by_id(bloom, filter.from_block, filter.to_block)
            })
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
        tx_hashes: &[H256],
        ctx_pub: &Sender<(String, Vec<u8>)>,
    ) {
        let ctx_pub_clone = ctx_pub.clone();
        let version_opt = self.version.read();
        if version_opt.is_none() {
            trace!("delivery_block_tx_hashes : version is not ready!");
            return;
        }
        let mut block_tx_hashes = BlockTxHashes::new();
        block_tx_hashes.set_height(block_height);
        {
            block_tx_hashes.set_check_quota(self.check_quota.load(Ordering::Relaxed));
            block_tx_hashes
                .set_block_quota_limit(self.block_quota_limit.load(Ordering::SeqCst) as u64);
            block_tx_hashes.set_account_quota_limit(self.account_quota_limit.read().clone());
            block_tx_hashes.set_admin_address(
                self.admin_address
                    .read()
                    .map(|admin| admin.to_vec())
                    .unwrap_or_else(Vec::new),
            );
            block_tx_hashes.set_version(version_opt.unwrap());
        }

        let mut tx_hashes_in_u8 = Vec::new();
        for tx_hash_in_h256 in tx_hashes {
            tx_hashes_in_u8.push(tx_hash_in_h256.to_vec());
        }
        block_tx_hashes.set_tx_hashes(tx_hashes_in_u8.into());
        let msg: Message = block_tx_hashes.into();

        ctx_pub_clone
            .send((
                routing_key!(Chain >> BlockTxHashes).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
        trace!("delivery block's tx hashes for height: {}", block_height);
    }

    /// Delivery current rich status to consensus
    /// Consensus should resend block if chain commit block failed.
    pub fn delivery_current_rich_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let header = &*self.current_header.read();
        let version_opt = self.version.read();

        if self.nodes.read().is_empty() || version_opt.is_none() {
            trace!("delivery_current_rich_status : node list or version is not ready!");
            return;
        }
        let current_hash = header.hash();
        let current_height = header.number();
        let nodes: Vec<Address> = self.nodes.read().clone();
        let block_interval = self.block_interval.read();

        let mut rich_status = ProtoRichStatus::new();
        rich_status.set_hash(current_hash.0.to_vec());
        rich_status.set_height(current_height);
        rich_status.set_nodes(nodes.into_iter().map(|address| address.to_vec()).collect());
        rich_status.set_interval(*block_interval);
        rich_status.set_version(version_opt.unwrap());

        let msg: Message = rich_status.into();
        ctx_pub
            .send((
                routing_key!(Chain >> RichStatus).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: H256) -> Option<BlockReceipts> {
        let result = self
            .db
            .read()
            .read_with_cache(db::COL_EXTRA, &self.block_receipts, &hash);
        self.cache_man
            .lock()
            .note_used(CacheId::BlockReceipts(hash));
        result
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, address: &TransactionAddress) -> Option<Receipt> {
        self.block_receipts(address.block_hash)
            .map(|r| r.receipts[address.index].clone())
    }

    /// Current status
    fn current_status(&self) -> Status {
        let mut status = Status::default();
        status.set_hash(self.get_current_hash());
        status.set_number(self.get_current_height());
        status
    }

    pub fn validate_hash(&self, block_hash: &H256) -> bool {
        let current_hash = self.get_current_hash();
        if current_hash == *block_hash {
            true
        } else {
            warn!(
                "Hash is not right, validate_hash current_hash {:?} block_hash {:?}",
                current_hash, block_hash
            );
            false
        }
    }

    pub fn validate_height(&self, block_number: u64) -> bool {
        let current_height = self.get_current_height();
        trace!(
            "validate_height current_height {:?} need validate block_number {:?}",
            current_height,
            block_number
        );
        current_height + 1 == block_number
    }

    // Get the height of proof.
    pub fn get_block_proof_height(&self, block: &Block) -> usize {
        match block.proof_type() {
            Some(ProofType::Bft) => {
                let proof = BftProof::from(block.proof().clone());
                //block height 1's proof is height MAX
                if proof.height == ::std::usize::MAX {
                    return 0;
                }
                proof.height
            }
            _ => block.number() as usize,
        }
    }

    // Get block proof by height.
    pub fn get_block_proof_by_height(&self, height: u64) -> Option<ProtoProof> {
        match self.current_header.read().proof_type() {
            Some(ProofType::Bft) => {
                // TODO: use CONSTANT to replace the '1'.
                self.block_by_height(height + 1)
                    .and_then(|block| Some(block.header.proof().clone()))
            }
            _ => None,
        }
    }

    /// Broadcast new status
    pub fn broadcast_status(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        if self.get_max_store_height() == 0 {
            return;
        }
        let status = self.current_status().protobuf();
        info!(
            "new chain status height {}, hash {}",
            status.get_height(),
            status.get_hash().lower_hex()
        );
        let sync_msg: Message = status.into();
        ctx_pub
            .send((
                routing_key!(Chain >> Status).into(),
                sync_msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    pub fn set_block_body(&self, height: BlockNumber, block: &Block) {
        let mut batch = DBTransaction::new();
        {
            let mut write_bodies = self.block_bodies.write();
            batch.write_with_cache(
                db::COL_BODIES,
                &mut *write_bodies,
                height,
                block.body().clone(),
                CacheUpdatePolicy::Overwrite,
            );
            self.cache_man
                .lock()
                .note_used(CacheId::BlockBodies(height as BlockNumber));
        }
        batch.write(db::COL_EXTRA, &CurrentHeight, &height);
        let _ = self.db.read().write(batch);
    }

    pub fn compare_status(&self, st: &Status) -> (u64, u64) {
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

            block_headers.heap_size_of_children()
                + block_bodies.heap_size_of_children()
                + block_hashes.heap_size_of_children()
                + transaction_addresses.heap_size_of_children()
                + blocks_blooms.heap_size_of_children()
                + block_receipts.heap_size_of_children()
        });
    }

    pub fn poll_filter(&self) -> Arc<Mutex<PollManager<PollFilter>>> {
        Arc::clone(&self.polls_filter)
    }

    /// clear sync block
    pub fn clear_block_map(&self) {
        let mut block_map = self.block_map.write();
        let mut new_block_map: BTreeMap<u64, BlockInQueue> = BTreeMap::new();
        block_map
            .clone()
            .into_iter()
            .filter_map(|(key, value)| match value {
                BlockInQueue::SyncBlock(_) => None,
                _ => Some((key, value)),
            })
            .for_each(|(key, value)| {
                new_block_map.insert(key, value);
            });
        *block_map = new_block_map;
        self.set_max_store_height(self.get_current_height());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cita_types::H256;

    #[test]
    fn test_heapsizeof() {
        let test: Vec<String> = Vec::new();
        assert_eq!(test.heap_size_of_children(), 0);
    }
    #[test]
    fn test_cache_size() {
        let transaction_addresses: HashMap<TransactionId, TransactionAddress> = HashMap::new();
        let blocks_blooms: HashMap<LogGroupPosition, LogBloomGroup> = HashMap::new();
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
}
