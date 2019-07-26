// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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
use cita_types::Address;
use itertools::Itertools;
use libproto::{ExecutedResult, Proof};
use std::cmp::min;
use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone)]
pub enum Priority {
    Proposal = 1,
    Synchronized = 2,
    BlockWithProof = 3,
}

pub struct Backlog {
    open_block: Option<OpenBlock>,
    proof: Option<Proof>,
    closed_block: Option<ClosedBlock>,
    priority: Option<Priority>,
}

impl Default for Backlog {
    fn default() -> Self {
        Backlog {
            open_block: None,
            proof: None,
            closed_block: None,
            priority: None,
        }
    }
}

impl Backlog {
    // When a block is processed done and proofed, we can say this block is completed. So that our
    // chain grows up.
    // So, a Backlog `is_completed` return true if open block, proof, and closed block for that
    // height are all exist and matched.
    pub fn is_completed(&self) -> bool {
        self.all_exist() && self.is_matched()
    }

    fn all_exist(&self) -> bool {
        trace!(
            "open_block: {}, proof: {}, closed_block: {}",
            self.open_block.is_some(),
            self.proof.is_some(),
            self.closed_block.is_some()
        );
        self.open_block.is_some() && self.proof.is_some() && self.closed_block.is_some()
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

    pub fn is_block_ok(&self, current_hash: &cita_types::H256, current_height: u64) -> bool {
        let (parent_hash, height) = {
            let open_block = self.open_block.as_ref().unwrap();
            (open_block.parent_hash(), open_block.number())
        };
        if parent_hash != current_hash {
            trace!(
                "invalid open_block, open_block.parent_hash({:?}) != current_hash({:?})",
                parent_hash,
                current_hash,
            );
        }
        parent_hash == current_hash && height == current_height + 1
    }

    // Mark this Backlog as completed.
    //
    // Make sure the backlog is really completed, otherwise it would panic.
    pub fn complete(self) -> ClosedBlock {
        assert!(self.is_completed());

        let mut closed_block = self.closed_block.unwrap();
        let proof = self.proof.unwrap();

        closed_block.set_proof(proof);
        closed_block.rehash();
        closed_block
    }

    pub fn insert_open(
        &mut self,
        height: u64,
        priority: Priority,
        open_block: OpenBlock,
        proof: Option<Proof>,
    ) -> bool {
        // check priority
        if let Some(ref present_priority) = self.priority {
            if *present_priority as u8 > priority as u8 {
                trace!(
                    "arrived {}-th OpenBlock with priority({:?}) < present.priority({:?})",
                    height,
                    priority,
                    present_priority,
                );
                return false;
            }
        }

        // check proof.height
        if let Some(ref proof) = proof {
            let bft_proof = proof::BftProof::from(proof.clone());
            let proof_height = wrap_height(bft_proof.height);
            if proof_height + 1 != height {
                trace!(
                    "arrived {}-th OpenBlock with invalid proof.height({})",
                    height,
                    proof_height,
                );
                return false;
            }
        }

        self.priority = Some(priority);
        self.open_block = Some(open_block);
        self.proof = proof;
        true
    }

    pub fn get_open_block(&self) -> Option<&OpenBlock> {
        self.open_block.as_ref()
    }

    pub fn get_proof(&self) -> Option<&Proof> {
        self.proof.as_ref()
    }
}

pub struct Backlogs {
    // block height of current local chain, should be equal to `min(backlog.keys())`
    current_height: u64,

    // block hash of current local chain
    current_hash: cita_types::H256,

    // {height => Block}, which indicates pending processing blocks
    backlogs: BTreeMap<u64, Backlog>,

    // {height => ExecutedResult}, which indicates the executed results of elder blocks
    completed: BTreeMap<u64, ExecutedResult>,
}

impl Backlogs {
    pub fn new(current_height: u64, current_hash: cita_types::H256) -> Backlogs {
        Backlogs {
            current_height,
            current_hash,
            backlogs: BTreeMap::new(),
            completed: BTreeMap::new(),
        }
    }

    pub fn get_current_height(&self) -> u64 {
        self.current_height
    }

    pub fn get_current_hash(&self) -> &cita_types::H256 {
        &self.current_hash
    }

    pub fn get_completed_result(&self, height: u64) -> Option<&ExecutedResult> {
        self.completed.get(&height)
    }

    pub fn insert_completed_result(&mut self, height: u64, executed_result: ExecutedResult) {
        self.completed.insert(height, executed_result);
    }

    pub fn insert_proposal(&mut self, open_block: OpenBlock) -> bool {
        let height = wrap_height(open_block.number() as usize);
        self.insert_open(height, Priority::Proposal, open_block, None)
    }

