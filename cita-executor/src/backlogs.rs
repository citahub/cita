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

use super::core::libexecutor::block::{ClosedBlock, OpenBlock};
use libproto::ExecutedResult;
use proof::BftProof as Proof;
use std::collections::BTreeMap;
use util::Itertools;

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
    // When a block is processed done and proofed, we can say this block is completed. So that our
    // chain grows up.
    // So, a Backlog `is_completed` return true if block, proof, and executed_result for that height
    // are all exist and matched.
    pub fn is_completed(&self) -> bool {
        if self.open_block.is_none()
            || self.proof.is_none()
            || self.executed_result.is_none()
            || self.closed_block.is_none()
        {
            return false;
        }

        self.is_matched()
    }

    // `is_matched` check whether the inside closed_block is equal to open_block
    pub fn is_matched(&self) -> bool {
        if self.open_block.is_none() || self.closed_block.is_none() {
            return false;
        }

        let open_block = self.open_block.as_ref().unwrap();
        let closed_block = self.closed_block.as_ref().unwrap();
        closed_block.is_equivalent(open_block)
    }

    // Mark this Backlog as completed.
    //
    // Make sure the backlog is really completed, otherwise it would panic.
    pub fn complete(self) -> (ClosedBlock, ExecutedResult) {
        assert!(self.is_completed());

        let bft_proof = self.proof.unwrap();
        let mut closed_block = self.closed_block.unwrap();
        let executed_result = self.executed_result.unwrap();
        closed_block.header.set_proof(bft_proof.into());
        closed_block.rehash();

        (closed_block, executed_result)
    }

    pub fn get_open_block(&self) -> Option<&OpenBlock> {
        self.open_block.as_ref()
    }
}

pub struct Backlogs {
    // current height of local chain, should be equal to `min(backlog.keys())`
    current_height: u64,

    // {height => Block}, which indicates pending processing blocks
    backlogs: BTreeMap<u64, Backlog>,

    // {height => ExecutedResult}, which indicates the executed results of elder blocks
    completed: BTreeMap<u64, ExecutedResult>,
}

impl Backlogs {
    pub fn new(current_height: u64) -> Backlogs {
        Backlogs {
            current_height,
            backlogs: BTreeMap::new(),
            completed: BTreeMap::new(),
        }
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_height
    }

    pub fn get_open_block(&self, height: u64) -> Option<&OpenBlock> {
        match self.backlogs.get(&height) {
            Some(ref backlog) => backlog.get_open_block(),
            None => None,
        }
    }

    pub fn get_completed_result(&self, height: u64) -> Option<&ExecutedResult> {
        self.completed.get(&height)
    }

    pub fn insert_completed_result(&mut self, height: u64, executed_result: ExecutedResult) {
        self.completed.insert(height, executed_result);
    }

    pub fn insert_open_block(&mut self, height: u64, open_block: OpenBlock) -> bool {
        if !self.assert_height(height) {
            return false;
        }

        let backlog = self.backlogs.entry(height).or_default();
        backlog.open_block = Some(open_block);
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
        backlog.closed_block = Some(closed_block);
        backlog.executed_result = Some(executed_result);
        true
    }

    pub fn is_completed(&self, height: u64) -> bool {
        match self.backlogs.get(&height) {
            Some(backlog) => backlog.is_completed(),
            None => false,
        }
    }

    pub fn is_matched(&self, height: u64) -> bool {
        match self.backlogs.get(&height) {
            Some(backlog) => backlog.is_matched(),
            None => false,
        }
    }

    // move "front of backlogs" into "back of completed"
    pub fn complete(&mut self, height: u64) -> ClosedBlock {
        let backlog = self.backlogs.remove(&height).unwrap();
        let (closed_block, executed_result) = backlog.complete();

        self.completed.insert(height, executed_result);
        self.current_height += 1;
        closed_block
    }

    pub fn completed_keys(&self) -> ::std::vec::Vec<&u64> {
        self.completed.keys().sorted()
    }

