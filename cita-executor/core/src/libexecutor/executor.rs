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
use contracts::{
    native::factory::Factory as NativeFactory,
    solc::{
        AccountQuotaLimit, EmergencyBrake, NodeManager, PermissionManagement, QuotaManager,
        Resource, SysConfig, UserManagement, VersionManager, AUTO_EXEC_QL_VALUE,
    },
};
use db;
use db::*;
use engines::{Engine, NullEngine};
use evm::env_info::LastHashes;
use evm::Factory as EvmFactory;
use factory::*;
use header::*;
pub use libexecutor::block::*;
use libexecutor::genesis::Genesis;
use types::extras::*;

use libproto::{ConsensusConfig, ExecutedResult};

use super::command::{Command, CommandResp, Commander};
use super::economical_model::EconomicalModel;
use super::fsm::FSM;
use cita_types::{Address, H256};
use crossbeam_channel::{Receiver, Sender};
use state_db::StateDB;
use std::collections::{HashMap, VecDeque};
use std::convert::{From, Into};
use std::sync::Arc;
use std::time::Instant;
use types::ids::BlockId;
use util::journaldb;
use util::kvdb::*;
use util::kvdb::{Database, DatabaseConfig};
use util::trie::{TrieFactory, TrieSpec};
use util::RwLock;
use util::UtilError;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GlobalSysConfig {
    pub nodes: Vec<Address>,
    pub validators: Vec<Address>,
    pub block_quota_limit: usize,
    pub account_quota_limit: AccountQuotaLimit,
    pub delay_active_interval: usize,
    pub changed_height: usize,
    pub check_quota: bool,
    pub check_permission: bool,
    pub check_send_tx_permission: bool,
    pub check_create_contract_permission: bool,
    pub check_fee_back_platform: bool,
    pub chain_owner: Address,
    pub account_permissions: HashMap<Address, Vec<Resource>>,
    pub group_accounts: HashMap<Address, Vec<Address>>,
    pub super_admin_account: Option<Address>,
    pub block_interval: u64,
    pub emergency_brake: bool,
    pub chain_version: u32,
    pub auto_exec_quota_limit: u64,
    pub auto_exec: bool,
}

impl Default for GlobalSysConfig {
    fn default() -> Self {
        GlobalSysConfig {
            nodes: Vec::new(),
            validators: Vec::new(),
            block_quota_limit: 18_446_744_073_709_551_615,
            account_quota_limit: AccountQuotaLimit::new(),
            delay_active_interval: 1,
            changed_height: 0,
            check_quota: false,
            check_permission: false,
            check_send_tx_permission: false,
            check_create_contract_permission: false,
            check_fee_back_platform: false,
            chain_owner: Address::from(0),
            account_permissions: HashMap::new(),
            group_accounts: HashMap::new(),
            super_admin_account: None,
            block_interval: 3000,
            emergency_brake: false,
            chain_version: 0,
            auto_exec_quota_limit: AUTO_EXEC_QL_VALUE,
            auto_exec: false,
        }
    }
}

pub struct CheckOptions {
    pub permission: bool,
    pub quota: bool,
    pub fee_back_platform: bool,
    pub send_tx_permission: bool,
    pub create_contract_permission: bool,
}

pub struct Executor {
    pub current_header: RwLock<Header>,
    pub db: RwLock<Arc<KeyValueDB>>,
    pub state_db: RwLock<StateDB>,
    pub factories: Factories,

    pub sys_config: GlobalSysConfig,
    pub economical_model: RwLock<EconomicalModel>,
    pub engine: Box<Engine>,

    // block-hashes of recent 256 blocks, which used for `BLOCKHASH` opcode
    pub last_hashes: RwLock<VecDeque<H256>>,

    pub fsm_req_receiver: Receiver<OpenBlock>,
    pub fsm_resp_sender: Sender<(ClosedBlock, ExecutedResult)>,
    pub command_req_receiver: Receiver<Command>,
    pub command_resp_sender: Sender<CommandResp>,
}

