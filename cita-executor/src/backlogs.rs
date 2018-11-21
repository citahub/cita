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

use core::header::Header;
use core::libexecutor::block::{ClosedBlock, OpenBlock};
use libproto::ExecutedResult;
use std::collections::BTreeMap;
use util::Itertools;
// TODO compact Proof trait but not only BftProof
use proof::BftProof as Proof;

#[derive(Clone)]
pub struct Backlog {
    open_block: Option<OpenBlock>,
    proof: Option<Proof>,
    closed_block: Option<ClosedBlock>,
    executed_result: Option<ExecutedResult>,
}

impl Default for Backlog {
    fn default() -> Self {
        Backlog {
            open_block: None,
            proof: None,
            closed_block: None,
            executed_result: None,
        }
    }
}

impl Backlog {
    // is_completed return true if block, proof, and executed_result are all exist and matched
    pub fn is_completed(&self) -> bool {
        if self.open_block.is_none()
            || self.proof.is_none()
            || self.executed_result.is_none()
            || self.closed_block.is_none()
        {
            false
        } else {
            let open_block = self.open_block.as_ref().unwrap();
            let closed_block = self.closed_block.as_ref().unwrap();
            closed_block.is_equivalent(open_block)
        }
    }

    pub fn complete(&mut self) {
        assert!(self.is_completed());
        let bft_proof = self.clone_proof();
        let mut closed_block = self.clone_closed_block();
        closed_block.header.set_proof(bft_proof.into());
        closed_block.rehash();

        self.closed_block = Some(closed_block);
    }

    pub fn is_equivalent_block(&self, header: &Header) -> bool {
        match self.open_block {
            Some(ref open_block) => open_block.is_equivalent(header),
            None => false,
        }
    }

    pub fn clone_closed_block(&self) -> ClosedBlock {
        self.closed_block.clone().unwrap()
    }

    pub fn clone_proof(&self) -> Proof {
        self.proof.clone().unwrap()
    }
}

#[derive(Clone)]
pub struct Backlogs {
    // max height within backlogs, should be equal to `max(backlog.keys())`
    max_height: u64,

    // current height of local chain, should be equal to `min(backlog.keys())`
    current_height: u64,

    // {height => Block}, which indicates pending processing blocks
    backlogs: BTreeMap<u64, Backlog>,

    // {height => ClosedBlock}, which indicates the executed results of elder blocks
    completed: BTreeMap<u64, Backlog>,
}

impl Backlogs {
    pub fn new(current_height: u64) -> Backlogs {
        Backlogs {
            current_height,
            max_height: current_height,
            backlogs: BTreeMap::new(),
            completed: BTreeMap::new(),
        }
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_height
    }

    pub fn get_open_block(&self, height: u64) -> Option<OpenBlock> {
        match self.backlogs.get(&height) {
            Some(backlog) => backlog.open_block.clone(),
            None => None,
        }
    }

    pub fn get_closed_block(&self, height: u64) -> Option<ClosedBlock> {
        match self.backlogs.get(&height) {
            Some(backlog) => backlog.closed_block.clone(),
            None => None,
        }
    }

    pub fn get_completed_result(&self, height: u64) -> Option<ExecutedResult> {
        match self.completed.get(&height) {
            Some(backlog) => backlog.executed_result.clone(),
            None => None,
        }
    }

    pub fn insert_open_block(&mut self, height: u64, open_block: OpenBlock) -> bool {
        if !self.assert_height(height) {
            return false;
        }

        let backlog = self.backlogs.entry(height).or_default();
        backlog.open_block = Some(open_block);
        backlog.proof = None;

        if self.max_height < height {
            self.max_height = height;
        }
        true
    }

    pub fn insert_proof(&mut self, height: u64, proof: Proof) -> bool {
        if !self.assert_height(height) {
            return false;
        }

        let backlog = self.backlogs.entry(height).or_default();
        backlog.proof = Some(proof);
        true
    }

    pub fn insert_result(
        &mut self,
        height: u64,
        closed_block: ClosedBlock,
        executed_result: ExecutedResult,
    ) -> bool {
        if !self.assert_height(height) {
            return false;
        }

        let backlog = self.backlogs.entry(height).or_default();
        if !backlog.is_equivalent_block(closed_block.header()) {
            // discard staled executed result
            false
        } else {
            backlog.closed_block = Some(closed_block);
            backlog.executed_result = Some(executed_result);
            true
        }
    }

    pub fn is_completed(&self, height: u64) -> bool {
        match self.backlogs.get(&height) {
            Some(backlog) => backlog.is_completed(),
            None => false,
        }
    }