    pub fn prune(&mut self, height: u64) {
        // Importance guard: we must keep the executed result of the previous
        // height(current_height - 1), which used when postman check arrived
        // proof via `Postman::check_proof`
        if height < self.get_current_height() - 1 && self.get_current_height() != 0 {
            self.completed = self.completed.split_off(&height);
        }
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
    use core::libexecutor::block::{BlockBody, ClosedBlock, ExecutedBlock, OpenBlock};
    use core::libexecutor::economical_model::EconomicalModel;
    use core::libexecutor::executor::GlobalSysConfig;
    use core::state_db::StateDB;
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

    fn generate_executed_result() -> ExecutedResult {
        ExecutedResult::new()
    }

    fn generate_closed_block(open_block: OpenBlock) -> ClosedBlock {
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
        exec_block.close(EconomicalModel::Quota)
    }

    #[test]
    fn test_backlog_is_completed_with_none() {
        {
            let backlog = Backlog {
                open_block: None,
                proof: Some(generate_bft_proof()),
                closed_block: Some(generate_closed_block(generate_block())),
                executed_result: Some(generate_executed_result()),
            };
            assert_eq!(false, backlog.is_completed(), "block is none");
        }

        {
            let block = generate_block();
            let closed_block = generate_closed_block(block.clone());
            let backlog = Backlog {
                open_block: Some(block),
                proof: None,
                closed_block: Some(closed_block),
                executed_result: Some(generate_executed_result()),
            };
            assert_eq!(false, backlog.is_completed(), "proof is none");
        }

        {
            let backlog = Backlog {
                open_block: Some(generate_block()),
                proof: Some(generate_bft_proof()),
                closed_block: None,
                executed_result: Some(generate_executed_result()),
            };
            assert_eq!(false, backlog.is_completed(), "closed_block is none");
        }
    }

    #[test]
    fn test_is_completed_with_unequal_block() {
        {
            let mut block = generate_block();
            let closed_block = generate_closed_block(block.clone());
            block.header.set_timestamp(1);
            let backlog = Backlog {
                open_block: Some(block),
                proof: Some(generate_bft_proof()),
                closed_block: Some(closed_block),
                executed_result: Some(generate_executed_result()),
            };
            assert_eq!(
                false,
                backlog.is_completed(),
                "false cause block.timestamp is not equal"
            );
        }

        {
            let block = generate_block();
            let closed_block = generate_closed_block(block.clone());
            let backlog = Backlog {
                open_block: Some(block),
                proof: Some(generate_bft_proof()),
                closed_block: Some(closed_block),
                executed_result: Some(generate_executed_result()),
            };
            assert!(backlog.is_completed());
        }
    }

    #[test]
    #[should_panic]
    fn test_complete_but_is_completed_false() {
        let open_block = generate_block();
        let closed_block = generate_closed_block(open_block.clone());

        let backlog = Backlog {
            open_block: Some(open_block),
            proof: None,
            closed_block: Some(closed_block),
            executed_result: Some(generate_executed_result()),
        };
        assert_eq!(false, backlog.is_completed(), "false cause proof is none");

        // panic cause is_completed return false
        backlog.complete();
    }

    #[test]
    fn test_complete_normal() {
        let open_block = generate_block();
        let closed_block = generate_closed_block(open_block.clone());

        let backlog = Backlog {
            open_block: Some(open_block),
            proof: Some(generate_bft_proof()),
            closed_block: Some(closed_block),
            executed_result: Some(generate_executed_result()),
        };
        assert!(backlog.is_completed());
        backlog.complete();
        //        let closed_block = backlog.clone_closed_block();
        //        let proof = backlog.clone_proof();
        //        let proto_proof: Proof = proof.into();
        //        assert_eq!(proto_proof.content, closed_block.proof().content);
    }

    #[test]
    fn test_backlogs_whole_flow() {
        let open_block = generate_block();
        let closed_block = generate_closed_block(open_block.clone());

        // insert height 2 should be always failed
        let mut backlogs = Backlogs::new(3);
        assert_eq!(false, backlogs.insert_open_block(2, open_block.clone()));
        assert_eq!(false, backlogs.insert_proof(2, generate_bft_proof()));
        assert_eq!(
            false,
            backlogs.insert_result(2, closed_block, generate_executed_result()),
            "insert staled result should return false",
        );
        assert!(backlogs.get_open_block(2).is_none());

        // insert height 3 should be ok
        let closed_block = generate_closed_block(open_block.clone());
        assert_eq!(true, backlogs.insert_open_block(3, open_block.clone()));
        assert_eq!(true, backlogs.insert_proof(3, generate_bft_proof()));
        assert_eq!(
            true,
            backlogs.insert_result(3, closed_block, generate_executed_result()),
            "insert current result should return true",
        );
        assert!(backlogs.get_open_block(3).is_some());

        // complete height 3
        assert!(backlogs.is_completed(3));
        let _backlog = backlogs.complete(3);
        assert!(backlogs.get_completed_result(2).is_none());
        assert!(backlogs.get_completed_result(3).is_some());

        assert!(backlogs.get_open_block(3).is_none());
        assert_eq!(
            false,
            backlogs.insert_open_block(3, open_block.clone()),
            "insert staled open_block should return false",
        );
        assert_eq!(false, backlogs.insert_proof(3, generate_bft_proof()));
    }
}