impl Executor {
    #[allow(unknown_lints, clippy::too_many_arguments)] // TODO clippy
    pub fn init(
        genesis_path: &str,
        journaldb_type: &str,
        statedb_cache_size: usize,
        data_path: String,
        fsm_req_receiver: Receiver<OpenBlock>,
        fsm_resp_sender: Sender<(ClosedBlock, ExecutedResult)>,
        command_req_receiver: Receiver<Command>,
        command_resp_sender: Sender<CommandResp>,
    ) -> Executor {
        let mut genesis = Genesis::init(&genesis_path);
        let database = open_state_db(data_path);
        let database: Arc<KeyValueDB> = Arc::new(database);
        let journaldb_type = journaldb_type
            .parse()
            .unwrap_or(journaldb::Algorithm::Archive);
        let journal_db = journaldb::new(Arc::clone(&database), journaldb_type, COL_STATE);
        let state_db = StateDB::new(journal_db, statedb_cache_size);
        let trie_factory = TrieFactory::new(TrieSpec::Generic);
        let factories = Factories {
            vm: EvmFactory::default(),
            native: NativeFactory::default(),
            trie: trie_factory,
            accountdb: Default::default(),
        };
        let current_header = match get_current_header(&*database) {
            Some(header) => header,
            None => {
                genesis
                    .lazy_execute(&state_db, &factories)
                    .expect("failed to load genesis");
                genesis.block.header().clone()
            }
        };
        let hash = current_header.hash().unwrap();
        let number = current_header.number();
        let mut executor = Executor {
            current_header: RwLock::new(current_header),
            db: RwLock::new(database),
            state_db: RwLock::new(state_db),
            factories,
            last_hashes: RwLock::new(VecDeque::new()),
            sys_config: GlobalSysConfig::default(),
            economical_model: RwLock::new(EconomicalModel::Quota),
            engine: Box::new(NullEngine::cita()),
            fsm_req_receiver,
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        };

        executor.build_last_hashes(Some(hash), number);
        executor.sys_config = executor.load_sys_config(BlockId::Pending);

        info!(
            "executor init, current_height: {}, current_hash: {:?}",
            executor.get_current_height(),
            executor.get_current_hash(),
        );
        executor
    }

    pub fn close(&mut self) {
        // FIXME close database gracefully
        // self.db.read().close();
        info!(
            "executor closed, current_height: {}",
            self.get_current_height()
        );
    }

    pub fn do_loop(&mut self) {
        loop {
            match self.recv() {
                (None, None) | (Some(_), Some(_)) => return,
                (Some(command), None) => {
                    trace!("executor receive {}", command);
                    match self.operate(command) {
                        CommandResp::Exit => {
                            self.command_resp_sender.send(CommandResp::Exit);
                            return;
                        }
                        command_resp => self.command_resp_sender.send(command_resp),
                    }
                }
                (None, Some(block)) => {
                    let fsm_resp = self.into_fsm(block);
                    self.fsm_resp_sender.send(fsm_resp);
                }
            }
        }
    }

    fn recv(&self) -> (Option<Command>, Option<OpenBlock>) {
        let err_flag = (None, None);
        select! {
            recv(self.command_req_receiver, command_req) => {
                match command_req {
                    Some(command_req) => (Some(command_req), None),
                    None => err_flag,
                }
            },
            recv(self.fsm_req_receiver, fsm_req) => {
                match fsm_req {
                    Some(fsm_req) => (None, Some(fsm_req)),
                    None => err_flag,
                }
            }
        }
    }

    pub fn rollback_current_height(&self, rollback_id: BlockId) {
        let rollback_height: BlockNumber = match rollback_id {
            BlockId::Number(height) => height,
            BlockId::Earliest => 0,
            _ => unimplemented!(),
        };
        if self.get_current_height() != rollback_height {
            warn!(
                "executor roll back from {} to {}",
                self.get_current_height(),
                rollback_height
            );
            let rollback_hash = self
                .block_hash(rollback_height)
                .expect("the target block to roll back should exist");
            let mut batch = self.db.read().transaction();
            batch.write(db::COL_EXTRA, &CurrentHash, &rollback_hash);
            self.db.read().write(batch).unwrap();
        }
    }

