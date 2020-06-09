// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::bloomchain::group::{
    BloomGroup, BloomGroupChain, BloomGroupDatabase, GroupPosition as BloomGroupPosition,
};
use crate::bloomchain::{Bloom, Config as BloomChainConfig, Number as BloomChainNumber};
use crate::header::{BlockNumber, Header};
use crate::libchain::status::Status;
use crate::log_blooms::LogBloomGroup;
use crate::receipt::{Receipt, RichReceipt};
use hashable::Hashable;

use libproto::blockchain::{
    AccountGasLimit as ProtoAccountGasLimit, Proof as ProtoProof, ProofType,
    RichStatus as ProtoRichStatus, StateSignal,
};

use libproto::{
    executor::ExecutedResult, router::MsgType, router::RoutingKey, router::SubModules,
    BlockTxHashes, FullTransaction, Message, TryInto,
};
use proof::BftProof;
use pubsub::channel::Sender;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::Into;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use util::{Mutex, RwLock};

use crate::db_indexes::{
    BlockNumber2Body, BlockNumber2Header, CurrentHash, CurrentHeight, CurrentProof,
    Hash2BlockNumber, Hash2BlockReceipts, Hash2TransactionIndex, LogGroupPosition,
};

use crate::types::block::{Block, BlockBody, OpenBlock};
use crate::types::{
    block_number::BlockTag, block_number::Tag, block_number::TransactionHash,
    block_receipts::BlockReceipts, filter::Filter, log::LocalizedLog, log::Log,
    transaction::Action, transaction::SignedTransaction, transaction_index::TransactionIndex,
};
use cita_types::traits::LowerHex;
use cita_types::{Address, Bloom as LogBloom, H256, U256};

use crate::cita_db::RocksDB;
use crate::db_indexes::DBIndex;
use crate::filters::filterdb::FilterDB;
use cita_db::Database;
use rlp::{self, decode, Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

pub const VERSION: u32 = 0;
const LOG_BLOOMS_LEVELS: usize = 3;
const LOG_BLOOMS_ELEMENTS_PER_INDEX: usize = 16;

#[derive(Debug, Clone)]
pub struct RelayInfo {
    pub from_chain_id: U256,
    pub to_chain_id: U256,
    pub dest_contract: Address,
    pub dest_hasher: [u8; 4],
    pub cross_chain_nonce: u64,
}

#[derive(Debug, Clone)]
pub struct TxProof {
    tx: SignedTransaction,
    receipt: Receipt,
    receipt_proof: cita_merklehash::Proof,
    block_header: Header,
    next_proposal_header: Header,
    proposal_proof: ProtoProof,
}

impl Encodable for TxProof {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(6);
        s.append(&self.tx);
        s.append(&self.receipt);
        s.append(&self.receipt_proof);
        s.append(&self.block_header);
        s.append(&self.next_proposal_header);
        s.append(&self.proposal_proof);
    }
}

impl Decodable for TxProof {
    fn decode(r: &UntrustedRlp) -> Result<Self, DecoderError> {
        if r.item_count()? != 6 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        let tx_proof = TxProof {
            tx: r.val_at(0)?,
            receipt: r.val_at(1)?,
            receipt_proof: r.val_at(2)?,
            block_header: r.val_at(3)?,
            next_proposal_header: r.val_at(4)?,
            proposal_proof: r.val_at(5)?,
        };
        Ok(tx_proof)
    }
}

