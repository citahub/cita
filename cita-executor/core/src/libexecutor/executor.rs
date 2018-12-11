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

use bloomchain::group::{BloomGroup, BloomGroupDatabase, GroupPosition};
pub use byteorder::{BigEndian, ByteOrder};
use contracts::{native::factory::Factory as NativeFactory, solc::NodeManager};
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
use super::fsm::FSM;
use super::sys_config::GlobalSysConfig;
use cita_types::H256;
use crossbeam_channel::{Receiver, Sender};
use state_db::StateDB;
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

pub struct Executor {
    pub current_header: RwLock<Header>,
    pub db: RwLock<Arc<KeyValueDB>>,
    pub state_db: RwLock<StateDB>,
    pub factories: Factories,

    pub sys_config: GlobalSysConfig,
    pub engine: Box<Engine>,

    pub fsm_req_receiver: Receiver<OpenBlock>,
    pub fsm_resp_sender: Sender<ClosedBlock>,
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
        fsm_resp_sender: Sender<ClosedBlock>,
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
                warn!("Not found exist block within database. Loading genesis block...");
                genesis
                    .lazy_execute(&state_db, &factories)
                    .expect("failed to load genesis");
                genesis.block.header().clone()
            }
        };
        let mut executor = Executor {
            current_header: RwLock::new(current_header),
            db: RwLock::new(database),
            state_db: RwLock::new(state_db),
            factories,
            sys_config: GlobalSysConfig::default(),
            engine: Box::new(NullEngine::cita()),
            fsm_req_receiver,
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        };

        executor.sys_config = GlobalSysConfig::load(&executor, BlockId::Pending);

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

    pub fn rollback_current_height(&mut self, rollback_id: BlockId) {
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

        let rollback_header = self.block_header_by_height(rollback_height).unwrap();
        self.current_header = RwLock::new(rollback_header);
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
    pub fn build_last_hashes(&self, prevhash: Option<H256>, parent_height: u64) -> LastHashes {
        let parent_hash = prevhash.unwrap_or_else(|| {
            self.block_hash(parent_height)
                .unwrap_or_else(|| panic!("invalid block height: {}", parent_height))
        });

        let mut last_hashes = LastHashes::new();
        last_hashes.resize(256, H256::default());
        last_hashes[0] = parent_hash;
        for (i, last_hash) in last_hashes
            .iter_mut()
            .enumerate()
            .take(255 as usize)
            .skip(1)
        {
            if i >= parent_height as usize {
                break;
            }
            let height = parent_height - i as u64;
            *last_hash = self
                .block_hash(height)
                .expect("blocks lower then parent must exist");
        }
        last_hashes
    }

    // `executed_result_by_height` returns ExecutedResult which only contains system configs,
    // but not block data (like receipts).
    //
    // Q: So what is its called-scenario?
    // A: `executed_result_by_height` would only be called via `command::load_executed_result`;
    //    `command::load_executed_result` would only be called by Postman when it is at
    //    `bootstrap_broadcast` initializing phase;
    //    Postman do it to acquire recent 2 blocks' ExecutedResult and save them into backlogs,
    //    which be used to validate arrived Proof (ExecutedResult has "validators" config)
    pub fn executed_result_by_height(&self, height: u64) -> ExecutedResult {
        let block_id = BlockId::Number(height);
        let sys_config = GlobalSysConfig::load(&self, block_id);
        let consensus_config = make_consensus_config(sys_config);
        let executed_header = self
            .block_header(block_id)
            .unwrap()
            .generate_executed_header();
        let mut executed_result = ExecutedResult::new();
        executed_result.set_config(consensus_config);
        executed_result
            .mut_executed_info()
            .set_header(executed_header);
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

    pub fn to_executed_block(&self, open_block: OpenBlock) -> ExecutedBlock {
        let current_state_root = self.current_state_root();
        let last_hashes = self.build_last_hashes(None, open_block.number() - 1);
        let parent_hash = *open_block.parent_hash();

        ExecutedBlock::new(
            self.factories.clone(),
            &self.sys_config.block_sys_config,
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
        None
    }
}

fn open_state_db(data_path: String) -> Database {
    let database_config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let nosql_path = data_path + "/statedb";
    Database::open(&database_config, &nosql_path).unwrap()
}

pub fn make_consensus_config(sys_config: GlobalSysConfig) -> ConsensusConfig {
    let block_quota_limit = sys_config.block_quota_limit as u64;
    let account_quota_limit = sys_config.block_sys_config.account_quota_limit.into();
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
    consensus_config.set_check_quota(sys_config.block_sys_config.check_options.quota);
    consensus_config.set_block_interval(sys_config.block_interval);
    consensus_config.set_version(sys_config.chain_version);
    if sys_config.emergency_brake {
        let super_admin_account = sys_config
            .block_sys_config
            .super_admin_account
            .unwrap()
            .to_vec();
        consensus_config.set_admin_address(super_admin_account);
    }

    consensus_config
}
#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate tempdir;

    use cita_crypto::{CreateKey, KeyPair};
    use cita_types::traits::LowerHex;
    use cita_types::Address;
    use contracts::solc::sys_config::SysConfig;
    use core::receipt::ReceiptError;
    use libexecutor::command::Commander;
    use libexecutor::command::{Command, CommandResp};
    use libexecutor::fsm::FSM;
    use rustc_hex::FromHex;
    use std::str::FromStr;
    use std::thread;
    use std::time::Duration;
    use tests::helpers;
    use types::ids::BlockId;
    use types::reserved_addresses;

    #[test]
    fn test_contract_address_from_permission_denied() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let mut executor =
            helpers::init_executor(vec![("SysConfig.checkCreateContractPermission", "true")]);

        let chain = helpers::init_chain();
        let data = helpers::generate_contract();
        let block = helpers::create_block(&executor, Address::from(0), &data, (0, 1), &privkey);
        let inchain = chain.clone();
        let txs = block.body().transactions().clone();
        let hash = txs[0].hash();
        let h = executor.get_current_height() + 1;

        let closed_block = executor.into_fsm(block.clone());
        let executed_result = executor.grow(closed_block);
        inchain.set_block_body(h, &block);
        inchain.set_db_result(&executed_result, &block);

        let receipt = chain
            .localized_receipt(hash)
            .expect("failed to get localized_receipt");
        assert_eq!(receipt.contract_address, None);
        assert_eq!(receipt.error, Some(ReceiptError::NoContractPermission));
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

        let closed_block = executor.into_fsm(block);
        let _executed_result = executor.grow(closed_block);

        let chain_name_latest = SysConfig::new(&executor)
            .chain_name(BlockId::Latest)
            .unwrap();

        let chain_name_pending = SysConfig::new(&executor)
            .chain_name(BlockId::Pending)
            .unwrap();

        assert_eq!(chain_name_pending, "12345");
        assert_eq!(chain_name_latest, "abcd");
    }

    #[test]
    fn test_rollback_current_height() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let mut executor = helpers::init_executor(vec![]);

        let data = helpers::generate_contract();
        for _i in 0..5 {
            let block = helpers::create_block(&executor, Address::from(0), &data, (0, 1), &privkey);
            let closed_block = executor.into_fsm(block.clone());
            executor.grow(closed_block);
        }

        let current_height = executor.get_current_height();
        assert_eq!(current_height, 5);

        // rollback_height = current_height
        executor.rollback_current_height(BlockId::Number(current_height));
        assert_eq!(executor.get_current_height(), current_height);

        // rollback height = current_height - 3
        let rollback_to_2 = current_height - 3;
        executor.rollback_current_height(BlockId::Number(rollback_to_2));
        assert_eq!(executor.get_current_height(), 2);

        // rollback_height = 0
        executor.rollback_current_height(BlockId::Earliest);
        assert_eq!(executor.get_current_height(), 0);
    }

    #[test]
    fn test_closed_block_grow() {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let mut executor = helpers::init_executor(vec![]);

        let data = helpers::generate_contract();
        let block = helpers::create_block(&executor, Address::from(0), &data, (0, 1), &privkey);
        let closed_block = executor.into_fsm(block.clone());
        let closed_block_height = closed_block.number();
        let closed_block_hash = closed_block.hash();
        executor.grow(closed_block);

        let current_height = executor.get_current_height();
        let current_hash = executor.block_hash(current_height);
        assert_eq!(closed_block_height, current_height);
        assert_eq!(closed_block_hash, current_hash);
    }

    #[test]
    fn test_executor_exit() {
        let (_fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
        let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
        let (command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
        let (command_resp_sender, command_resp_receiver) = crossbeam_channel::bounded(0);
        let mut executor = helpers::init_executor2(
            vec![],
            fsm_req_receiver.clone(),
            fsm_resp_sender,
            command_req_receiver,
            command_resp_sender,
        );

        let handle = thread::spawn(move || {
            executor.do_loop();
        });
        // send Command, this cause executor exit
        command_req_sender.send(Command::Exit(BlockId::Number(0)));

        ::std::thread::sleep(Duration::new(2, 0));
        let resp: CommandResp = command_resp_receiver.recv().unwrap();
        assert_eq!(format!("{}", resp), format!("{}", CommandResp::Exit));

        handle.join().expect("
            We send command exit and expect executor thread return, so this test execute successfully.
            If executor did not died, this test will run in loop endless.
        ");
    }
}