    /// Write data to db
    /// 1. Header
    /// 2. CurrentHash
    /// 3. State
    pub fn write_batch(&self, block: ClosedBlock) {
        let mut batch = self.db.read().transaction();
        let height = block.number();
        let hash = block.hash().unwrap();
        let version = block.version();
        trace!(
            "commit block in db hash {:?}, height {:?}, version {}",
            hash,
            height,
            version
        );

        batch.write(db::COL_HEADERS, &hash, block.header());
        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        batch.write(db::COL_EXTRA, &height, &hash);

        let mut state = block.drain();
        // Store triedb changes in journal db
        state
            .journal_under(&mut batch, height, &hash)
            .expect("DB commit failed");
        // state.sync_cache();
        self.db.read().write_buffered(batch);

        self.prune_ancient(state).expect("mark_canonical failed");

        // Saving in db
        let now = Instant::now();
        self.db.read().flush().expect("DB write failed.");
        let new_now = Instant::now();
        debug!("db write use {:?}", new_now.duration_since(now));
    }

    /// Get block hash by number
    fn block_hash(&self, index: BlockNumber) -> Option<H256> {
        self.db.read().read(db::COL_EXTRA, &index)
    }

    fn current_state_root(&self) -> H256 {
        *self.current_header.read().state_root()
    }

    pub fn genesis_header(&self) -> Header {
        self.block_header(BlockId::Earliest)
            .expect("failed to fetch genesis header")
    }

