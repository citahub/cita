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
pub use byteorder::{BigEndian, ByteOrder};
use call_analytics::CallAnalytics;
use contracts::{AccountGasLimit, AccountManager, NodeManager, QuotaManager};
use db;
use db::*;
use engines::NullEngine;
use env_info::{EnvInfo, LastHashes};
use error::CallError;
use evm::Factory as EvmFactory;
use executive::{Executed, Executive, TransactOptions};
use factory::*;
use header::*;
pub use libexecutor::block::*;
use libexecutor::call_request::CallRequest;
use libexecutor::extras::*;
use libexecutor::genesis::Genesis;
pub use libexecutor::transaction::*;

use libproto::*;
use libproto::blockchain::{Proof as ProtoProof, ProofType};

use native::Factory as NativeFactory;
use protobuf::Message;
use serde_json;
use state::State;
use state_db::StateDB;
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::time::Instant;
use types::ids::BlockId;
use types::transaction::{Action, SignedTransaction, Transaction};
use util::{journaldb, Address, Bytes, H256, U256};
use util::RwLock;
use util::UtilError;
use util::kvdb::*;
use util::trie::{TrieFactory, TrieSpec};

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

impl bc::group::BloomGroupDatabase for Executor {
    fn blooms_at(&self, position: &bc::group::GroupPosition) -> Option<bc::group::BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self.db.read(db::COL_EXTRA, &position).map(Into::into);
        result
    }
}

#[derive(Debug, Clone)]
pub enum BlockInQueue {
    Proposal(Block),
    ConsensusBlock(Block, ProtoProof),
    SyncBlock((Block, Option<ProtoProof>)),
}

/// Rules
/// 1. When executor receives proposal from consensus, pre-execute it firstly, set stage to `ExecutingProposal`.
/// 2. When it receives another proposal,
/// 2.1 and the new proposal is different from the current one(the same transaction root),
///     interrupt the current executing and redo the new proposal;
/// 2.2 otherwise ignore it.
/// 3. When executor receives a consensus block, compares to the current excuting proposal,
/// 3.1 if they are the same, replace the proposal to consensus block, change the stage to `ExecutingBlock`.
/// 3.2 Otherwise check whether the propposal is executing,
/// 3.2.1 if yes, interrupt the current proposal and execute the consensus block,
/// 3.2.2 otherwise execute the consensus block.
/// 4. When executor finishes executing proposal, check the stage,
/// 4.1 if `ExecutingBlock`, continue;
/// 4.2 if `ExecutingProposal`, go to `WaitFinalized`,
/// 4.3 if `is_interrupt`, ignore.
#[derive(Debug, Clone)]
pub enum Stage {
    /// Exeuting block
    ExecutingBlock,
    /// Executing proposal
    ExecutingProposal,
    /// Finish executing proposal and wait
    WaitFinalized,
    /// Finalized
    Idle,
}

pub struct Executor {
    pub current_header: RwLock<Header>,
    pub is_sync: AtomicBool,
    /// Interrupt current proposal executing
    pub is_interrupted: AtomicBool,
    /// Max height in block map
    pub max_height: AtomicUsize,
    pub block_map: RwLock<BTreeMap<u64, BlockInQueue>>,
    pub stage: RwLock<Stage>,
    pub db: Arc<KeyValueDB>,
    pub state_db: StateDB,
    pub factories: Factories,
    /// Hash of the given block - only works for 256 most recent blocks excluding current
    pub last_hashes: RwLock<VecDeque<H256>>,

    /// extra caches
    pub nodes: RwLock<Vec<Address>>,
    pub block_gas_limit: AtomicUsize,
    pub account_gas_limit: RwLock<AccountGasLimit>,
    /// System contract config cache
    senders: RwLock<HashSet<Address>>,
    creators: RwLock<HashSet<Address>>,

    /// switch, check permission or not
    pub check_permission: bool,
    pub check_quota: bool,

    /// send this to chain after block that executed
    pub executed_result: RwLock<ExecutedResult>,

    /// Switch, check proof type for add_sync_block
    pub check_prooftype: u8,
}

