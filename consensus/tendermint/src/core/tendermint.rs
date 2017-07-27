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

use serde_types::hash::{H256, H520};
use serde_types::hash::Address;
use util::hash::H256 as Hash256;
use util::sha3::*;
use core::params::TendermintParams;
use engine::{EngineError, Mismatch, Engine, unix_now, AsMillis};
use core::BareHash;
use core::voteset::*;
use libproto::blockchain::{Block, Transaction, Status, BlockBody};
use libproto::*;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use tx_pool::Pool;
use proof::TendermintProof;
use protobuf::{Message, RepeatedField};
use bincode::{serialize, deserialize, Infinite};
use pubsub::Pub;
use std::time::{Duration, Instant};
use std::sync::mpsc::Sender;
use std::thread;
use crypto::{Signature, sign, recover, pubkey_to_address};
use protobuf::core::parse_from_bytes;
use std::collections::HashMap;
use util::FixedHash;

const INIT_HEIGHT: usize = 1;
const LOOK_AHEAD: usize = 3;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    Commit,
}

impl Default for Step {
    fn default() -> Step {
        Step::Propose
    }
}

impl From<u8> for Step {
    fn from(s: u8) -> Step {
        match s {
            0u8 => Step::Propose,
            1 => Step::Prevote,
            2 => Step::Precommit,
            _ => panic!("Invalid step."),
        }
    }
}

impl BareHash for BlockBody {
    fn bare_hash(&self) -> H256 {
        let binary = self.write_to_bytes().unwrap();
        binary.sha3().into()
    }
}

pub struct Tendermint {
    params: TendermintParams,
    height: AtomicUsize,
    round: AtomicUsize,
    step: RwLock<Step>,
    proof: RwLock<TendermintProof>,
    pre_hash: RwLock<Option<H256>>,
    votes: RwLock<VoteCollector>,
    proposals: RwLock<ProposalCollector>,
    proposal: RwLock<Option<H256>>,
    lock_round: RwLock<Option<usize>>,
    locked_vote: RwLock<Option<VoteSet>>,
    locked_block: RwLock<Option<Block>>,
    tx_pool: Arc<RwLock<Pool>>,
    ready: Mutex<Sender<(usize,H256)>>,
    height_changed : AtomicUsize,
}

impl Tendermint {
    pub fn new(params: TendermintParams, ready: Sender<(usize,H256)>) -> Result<Arc<Self>, EngineError> {
        let mut proof = TendermintProof::default();
        proof.load();
        trace!("load proof {:?}", proof);
        if params.is_test {
            trace!("----Run for test!----");
        }
        let engine = Arc::new(Tendermint {
            tx_pool:
            Arc::new(RwLock::new(Pool::new(params.tx_filter_size,
                                           params.block_tx_limit))),
            params: params,
            height: AtomicUsize::new(0),
            round: AtomicUsize::new(0),
            step: RwLock::new(Step::Propose),
            proof: RwLock::new(proof),
            pre_hash: RwLock::new(None),
            votes: RwLock::new(VoteCollector::new()),
            proposals: RwLock::new(ProposalCollector::new()),
            proposal: RwLock::new(None),
            lock_round: RwLock::new(None),
            locked_vote: RwLock::new(None),
            locked_block: RwLock::new(None),
            ready: Mutex::new(ready),
            height_changed: AtomicUsize::new(0),
        });

        Ok(engine)
    }

    pub fn timeout(&self) -> Duration {
        match *self.step.read() {
            Step::Propose => self.params.timer.propose,
            Step::Prevote => self.params.timer.prevote,
            Step::Precommit => self.params.timer.precommit,
            Step::Commit => self.params.timer.commit,
        }
    }

    fn is_above_threshold(&self, n: usize) -> bool {
        n > self.params.authority_n * 2 / 3
    }

