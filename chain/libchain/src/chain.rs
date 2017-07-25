pub use block::*;
pub use transaction::*;
pub use sha3::*;
pub use byteorder::{BigEndian, ByteOrder};
use bloomchain as bc;
use state::header::*;
use state::basic_types::*;
use state::blooms::*;
use std::collections::{HashMap, HashSet};
use libproto::request::FullTransaction;
use libproto::blockchain::{ProofType, BlockBody, Content, SignedTransaction as PSignedTransaction};
pub use protobuf::{Message, RepeatedField};
use protobuf::core::parse_from_bytes;

use std::collections::{BTreeMap, VecDeque};
use std::sync::mpsc::Sender;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::sync::Arc;
use parking_lot::{RwLock, Mutex};

use genesis::Genesis;
use proof::TendermintProof;
use util::kvdb::*;
use util::trie::{TrieFactory, TrieSpec};
use util::{journaldb, FixedHash, H256, U256, H2048, Address, Uint, Bytes};
use extras::*;
use state::types::ids::{BlockId, TransactionId};
use state::env_info::{LastHashes, EnvInfo};
use state::evm::Factory as EvmFactory;
use state::db;
use state::db::*;
use state::factory::*;
use state::state_db::StateDB;
use state::state::State;
use state::call_analytics::CallAnalytics;
use state::executive::{Executive, Executed, TransactOptions, contract_address};
use state::error::CallError;
use state::types::log_entry::{LogEntry, LocalizedLogEntry};
use state::types::filter::Filter;

use state::engines::NullEngine;
use cita_transaction::eth_transaction::{SignedTransaction, EthTransaction, Action};
use state::receipt::{Receipt, LocalizedReceipt};
use call_request::CallRequest;
use std::str::FromStr;

pub const VERSION: u32 = 0;
pub const CURRENT_HASH: &[u8] = b"current_hash";
pub const CURRENT_HEIGHT: &[u8] = b"current_height";

const LOG_BLOOMS_LEVELS: usize = 3;
const LOG_BLOOMS_ELEMENTS_PER_INDEX: usize = 16;

#[derive(PartialEq, Clone, Debug)]
pub enum BlockSource {
    CONSENSUS = 0,
    NET = 1,
}

impl bc::group::BloomGroupDatabase for Chain {
    fn blooms_at(&self, position: &bc::group::GroupPosition) -> Option<bc::group::BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self.db.read(db::COL_EXTRA, &position).map(Into::into);
        result
    }
}

// TODO: Chain Errors
pub trait TransactionHash {
    fn transaction_hashes(&self) -> Vec<H256>;
}

impl TransactionHash for BlockBody {
    fn transaction_hashes(&self) -> Vec<H256> {
        self.transactions.iter().map(|ts| ts.sha3()).collect()
    }
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
}

pub fn save_genesis(db: &KeyValueDB, genesis: &Genesis) -> Result<(), String> {
    let mut batch = db.transaction();
    batch.put_vec(
        db::COL_BLOCKS,
        &genesis.hash.0,
        genesis.block.write_to_bytes().unwrap(),
    );
    db.write(batch)
}

/// Get latest status
pub fn get_chain(db: &KeyValueDB) -> Option<(H256, u64)> {
    let current_hash = db.get(db::COL_BLOCKS, CURRENT_HASH);
    if let Ok(Some(hash)) = current_hash {
        let current_height = db.get(db::COL_BLOCKS, CURRENT_HEIGHT);
        if let Ok(Some(height)) = current_height {
            Some((
                H256::from(hash.to_vec().as_slice()),
                BigEndian::read_u64(&*height),
            ))
        } else {
            warn!("not expected get_chain.");
            None
        }
    } else {
        None
    }
}

