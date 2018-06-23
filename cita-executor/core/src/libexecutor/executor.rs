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

use bloomchain::group::{BloomGroup, BloomGroupDatabase, GroupPosition};
pub use byteorder::{BigEndian, ByteOrder};
use call_analytics::CallAnalytics;
use contracts::{
    AccountGasLimit, NodeManager, PermissionManagement, QuotaManager, Resource, SysConfig,
    UserManagement,
};
use db;
use db::*;
use engines::NullEngine;
use error::CallError;
use evm::env_info::{EnvInfo, LastHashes};
use evm::Factory as EvmFactory;
use executive::{Executed, Executive, TransactOptions};
use factory::*;
use header::*;
use libexecutor::blacklist::BlackList;
pub use libexecutor::block::*;
use libexecutor::call_request::CallRequest;
use libexecutor::extras::*;
use libexecutor::genesis::Genesis;
pub use libexecutor::transaction::*;
use libexecutor::ServiceMap;

use libproto::blockchain::{Proof as ProtoProof, ProofType, RichStatus};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::{ConsensusConfig, ExecutedResult, Message};

use bincode::{deserialize as bin_deserialize, serialize as bin_serialize, Infinite};
use cita_types::{Address, H256, U256};
use native::factory::Factory as NativeFactory;
use state::State;
use state_db::StateDB;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::convert::{Into, TryInto};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::time::Instant;
use types::ids::BlockId;
use types::receipt::ReceiptError;
use types::transaction::{Action, SignedTransaction, Transaction};
use util::kvdb::*;
use util::trie::{TrieFactory, TrieSpec};
use util::RwLock;
use util::UtilError;
use util::{journaldb, Bytes};

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub prooftype: u8,
    pub journaldb_type: String,
    pub grpc_port: u16,
}

impl Config {
    pub fn default() -> Self {
        Config {
            prooftype: 2,
            journaldb_type: String::from("archive"),
            grpc_port: 5000,
        }
    }

    pub fn new(path: &str) -> Self {
        parse_config!(Config, path)
    }
}