    pub fn new_proposal(&self, _pub: &mut Pub) {
        let height = self.height.load(Ordering::SeqCst);
        let round = self.round.load(Ordering::SeqCst);
        //use lock block at first
        let lock_round = *self.lock_round.read();
        if lock_round.is_some() {
            let ref lock_blk = *self.locked_block.read();
            let ref lock_vote = *self.locked_vote.read();
            let lock_blk = lock_blk.clone().unwrap();
            {
                let lock_blk_hash = H256::from(lock_blk.sha3());
                info!("proposal lock block-----{:?}------{:?}",
                height,
                lock_blk_hash);
                *self.proposal.write() = Some(lock_blk_hash);
            }
            let blk = lock_blk.write_to_bytes().unwrap();
            let proposal = Proposal {
                block: blk,
                lock_round: lock_round,
                lock_votes: lock_vote.clone(),
            };
            trace!("pub proposal");
            self.pub_proposal(&proposal, _pub);
            trace!("proposor vote myself");
            self.proposals.write().add(height, round, proposal);
            return;
        }
        // proposal new blk
        let mut block = Block::new();
        let block_time = unix_now();
        block.mut_header().set_timestamp(block_time.as_millis());
        block.mut_header().set_height(height as u64);
        {
            let pre_hash = &*self.pre_hash.read();
            block
                .mut_header()
                .set_prevhash(pre_hash.unwrap().0.to_vec());
            let proof = self.proof.read().clone();
            if proof.is_default() && height != INIT_HEIGHT {
                warn!("there is no proof");
                return;
            }
            if height != INIT_HEIGHT {
                if proof.height != height - 1 {
                    warn!("proof is old");
                    return;
                }
            }
            block.mut_header().set_proof(proof.into());
        }
        {
            let mut tx_pool = self.tx_pool.write();
            let txs: Vec<Transaction> = tx_pool.package(height as u64);
            let hashvec: Vec<H256> = txs.iter().map(|tx| tx.sha3().into()).collect();
            trace!("new proposal {:?} {:?} {:?}", height, txs.len(), hashvec);
            block
                .mut_body()
                .set_transactions(RepeatedField::from_slice(&txs[..]));
        }
        let bh = block.sha3();
        info!("proposal new block------{:?}-----{:?}", height, bh);
        let locked_hash = Some(bh);
        {
            *self.proposal.write() = locked_hash.map(|x| x.into());
            *self.locked_block.write() = Some(block.clone());
        }
        let blk = block.write_to_bytes().unwrap();
        let proposal = Proposal {
            block: blk,
            lock_round: None,
            lock_votes: None,
        };
        trace!("pub proposal");
        self.pub_proposal(&proposal, _pub);
        trace!("proposor vote myslef");
        self.proposals.write().add(height, round, proposal);
    }

    fn is_round_proposer(&self,
                         height: usize,
                         round: usize,
                         address: &Address)
                         -> Result<(), EngineError> {
        let ref p = self.params;
        let proposer_nonce = height + round;
        let proposer = p.authorities
            .get(proposer_nonce % p.authority_n)
            .expect("There are authority_n authorities; taking number modulo authority_n gives \
                       number in authority_n range; qed");
        if proposer == address {
            Ok(())
        } else {
            Err(EngineError::NotProposer(Mismatch {
                                             expected: proposer.clone().into(),
                                             found: address.clone().into(),
                                         }))
        }
    }

    fn is_authority(&self, address: &Address) -> bool {
        self.params.authorities.contains(address.into())
    }

    fn is_proposer(&self) -> Result<(), EngineError> {
        let ret = self.is_round_proposer(self.height.load(Ordering::SeqCst),
                                         self.round.load(Ordering::SeqCst),
                                         &self.params.signer.address);
        trace!("is_proposer---{:?}", ret);
        ret
    }

    fn pub_message(&self, message: Vec<u8>, _pub: &mut Pub) {
        let msg = factory::create_msg(submodules::CONSENSUS,
                                      topics::CONSENSUS_MSG,
                                      communication::MsgType::MSG,
                                      message);
        _pub.publish("consensus.msg", msg.write_to_bytes().unwrap());
    }