    pub fn insert_synchronized(&mut self, open_block: OpenBlock) -> bool {
        let height = wrap_height(open_block.number() as usize);
        let proof = open_block.proof().clone();
        self.insert_open(height, Priority::Synchronized, open_block, Some(proof))

        // FIXME if block_height == 0, then block is none, proof is some,
        //       AND document it !!!
        // let previous_bft_proof = BftProof::from(previous_proof.clone());
        // let proof_height = wrap_height(previous_bft_proof.height);
        // if proof_height > 0 && block_height == 0 {
        //     info!(
        //         "current_height: {}, receive latest SyncBlock(block.height: {}, proof.height: {})",
        //         self.get_current_height(),
        //         block_height,
        //         proof_height,
        //     );
        //     self.backlogs.insert_proof(proof_height + 1, previous_proof);
        //     continue;
        // }
    }

    pub fn insert_block_with_proof(
        &mut self,
        open_block: OpenBlock,
        present_proof: &Proof,
    ) -> bool {
        let block_height = wrap_height(open_block.number() as usize);
        if block_height != (self.get_current_height() + 1) {
            warn!("executor had received {}=th blockWithProof", block_height);
        }
        let previous_proof = open_block.proof().clone();
        let previous_bft_proof: proof::BftProof = previous_proof.clone().into();
        assert_eq!(
            block_height - 1,
            wrap_height(previous_bft_proof.height),
            "{}-th block's height != {}-th previous_proof.height",
            block_height - 1,
            wrap_height(previous_bft_proof.height)
        );

        let present_bft_proof: proof::BftProof = present_proof.clone().into();
        let present_bft_height = present_bft_proof.height;
        assert_eq!(
            block_height,
            wrap_height(present_bft_height),
            "{}-th block's height != {}-th present_proof.height",
            block_height,
            wrap_height(present_bft_height)
        );

        if !self.is_proof_ok(present_bft_proof.height as u64, &present_proof) {
            warn!(
                "{}-th present bft proof is invalid",
                wrap_height(present_bft_proof.height)
            );
            return false;
        }

        self.insert_open(
            block_height,
            Priority::BlockWithProof,
            open_block,
            Some(previous_proof),
        )
    }

    fn insert_open(
        &mut self,
        height: u64,
        priority: Priority,
        open_block: OpenBlock,
        proof: Option<Proof>,
    ) -> bool {
        // discard staled
        if height <= self.get_current_height() {
            return false;
        }
        self.backlogs
            .entry(height)
            .or_default()
            .insert_open(height, priority, open_block, proof)
    }

    pub fn insert_closed(&mut self, height: u64, closed_block: ClosedBlock) -> bool {
        if self.get_current_height() >= height {
            return false;
        }
        let backlog = self.backlogs.entry(height).or_default();
        backlog.closed_block = Some(closed_block);
        true
    }

    pub fn check_completed(&self, height: u64) -> Result<(), String> {
        assert_eq!(self.get_current_height(), height - 1);

        if self.backlogs.get(&height).is_none() {
            return Err(format!(
                "{}-th is not completed cause backlog.open_block is None",
                height
            ));
        }
        let backlog = &self.backlogs[&height];
        if !backlog.is_completed() {
            return Err(format!(
                "{}-th is not completed cause backlog.is_completed is false",
                height,
            ));
        }
        let proof = backlog.get_proof().unwrap();
        if !self.is_proof_ok(height - 1, proof) {
            return Err(format!(
                "{}-th is not completed cause backlog.proof is invalid",
                height
            ));
        }

        Ok(())
    }

    pub fn ready(&self, height: u64) -> Result<&OpenBlock, String> {
        assert_eq!(self.get_current_height(), height - 1);
        match self.backlogs.get(&height) {
            None => Err(format!("{}-th OpenBlock not found", height)),
            Some(backlog) => {
                let current_hash = self.get_current_hash();
                let current_height = self.get_current_height();
                if !backlog.is_block_ok(current_hash, current_height) {
                    return Err(format!("{}-th OpenBlock is invalid", height));
                }
                if backlog.is_matched() {
                    return Err(format!("{}-th OpenBlock has already been executed", height));
                }
                Ok(backlog.get_open_block().unwrap())
            }
        }
    }