impl TxProof {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        rlp::decode(bytes)
    }

    pub fn verify(&self, authorities: &[Address]) -> bool {
        // Calculate transaction hash, and it should be same as the transaction hash in receipt.
        let tx_hash = self.tx.calc_transaction_hash();
        if self.receipt.transaction_hash == tx_hash {
        } else {
            warn!("txproof verify transaction_hash failed");
            return false;
        };
        // Use receipt_proof and receipt_root to prove the receipt in the block.
        let receipt_merkle_proof: cita_merklehash::MerkleProof<H256> =
            self.receipt_proof.clone().into();
        if receipt_merkle_proof.verify(
            self.block_header.receipts_root(),
            self.receipt.clone().rlp_bytes().into_vec().crypt_hash(),
            cita_merklehash::merge,
        ) {
        } else {
            warn!("txproof verify receipt root merklehash failed");
            return false;
        };
        // Calculate block header hash, and is should be same as the parent_hash in next header
        if self.block_header.hash().unwrap() == *self.next_proposal_header.parent_hash() {
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
            let from_chain_id = U256::from(iter.next().unwrap());
            let to_chain_id = U256::from(iter.next().unwrap());
            let dest_contract = Address::from(H256::from(iter.next().unwrap()));
            let dest_hasher = iter.next().unwrap()[..4].iter().take(4).enumerate().fold(
                [0u8; 4],
                |mut acc, (idx, val)| {
                    acc[idx] = *val;
                    acc
                },
            );
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
        my_chain_id: U256,
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
                        // sendToSideChain(uint256 toChainId, address destContract, bytes txData)
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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Config {
    pub prooftype: u8,
}

impl Config {
    pub fn default() -> Self {
        Config { prooftype: 2 }
    }

    pub fn new(path: &str) -> Self {
        let c: Config = parse_config!(Config, path);
        c
    }
}

impl BloomGroupDatabase for Chain {
    fn blooms_at(&self, position: &BloomGroupPosition) -> Option<BloomGroup> {
        let p = LogGroupPosition::from(position.clone());
        self.db
            .get(Some(cita_db::DataCategory::Extra), &p.get_index())
            .unwrap_or(None)
            .map(|blooms| {
                let g: LogBloomGroup = rlp::decode(&blooms);
                g.into()
            })
    }
}

#[derive(Debug, Clone)]
pub enum BlockInQueue {
    Proposal(OpenBlock),
    ConsensusBlock(OpenBlock, ProtoProof),
    SyncBlock((OpenBlock, Option<ProtoProof>)),
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
    pub db: Arc<RocksDB>,

    pub nodes: RwLock<Vec<Address>>,
    pub validators: RwLock<Vec<Address>>,
    pub block_interval: RwLock<u64>,

    pub block_quota_limit: AtomicUsize,
    pub account_quota_limit: RwLock<ProtoAccountGasLimit>,
    pub check_quota: AtomicBool,

    /// Filter Database
    pub filterdb: Arc<Mutex<FilterDB>>,
    /// Proof type
    pub prooftype: u8,
    // snapshot flag
    pub is_snapshot: RwLock<bool>,
    admin_address: RwLock<Option<Address>>,
    pub version: RwLock<Option<u32>>,
}

/// Get latest status
pub fn get_chain(db: &RocksDB) -> Option<Header> {
    let res = db
        .get(
            Some(cita_db::DataCategory::Extra),
            &CurrentHash.get_index().to_vec(),
        )
        .unwrap_or(None)
        .map(|h| decode::<H256>(&h));

    if let Some(hash) = res {
        trace!("Get block height from hash : {:?}", hash);
        let hash_key = Hash2BlockNumber(hash).get_index();
        let header = db
            .get(Some(cita_db::DataCategory::Extra), &hash_key)
            .unwrap_or(None)
            .map(|n| {
                let height = decode::<BlockNumber>(&n);
                trace!("Get chain from height : {:?}", height);
                let height_key = BlockNumber2Header(height).get_index();
                db.get(Some(cita_db::DataCategory::Headers), &height_key)
                    .unwrap_or(None)
                    .map(|res| {
                        let header: Header = rlp::decode(&res);
                        header
                    })
            })
            .and_then(|x| x);
        return header;
    }
    None
}

pub fn get_chain_body_height(db: &RocksDB) -> Option<BlockNumber> {
    db.get(
        Some(cita_db::DataCategory::Extra),
        &CurrentHeight.get_index(),
    )
    .unwrap_or(None)
    .map(|res| {
        let block_number: BlockNumber = rlp::decode(&res);
        block_number
    })
}

pub fn contract_address(address: &Address, nonce: &U256) -> Address {
    let mut stream = RlpStream::new_list(2);
    stream.append(address);
    stream.append(nonce);
    From::from(stream.out().crypt_hash())
}

impl Chain {
    pub fn init_chain(db: Arc<RocksDB>, chain_config: Config) -> Chain {
        info!("chain config: {:?}", chain_config);

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
            current_header: RwLock::new(header),
            current_height,
            max_store_height,
            block_map: RwLock::new(BTreeMap::new()),
            db,
            filterdb: Arc::new(Mutex::new(FilterDB::new())),
            nodes: RwLock::new(Vec::new()),
            validators: RwLock::new(Vec::new()),
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

    /// Get block number by BlockTag
    fn block_number(&self, tag: BlockTag) -> Option<BlockNumber> {
        match tag {
            BlockTag::Height(number) => Some(number),
            BlockTag::Hash(hash) => self.block_height_by_hash(hash),
            BlockTag::Tag(Tag::Earliest) => Some(0),
            BlockTag::Tag(Tag::Latest) => Some(self.get_latest_height()),
            BlockTag::Tag(Tag::Pending) => Some(self.get_pending_height()),
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
        let hash_key = Hash2BlockNumber(hash).get_index();
        self.db
            .get(Some(cita_db::DataCategory::Extra), &hash_key)
            .unwrap_or(None)
            .map(|res| decode::<BlockNumber>(&res))
    }

    fn set_config(&self, ret: &ExecutedResult) {
        let conf = ret.get_config();
        let nodes = conf.get_nodes();
        let nodes: Vec<Address> = nodes
            .iter()
            .map(|vecaddr| Address::from_slice(&vecaddr[..]))
            .collect();
        let validators = conf.get_validators();
        let validators: Vec<Address> = validators
            .iter()
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
        *self.nodes.write() = nodes;
        *self.validators.write() = validators;
        *self.block_interval.write() = block_interval;
        *self.admin_address.write() = if conf.get_admin_address().is_empty() {
            None
        } else {
            Some(Address::from(conf.get_admin_address()))
        };
        *self.version.write() = Some(version);
    }

    pub fn set_db_result(&self, ret: &ExecutedResult, block: &OpenBlock) {
        let info = ret.get_executed_info();
        let number = info.get_header().get_height();
        let log_bloom = LogBloom::from(info.get_header().get_log_bloom());
        let header = Header::from_executed_info(ret.get_executed_info(), &block.header);
        let header_hash = header.hash().unwrap();

        let block_transaction_indexes = block.body().transaction_indexes(header_hash);
        let blocks_blooms: HashMap<LogGroupPosition, LogBloomGroup> = if log_bloom.is_zero() {
            HashMap::new()
        } else {
            let group = BloomGroupChain::new(self.blooms_config, self);
            group
                .insert(
                    number as BloomChainNumber,
                    Bloom::from(Into::<[u8; 256]>::into(log_bloom)),
                )
                .into_iter()
                .map(|p| (From::from(p.0), From::from(p.1)))
                .collect()
        };

        // Save hash -> receipts
        if !info.get_receipts().is_empty() {
            let receipts: Vec<Receipt> = info
                .get_receipts()
                .iter()
                .map(|r| Receipt::from(r.get_receipt().clone()))
                .collect();
            let block_receipts = BlockReceipts::new(receipts);
            let hash_key = Hash2BlockReceipts(header_hash).get_index();
            let _ = self.db.insert(
                Some(cita_db::DataCategory::Extra),
                hash_key,
                rlp::encode(&block_receipts).into_vec(),
            );
        }

        // Save block transaction indexes
        if !block_transaction_indexes.is_empty() {
            for (k, v) in block_transaction_indexes.iter() {
                let hash_key = Hash2TransactionIndex(*k).get_index();
                let _ = self.db.insert(
                    Some(cita_db::DataCategory::Extra),
                    hash_key,
                    rlp::encode(v).into_vec(),
                );
            }
        }

        // Save number -> header
        trace!("Save ExecutedResult's header: {:?}", header);
        let number_key = BlockNumber2Header(number).get_index();
        let _ = self.db.insert(
            Some(cita_db::DataCategory::Headers),
            number_key,
            rlp::encode(&header).into_vec(),
        );

        // Save Body
        let mheight = self.get_max_store_height();
        if mheight < number || (number == 0 && mheight == 0) {
            let number_key = BlockNumber2Body(number).get_index();
            let _ = self.db.insert(
                Some(cita_db::DataCategory::Bodies),
                number_key,
                rlp::encode(block.body()).into_vec(),
            );
        }

        // Save hash -> blockNumber
        let hash_key = Hash2BlockNumber(header_hash).get_index();
        let _ = self.db.insert(
            Some(cita_db::DataCategory::Extra),
            hash_key,
            rlp::encode(&number).into_vec(),
        );

        // Save blocks blooms
        for (k, v) in blocks_blooms.iter() {
            let _ = self.db.insert(
                Some(cita_db::DataCategory::Extra),
                k.get_index(),
                rlp::encode(v).into_vec(),
            );
        }

        // Save current hash
        let _ = self.db.insert(
            Some(cita_db::DataCategory::Extra),
            CurrentHash.get_index(),
            rlp::encode(&header_hash).into_vec(),
        );

        *self.current_header.write() = header;
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
            let blk = OpenBlock::default();
            self.set_db_result(ret, &blk);
            let block_tx_hashes = Vec::new();
            self.delivery_block_tx_hashes(number, &block_tx_hashes, &ctx_pub);
            self.broadcast_current_status(&ctx_pub);
            return;
        }

        // Duplicated block
        if number <= self.get_current_height() {
            let tx_hashes = self
                .block_body_by_height(self.get_current_height())
                .unwrap()
                .transaction_hashes();
            self.delivery_block_tx_hashes(self.get_current_height(), &tx_hashes, &ctx_pub);
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

    /// Get block by BlockTag
    pub fn block(&self, tag: BlockTag) -> Option<Block> {
        match tag {
            BlockTag::Hash(hash) => self.block_by_hash(hash),
            BlockTag::Height(number) => self.block_by_height(number),
            BlockTag::Tag(Tag::Earliest) => self.block_by_height(0),
            BlockTag::Tag(Tag::Latest) => self.block_by_height(self.get_latest_height()),
            BlockTag::Tag(Tag::Pending) => self.block_by_height(self.get_pending_height()),
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

    /// Get block header by BlockTag
    pub fn block_header(&self, tag: BlockTag) -> Option<Header> {
        match tag {
            BlockTag::Hash(hash) => self.block_header_by_hash(hash),
            BlockTag::Height(number) => self.block_header_by_height(number),
            BlockTag::Tag(Tag::Earliest) => self.block_header_by_height(0),
            BlockTag::Tag(Tag::Latest) => self.block_header_by_height(self.get_latest_height()),
            BlockTag::Tag(Tag::Pending) => self.block_header_by_height(self.get_pending_height()),
        }
    }

    /// Get block header by hash
    pub fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.hash().unwrap() == hash {
                return Some(header.clone());
            }
        }
        self.block_height_by_hash(hash)
            .and_then(|h| self.block_header_by_height(h))
    }

    fn block_header_by_height(&self, number: BlockNumber) -> Option<Header> {
        let number_key = BlockNumber2Header(number).get_index();
        self.db
            .get(Some(cita_db::DataCategory::Headers), &number_key)
            .unwrap_or(None)
            .map(|res| {
                let header: Header = rlp::decode(&res);
                header
            })
    }

    /// Get block body by BlockTag
    pub fn block_body(&self, tag: BlockTag) -> Option<BlockBody> {
        match tag {
            BlockTag::Hash(hash) => self.block_body_by_hash(hash),
            BlockTag::Height(number) => self.block_body_by_height(number),
            BlockTag::Tag(Tag::Earliest) => self.block_body_by_height(0),
            BlockTag::Tag(Tag::Latest) => self.block_body_by_height(self.get_latest_height()),
            BlockTag::Tag(Tag::Pending) => self.block_body_by_height(self.get_pending_height()),
        }
    }

    pub fn block_hash_by_height(&self, height: BlockNumber) -> Option<H256> {
        self.block_header_by_height(height)
            .map(|hdr| hdr.hash().unwrap())
    }

    /// Get block body by hash
    fn block_body_by_hash(&self, hash: H256) -> Option<BlockBody> {
        self.block_height_by_hash(hash)
            .and_then(|h| self.block_body_by_height(h))
    }

    /// Get block body by height
    fn block_body_by_height(&self, number: BlockNumber) -> Option<BlockBody> {
        let number_key = BlockNumber2Body(number).get_index();
        self.db
            .get(Some(cita_db::DataCategory::Bodies), &number_key)
            .unwrap_or(None)
            .map(|res| {
                let body: BlockBody = rlp::decode(&res);
                body
            })
    }

    /// Get block tx hashes
    pub fn block_tx_hashes(&self, number: BlockNumber) -> Option<Vec<H256>> {
        self.block_body_by_height(number)
            .map(|body| body.transaction_hashes())
    }

    /// Get transaction by hash
    pub fn transaction(&self, hash: TransactionHash) -> Option<SignedTransaction> {
        self.transaction_index(hash).and_then(|addr| {
            let index = addr.index;
            let hash = addr.block_hash;
            self.transaction_by_address(hash, index)
        })
    }

    /// Get address of transaction by hash.
    fn transaction_index(&self, hash: TransactionHash) -> Option<TransactionIndex> {
        let hash_key = Hash2TransactionIndex(hash).get_index();
        self.db
            .get(Some(cita_db::DataCategory::Extra), &hash_key)
            .unwrap_or(None)
            .map(|res| {
                let tx_index: TransactionIndex = rlp::decode(&res);
                tx_index
            })
    }

    /// Get transaction by address
    fn transaction_by_address(&self, hash: H256, index: usize) -> Option<SignedTransaction> {
        self.block_body_by_hash(hash)
            .map(|body| body.transactions()[index].clone())
    }

    /// Get transaction hashes by block hash
    pub fn transaction_hashes(&self, tag: BlockTag) -> Option<Vec<H256>> {
        self.block_body(tag).map(|body| body.transaction_hashes())
    }

    /// Get full transaction by hash
    pub fn full_transaction(&self, hash: TransactionHash) -> Option<FullTransaction> {
        self.transaction_index(hash).and_then(|addr| {
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

    pub fn get_transaction_proof(&self, hash: TransactionHash) -> Option<Vec<u8>> {
        self.transaction_index(hash)
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
                        cita_merklehash::Tree::from_hashes(
                            receipts
                                .receipts
                                .iter()
                                .map(|r| r.rlp_bytes().into_vec().crypt_hash())
                                .collect::<Vec<_>>(),
                            cita_merklehash::merge,
                        )
                        .get_proof_by_input_index(index)
                        .map(|receipt_proof| (index, block, receipt.clone(), receipt_proof))
                    })
            })
            .and_then(|(index, block, receipt, receipt_proof)| {
                block.body().transactions().get(index).map(|tx| {
                    (
                        tx.clone(),
                        receipt,
                        receipt_proof.into(),
                        block.header().clone(),
                    )
                })
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

    pub fn get_block_header_bytes(&self, tag: BlockTag) -> Option<Vec<u8>> {
        self.block_header(tag).map(|x| x.rlp_bytes().into_vec())
    }

    pub fn get_rich_receipt(&self, tx_hash: TransactionHash) -> Option<RichReceipt> {
        trace!("Get receipt by hash: {:?}", tx_hash);
        if let Some(transaction_index) = self.transaction_index(tx_hash) {
            let block_hash = transaction_index.block_hash;
            let tx_index = transaction_index.index;

            if let Some(res) = self.block_receipts(block_hash) {
                let mut receipts = res.receipts;
                receipts.truncate(tx_index + 1);

                let last_receipt = receipts.pop().expect("Current receipt is provided; qed");
                let prior_quota_used = receipts.last().map_or(0.into(), |r| r.quota_used);
                let log_position_block = receipts.iter().fold(0, |acc, r| acc + r.logs.len());

                if last_receipt.transaction_hash == tx_hash {
                    let stx = self
                        .transaction_by_address(block_hash, tx_index)
                        .unwrap_or_default();
                    let block_number = self.block_height_by_hash(block_hash).unwrap_or(0);
                    let contract_address = match *stx.action() {
                        Action::Create if last_receipt.error.is_none() => {
                            Some(contract_address(stx.sender(), &last_receipt.account_nonce))
                        }
                        _ => None,
                    };

                    let receipt = RichReceipt {
                        transaction_hash: tx_hash,
                        transaction_index: tx_index,
                        block_hash,
                        block_number,
                        cumulative_quota_used: last_receipt.quota_used,
                        quota_used: last_receipt.quota_used - prior_quota_used,
                        contract_address,
                        logs: last_receipt
                            .logs
                            .into_iter()
                            .enumerate()
                            .map(|(i, log)| LocalizedLog {
                                log,
                                block_hash,
                                block_number,
                                transaction_hash: tx_hash,
                                transaction_index: tx_index,
                                transaction_log_index: i,
                                log_index: log_position_block + i,
                            })
                            .collect(),
                        log_bloom: last_receipt.log_bloom,
                        state_root: last_receipt.state_root,
                        error: last_receipt.error,
                    };
                    return Some(receipt);
                }
            }
        }
        info!("Get receipt by hash failed {:?}", tx_hash);
        None
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
        self.current_header.read().hash().unwrap()
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
        self.db
            .get(
                Some(cita_db::DataCategory::Extra),
                &CurrentProof.get_index(),
            )
            .unwrap_or(None)
            .map(|res| {
                let proto_proof: ProtoProof = rlp::decode(&res);
                proto_proof
            })
    }

    pub fn save_current_block_poof(&self, proof: &ProtoProof) {
        let _ = self.db.insert(
            Some(cita_db::DataCategory::Extra),
            CurrentProof.get_index(),
            rlp::encode(proof).into_vec(),
        );
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
    ) -> Vec<LocalizedLog>
    where
        F: Fn(&Log) -> bool,
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
                    unreachable!();
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
                            LocalizedLog {
                                log,
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
            .filter(|log| matches(&log.log))
            .take(limit.unwrap_or(::std::usize::MAX))
            .collect::<Vec<LocalizedLog>>();
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

    /// Returns numbers of blocks containing given bloom by BlockTag.
    pub fn blocks_with_bloom_by_id(
        &self,
        bloom: &LogBloom,
        from_block: BlockTag,
        to_block: BlockTag,
    ) -> Option<Vec<BlockNumber>> {
        match (
            self.block_number(from_block),
            self.block_number(to_block),
            self.block_number(BlockTag::Tag(Tag::Pending)),
        ) {
            (Some(from), Some(to), Some(pending)) => {
                let end = if to > pending { pending } else { to };
                Some(self.blocks_with_bloom(bloom, from, end))
            }
            _ => None,
        }
    }

    pub fn get_logs(&self, filter: &Filter) -> Vec<LocalizedLog> {
        let blocks = filter
            .zip_blooms()
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
        let current_hash = header.hash().unwrap();
        let current_height = header.number();
        let nodes: Vec<Address> = self.nodes.read().clone();
        let validators: Vec<Address> = self.validators.read().clone();
        let block_interval = self.block_interval.read();
        let current_timestamp = header.timestamp();

        let mut rich_status = ProtoRichStatus::new();
        rich_status.set_hash(current_hash.0.to_vec());
        rich_status.set_height(current_height);
        rich_status.set_nodes(nodes.into_iter().map(|address| address.to_vec()).collect());
        rich_status.set_validators(validators.into_iter().map(|a| a.to_vec()).collect());
        rich_status.set_interval(*block_interval);
        rich_status.set_version(version_opt.unwrap());
        rich_status.set_timestamp(current_timestamp);

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
        let hash_key = Hash2BlockReceipts(hash).get_index();
        self.db
            .get(Some(cita_db::DataCategory::Extra), &hash_key)
            .unwrap_or(None)
            .map(|res| {
                let block_receipts: BlockReceipts = rlp::decode(&res);
                block_receipts
            })
    }

    /// Get transaction receipt.
    pub fn transaction_receipt(&self, address: &TransactionIndex) -> Option<Receipt> {
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
                    .map(|block| block.header.proof().clone())
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

    pub fn set_block_body(&self, height: BlockNumber, block: &OpenBlock) {
        // Save number -> body
        let height_key = BlockNumber2Body(height).get_index();
        let _ = self.db.insert(
            Some(cita_db::DataCategory::Bodies),
            height_key,
            rlp::encode(block.body()).into_vec(),
        );

        // Save current height
        let _ = self.db.insert(
            Some(cita_db::DataCategory::Extra),
            CurrentHeight.get_index(),
            rlp::encode(&height).into_vec(),
        );
    }

    pub fn compare_status(&self, st: &Status) -> (u64, u64) {
        let current_height = self.get_current_height();
        if st.number() > current_height {
            (current_height + 1, st.number() - current_height)
        } else {
            (0, 0)
        }
    }

    pub fn filter_db(&self) -> Arc<Mutex<FilterDB>> {
        Arc::clone(&self.filterdb)
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