/// Get latest header
pub fn get_current_header(db: &KeyValueDB) -> Option<Header> {
    let h: Option<H256> = db.read(db::COL_EXTRA, &CurrentHash);
    if let Some(hash) = h {
        db.read(db::COL_HEADERS, &hash)
    } else {
        warn!("not expected get_current_header.");
        None
    }
}

impl Executor {
    pub fn init_executor<R>(db: Arc<KeyValueDB>, mut genesis: Genesis, sconfig: R) -> Executor
    where
        R: Read,
    {
        let trie_factory = TrieFactory::new(TrieSpec::Generic);
        let factories = Factories {
            vm: EvmFactory::default(),
            native: NativeFactory::default(),
            trie: trie_factory,
            accountdb: Default::default(),
        };

        let journal_db = journaldb::new(Arc::clone(&db), journaldb::Algorithm::Archive, COL_STATE);
        let state_db = StateDB::new(journal_db);

        let mut executed_ret = ExecutedResult::new();
        let header = match get_current_header(&*db) {
            Some(header) => {
                let executed_header = header.clone().generate_executed_header();
                executed_ret
                    .mut_executed_info()
                    .set_header(executed_header.clone());
                header
            }
            _ => {
                genesis
                    .lazy_execute(&state_db, &factories)
                    .expect("Failed to save genesis.");
                info!("init genesis {:?}", genesis);

                let executed_header = genesis.block.header().clone().generate_executed_header();
                executed_ret
                    .mut_executed_info()
                    .set_header(executed_header.clone());
                genesis.block.header().clone()
            }
        };

        let sc: Config = serde_json::from_reader(sconfig).expect("Failed to load json file.");
        info!("config check: {:?}", sc);

        let max_height = AtomicUsize::new(0);
        max_height.store(header.number() as usize, Ordering::SeqCst);

        let executor = Executor {
            current_header: RwLock::new(header.clone()),
            is_sync: AtomicBool::new(false),
            is_interrupted: AtomicBool::new(false),
            max_height: max_height,
            block_map: RwLock::new(BTreeMap::new()),
            stage: RwLock::new(Stage::Idle),
            db: db,
            state_db: state_db,
            factories: factories,
            last_hashes: RwLock::new(VecDeque::new()),
            nodes: RwLock::new(Vec::new()),
            senders: RwLock::new(HashSet::new()),
            creators: RwLock::new(HashSet::new()),
            block_gas_limit: AtomicUsize::new(18_446_744_073_709_551_615),
            account_gas_limit: RwLock::new(AccountGasLimit::new()),
            check_permission: sc.check_permission,
            check_quota: sc.check_quota,
            executed_result: RwLock::new(executed_ret),
            check_prooftype: sc.check_prooftype,
        };

        // Build executor config
        executor.build_last_hashes(Some(header.hash()), header.number());
        executor.reload_config();
        {
            executor.set_gas_and_nodes();
        }

        executor
    }

    /// Get block hash by number
    pub fn block_hash(&self, index: BlockNumber) -> Option<H256> {
        let result = self.db.read(db::COL_EXTRA, &index);
        result
    }