    // validate proof based on executed result of its previous block
    pub fn is_proof_ok(&self, height: u64, proof: &Proof) -> bool {
        // 0-th proof is always valid
        if height == 0 {
            return true;
        }

        let prev_height = height - 1;
        if !self.completed.contains_key(&prev_height) {
            warn!("{}-th ExecutedResult not exist", prev_height);
            return false;
        }

        let executed_result = &self.completed[&prev_height];
        let validators = executed_result.get_config().get_validators();
        let proof_checkers: Vec<Address> = validators
            .iter()
            .map(|vec| Address::from_slice(&vec[..]))
            .collect();

        // FIXME for unit tests only. Should be remove latter
        if proof_checkers.is_empty() {
            return true;
        }
        let bft_proof = proof::BftProof::from(proof.clone());
        // FIXME check proof. Integration tests have bug, uncomment it latter
        if !bft_proof.check(height as usize, &proof_checkers) {
            trace!(
                "bft_proof is invalid, proof_checkers: {:?}, bft_proof: {:?}",
                proof_checkers,
                bft_proof
            );
        }
        true
    }

    pub fn complete(&mut self, height: u64) -> Result<ClosedBlock, String> {
        if let Err(reason) = self.check_completed(height) {
            return Err(reason);
        }

        let backlog = self.backlogs.remove(&height).unwrap();
        let closed_block = backlog.complete();
        self.current_height += 1;
        self.current_hash = closed_block
            .hash()
            .expect("already rehash at backlog.complete below");
        Ok(closed_block)
    }

    pub fn completed_keys(&self) -> ::std::vec::Vec<&u64> {
        self.completed.keys().sorted()
    }