    fn pub_and_broadcast_message(&self, step: Step, hash: Option<H256>, _pub: &mut Pub) {
        let h = self.height.load(Ordering::SeqCst);
        let r = self.round.load(Ordering::SeqCst);
        let ref author = self.params.signer;
        let msg = serialize(&(h, r, step, author.address.clone(), hash.clone()),
                            Infinite)
                .unwrap();
        let signature = sign(&author.privkey(), &msg.sha3().into()).unwrap();
        let sig: H520 = signature.clone().into();
        let msg = serialize(&(msg, sig), Infinite).unwrap();
        self.pub_message(msg, _pub);
        self.votes
            .write()
            .add(h,
                 r,
                 step,
                 author.address.clone(),
                 VoteMessage {
                     proposal: hash.clone(),
                     signature: signature.into(),
                 });
    }

    pub fn pub_block(&self, block: &Block, _pub: &mut Pub) {
        let msg = factory::create_msg(submodules::CONSENSUS,
                                      topics::NEW_BLK,
                                      communication::MsgType::BLOCK,
                                      block.write_to_bytes().unwrap());
        _pub.publish("consensus.blk", msg.write_to_bytes().unwrap());
    }

    pub fn pub_transaction(&self, tx: &Transaction, _pub: &mut Pub, _origin: u32) {
        let mut operate = communication::OperateType::SUBTRACT;
        if _origin == factory::ZERO_ORIGIN{
            operate = communication::OperateType::BROADCAST;
        }
        trace!("communication OperateType is {:?}", operate);
        
        let msg = factory::create_msg_ex(submodules::CONSENSUS,
                                      topics::NEW_TX,
                                      communication::MsgType::TX,
                                      operate,
                                      _origin,
                                      tx.write_to_bytes().unwrap());
        _pub.publish("consensus.tx", msg.write_to_bytes().unwrap());
    }

    pub fn pub_proposal(&self, proposal: &Proposal, _pub: &mut Pub) {
        let height = self.height.load(Ordering::SeqCst);
        let round = self.round.load(Ordering::SeqCst);
        let message = serialize(&(height, round, proposal), Infinite).unwrap();
        let ref author = self.params.signer;
        let signature = sign(&author.privkey(), &message.sha3().into()).unwrap();
        trace!("pub_proposal---{}---{}---{}---{}",
               height,
               round,
               message.sha3(),
               signature);
        let sig: H520 = signature.into();
        let msg = factory::create_msg(submodules::CONSENSUS,
                                      topics::NEW_PROPOSAL,
                                      communication::MsgType::MSG,
                                      serialize(&(message, sig), Infinite).unwrap());
        _pub.publish("consensus.msg", msg.write_to_bytes().unwrap());
    }

    fn proc_proposal(&self, height: usize, round: usize) -> bool {
        let proposal = self.proposals.write().get_proposal(height, round);
        if let Some(proposal) = proposal {
            if !proposal.check(height, &self.params.authorities) {
                //trace!("propsal check failed");
                return false;
            }

            let pre_hash = *self.pre_hash.read();
            if let Some(hash) = pre_hash {
                let block = parse_from_bytes::<Block>(&proposal.block).unwrap();
                let mut block_prehash = Vec::new();
                block_prehash.extend_from_slice(block.get_header().get_prevhash());
                {
                    if hash != Hash256::from(block_prehash.as_slice()).into() {
                        return false;
                    }
                }
                let block_proof = block.get_header().get_proof();
                let proof = TendermintProof::from(block_proof.clone());
                if !proof.check(height - 1, &self.params.authorities) {
                    return false;
                }
            }

            let proposal_lock_round = proposal.lock_round;
            let lock_round = *self.lock_round.read();
            //we have lock block
            if lock_round.is_some() && proposal_lock_round.is_some() {
                if lock_round.unwrap() < proposal_lock_round.unwrap() &&
                   proposal_lock_round.unwrap() < round {
                    //we see new lock block unlock mine
                    info!("unlock lock block------{:?}-----{:?}",
                          height,
                          *self.proposal.read());
                    *self.lock_round.write() = None;
                    *self.locked_vote.write() = None;
                    *self.locked_block.write() = None;
                }
            }
            // still lock on a blk, prevote it
            let mut has_locked_block = false;
            {
                if self.lock_round.read().is_some() {
                    has_locked_block = true;
                }
            }
            if has_locked_block {
                let ref lock_block = self.locked_block.read().clone().unwrap();
                trace!("still have lock block");
                *self.proposal.write() = Some(lock_block.sha3().into());
            } else {
                // else use proposal block
                let block = parse_from_bytes::<Block>(&proposal.block).unwrap();
                let block_hash = block.sha3();
                info!("lock block change------{:?}------{:?}", height, block_hash);
                *self.proposal.write() = Some(block_hash.into());
                *self.locked_block.write() = Some(block);
            }
            return true;
        }
        return false;
    }