    pub fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
    }

    /// Get block header by BlockId
    pub fn block_header(&self, id: BlockId) -> Option<Header> {
        match id {
            BlockId::Latest => self.block_header_by_height(self.get_current_height()),
            BlockId::Hash(hash) => self.block_header_by_hash(hash),
            BlockId::Number(number) => self.block_header_by_height(number),
            BlockId::Earliest => self.block_header_by_height(0),
        }
    }

    /// Get block header by height
    fn block_header_by_height(&self, number: BlockNumber) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.number() == number {
                return Some(header.clone());
            }
        }
        self.block_hash(number)
            .map_or(None, |h| self.block_header_by_hash(h))
    }

    /// Get block header by hash
    fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.hash() == hash {
                return Some(header.clone());
            }
        }
        let result = self.db.read(db::COL_HEADERS, &hash);
        result
    }

    fn last_hashes(&self) -> LastHashes {
        LastHashes::from(self.last_hashes.read().clone())
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_header.read().number()
    }

    pub fn get_max_height(&self) -> u64 {
        self.max_height.load(Ordering::SeqCst) as u64
    }

    pub fn get_current_hash(&self) -> H256 {
        self.current_header.read().hash()
    }

    pub fn get_prooftype(&self) -> Option<ProofType> {
        match self.check_prooftype {
            0 => Some(ProofType::AuthorityRound),
            1 => Some(ProofType::Raft),
            2 => Some(ProofType::Tendermint),
            _ => None,
        }
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

    /// Build last 256 block hashes.
    fn build_last_hashes(&self, prevhash: Option<H256>, parent_height: u64) -> Arc<LastHashes> {
        let parent_hash = prevhash.unwrap_or_else(|| {
            self.block_hash(parent_height)
                .expect("Block height always valid.")
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
            match self.block_hash(height) {
                Some(hash) => {
                    let index = (i + 1) as usize;
                    last_hashes[index] = hash;
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
        hashes.push_front(*hash);
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

    /// Attempt to get a copy of a specific block's final state.
    pub fn state_at(&self, id: BlockId) -> Option<State<StateDB>> {
        self.block_header(id)
            .map_or(None, |h| self.gen_state(*h.state_root()))
    }

    /// generate block's final state.
    pub fn gen_state(&self, root: H256) -> Option<State<StateDB>> {
        let db = self.state_db.boxed_clone();
        State::from_existing(db, root, U256::from(0), self.factories.clone()).ok()
    }

    /// Get a copy of the best block's state.
    pub fn state(&self) -> State<StateDB> {
        let mut state = self.gen_state(self.current_state_root())
            .expect("State root of current block is invalid.");
        state.senders = self.senders.read().clone();
        state.creators = self.creators.read().clone();
        state
    }

    /// Get code by address
    pub fn code_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id)
            .and_then(|s| s.code(address).ok())
            .map(|c| c.map(|c| (&*c).clone()))
    }

    pub fn nonce(&self, address: &Address, id: BlockId) -> Option<U256> {
        self.state_at(id).and_then(|s| s.nonce(address).ok())
    }

    pub fn eth_call(&self, request: CallRequest, id: BlockId) -> Result<Bytes, String> {
        let mut signed = self.sign_call(request);
        let result = self.call(&mut signed, id, Default::default());
        result
            .map(|b| b.output.into())
            .or_else(|e| Err(format!("Call Error {}", e)))
    }

    fn sign_call(&self, request: CallRequest) -> SignedTransaction {
        let from = request.from.unwrap_or_else(Address::zero);
        Transaction {
            nonce: "".to_string(),
            action: Action::Call(request.to),
            gas: U256::from(50_000_000),
            gas_price: U256::zero(),
            value: U256::zero(),
            data: request.data.map_or_else(Vec::new, |d| d.to_vec()),
            block_limit: u64::max_value(),
        }.fake_sign(from)
    }

    fn call(
        &self,
        t: &mut SignedTransaction,
        block_id: BlockId,
        analytics: CallAnalytics,
    ) -> Result<Executed, CallError> {
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

        state.senders = self.senders.read().clone();
        state.creators = self.creators.read().clone();

        let engine = NullEngine::default();

        let options = TransactOptions {
            tracing: analytics.transaction_tracing,
            vm_tracing: analytics.vm_tracing,
            check_permission: false,
            check_quota: false,
        };

        let ret = Executive::new(
            &mut state,
            &env_info,
            &engine,
            &self.factories.vm,
            &self.factories.native,
        ).transact(t, options)?;

        Ok(ret)
    }

    pub fn set_gas_and_nodes(&self) {
        let mut executed_result = self.executed_result.write();
        let mut config = ConsensusConfig::new();
        let nodes: Vec<Address> = { self.nodes.read().clone() };
        let node_list = nodes.into_iter().map(|address| address.to_vec()).collect();
        config.set_block_gas_limit(self.block_gas_limit.load(Ordering::SeqCst) as u64);
        config.set_account_gas_limit(self.account_gas_limit.read().clone().into());
        trace!("node_list : {:?}", node_list);
        config.set_nodes(node_list);
        executed_result.set_config(config);
    }

    fn set_executed_result(&self, block: &ClosedBlock) {
        self.set_gas_and_nodes();
        let mut executed_result = self.executed_result.write();
        executed_result.set_executed_info(block.protobuf());
    }

    pub fn send_executed_info_to_chain(&self, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let executed_result = { self.executed_result.read().clone() };
        let msg = factory::create_msg(
            submodules::EXECUTOR,
            topics::EXECUTED_RESULT,
            MsgClass::EXECUTED(executed_result),
        );
        ctx_pub
            .send(("executor.result".to_string(), msg.write_to_bytes().unwrap()))
            .unwrap();
    }

    ///  write data to batch
    ///1、header
    ///2、currenthash
    ///3、state
    pub fn write_batch(&self, batch: &mut DBTransaction, block: ClosedBlock) {
        let height = block.number();
        let hash = block.hash();
        trace!("commit block in db {:?}, {:?}", hash, height);

        batch.write(db::COL_HEADERS, &hash, block.header());
        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        batch.write(db::COL_EXTRA, &height, &hash);

        let mut state = block.drain();
        // Store triedb changes in journal db
        state
            .journal_under(batch, height, &hash)
            .expect("DB commit failed");
        self.prune_ancient(state).expect("mark_canonical failed");
    }

    /// Finalize block
    /// 1. Delivery rich status
    /// 2. Update cache
    /// 3. Commited data to db
    pub fn finalize_block(&self, closed_block: ClosedBlock, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let mut batch = self.db.transaction();
        // Reload config
        self.reload_config();

        let header = closed_block.header().clone();
        {
            *self.current_header.write() = header;
        }
        self.set_executed_result(&closed_block);
        self.send_executed_info_to_chain(ctx_pub);
        self.write_batch(&mut batch, closed_block);

        // Saving in db
        let now = Instant::now();
        self.db.write(batch).expect("DB write failed.");
        let new_now = Instant::now();
        info!("db write use {:?}", new_now.duration_since(now));
        self.update_last_hashes(&self.get_current_hash());
    }

    pub fn finalize_proposal(
        &self,
        mut closed_block: ClosedBlock,
        comming: Block,
        ctx_pub: &Sender<(String, Vec<u8>)>,
    ) {
        closed_block.header.set_timestamp(comming.timestamp());
        closed_block.header.set_proof(comming.proof().clone());
        self.finalize_block(closed_block, ctx_pub);
    }

    /// Reload system config from system contract
    /// 1. Senders and creators
    /// 2. Consensus nodes
    /// 3. BlockGasLimit and AccountGasLimit
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
            self.block_gas_limit
                .swap(block_gas_limit as usize, Ordering::SeqCst);
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

    /// Execute Block
    /// And set state_root, receipt_root, log_bloom of header
    pub fn execute_block(&self, block: Block, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let now = Instant::now();
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let senders = self.senders.read().clone();
        let creators = self.creators.read().clone();
        let check_permission = self.check_permission;
        let mut open_block = OpenBlock::new(
            self.factories.clone(),
            senders,
            creators,
            false,
            block,
            self.state_db.boxed_clone(),
            current_state_root,
            last_hashes.into(),
            &self.account_gas_limit.read().clone(),
        ).unwrap();
        if open_block.apply_transactions(self, check_permission, self.check_quota) {
            let closed_block = open_block.into_closed_block();
            let new_now = Instant::now();
            info!("execute block use {:?}", new_now.duration_since(now));
            self.finalize_block(closed_block, ctx_pub);
        } else {
            warn!("executing block is interrupted.");
        }
    }

    pub fn execute_proposal(&self, block: Block) -> Option<ClosedBlock> {
        let now = Instant::now();
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let senders = self.senders.read().clone();
        let creators = self.creators.read().clone();
        let check_permission = self.check_permission;
        let mut open_block = OpenBlock::new(
            self.factories.clone(),
            senders,
            creators,
            false,
            block,
            self.state_db.boxed_clone(),
            current_state_root,
            last_hashes.into(),
            &self.account_gas_limit.read().clone(),
        ).unwrap();
        if open_block.apply_transactions(self, check_permission, self.check_quota) {
            let closed_block = open_block.into_closed_block();
            let new_now = Instant::now();
            info!("execute proposal use {:?}", new_now.duration_since(now));
            let h = closed_block.number();
            info!("execute height {} proposal finish !", h);
            Some(closed_block)
        } else {
            warn!("executing block is interrupted.");
            None
        }
    }
}