    pub fn prune(&mut self, height: u64) {
        // Importance guard: we must keep the executed result of the recent
        // 3 height(current_height, current_height - 1, current_height - 2),
        // which used when postman check arrived proof via `Postman::check_proof`
        if self.get_current_height() > 2 {
            let split_height = min(height, self.get_current_height() - 2);
            self.completed = self.completed.split_off(&split_height);
        }
    }
}

// System Convention: 0-th block's proof is `::std::usize::MAX`
// Ok, I know it is tricky, but ... just keep it in mind, it is tricky ...
pub fn wrap_height(height: usize) -> u64 {
    match height {
        ::std::usize::MAX => 0,
        _ => height as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::{wrap_height, Backlog, Backlogs, Priority};
    use crate::cita_db::journaldb;
    use crate::cita_db::kvdb::{in_memory, KeyValueDB};
    use crate::core::header::OpenHeader;
    use crate::core::libexecutor::block::{BlockBody, ClosedBlock, ExecutedBlock, OpenBlock};
    use crate::core::libexecutor::sys_config::BlockSysConfig;
    use crate::core::state_db::StateDB;
    use hashable::HASH_NULL_RLP;
    use std::sync::Arc;

    fn generate_block_body() -> BlockBody {
        let mut stx = SignedTransaction::default();
        use crate::types::transaction::SignedTransaction;
        stx.data = vec![1; 200];
        let transactions = vec![stx; 200];
        BlockBody { transactions }
    }

    fn generate_block_header() -> OpenHeader {
        OpenHeader::default()
    }

    fn generate_block(height: u64) -> OpenBlock {
        let block_body = generate_block_body();
        let mut block_header = generate_block_header();
        block_header.set_number(height);
        OpenBlock {
            body: block_body,
            header: block_header,
        }
    }

    fn generate_proof(height: u64) -> libproto::Proof {
        let mut commits = ::std::collections::HashMap::new();
        commits.insert(Default::default(), Default::default());
        let bft_proof = proof::BftProof::new(height as usize, 1, Default::default(), commits);
        bft_proof.into()
    }

    fn generate_executed_result() -> libproto::ExecutedResult {
        libproto::ExecutedResult::new()
    }

    fn generate_state_db() -> StateDB {
        let database = in_memory(7);
        let database: Arc<KeyValueDB> = Arc::new(database);
        let journaldb_type = journaldb::Algorithm::Archive;
        let journal_db = journaldb::new(Arc::clone(&database), journaldb_type, None);
        StateDB::new(journal_db, 5 * 1024 * 1024)
    }

    fn get_open_block(backlogs: &Backlogs, height: u64) -> Option<&OpenBlock> {
        backlogs
            .backlogs
            .get(&height)
            .and_then(|backlog| backlog.get_open_block())
    }

    #[test]
    fn test_backlog_is_completed_with_default() {
        assert_eq!(false, Backlog::default().is_completed());
    }

    fn generate_closed_block(open_block: OpenBlock) -> ClosedBlock {
        let state_db = generate_state_db();
        let exec_block = ExecutedBlock::create(
            Default::default(),
            &BlockSysConfig::default(),
            open_block.clone(),
            state_db,
            HASH_NULL_RLP,
            Arc::new(Vec::new()),
            false,
        )
        .unwrap();
        exec_block.close(&BlockSysConfig::default())
    }

    #[test]
    fn test_backlog_is_completed_with_none() {
        let height = 9;
        {
            let backlog = Backlog {
                priority: Some(Priority::BlockWithProof),
                open_block: None,
                proof: Some(generate_proof(height - 1)),
                closed_block: Some(generate_closed_block(generate_block(height))),
            };
            assert_eq!(false, backlog.is_completed(), "block is none");
        }

        {
            let block = generate_block(height);
            let closed_block = generate_closed_block(block.clone());
            let backlog = Backlog {
                priority: Some(Priority::BlockWithProof),
                open_block: Some(block),
                proof: None,
                closed_block: Some(closed_block),
            };
            assert_eq!(false, backlog.is_completed(), "proof is none");
        }

        {
            let open_block = generate_block(height);
            let backlog = Backlog {
                priority: Some(Priority::BlockWithProof),
                open_block: Some(open_block),
                proof: Some(generate_proof(height - 1)),
                closed_block: None,
            };
            assert_eq!(false, backlog.is_completed(), "closed_block is none");
        }
    }

    #[test]
    fn test_is_completed_with_unequal_block() {
        let height = 9;
        {
            let mut block = generate_block(height);
            let closed_block = generate_closed_block(block.clone());
            block.header.set_timestamp(1);
            let backlog = Backlog {
                priority: Some(Priority::BlockWithProof),
                open_block: Some(block),
                proof: Some(generate_proof(height - 1)),
                closed_block: Some(closed_block),
            };
            assert_eq!(
                false,
                backlog.is_completed(),
                "false cause block.timestamp is not equal"
            );
        }

        {
            let block = generate_block(height);
            let closed_block = generate_closed_block(block.clone());
            let backlog = Backlog {
                priority: Some(Priority::BlockWithProof),
                open_block: Some(block),
                proof: Some(generate_proof(height - 1)),
                closed_block: Some(closed_block),
            };
            assert!(backlog.is_completed());
        }
    }

    #[test]
    #[should_panic]
    fn test_complete_but_is_completed_false() {
        let height = 9;
        let open_block = generate_block(height);
        let closed_block = generate_closed_block(open_block.clone());

        let backlog = Backlog {
            priority: Some(Priority::BlockWithProof),
            open_block: Some(open_block),
            proof: None,
            closed_block: Some(closed_block),
        };
        assert_eq!(false, backlog.is_completed(), "false cause proof is none");

        // panic cause is_completed return false
        backlog.complete();
    }

    #[test]
    fn test_complete_normal() {
        let height = 9;
        let open_block = generate_block(height);
        let closed_block = generate_closed_block(open_block.clone());

        let backlog = Backlog {
            priority: Some(Priority::BlockWithProof),
            open_block: Some(open_block),
            proof: Some(generate_proof(height - 1)),
            closed_block: Some(closed_block),
        };
        assert!(backlog.is_completed());
        backlog.complete();
    }

    #[test]
    fn test_backlogs_whole_flow() {
        let open_block = generate_block(2);
        let closed_block = generate_closed_block(open_block.clone());

        // insert height 2 should be always failed
        let mut backlogs = Backlogs::new(2, Default::default());
        backlogs.insert_completed_result(1, generate_executed_result());
        backlogs.insert_completed_result(2, generate_executed_result());
        assert_eq!(
            false,
            backlogs.insert_open(
                2,
                Priority::Proposal,
                open_block.clone(),
                Some(generate_proof(1)),
            ),
            "insert staled block should return false"
        );
        assert_eq!(
            false,
            backlogs.insert_closed(2, closed_block),
            "insert staled result should return false",
        );
        assert!(get_open_block(&backlogs, 2).is_none());

        // insert height 3 should be ok
        let open_block = generate_block(3);
        let closed_block = generate_closed_block(open_block.clone());
        assert_eq!(
            true,
            backlogs.insert_open(
                3,
                Priority::BlockWithProof,
                open_block.clone(),
                Some(generate_proof(2)),
            ),
        );
        assert_eq!(
            true,
            backlogs.insert_closed(3, closed_block),
            "insert current result should return true",
        );
        assert!(get_open_block(&backlogs, 3).is_some());

        // complete height 3
        assert!(backlogs.check_completed(3).is_ok());
        let _backlog = backlogs.complete(3);
        assert!(backlogs.get_completed_result(2).is_some());

        assert!(get_open_block(&backlogs, 3).is_none());
        assert_eq!(
            false,
            backlogs.insert_open(
                3,
                Priority::Proposal,
                open_block.clone(),
                Some(generate_proof(2)),
            ),
            "insert staled open_block should return false",
        );
    }

    #[test]
    fn test_wrap_height() {
        assert_eq!(0, wrap_height(::std::usize::MAX));
        assert_eq!(
            ::std::usize::MAX as u64 - 1,
            wrap_height(::std::usize::MAX - 1)
        );
        assert_eq!(2, wrap_height(2));
    }
}