impl BloomGroupDatabase for Executor {
    fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        let result = self
            .db
            .read()
            .read(db::COL_EXTRA, &position)
            .map(Into::into);
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

enum_from_primitive! {
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EconomicalModel {
    /// Default model. Sending Transaction is free, should work with authority together.
    Quota,
    /// Transaction charges for gas * gasPrice. BlockProposer get the block reward.
    Charge,
}
}

impl Default for EconomicalModel {
    fn default() -> Self {
        EconomicalModel::Quota
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GlobalSysConfig {
    pub nodes: Vec<Address>,
    pub block_gas_limit: usize,
    pub account_gas_limit: AccountGasLimit,
    pub delay_active_interval: usize,
    pub changed_height: usize,
    pub check_quota: bool,
    pub check_permission: bool,
    pub account_permissions: HashMap<Address, Vec<Resource>>,
    pub group_accounts: HashMap<Address, Vec<Address>>,
    pub super_admin_account: Option<Address>,
    /// Interval time for creating a block (milliseconds)
    pub block_interval: u64,
}

impl GlobalSysConfig {
    fn new() -> GlobalSysConfig {
        GlobalSysConfig {
            nodes: Vec::new(),
            block_gas_limit: 18_446_744_073_709_551_615,
            account_gas_limit: AccountGasLimit::new(),
            delay_active_interval: 1,
            changed_height: 0,
            check_quota: false,
            check_permission: false,
            account_permissions: HashMap::new(),
            group_accounts: HashMap::new(),
            super_admin_account: None,
            block_interval: 3000,
        }
    }

    fn check_equal(&self, rhs: &GlobalSysConfig) -> bool {
        *&self == *&rhs
    }
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
    pub db: RwLock<Arc<KeyValueDB>>,
    pub state_db: RwLock<StateDB>,
    pub factories: Factories,
    /// Hash of the given block - only works for 256 most recent blocks excluding current
    pub last_hashes: RwLock<VecDeque<H256>>,

    /// Cache results after block that executed
    pub executed_result: RwLock<BTreeMap<u64, ExecutedResult>>,

    /// Proof type
    pub prooftype: u8,

    pub sys_configs: RwLock<VecDeque<GlobalSysConfig>>,

    pub service_map: Arc<ServiceMap>,
    pub economical_model: RwLock<EconomicalModel>,
    black_list_cache: RwLock<BlackListCache>,
}

/// Get latest header
pub fn get_current_header(db: &KeyValueDB) -> Option<Header> {
    let h: Option<H256> = db.read(db::COL_EXTRA, &CurrentHash);
    if let Some(hash) = h {
        db.read(db::COL_HEADERS, &hash)
    } else {
        warn!("Failed to get current_header from DB.");
        None
    }
}

impl Executor {
    pub fn init_executor(
        db: Arc<KeyValueDB>,
        mut genesis: Genesis,
        executor_config: Config,
    ) -> Executor {
        info!("executor config: {:?}", executor_config);

        let trie_factory = TrieFactory::new(TrieSpec::Generic);
        let factories = Factories {
            vm: EvmFactory::default(),
            native: NativeFactory::default(),
            trie: trie_factory,
            accountdb: Default::default(),
        };

        let journaldb_type = executor_config
            .journaldb_type
            .parse()
            .unwrap_or(journaldb::Algorithm::Archive);
        let journal_db = journaldb::new(Arc::clone(&db), journaldb_type, COL_STATE);
        let state_db = StateDB::new(journal_db, 5 * 1024 * 1024); // todo : cache_size would be set in config file.

        let header = match get_current_header(&*db) {
            Some(header) => header,
            _ => {
                genesis
                    .lazy_execute(&state_db, &factories)
                    .expect("Failed to save genesis.");
                trace!("init genesis {:?}", genesis);

                genesis.block.header().clone()
            }
        };
        let executed_header = header.clone().generate_executed_header();
        let mut executed_ret = ExecutedResult::new();
        executed_ret.mut_executed_info().set_header(executed_header);

        let mut executed_map = BTreeMap::new();
        executed_map.insert(header.number(), executed_ret);

        let max_height = AtomicUsize::new(0);
        max_height.store(header.number() as usize, Ordering::SeqCst);

        let executor = Executor {
            current_header: RwLock::new(header.clone()),
            is_sync: AtomicBool::new(false),
            is_interrupted: AtomicBool::new(false),
            max_height: max_height,
            block_map: RwLock::new(BTreeMap::new()),
            stage: RwLock::new(Stage::Idle),
            db: RwLock::new(db),
            state_db: RwLock::new(state_db),
            factories: factories,
            last_hashes: RwLock::new(VecDeque::new()),

            executed_result: RwLock::new(executed_map),
            prooftype: executor_config.prooftype,
            sys_configs: RwLock::new(VecDeque::new()),
            service_map: Arc::new(ServiceMap::new()),
            economical_model: RwLock::new(EconomicalModel::Quota),
            black_list_cache: RwLock::new(BlackListCache::new(10_000_000)),
        };

        // Build executor config
        executor.build_last_hashes(Some(header.hash()), header.number());

        if let Some(confs) = executor.load_config_from_db() {
            executor.set_sys_configs(confs);
        }

        executor.reorg_config();

        {
            executor.set_gas_and_nodes(header.number());
        }

        executor
    }

    pub fn set_service_map(&mut self, service_map: Arc<ServiceMap>) {
        self.service_map = service_map;
    }

    /// Get block hash by number
    pub fn block_hash(&self, index: BlockNumber) -> Option<H256> {
        let result = self.db.read().read(db::COL_EXTRA, &index);
        result
    }

    pub fn load_config_from_db(&self) -> Option<VecDeque<GlobalSysConfig>> {
        let res = self.db.read().read(db::COL_EXTRA, &ConfigHistory);
        if let Some(bres) = res {
            return bin_deserialize(&bres).ok();
        }
        None
    }

    pub fn set_sys_configs(&self, confs: VecDeque<GlobalSysConfig>) {
        *self.sys_configs.write() = confs;
    }

    pub fn get_sys_config(&self, now_height: BlockNumber) -> GlobalSysConfig {
        let confs = self.sys_configs.read().clone();
        let len = confs.len();
        if len > 0 {
            for i in 0..len {
                if confs[i].changed_height + confs[0].delay_active_interval <= now_height as usize {
                    return confs[i].clone();
                }
            }
            //for after geneis block
            return confs[0].clone();
        }
        //it can't hanppen,only in test
        GlobalSysConfig::new()
    }

    pub fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
    }

    pub fn genesis_header(&self) -> Header {
        self.block_header(BlockId::Earliest)
            .expect("get genesis error")
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
    pub fn block_header_by_height(&self, number: BlockNumber) -> Option<Header> {
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
    pub fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.hash() == hash {
                return Some(header.clone());
            }
        }
        let result = self.db.read().read(db::COL_HEADERS, &hash);
        result
    }

    fn last_hashes(&self) -> LastHashes {
        LastHashes::from(self.last_hashes.read().clone())
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_header.read().number()
    }

    pub fn get_current_timestamp(&self) -> u64 {
        self.current_header.read().timestamp()
    }

    pub fn get_max_height(&self) -> u64 {
        self.max_height.load(Ordering::SeqCst) as u64
    }

    pub fn set_max_height(&self, height: usize) {
        self.max_height.store(height, Ordering::SeqCst);
    }

    pub fn get_current_hash(&self) -> H256 {
        self.current_header.read().hash()
    }

    pub fn get_prooftype(&self) -> Option<ProofType> {
        match self.prooftype {
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

    /// Verify the block generation time interval
    /// Make sure it's longer than 3s
    pub fn validate_timestamp(&self, timestamp: u64) -> bool {
        let sys_config = SysConfig::new(self);
        let block_interval = sys_config.block_interval();
        let current_timestamp = self.get_current_timestamp();
        trace!(
            "validate_timestamp current_timestamp {:?} timestamp {:?}",
            current_timestamp,
            timestamp,
        );

        timestamp - current_timestamp >= block_interval
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
                            self.db.read().write_buffered(batch);
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
            .map_or(None, |h| self.gen_state(*h.state_root(), *h.parent_hash()))
    }

    /// Generate block's final state.
    pub fn gen_state(&self, root: H256, parent_hash: H256) -> Option<State<StateDB>> {
        let db = self.state_db.read().boxed_clone_canon(&parent_hash);
        State::from_existing(db, root, U256::from(0), self.factories.clone()).ok()
    }

    /// Get code by address
    pub fn code_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id)
            .and_then(|s| s.code(address).ok())
            .map(|c| c.map(|c| (&*c).clone()))
    }

    /// Get abi by address
    pub fn abi_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id)
            .and_then(|s| s.abi(address).ok())
            .map(|c| c.map(|c| (&*c).clone()))
    }