    /// Get block header by BlockId
    pub fn block_header(&self, id: BlockId) -> Option<Header> {
        match id {
            BlockId::Latest => self.block_header_by_height(self.get_latest_height()),
            BlockId::Hash(hash) => self.block_header_by_hash(hash),
            BlockId::Number(number) => self.block_header_by_height(number),
            BlockId::Earliest => self.block_header_by_height(0),
            BlockId::Pending => self.block_header_by_height(self.get_pending_height()),
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
            .and_then(|h| self.block_header_by_hash(h))
    }

    /// Get block header by hash
    fn block_header_by_hash(&self, hash: H256) -> Option<Header> {
        {
            let header = self.current_header.read();
            if header.hash().unwrap() == hash {
                return Some(header.clone());
            }
        }
        self.db.read().read(db::COL_HEADERS, &hash)
    }

    fn last_hashes(&self) -> LastHashes {
        LastHashes::from(self.last_hashes.read().clone())
    }

    #[inline]
    fn get_latest_height(&self) -> u64 {
        self.current_header.read().number().saturating_sub(1)
    }

    #[inline]
    fn get_pending_height(&self) -> u64 {
        self.current_header.read().number()
    }

    #[inline]
    pub fn get_current_height(&self) -> u64 {
        self.current_header.read().number()
    }

    #[inline]
    pub fn get_current_hash(&self) -> H256 {
        self.current_header.read().hash().unwrap()
    }

    /// Build last 256 block hashes.
    pub fn build_last_hashes(&self, prevhash: Option<H256>, parent_height: u64) -> Arc<LastHashes> {
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

    pub fn update_last_hashes(&self, hash: &H256) {
        let mut hashes = self.last_hashes.write();
        if hashes.len() > 255 {
            hashes.pop_back();
        }
        hashes.push_front(*hash);
    }

    pub fn make_consensus_config(&self) -> ConsensusConfig {
        let sys_config = self.sys_config.clone();
        let block_quota_limit = sys_config.block_quota_limit as u64;
        let account_quota_limit = sys_config.account_quota_limit.into();
        let node_list = sys_config
            .nodes
            .into_iter()
            .map(|address| address.to_vec())
            .collect();
        let validators = sys_config
            .validators
            .into_iter()
            .map(|address| address.to_vec())
            .collect();
        let mut consensus_config = ConsensusConfig::new();
        consensus_config.set_block_quota_limit(block_quota_limit);
        consensus_config.set_account_quota_limit(account_quota_limit);
        consensus_config.set_nodes(node_list);
        consensus_config.set_validators(validators);
        consensus_config.set_check_quota(sys_config.check_quota);
        consensus_config.set_block_interval(sys_config.block_interval);
        consensus_config.set_version(sys_config.chain_version);
        if sys_config.emergency_brake {
            let super_admin_account = sys_config.super_admin_account.unwrap().to_vec();
            consensus_config.set_admin_address(super_admin_account);
        }

        consensus_config
    }

    pub fn make_executed_result(&self, closed_block: &ClosedBlock) -> ExecutedResult {
        let consensus_config = self.make_consensus_config();
        let executed_info = closed_block.protobuf();
        let mut executed_result = ExecutedResult::new();
        executed_result.set_config(consensus_config);
        executed_result.set_executed_info(executed_info);
        executed_result
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

    #[inline]
    pub fn node_manager(&self) -> NodeManager {
        NodeManager::new(self, self.genesis_header().timestamp())
    }

    // TODO We have to update all default value when they was changed in .sol files.
    // Is there any better solution?
    /// ensure system configurations reloaded if has changed, of which address is stored within a
    /// special system contract
    ///   1. consensus nodes
    ///   2. BlockGasLimit and AccountQuotaLimit
    ///   3. account permissions
    ///   4. version
    pub fn load_sys_config(&self, block_id: BlockId) -> GlobalSysConfig {
        let mut conf = GlobalSysConfig::default();
        conf.nodes = self
            .node_manager()
            .shuffled_stake_nodes(block_id)
            .unwrap_or_else(NodeManager::default_shuffled_stake_nodes);
        conf.validators = self
            .node_manager()
            .nodes(block_id)
            .unwrap_or_else(NodeManager::default_shuffled_stake_nodes);

        let quota_manager = QuotaManager::new(self);
        conf.block_quota_limit = quota_manager
            .block_quota_limit(block_id)
            .unwrap_or_else(QuotaManager::default_block_quota_limit)
            as usize;
        conf.auto_exec_quota_limit = quota_manager
            .auto_exec_quota_limit(block_id)
            .unwrap_or_else(QuotaManager::default_auto_exec_quota_limit);
        let sys_config = SysConfig::new(self);
        conf.delay_active_interval = sys_config
            .delay_block_number(block_id)
            .unwrap_or_else(SysConfig::default_delay_block_number)
            as usize;
        conf.check_permission = sys_config
            .permission_check(block_id)
            .unwrap_or_else(SysConfig::default_permission_check);
        conf.check_send_tx_permission = sys_config
            .send_tx_permission_check(block_id)
            .unwrap_or_else(SysConfig::default_send_tx_permission_check);
        conf.check_create_contract_permission = sys_config
            .create_contract_permission_check(block_id)
            .unwrap_or_else(SysConfig::default_create_contract_permission_check);
        conf.check_quota = sys_config
            .quota_check(block_id)
            .unwrap_or_else(SysConfig::default_quota_check);
        conf.check_fee_back_platform = sys_config
            .fee_back_platform_check(block_id)
            .unwrap_or_else(SysConfig::default_fee_back_platform_check);
        conf.chain_owner = sys_config
            .chain_owner(block_id)
            .unwrap_or_else(SysConfig::default_chain_owner);
        conf.block_interval = sys_config
            .block_interval(block_id)
            .unwrap_or_else(SysConfig::default_block_interval);
        conf.auto_exec = sys_config
            .auto_exec(block_id)
            .unwrap_or_else(SysConfig::default_auto_exec);

        let permission_manager = PermissionManagement::new(self);
        conf.account_permissions = permission_manager.load_account_permissions(block_id);
        conf.super_admin_account = permission_manager.get_super_admin_account(block_id);

        let user_manager = UserManagement::new(self);
        conf.group_accounts = user_manager.load_group_accounts(block_id);
        {
            // FIXME move out this ugly code from here !!!
            *self.economical_model.write() = sys_config
                .economical_model(block_id)
                .unwrap_or_else(SysConfig::default_economical_model);
        }

        let common_quota_limit = quota_manager
            .account_quota_limit(block_id)
            .unwrap_or_else(QuotaManager::default_account_quota_limit);
        let specific = quota_manager.specific(block_id);

        conf.account_quota_limit
            .set_common_quota_limit(common_quota_limit);
        conf.account_quota_limit.set_specific_quota_limit(specific);
        conf.changed_height = self.get_current_height() as usize;

        let emergency_manager = EmergencyBrake::new(self);
        conf.emergency_brake = emergency_manager
            .state(block_id)
            .unwrap_or_else(EmergencyBrake::default_state);

        let version_manager = VersionManager::new(self);
        conf.chain_version = version_manager
            .get_version(block_id)
            .unwrap_or_else(VersionManager::default_version);

        conf
    }

    pub fn to_executed_block(&self, open_block: OpenBlock) -> ExecutedBlock {
        let current_state_root = self.current_state_root();
        let last_hashes = self.last_hashes();
        let sys_config = self.sys_config.clone();
        let parent_hash = *open_block.parent_hash();

        ExecutedBlock::new(
            self.factories.clone(),
            sys_config,
            false,
            open_block,
            self.state_db.read().boxed_clone_canon(&parent_hash),
            current_state_root,
            last_hashes.into(),
        )
        .unwrap()
    }
}

impl<'a> BloomGroupDatabase for Executor {
    fn blooms_at(&self, position: &GroupPosition) -> Option<BloomGroup> {
        let position = LogGroupPosition::from(position.clone());
        self.db
            .read()
            .read(db::COL_EXTRA, &position)
            .map(Into::into)
    }
}

pub fn get_current_header(db: &KeyValueDB) -> Option<Header> {
    let h: Option<H256> = db.read(db::COL_EXTRA, &CurrentHash);
    if let Some(hash) = h {
        db.read(db::COL_HEADERS, &hash)
    } else {
        warn!("Failed to get current_header from DB.");
        None
    }
}

fn open_state_db(data_path: String) -> Database {
    let database_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let nosql_path = data_path + "/statedb";
    Database::open(&database_config, &nosql_path).unwrap()
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate tempdir;

    use super::*;
    use cita_crypto::{CreateKey, KeyPair};
    use cita_types::traits::LowerHex;
    use cita_types::Address;
    use core::receipt::ReceiptError;
    use rustc_hex::FromHex;
    use std::str::FromStr;
    use tests::helpers;
    use types::reserved_addresses;

    #[test]
    fn test_contract_address_from_permission_denied() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let mut executor =
            helpers::init_executor(vec![("SysConfig.checkCreateContractPermission", "true")]);

        let chain = helpers::init_chain();
        let data = generate_contract();
        let block = helpers::create_block(&executor, Address::from(0), &data, (0, 1), &privkey);
        let inchain = chain.clone();
        let txs = block.body().transactions().clone();
        let hash = txs[0].hash();
        let h = executor.get_current_height() + 1;

        let resp = executor.into_fsm(block.clone());
        let (_closed_block, executed_result) = resp;
        inchain.set_block_body(h, &block);
        inchain.set_db_result(&executed_result, &block);

        let receipt = chain
            .localized_receipt(hash)
            .expect("failed to get localized_receipt");
        assert_eq!(receipt.contract_address, None);
        assert_eq!(receipt.error, Some(ReceiptError::NoContractPermission));
    }

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
        let (data, _) = helpers::solc("ConstructSol", source);
        data
    }