    pub fn wait_proposal(&self, _pub: &mut Pub) {
        info!("-----------wait_proposal-----------");
        let height = self.height.load(Ordering::SeqCst);
        let round = self.round.load(Ordering::SeqCst);
        let time_out = self.timeout() * (2u32.pow(round as u32));
        let now = Instant::now();
        while now.elapsed() < time_out {
            if self.proc_proposal(height, round) {
                info!("Got proposal!");
                return;
            }
            thread::sleep(Duration::new(0, 3000000));
        }
        info!("-----------wait proposal timeout!-----------");
        // still lock on a blk, prevote it
        let mut has_locked_block = false;
        {
            if self.lock_round.read().is_some() {
                has_locked_block = true;
            }
        }
        if has_locked_block {
            let ref lock_block = self.locked_block.read().clone().unwrap();
            trace!("still have lock block, vote it {:?}", lock_block.sha3());
            *self.proposal.write() = Some(lock_block.sha3().into());
        } else {
            // else vote none
            *self.proposal.write() = None;
        }
    }

    fn proc_polc(&self, hash: &H256, proposal: &H256) -> bool {
        if hash.0.is_zero() || hash != proposal {
            *self.proposal.write() = None;
            // polc for nil unlock
            *self.lock_round.write() = None;
            *self.locked_block.write() = None;
            *self.locked_vote.write() = None;
            false
        } else {
            // lock it
            *self.proposal.write() = Some(*hash);
            *self.lock_round.write() = Some(self.round.load(Ordering::SeqCst));
            true
            //*self.locked_vote.write() = Some(vote_set);
        }
    }

    fn proc_prevote(&self, height: usize, round: usize, origin_round: usize) -> bool {
        let mut has_vote_set = false;
        {
            let step = *self.step.read();
            let vote_set = self.votes.write().get_voteset(height, round, step);
            if vote_set.is_some() {
                has_vote_set = true;
            }
        }

        if has_vote_set {
            let mut is_lock = false;
            let mut is_ok = false;
            let step = *self.step.read();
            let vote_set = self.votes.write().get_voteset(height, round, step).unwrap();
            if self.is_above_threshold(vote_set.count.clone()) {
                if round > origin_round {
                    // skip round
                    //trace!("skip round {:?}", round);
                    self.round.store(round, Ordering::SeqCst);
                    {
                        *self.proposal.write() = None;
                    }
                }
                for (hash, count) in &vote_set.votes_by_proposal {
                    if self.is_above_threshold(count.clone()) {
                        // should check hash and proposal hash
                        let mut is_proposal_none = false;
                        {
                            if self.proposal.read().is_none() {
                                is_proposal_none = true;
                            }
                        }
                        if is_proposal_none {
                            self.proc_proposal(height, round);
                        }
                        let mut proposal = H256::default();
                        {
                            if let Some(hash) = *self.proposal.read() {
                                proposal = hash;
                            }
                        }
                        is_lock = self.proc_polc(hash, &proposal);
                        is_ok = true;
                        break;
                    }
                }
            }

            if is_lock {
                *self.locked_vote.write() = Some(vote_set);
            }
            if is_ok {
                return true;
            }
        }
        return false;
    }