impl Chain {
    fn save_status(&self) -> Status {
        let current_height = self.current_height.load(Ordering::SeqCst) as u64;
        let current_hash = *self.current_hash.read();

        let mut batch = self.db.transaction();
        let mut height = vec![0; 8];
        let mut wtr = vec![0; 8];
        BigEndian::write_u64(&mut wtr, current_height);
        BigEndian::write_u64(&mut height, current_height);
        batch.put_vec(db::COL_BLOCKS, height.as_slice(), current_hash.to_vec());
        batch.put_vec(db::COL_BLOCKS, CURRENT_HASH, current_hash.to_vec());
        batch.put_vec(db::COL_BLOCKS, CURRENT_HEIGHT, wtr);
        self.db.write(batch).unwrap();

        //return status
        let mut status = Status::new();
        status.set_hash(current_hash.to_vec());
        status.set_height(current_height);
        status
    }

    pub fn init_chain(db: Arc<KeyValueDB>,
                      mut genesis: Genesis,
                      sync_sender: Sender<u64>)
                      -> (Arc<Chain>, Status) {
        let trie_factory = TrieFactory::new(TrieSpec::Fat);
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
                                     db: db,
                                     state_db: state_db,
                                     factories: factories,
                                     sync_sender: Mutex::new(sync_sender),
                                     last_hashes: RwLock::new(VecDeque::new()),
                                 });

            chain.build_last_hashes(Some(hash.into()), height);
            (chain, status)
        } else {
            let _ = genesis.lazy_execute();
            save_genesis(&*db, &genesis).expect("Failed to save genesis.");
            info!("init genesis {:?}", genesis);

            let chain = Arc::new(Chain {
                                     blooms_config: blooms_config,
                                     current_hash: RwLock::new(genesis.hash.into()),
                                     current_height: AtomicUsize::new(0),
                                     is_sync: AtomicBool::new(false),
                                     max_height: AtomicUsize::new(0),
                                     block_map: RwLock::new(BTreeMap::new()),
                                     db: db,
                                     state_db: state_db,
                                     factories: factories,
                                     sync_sender: Mutex::new(sync_sender),
                                     last_hashes: RwLock::new(VecDeque::new()),
                                 });
            let status = chain.save_status();
            chain.build_last_hashes(Some(genesis.hash.into()), 0);
            (chain, status)
        }
    }

    /// Validate transaction by unique hash
    pub fn validate_transaction(&self, ts: &Transaction) -> bool {
        let hash = ts.sha3();

        self.transaction_address(hash).is_none()
    }

    /// Get raw block by height
    fn block_by_height(&self, number: u64) -> Option<Vec<u8>> {
        let mut height = vec![0; 8];
        BigEndian::write_u64(&mut height, number);
        let hash = self.get_by_hash(db::COL_BLOCKS, height);
        match hash {
            Some(rmsg) => self.block_by_hash(H256::from(rmsg.to_vec().as_slice())),
            None => None,
        }
    }

    /// Get raw block by hash
    fn block_by_hash(&self, hash: H256) -> Option<Vec<u8>> {
        self.get_by_hash(db::COL_BLOCKS, hash.to_vec())
    }

    /// Get block by BlockId
    pub fn block(&self, id: BlockId) -> Option<Block> {
        match id {
            BlockId::Hash(hash) => self.block_by_hash(hash),
            BlockId::Number(number) => self.block_by_height(number),
            BlockId::Earliest => self.block_by_height(0),
            BlockId::Latest => self.block_by_height(self.get_current_height()),
        }.and_then(|b| parse_from_bytes::<Block>(b.as_slice()).ok())
    }

    /// Get transaction by hash
    pub fn transaction(&self, hash: TransactionId) -> Option<FullTransaction> {
        match self.transaction_address(hash) {
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

    pub fn localized_receipt(
        &self,
        id: TransactionId,
        tx_address: TransactionAddress,
    ) -> Option<LocalizedReceipt> {
        trace!("Get receipt id: {:?}, tx_address: {:?}", id, tx_address);
        let mut previous_receipts = (0..tx_address.index + 1)
            .map(|index| {
                let mut address = tx_address.clone();
                address.index = index;
                self.transaction_receipt(&address)
            })
            .collect::<Vec<Option<Receipt>>>();

        let last_receipt = previous_receipts.pop().expect(
            "Current receipt is provided; qed",
        );
        let prior_gas_used = match tx_address.index {
            0 => 0.into(),
            i => {
                previous_receipts.clone().into_iter().nth(i - 1).map_or(
                    0.into(),
                    |receipt| {
                        receipt.map_or(0.into(), |r| r.gas_used)
                    },
                )
            }
        };
        let no_of_logs = previous_receipts
            .into_iter()
            .map(|receipt| receipt.map_or(0, |r| r.logs.len()))
            .sum::<usize>();

        last_receipt.and_then(|last_receipt| {
            // Get sender
            let full_transaction = self.transaction(id).unwrap();
            let transaction = full_transaction.get_transaction();
            let from = transaction.get_from();
            let sender = Address::from_str(from).unwrap();

            let signed_tx = parse_from_bytes::<PSignedTransaction>(transaction.get_content())
                .unwrap();
            let raw_tx = parse_from_bytes::<Transaction>(signed_tx.get_transaction()).unwrap();
            let content = parse_from_bytes::<Content>(raw_tx.get_content()).unwrap();
            let block_hash = tx_address.block_hash;
            let block_number = self.block_number(BlockId::Hash(block_hash.clone()))
                .unwrap_or(0);

            let contract_address = match transaction.get_to().is_empty() {
                false => None,
                true => Some(contract_address(
                    &sender,
                    &content.get_nonce().parse::<U256>().unwrap_or_default(),
                )),
            };

            let receipt = LocalizedReceipt {
                transaction_hash: id,
                transaction_index: tx_address.index,
                block_hash: block_hash,
                block_number: block_number,
                cumulative_gas_used: last_receipt.gas_used,
                gas_used: last_receipt.gas_used - prior_gas_used,
                contract_address: contract_address,
                logs: last_receipt
                    .logs
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

    // Query from db
    pub fn get_by_hash(&self, column: Option<u32>, hash: Vec<u8>) -> Option<Vec<u8>> {
        match self.db.get(column, hash.as_slice()) {
            Ok(value) => value.map(|v| v.to_vec()),
            _ => None,
        }
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_height.load(Ordering::SeqCst) as u64
    }

    pub fn get_max_height(&self) -> u64 {
        self.max_height.load(Ordering::SeqCst) as u64
    }

    /// Filter invalid transaction.
    pub fn filter_transactions(&self, mut block: Block) -> Block {
        let mut body = block.take_body();
        let transactions = body.take_transactions();

        let txs = transactions.into_iter().filter(
            |x| self.validate_transaction(&x),
        );

        body.set_transactions(RepeatedField::from_vec(txs.collect::<Vec<_>>()));
        block.set_body(body);
        block
    }


    pub fn validate_hash(&self, block_hash: &[u8]) -> bool {
        let current_hash = *self.current_hash.read();
        let block_hash = H256::from_slice(block_hash);
        trace!(
            "validate_hash current_hash {:?} block_hash {:?}",
            current_hash,
            block_hash
        );
        current_hash == block_hash
    }

    pub fn validate_height(&self, block_number: u64) -> bool {
        let current_height = self.current_height.load(Ordering::SeqCst) as u64;
        trace!(
            "validate_height current_height {:?} block_number {:?}",
            current_height,
            block_number - 1
        );
        current_height == block_number - 1
    }

    /// Execute block in vm
    fn execute_block(&self, block: Block) -> OpenBlock {
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let mut open_block = OpenBlock::new(
            self.factories.clone(),
            false,
            block,
            self.state_db.boxed_clone(),
            current_state_root,
            last_hashes.into(),
        ).unwrap();
        open_block.apply_transactions();

        open_block
    }

    /// Get Block Hash by height
    fn get_hash_by_height(&self, height: u64) -> Option<H256> {
        let mut wtr = vec![0; 8];
        BigEndian::write_u64(&mut wtr, height);
        self.get_by_hash(db::COL_BLOCKS, wtr).map(|h| {
            H256::from(h.as_slice())
        })
    }

    fn last_hashes(&self) -> LastHashes {
        LastHashes::from(self.last_hashes.read().clone())
    }

    fn block_body(&self, hash: &H256) -> Option<BlockBody> {
        self.block(BlockId::Hash(*hash)).map_or(None, |mut v| {
            Some(v.take_body())
        })
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
        let mut logs = blocks.into_iter()
			.filter_map(|number| self.get_hash_by_height(number).map(|hash| (number, hash)))
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
						logs.into_iter()
							.enumerate()
							.map(move |(i, log)| LocalizedLogEntry {
								entry: log,
								block_hash: hash,
								block_number: number,
								transaction_hash: tx_hash,
								// iterating in reverse order
								transaction_index: receipts_len - index - 1,
								transaction_log_index: no_of_logs - i - 1,
								log_index: current_log_index - i - 1,
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
        self.block(BlockId::Hash(*hash)).map_or(None, |v| {
            Some(v.get_header().get_height())
        })
    }

    /// Returns numbers of blocks containing given bloom.
    pub fn blocks_with_bloom(
        &self,
        bloom: &H2048,
        from_block: BlockNumber,
        to_block: BlockNumber,
    ) -> Vec<BlockNumber> {
        let range = from_block as bc::Number..to_block as bc::Number;
        let chain = bc::group::BloomGroupChain::new(self.blooms_config, self);
        chain
            .with_bloom(&range, &Bloom::from(bloom.clone()).into())
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
        let parent_hash = prevhash.unwrap_or_else(|| {
            self.get_hash_by_height(parent_height).expect(
                "Block height always valid.",
            )
        });
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
            match self.get_hash_by_height(height) {
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

    // TODO: should get from header cache
    fn current_state_root(&self) -> H256 {
        info!("current_hash: {:?}", self.current_hash.read().clone());
        let block = self.get_by_hash(db::COL_BLOCKS, self.current_hash.read().clone().to_vec())
            .expect("Current hash always stores in db.");
        let blk = parse_from_bytes::<Block>(block.as_slice()).expect("Current hash always valid.");
        let header = blk.get_header();
        let commit = header.get_commit();
        H256::from(commit.get_state_root())
    }

    /// Commit block in db, including:
    /// 1. Block including transactions
    /// 2. TransactionAddress
    /// 3. State
    /// 3. Receipts
    /// 4. Bloom
    pub fn commit_block(&self, mut batch: DBTransaction, block: ClosedBlock) {
        let height = block.get_header().get_height();
        let hash = block.hash.clone();
        let block_data = block.write_to_bytes().unwrap();
        trace!("commit block in db {:?}, {:?}", hash, height);
        // Store transaction address
        for (key, value) in block.transactions.clone() {
            trace!("commit transaction in db {:?}", key);
            batch.write(db::COL_EXTRA, &key, &value)
        }
        // Store blocks blooms
        let log_bloom = block.receipts.clone().into_iter().filter_map(|r| r).fold(
            LogBloom::zero(),
            |mut b, r| {
                b = &b | &r.log_bloom;
                b
            },
        );
        let blocks_blooms: HashMap<LogGroupPosition, BloomGroup> = match log_bloom.is_zero() {
            true => HashMap::new(),
            false => {
                let chain = bc::group::BloomGroupChain::new(self.blooms_config, self);
                chain
                    .insert(height as bc::Number, Bloom::from(log_bloom).into())
                    .into_iter()
                    .map(|p| (From::from(p.0), From::from(p.1)))
                    .collect()
            }
        };
        for (key, value) in blocks_blooms {
            batch.write(db::COL_EXTRA, &key, &value)
        }
        // Store receipts
        let block_receipts = BlockReceipts::new(block.receipts.clone());
        batch.write(db::COL_EXTRA, &hash, &block_receipts);
        let mut state = block.drain();
        // Store block in db
        batch.put_vec(db::COL_BLOCKS, &hash, block_data);
        // Store triedb changes in journal db
        state.journal_under(&mut batch, height, &hash).expect(
            "DB commit failed",
        );

        self.db.write(batch).expect("DB write failed.");
    }

    /// Get the address of transaction with given hash.
    pub fn transaction_address(&self, id: TransactionId) -> Option<TransactionAddress> {
        self.db.read(db::COL_EXTRA, &id)
    }

    // TODO: cache it after transact
    /// Get receipts of block with given hash.
    pub fn block_receipts(&self, hash: &H256) -> Option<BlockReceipts> {
        self.db.read(db::COL_EXTRA, hash)
    }

    pub fn cita_call(&self, request: CallRequest, id: BlockId) -> Result<Bytes, String> {
        let signed = self.sign_call(request);
        let result = self.call(&signed, id, Default::default());
        result.map(|b| b.output.into()).or_else(|_| {
            Err(String::from("Call Error"))
        })
    }

    fn sign_call(&self, request: CallRequest) -> SignedTransaction {
        let from = request.from.unwrap_or(Address::zero());
        EthTransaction {
            nonce: U256::zero(),
            action: Action::Call(request.to),
            gas: U256::from(50_000_000),
            gas_price: U256::zero(),
            value: U256::zero(),
            data: request.data.map_or_else(Vec::new, |d| d.to_vec()),
            hash: H256::default(),
        }.fake_sign(from)
    }

    /// Attempt to get a copy of a specific block's final state.
    pub fn state_at(&self, id: BlockId) -> Option<State<StateDB>> {
        self.block(id).and_then(|block| {
            let db = self.state_db.boxed_clone();
            let header = block.get_header();
            let commit = header.get_commit();
            let root = commit.get_state_root();
            State::from_existing(
                db,
                H256::from_slice(root),
                U256::from(0),
                self.factories.clone(),
            ).ok()
        })
    }

    // TODO: cache state_root
    /// Get a copy of the best block's state.
    pub fn state(&self) -> State<StateDB> {
        let current_hash = *self.current_hash.read();
        let block_id = BlockId::Hash(current_hash);
        self.state_at(block_id).expect(
            "State root of current block always valid.",
        )
    }

    //get account
    pub fn code_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id).and_then(|s| s.code(address).ok()).map(
            |c| {
                c.map(|c| (&*c).clone())
            },
        )
    }

    //account  transaction count
    pub fn nonce(&self, address: &Address, id: BlockId) -> Option<U256> {
        self.state_at(id).and_then(|s| s.nonce(address).ok())
    }

    fn call(
        &self,
        t: &SignedTransaction,
        block_id: BlockId,
        analytics: CallAnalytics,
    ) -> Result<Executed, CallError> {
        let block = self.block(block_id).ok_or(CallError::StatePruned)?;
        let header = block.get_header();
        let commit = header.get_commit();
        let last_hashes = self.build_last_hashes(None, header.get_height());
        let env_info = EnvInfo {
            number: header.get_height(),
            author: Address::default(),
            timestamp: header.get_timestamp(),
            difficulty: U256::default(),
            last_hashes: last_hashes,
            gas_used: U256::from(commit.get_gas_used()),
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
        let ret = Executive::new(&mut state, &env_info, &engine, &self.factories.vm)
            .transact(t, options)?;

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
    pub fn add_block(&self, mut blk: Block) -> Option<H256> {
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
            if blk.get_header().has_proof1() {
                let proof1 = blk.mut_header().take_proof1();
                if proof1.get_field_type() == ProofType::Tendermint {
                    let proof1 = TendermintProof::from(proof1);
                    if proof1.simple_check(height) {
                        proof1.store();
                    }
                }
            }

            let block = self.filter_transactions(blk);
            let batch = self.db.transaction();
            let open_block = self.execute_block(block.clone());
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
        trace!("set_block-----{:?}-----{:?}", blk_height, block.sha3());
        if self.validate_height(blk_height) {
            if let Some(current_hash) = self.add_block(block) {
                trace!(
                    "set_block current_hash!!!!!!{:?} {:?}",
                    blk_height,
                    H256::from(current_hash)
                );
                *self.current_hash.write() = current_hash;
                self.current_height.fetch_add(1, Ordering::SeqCst);
                let status = self.save_status();
                info!("-------chain update {:?}-------", blk_height);
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
}