    // move "front of backlogs" into "back of completed"
    pub fn complete(&mut self, height: u64) -> Backlog {
        let mut backlog = self.backlogs.remove(&height).unwrap();
        backlog.complete();

        self.completed.insert(height, backlog.clone());
        self.current_height += 1;
        backlog
    }

    pub fn completed_keys(&self) -> ::std::vec::Vec<&u64> {
        self.completed.keys().sorted()
    }

    pub fn prune(&mut self, height: u64) {
        self.completed = self.completed.split_off(&height);
    }

    fn assert_height(&self, height: u64) -> bool {
        if self.current_height > height {
            error!(
                "unexpected height, current height({}) > arrived height({})",
                self.current_height, height,
            );
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Backlog;
    use super::Backlogs;
    use cita_types::H256;
    use core::header::OpenHeader;
    use core::libexecutor::block::{BlockBody, ExecutedBlock, OpenBlock};
    use core::libexecutor::economical_model::EconomicalModel;
    use core::libexecutor::executor::GlobalSysConfig;
    use core::state_db::StateDB;
    use libproto::blockchain::Proof;
    use libproto::ExecutedResult;
    use proof::BftProof;
    use std::collections::HashMap;
    use std::sync::Arc;
    use util::journaldb;
    use util::kvdb::{in_memory, KeyValueDB};

    fn generate_block_body() -> BlockBody {
        let mut stx = SignedTransaction::default();
        use types::transaction::SignedTransaction;
        stx.data = vec![1; 200];
        let transactions = vec![stx; 200];
        BlockBody { transactions }
    }

    fn generate_block_header() -> OpenHeader {
        OpenHeader::default()
    }

    fn generate_block() -> OpenBlock {
        let block_body = generate_block_body();
        let block_header = generate_block_header();
        OpenBlock {
            body: block_body,
            header: block_header,
        }
    }

    fn generate_bft_proof() -> BftProof {
        BftProof::new(0, 1, H256::default(), HashMap::new())
    }

    fn generate_state_db() -> StateDB {
        let database = in_memory(7);
        let database: Arc<KeyValueDB> = Arc::new(database);
        let journaldb_type = journaldb::Algorithm::Archive;
        let journal_db = journaldb::new(Arc::clone(&database), journaldb_type, None);
        StateDB::new(journal_db, 5 * 1024 * 1024)
    }

    #[test]
    fn test_backlog_is_completed_with_default() {
        assert_eq!(false, Backlog::default().is_completed());
    }

    #[test]
    fn test_backlog_is_completed_with_none() {
        let open_block = generate_block();
        let proof = generate_bft_proof();
        let executed_result = ExecutedResult::new();
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::new(
            Default::default(),
            GlobalSysConfig::default(),
            false,
            open_block.clone(),
            state_db,
            util::hashable::HASH_NULL_RLP,
            Arc::new(Vec::new()),
        )
        .unwrap();
        let closed_block = exec_block.close(EconomicalModel::Quota);

        {
            let backlog = Backlog {
                open_block: None,
                proof: Some(proof.clone()),
                closed_block: Some(closed_block.clone()),
                executed_result: Some(executed_result.clone()),
            };
            assert_eq!(false, backlog.is_completed(), "block is none");
        }

        {
            let backlog = Backlog {
                open_block: Some(open_block.clone()),
                proof: None,
                closed_block: Some(closed_block.clone()),
                executed_result: Some(executed_result.clone()),
            };
            assert_eq!(false, backlog.is_completed(), "proof is none");
        }

        {
            let backlog = Backlog {
                open_block: Some(open_block.clone()),
                proof: Some(proof.clone()),
                closed_block: None,
                executed_result: Some(executed_result.clone()),
            };
            assert_eq!(false, backlog.is_completed(), "closed_block is none");
        }
    }

    #[test]
    fn test_is_completed_with_unequal_block() {
        let open_block = generate_block();
        let proof = generate_bft_proof();
        let executed_result = ExecutedResult::new();
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::new(
            Default::default(),
            GlobalSysConfig::default(),
            false,
            open_block.clone(),
            state_db,
            util::hashable::HASH_NULL_RLP,
            Arc::new(Vec::new()),
        )
        .unwrap();
        let closed_block = exec_block.close(EconomicalModel::Quota);

        {
            let mut block = open_block.clone();
            block.header.set_timestamp(1);
            let backlog = Backlog {
                open_block: Some(block.clone()),
                proof: Some(proof.clone()),
                closed_block: Some(closed_block.clone()),
                executed_result: Some(executed_result.clone()),
            };
            assert_eq!(false, backlog.is_completed());
        }

        {
            let backlog = Backlog {
                open_block: Some(open_block.clone()),
                proof: Some(proof.clone()),
                closed_block: Some(closed_block.clone()),
                executed_result: Some(executed_result.clone()),
            };
            assert!(backlog.is_completed());
        }
    }

    #[test]
    #[should_panic]
    fn test_complete_but_is_completed_false() {
        let open_block = generate_block();
        let executed_result = ExecutedResult::new();
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::new(
            Default::default(),
            GlobalSysConfig::default(),
            false,
            open_block.clone(),
            state_db,
            util::hashable::HASH_NULL_RLP,
            Arc::new(Vec::new()),
        )
        .unwrap();
        let closed_block = exec_block.close(EconomicalModel::Quota);

        let mut backlog = Backlog {
            open_block: Some(open_block),
            proof: None,
            closed_block: Some(closed_block.clone()),
            executed_result: Some(executed_result.clone()),
        };
        backlog.complete();
    }

    #[test]
    fn test_complete_normal() {
        let open_block = generate_block();
        let proof = generate_bft_proof();
        let executed_result = ExecutedResult::new();
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::new(
            Default::default(),
            GlobalSysConfig::default(),
            false,
            open_block.clone(),
            state_db,
            util::hashable::HASH_NULL_RLP,
            Arc::new(Vec::new()),
        )
        .unwrap();
        let closed_block = exec_block.close(EconomicalModel::Quota);

        let mut backlog = Backlog {
            open_block: Some(open_block),
            proof: Some(proof),
            closed_block: Some(closed_block.clone()),
            executed_result: Some(executed_result.clone()),
        };
        backlog.complete();
        let closed_block = backlog.clone_closed_block();
        let proof = backlog.clone_proof();
        let proto_proof: Proof = proof.into();
        assert_eq!(proto_proof.content, closed_block.proof().content);
    }

    #[test]
    fn test_backlogs_whole_flow() {
        let open_block = generate_block();
        let proof = generate_bft_proof();
        let executed_result = ExecutedResult::new();
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::new(
            Default::default(),
            GlobalSysConfig::default(),
            false,
            open_block.clone(),
            state_db,
            util::hashable::HASH_NULL_RLP,
            Arc::new(Vec::new()),
        )
        .unwrap();
        let closed_block = exec_block.close(EconomicalModel::Quota);

        let mut backlogs = Backlogs::new(3);
        assert_eq!(false, backlogs.insert_open_block(2, open_block.clone()));
        assert_eq!(false, backlogs.insert_proof(2, proof.clone()));
        assert_eq!(
            false,
            backlogs.insert_result(2, closed_block.clone(), executed_result.clone()),
            "insert staled result should return false",
        );
        assert!(backlogs.get_open_block(2).is_none());
        assert!(backlogs.get_closed_block(2).is_none());

        assert_eq!(true, backlogs.insert_open_block(3, open_block.clone()));
        assert_eq!(true, backlogs.insert_proof(3, proof.clone()));
        assert_eq!(
            true,
            backlogs.insert_result(3, closed_block.clone(), executed_result.clone()),
            "insert current result should return true",
        );
        assert!(backlogs.get_open_block(3).is_some());
        assert!(backlogs.get_closed_block(3).is_some());

        let _backlog = backlogs.complete(3);
        assert!(backlogs.get_completed_result(2).is_none());
        assert!(backlogs.get_completed_result(3).is_some());

        assert!(backlogs.get_open_block(3).is_none());
        assert_eq!(
            false,
            backlogs.insert_open_block(3, open_block.clone()),
            "insert staled open_block should return false",
        );
        assert_eq!(false, backlogs.insert_proof(3, proof.clone()));
    }

    #[test]
    fn test_insert_unequal_result() {
        let open_block = generate_block();
        let mut open_block2 = generate_block();
        open_block2.set_timestamp(2222);
        let proof = generate_bft_proof();
        let executed_result = ExecutedResult::new();
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::new(
            Default::default(),
            GlobalSysConfig::default(),
            false,
            open_block.clone(),
            state_db,
            util::hashable::HASH_NULL_RLP,
            Arc::new(Vec::new()),
        )
        .unwrap();
        let closed_block = exec_block.close(EconomicalModel::Quota);

        let mut backlogs = Backlogs::new(3);
        assert_eq!(true, backlogs.insert_open_block(3, open_block2));
        assert_eq!(true, backlogs.insert_proof(3, proof));
        assert_eq!(
            false,
            backlogs.insert_result(3, closed_block, executed_result),
            "insert unequal closed_block should return false",
        );
    }
}