    pub fn wait_prevote(&self, _pub: &mut Pub) {
        info!("-----------wait_prevote-----------");
        let height = self.height.load(Ordering::SeqCst);
        let origin_round = self.round.load(Ordering::SeqCst);
        let mut time_out = self.timeout() * (2u32.pow(origin_round as u32));
        let now = Instant::now();
        while now.elapsed() < time_out {
            for round in origin_round..origin_round + LOOK_AHEAD {
                if self.proc_prevote(height, round, origin_round) {
                    info!("Got prevote! {:?}", round);
                    return;
                }
            }
            let new_round = self.round.load(Ordering::SeqCst);
            if new_round != origin_round {
                time_out = self.timeout() * (2u32.pow(new_round as u32));
            }
            thread::sleep(Duration::new(0, 3000000));
        }
        info!("-----------wait_prevote timeout!-----------");
        *self.proposal.write() = None;
        // unchange lock for this case
    }

    fn proc_precommit(&self, height: usize, round: usize, origin_round: usize) -> bool {
        let step = *self.step.read();
        let vote_set = self.votes.write().get_voteset(height, round, step);
        if let Some(vote_set) = vote_set {
            if self.is_above_threshold(vote_set.count.clone()) {
                if round > origin_round {
                    // skip round
                    //trace!("skip round {:?}", round);
                    self.round.store(round, Ordering::SeqCst);
                    {
                        *self.proposal.write() = None;
                    }
                }
                let mut is_proposal_none = false;
                {
                    if self.proposal.read().is_none() {
                        is_proposal_none = true;
                    }
                }
                if is_proposal_none {
                    self.proc_proposal(height, round);
                }
                let mut proposal = H256::default();
                {
                    if let Some(hash) = *self.proposal.read() {
                        proposal = hash;
                    }
                }
                for (hash, count) in &vote_set.votes_by_proposal {
                    if self.is_above_threshold(count.clone()) {
                        if hash.0.is_zero() || hash != &proposal {
                            *self.proposal.write() = None;
                        } else {
                            *self.proposal.write() = Some(*hash);
                        }
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn wait_precommit(&self, _pub: &mut Pub) {
        info!("-----------wait_precommit-----------");
        let height = self.height.load(Ordering::SeqCst);
        let origin_round = self.round.load(Ordering::SeqCst);
        let mut time_out = self.timeout() * (2u32.pow(origin_round as u32));
        let now = Instant::now();
        while now.elapsed() < time_out {
            for round in origin_round..origin_round + LOOK_AHEAD {
                if self.proc_precommit(height, round, origin_round) {
                    return;
                }
            }
            let new_round = self.round.load(Ordering::SeqCst);
            if new_round != origin_round {
                time_out = self.timeout() * (2u32.pow(new_round as u32));
            }
            thread::sleep(Duration::new(0, 3000000));
        }
        info!("-----------wait precommit timeout!-----------");
        *self.proposal.write() = None;
    }

    fn commit_block(&self, _pub: &mut Pub) -> bool {
        // Commit the block using a complete signature set.
        let round = self.round.load(Ordering::SeqCst);
        let height = self.height.load(Ordering::SeqCst);
        if self.params.is_test && round == 0 {
            if self.is_proposer().is_ok() {
                thread::sleep(Duration::new(3, 0));
            } else {
                if let Some(ref mut locked_block) = *self.locked_block.write() {
                    let ts = locked_block.get_header().get_timestamp();
                    locked_block.mut_header().set_timestamp(ts + 1);
                }
                return false;
            }
        }
        if let Some(hash) = *self.proposal.read() {
            if let Some(ref mut locked_block) = *self.locked_block.write() {
                //generate proof
                let mut commits = HashMap::new();
                {
                    let vote_set = self.votes
                        .write()
                        .get_voteset(height, round, Step::Precommit);
                    if let Some(vote_set) = vote_set {
                        for (sender, vote) in &vote_set.votes_by_sender {
                            if vote.proposal.is_none() {
                                continue;
                            }
                            if vote.proposal.unwrap() == hash {
                                commits.insert(*sender, vote.signature);
                            }
                        }
                    }
                }
                let mut proof = TendermintProof::default();
                proof.height = height;
                proof.round = round;
                proof.proposal = hash;
                proof.commits = commits;
                {
                    *self.proof.write() = proof.clone();
                }
                locked_block.mut_header().set_proof1(proof.into());
                self.pub_block(&locked_block, _pub);
                {
                    //update tx pool
                    let txs = locked_block.get_body().get_transactions();
                    self.tx_pool.write().update(txs);
                }

                return true;
            }
        }
        //goto next round
        return false;
    }

    fn increment_round(&self, n: usize) {
        self.round.fetch_add(n, Ordering::SeqCst);
    }
}

impl Engine for Tendermint {
    fn name(&self) -> &str {
        "Tendermint"
    }

    fn duration(&self) -> Duration {
        self.params.duration.clone()
    }

    fn verify_block(&self, _: &Block) -> Result<(), EngineError> {
        unimplemented!();
    }

    fn handle_message(&self, message: Vec<u8>, _pub: &mut Pub) -> Result<(), EngineError> {
        let height = self.height.load(Ordering::SeqCst);
        let round = self.round.load(Ordering::SeqCst);
        let decoded = deserialize(&message[..]).unwrap();
        let (message, signature) = decoded;
        let message: Vec<u8> = message;
        let signature: H520 = signature;
        let signature = Signature::from(signature);
        if let Ok(pubkey) = recover(&signature, &message.sha3().into()) {
            let decoded = deserialize(&message[..]).unwrap();
            let (h, r, step, sender, hash) = decoded;
            if ((h == height && r >= round) || h > height) && self.is_authority(&sender) &&
               pubkey_to_address(&pubkey) == sender {
                info!("get vote---{:?}---{:?}---{:?}---{:?}---{:?}---{}",
                      h,
                      r,
                      step,
                      sender,
                      hash,
                      signature);
                let ret = self.votes
                    .write()
                    .add(h,
                         r,
                         step,
                         sender,
                         VoteMessage {
                             proposal: hash,
                             signature: signature.into(),
                         });
                if ret {
                    info!("vote ok!");
                    return Ok(());
                }
                return Err(EngineError::DoubleVote(sender.into()));
            } else if h < height || (h == height && r < round) {
                info!("get delay vote info ---{:?}---{:?}---{:?}---{:?}---{:?}---{}",
                      h,
                      r,
                      step,
                      sender,
                      hash,
                      signature);
                info!("The current height is：{:?} and round is : {:?}", height as usize, round as usize);
                return Err(EngineError::VoteMsgDelay(height as usize))
            }
        }
        Err(EngineError::BadSignature(signature))
    }

    fn handle_proposal(&self, msg: Vec<u8>, _pub: &mut Pub) -> Result<(), EngineError> {
        let decoded = deserialize(&msg[..]).unwrap();
        let (message, signature) = decoded;
        let message: Vec<u8> = message;
        let signature: H520 = signature;
        let signature = Signature::from(signature);
        trace!("handle proposal message {:?} signature {}",
               message.sha3(),
               signature);
        if let Ok(pubkey) = recover(&signature, &message.sha3().into()) {
            let decoded = deserialize(&message[..]).unwrap();
            let (height, round, proposal) = decoded;
            let proposal: Proposal = proposal;
            trace!("handle_proposal height {:?}, round {:?} sender {:?}",
                   height,
                   round,
                   pubkey_to_address(&pubkey));
            let ret = self.is_round_proposer(height, round, &pubkey_to_address(&pubkey));
            if ret.is_err() {
                return ret;
            }

            if (height == self.height.load(Ordering::SeqCst) &&
                round >= self.round.load(Ordering::SeqCst)) ||
               height > self.height.load(Ordering::SeqCst) {
                info!("add proposal height {} round {}!", height, round);
                self.proposals.write().add(height, round, proposal);
                return Ok(());
            }
        }
        return Err(EngineError::BadSignature(signature));
    }

    fn receive_new_transaction(&self, tx: &Transaction, _pub: &mut Pub, _origin: u32, from_broadcast: bool) {
        let mut content = blockchain::TxResponse::new();
        let hash: Hash256 = tx.sha3();
        {
            let mut tx_pool = self.tx_pool.write();
            content.set_hash(hash.to_vec());
            let success = tx_pool.enqueue(tx.clone(), hash);
            if success {
                info!("receive_new_transaction {:?}", hash);
                content.set_result(String::from("4:OK").into_bytes());
                self.pub_transaction(tx, _pub, _origin);
            } else {
                content.set_result(String::from("4:DUP").into_bytes());
            }
            if !from_broadcast {
                let msg = factory::create_msg(submodules::CONSENSUS,
                                              topics::TX_RESPONSE,
                                              communication::MsgType::TX_RESPONSE,
                                              content.write_to_bytes().unwrap());
                trace!("response new tx {:?}", tx);
                _pub.publish("consensus.rpc", msg.write_to_bytes().unwrap());
            }
        }
    }

    fn receive_new_status(&self, status: Status) {
        let new_height = (status.height + 1) as usize;
        let height = self.height.load(Ordering::SeqCst);
        let pre_hash = H256::from(Hash256::from_slice(&status.hash));
        if new_height > height {
            trace!("new_status status {:?} new height {:?}", status, new_height);
            let _ = self.ready.lock().unwrap().send((new_height,pre_hash));
            self.height_changed.store(1, Ordering::SeqCst);
        }
    }

    fn receive_new_block(&self, _: &Block, _pub: &mut Pub) {
        unimplemented!();
    }

    fn set_new_status(&self, height: usize, pre_hash: H256) {
        self.height_changed.store(0, Ordering::SeqCst);
         self.height.store(height, Ordering::SeqCst);
        {
            *self.pre_hash.write() = Some(pre_hash);
        }
    }

    // call when time to seal block
    fn new_block(&self, _pub: &mut Pub) {
        let origin_height = self.height.load(Ordering::SeqCst);
        info!("-----------new block begin, origin_height {:?}-----------",
              origin_height);
        self.round.store(0, Ordering::SeqCst);
        {
            *self.lock_round.write() = None;
            *self.locked_vote.write() = None;
            *self.locked_block.write() = None;
        }
        loop {
            info!("-----------new round {:?}-----------",
                  self.round.load(Ordering::SeqCst));
            {
                *self.step.write() = Step::Propose;
            }
            {
                *self.proposal.write() = None;
            }
            // Only proposer can generate seal if None was generated.
            if self.is_proposer().is_ok() {
                info!("I'm proposor!");
                self.new_proposal(_pub);
            } else {
                info!("I'm not proposor!");
                self.wait_proposal(_pub);
            }

            info!("-----------new_proposal round {} {:?}-----------",
                  self.round.load(Ordering::SeqCst),
                  {
                      *self.proposal.read()
                  });

            {
                *self.step.write() = Step::Prevote;
            }
            let hash = self.proposal.read().clone();
            info!("-----------to_step Step::Prevote-----------");
            if hash.is_none() {
                info!("-----------Prevote empty-----------");
            }
            self.pub_and_broadcast_message(Step::Prevote, hash, _pub);

            self.wait_prevote(_pub);

            {
                *self.step.write() = Step::Precommit;
            }
            let hash = self.proposal.read().clone();
            info!("-----------to_step Step::Precommit-----------");
            if hash.is_none() {
                info!("-----------Precommit empty!-----------");
            }

            self.pub_and_broadcast_message(Step::Precommit, hash, _pub);

            self.wait_precommit(_pub);

            info!("-----------to_step Step::Commit-----------");
            if self.commit_block(_pub) {
                //next height
                return;
            }
            self.increment_round(1);
            info!("---Enter new round:{:?} for the the height:{:?}",
                self.round.load(Ordering::SeqCst),
                origin_height);

            //check height changed
            if self.height_changed.load(Ordering::SeqCst) > 0 {
                info!("-----------height changed-----------");
                return;
            }
        }
    }
}