    /// Get balance by address
    pub fn balance_at(&self, address: &Address, id: BlockId) -> Option<Option<Bytes>> {
        self.state_at(id)
            .and_then(|s| s.balance(address).ok())
            .map(|c| {
                let mut bytes = [0u8; 32];
                c.to_big_endian(&mut bytes);
                Some(bytes.to_vec())
            })
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
            // TODO: Should Fixed?
            chain_id: u32::min_value(),
            version: 0u32,
        }.fake_sign(from)
    }

    fn call(
        &self,
        t: &SignedTransaction,
        block_id: BlockId,
        analytics: CallAnalytics,
    ) -> Result<Executed, CallError> {
        let header = self.block_header(block_id).ok_or(CallError::StatePruned)?;
        let last_hashes = self.build_last_hashes(None, header.number());
        let env_info = EnvInfo {
            number: header.number(),
            author: header.proposer().clone(),
            timestamp: header.timestamp(),
            difficulty: U256::default(),
            last_hashes: last_hashes,
            gas_used: *header.gas_used(),
            gas_limit: *header.gas_limit(),
            account_gas_limit: u64::max_value().into(),
        };
        // that's just a copy of the state.
        let mut state = self.state_at(block_id).ok_or(CallError::StatePruned)?;

        let engine = NullEngine::default();

        // Never check permission and quota
        let options = TransactOptions {
            tracing: analytics.transaction_tracing,
            vm_tracing: analytics.vm_tracing,
            check_permission: false,
            check_quota: false,
        };

        Executive::new(
            &mut state,
            &env_info,
            &engine,
            &self.factories.vm,
            &self.factories.native,
            false,
            EconomicalModel::Quota,
        ).transact(t, options)
            .map_err(Into::into)
    }

    pub fn set_gas_and_nodes(&self, height: u64) {
        let mut executed_map = self.executed_result.write();

        //send the next height's config to chain,and transfer to auth
        let conf = self.get_sys_config(height + 1);

        let mut send_config = ConsensusConfig::new();
        let node_list = conf
            .nodes
            .into_iter()
            .map(|address| address.to_vec())
            .collect();
        send_config.set_block_gas_limit(conf.block_gas_limit as u64);
        send_config.set_account_gas_limit(conf.account_gas_limit.into());
        send_config.set_check_quota(conf.check_quota);
        trace!("node_list : {:?}", node_list);
        send_config.set_nodes(node_list);
        send_config.set_block_interval(conf.block_interval);

        executed_map
            .entry(height)
            .or_insert(ExecutedResult::new())
            .set_config(send_config);
    }

    fn set_executed_result(&self, block: &ClosedBlock) {
        self.set_gas_and_nodes(block.number());
        let mut executed_map = self.executed_result.write();

        executed_map
            .get_mut(&block.number())
            .unwrap()
            .set_executed_info(block.protobuf());
    }

    pub fn send_executed_info_to_chain(&self, height: u64, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let executed_result = match self.executed_result.read().get(&height) {
            Some(execute_result) => execute_result.clone(),
            None => {
                // The executed result not found in the cache here can be generated by reading the database reconstruct.
                // However, the current sysconfig only stores the latest ten data that have changed, which means that
                // it may not be able to construct the desired result.
                // And the probability of this error is minimal and it is currently processed as a log error log.
                error!("There is no block {} execute result in the cache", height);
                return;
            }
        };

        let msg: Message = executed_result.into();
        ctx_pub
            .send((
                routing_key!(Executor >> ExecutedResult).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
    }

    ///  write data to batch
    ///1、header
    ///2、currenthash
    ///3、state
    pub fn write_batch(&self, block: ClosedBlock) {
        let mut batch = self.db.read().transaction();
        let height = block.number();
        let hash = block.hash();
        trace!("commit block in db {:?}, {:?}", hash, height);

        let confs = self.sys_configs.read().clone();
        let res = bin_serialize(&confs, Infinite).expect("serialize sys config error?");
        batch.write(db::COL_EXTRA, &ConfigHistory, &res);

        batch.write(db::COL_HEADERS, &hash, block.header());
        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        batch.write(db::COL_EXTRA, &height, &hash);

        let mut state = block.drain();
        // Store triedb changes in journal db
        state
            .journal_under(&mut batch, height, &hash)
            .expect("DB commit failed");
        state.sync_cache(&[], &[], true);
        self.db.read().write_buffered(batch);

        self.prune_ancient(state).expect("mark_canonical failed");

        // Saving in db
        let now = Instant::now();
        self.db.read().flush().expect("DB write failed.");
        let new_now = Instant::now();
        debug!("db write use {:?}", new_now.duration_since(now));
    }

    /// Finalize block
    /// 1. Delivery rich status
    /// 2. Update cache
    /// 3. Commited data to db
    /// Notice: Write db if and only if finalize block.
    pub fn finalize_block(&self, closed_block: ClosedBlock, ctx_pub: &Sender<(String, Vec<u8>)>) {
        self.reorg_config();
        self.set_executed_result(&closed_block);
        self.pub_black_list(&closed_block, ctx_pub);
        self.send_executed_info_to_chain(closed_block.number(), ctx_pub);
        self.write_batch(closed_block.clone());
        let header = closed_block.header().clone();
        {
            *self.current_header.write() = header;
        }
        self.update_last_hashes(&self.get_current_hash());
    }

    pub fn finalize_proposal(
        &self,
        mut closed_block: ClosedBlock,
        comming: Block,
        ctx_pub: &Sender<(String, Vec<u8>)>,
    ) {
        closed_block.header.set_proof(comming.proof().clone());
        self.finalize_block(closed_block, ctx_pub);
    }

    pub fn node_manager(&self) -> NodeManager {
        NodeManager::new(self, self.genesis_header().timestamp())
    }

    /// Reorg system config from system contract
    /// 1. Consensus nodes
    /// 2. BlockGasLimit and AccountGasLimit
    /// 3. Account permissions
    /// 4. Prune history
    pub fn reorg_config(&self) {
        let mut conf = GlobalSysConfig::new();
        conf.nodes = self.node_manager().shuffled_stake_nodes();
        conf.block_gas_limit = QuotaManager::block_gas_limit(self) as usize;
        let sys_config = SysConfig::new(self);
        conf.delay_active_interval = sys_config.delay_block_number() as usize;
        conf.check_permission = sys_config.permission_check();
        conf.check_quota = sys_config.quota_check();
        conf.block_interval = sys_config.block_interval();
        conf.account_permissions = PermissionManagement::load_account_permissions(self);
        conf.super_admin_account = PermissionManagement::get_super_admin_account(self);
        conf.group_accounts = UserManagement::load_group_accounts(self);
        {
            *self.economical_model.write() = sys_config.economical_model();
        }

        let common_gas_limit = QuotaManager::account_gas_limit(self);
        let specific = QuotaManager::specific(self);

        conf.account_gas_limit
            .set_common_gas_limit(common_gas_limit);
        conf.account_gas_limit.set_specific_gas_limit(specific);

        //fixbug when max_height is not equal to current_height such as sync
        let tmp_height = self.get_current_height();
        if let Some(inconf) = self.sys_configs.read().front() {
            //don't compare the changed height
            conf.changed_height = inconf.changed_height;
            if inconf.check_equal(&conf) {
                return;
            }
            conf.changed_height = tmp_height as usize;
        }

        {
            let mut confs = self.sys_configs.write();
            confs.push_front(conf);
            // Prune history config
            // TODO: shoud be delay_active_interval + 1? 10 should be enough.
            confs.truncate(10);
        }
    }

    /// Execute Block
    /// And set state_root, receipt_root, log_bloom of header
    pub fn execute_block(&self, block: Block, ctx_pub: &Sender<(String, Vec<u8>)>) {
        let now = Instant::now();
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let conf = self.get_sys_config(self.get_max_height());
        let parent_hash = block.parent_hash().clone();
        let mut open_block = OpenBlock::new(
            self.factories.clone(),
            conf.clone(),
            false,
            block,
            self.state_db.read().boxed_clone_canon(&parent_hash),
            current_state_root,
            last_hashes.into(),
        ).unwrap();
        if open_block.apply_transactions(self, conf.check_permission, conf.check_quota) {
            let closed_block = open_block.close();
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
        let conf = self.get_sys_config(self.get_max_height());
        let perm = conf.check_permission;
        let check_quota = conf.check_quota;
        let parent_hash = block.parent_hash().clone();
        let mut open_block = OpenBlock::new(
            self.factories.clone(),
            conf,
            false,
            block,
            self.state_db.read().boxed_clone_canon(&parent_hash),
            current_state_root,
            last_hashes.into(),
        ).unwrap();
        if open_block.apply_transactions(self, perm, check_quota) {
            let closed_block = open_block.close();
            let new_now = Instant::now();
            debug!("execute proposal use {:?}", new_now.duration_since(now));
            let h = closed_block.number();
            debug!("execute height {} proposal finish !", h);
            Some(closed_block)
        } else {
            warn!("executing block is interrupted.");
            None
        }
    }

    /// Prune executed_result on `BTreeMap`
    pub fn prune_execute_result_cache(&self, status: &RichStatus) {
        let height = status.get_height();
        if height > 1 {
            let mut executed_map = self.executed_result.write();
            *executed_map = executed_map.split_off(&(height - 1));
        }
    }

    /// Find the public key of all senders that caused the specified error message, and then publish it
    fn pub_black_list(&self, close_block: &ClosedBlock, ctx_pub: &Sender<(String, Vec<u8>)>) {
        match *self.economical_model.read() {
            EconomicalModel::Charge => {
                // Get all transaction hash that is reported as not enough gas
                let blacklist_transaction_hash: Vec<H256> = close_block
                    .receipts
                    .iter()
                    .filter(|ref receipt_option| match receipt_option {
                        Some(receipt) => match receipt.error {
                            Some(ReceiptError::NotEnoughBaseGas) => true,
                            _ => false,
                        },
                        _ => false,
                    })
                    .map(|receipt_option| match receipt_option {
                        Some(receipt) => receipt.transaction_hash,
                        None => H256::default(),
                    })
                    .filter(|hash| hash != &H256::default())
                    .collect();

                // Filter out accounts in the black list where the account balance has reached the benchmark value
                let mut clear_list: Vec<Address> = close_block
                    .state
                    .cache()
                    .iter()
                    .filter(|&(_, ref a)| a.is_dirty())
                    .map(|(address, ref mut a)| match a.account() {
                        Some(ref account)
                            if self.black_list_cache.read().contains(address)
                                && account.balance() >= &U256::from(100) =>
                        {
                            *address
                        }
                        None | Some(_) => Address::default(),
                    })
                    .filter(|address| address != &Address::default())
                    .collect();

                // Get address of sending account by transaction hash
                let blacklist: Vec<Address> = close_block
                    .body()
                    .transactions()
                    .iter()
                    .filter(|tx| blacklist_transaction_hash.contains(&tx.get_transaction_hash()))
                    .map(|tx| *tx.sender())
                    .collect();

                {
                    let mut black_list_cache = self.black_list_cache.write();
                    black_list_cache
                        .prune(&clear_list)
                        .extend(&blacklist, close_block.number());
                    clear_list.extend(black_list_cache.lru().iter());
                }

                let black_list = BlackList::new()
                    .set_black_list(blacklist)
                    .set_clear_list(clear_list);

                if black_list.len() > 0 {
                    let black_list_bytes: Message = black_list.protobuf().into();

                    info!("black list is {:?}", black_list.black_list());

                    ctx_pub
                        .send((
                            routing_key!(Executor >> BlackList).into(),
                            black_list_bytes.try_into().unwrap(),
                        ))
                        .unwrap();
                }
            }
            EconomicalModel::Quota => {}
        }
    }
}

/// This structure is used to perform lru based on block height
struct BlackListCache {
    cache_by_block_number: BTreeMap<u64, Vec<Address>>,
    cache_by_address: BTreeMap<Address, u64>,
    lru_number: u64,
}

impl BlackListCache {
    pub fn new(lru_number: u64) -> Self {
        BlackListCache {
            cache_by_block_number: BTreeMap::new(),
            cache_by_address: BTreeMap::new(),
            lru_number: lru_number,
        }
    }

    pub fn contains(&self, key: &Address) -> bool {
        self.cache_by_address.contains_key(key)
    }

    pub fn extend(&mut self, extend: &Vec<Address>, height: u64) -> &mut Self {
        extend.clone().into_iter().for_each(|address| {
            let _ = self.cache_by_address.insert(address, height);
        });
        self.cache_by_block_number.insert(height, extend.to_owned());
        self
    }

    pub fn prune(&mut self, clear_list: &Vec<Address>) -> &mut Self {
        let heights: HashSet<u64> = clear_list
            .clone()
            .iter()
            .map(|address| self.cache_by_address.remove(&address).unwrap())
            .collect();

        heights.iter().for_each(|&height| {
            self.cache_by_block_number
                .entry(height)
                .and_modify(|values| {
                    let _ = values
                        .iter()
                        .filter(|&value| !clear_list.contains(&value))
                        .map(|&value| value)
                        .collect::<Vec<Address>>();
                });
        });
        self
    }

    pub fn lru(&mut self) -> Vec<Address> {
        if self.lru_number <= self.cache_by_address.len() as u64 {
            let temp = self.cache_by_block_number.clone();
            let (k, v) = temp.iter().next().unwrap();
            self.cache_by_block_number.remove(k);
            v.iter().for_each(|address| {
                let _ = self.cache_by_address.remove(address);
            });
            v.clone()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::*;
    use cita_types::Address;
    use core::libchain::block::Block as ChainBlock;
    use core::receipt::ReceiptError;
    use libproto::router::{MsgType, RoutingKey, SubModules};
    use libproto::Message;
    use std::convert::TryFrom;
    use std::sync::mpsc::channel;
    use tests::helpers::{create_block, init_chain, init_executor, solc};

    fn generate_contract() -> Vec<u8> {
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
        data
    }

    #[test]
    fn test_contract_address_from_permission_denied() {
        let executor = init_executor(vec![("SysConfig.check_permission", "true")]);
        let chain = init_chain();

        let data = generate_contract();
        let block = create_block(&executor, Address::from(0), &data, (0, 1));

        let (send, recv) = channel::<(String, Vec<u8>)>();
        let inchain = chain.clone();

        let txs = block.body().transactions().clone();
        let hash = txs[0].hash();

        let h = executor.get_current_height() + 1;

        executor.execute_block(block.clone(), &send);

        if let Ok((key, msg_vec)) = recv.recv() {
            let mut msg = Message::try_from(&msg_vec).unwrap();
            match RoutingKey::from(&key) {
                routing_key!(Executor >> ExecutedResult) => {
                    let info = msg.take_executed_result().unwrap();
                    let pro = block.protobuf();
                    let chain_block = ChainBlock::from(pro);
                    inchain.set_block_body(h, &chain_block);
                    inchain.set_db_result(&info, &chain_block);
                }
                _ => {}
            }
        }

        let receipt = chain.localized_receipt(hash).unwrap();
        assert_eq!(receipt.contract_address, None);
        assert_eq!(receipt.error, Some(ReceiptError::NoTransactionPermission));
    }

    #[test]
    fn test_global_sys_config_equal() {
        let mut lhs = GlobalSysConfig::new();

        lhs.nodes.push(Address::from(0x100003));
        lhs.nodes.push(Address::from(0x100004));

        let mut rhs = GlobalSysConfig::new();

        rhs.nodes.push(Address::from(0x100003));
        rhs.nodes.push(Address::from(0x100004));

        assert_eq!(lhs, rhs);
    }

}