    #[test]
    fn test_global_sys_config_equal() {
        let mut lhs = GlobalSysConfig::default();

        lhs.nodes.push(Address::from(0x100003));
        lhs.nodes.push(Address::from(0x100004));

        let mut rhs = GlobalSysConfig::default();

        rhs.nodes.push(Address::from(0x100003));
        rhs.nodes.push(Address::from(0x100004));

        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_chain_name_valid_block_number() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let addr = keypair.address().lower_hex();

        let mut executor = helpers::init_executor(vec![
            ("SysConfig.chainName", "abcd"),
            ("Admin.admin", &addr),
        ]);

        let to = Address::from_str(reserved_addresses::SYS_CONFIG).unwrap();
        let data = "c0c41f220000000000000000000000000000000000000000000\
                    000000000000000000020000000000000000000000000000000\
                    000000000000000000000000000000000531323334350000000\
                    00000000000000000000000000000000000000000000000";
        let code = data.from_hex().unwrap();
        let block = helpers::create_block(&executor, to, &code, (0, 1), &privkey);

        let (closed_block, _executed_result) = executor.into_fsm(block);
        executor.grow(closed_block);

        let chain_name_latest = SysConfig::new(&executor)
            .chain_name(BlockId::Latest)
            .unwrap();

        let chain_name_pending = SysConfig::new(&executor)
            .chain_name(BlockId::Pending)
            .unwrap();

        assert_eq!(chain_name_pending, "12345");
        assert_eq!(chain_name_latest, "abcd");
    }
}
