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

use bincode::{serialize, deserialize, Infinite};
use core::dispatchtx::Dispatchtx;
use core::params::TendermintParams;
use core::voteset::{VoteCollector, ProposalCollector, VoteSet, Proposal, VoteMessage};

use core::votetime::{WaitTimer, TimeoutInfo};
use core::wal::Wal;

use crypto::{CreateKey, Signature, Sign, pubkey_to_address};
use engine::{EngineError, Mismatch, unix_now, AsMillis};
use libproto;
use libproto::{communication, submodules, topics, MsgClass};
use libproto::blockchain::{Block, SignedTransaction, Status};

//use tx_pool::Pool;
use proof::TendermintProof;
use protobuf::{Message, RepeatedField};
use protobuf::core::parse_from_bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver, RecvError};
use std::time::Instant;
use util::{H256, Address, Hashable};

const INIT_HEIGHT: usize = 1;
const INIT_ROUND: usize = 0;

const LOG_TYPE_PROPOSE: u8 = 1;
const LOG_TYPE_VOTE: u8 = 2;
const LOG_TYPE_STATE: u8 = 3;
const LOG_TYPE_PREV_HASH: u8 = 4;
const LOG_TYPE_COMMITS: u8 = 5;

const ID_CONSENSUS_MSG: u32 = (submodules::CONSENSUS << 16) + topics::CONSENSUS_MSG as u32;
const ID_NEW_PROPOSAL: u32 = (submodules::CONSENSUS << 16) + topics::NEW_PROPOSAL as u32;
//const ID_NEW_STATUS: u32 = (submodules::CHAIN << 16) + topics::NEW_STATUS as u32;

const TIMEOUT_RETRANSE_MULTIPLE: u32 = 5;
const TIMEOUT_LOW_ROUND_MESSAGE_MULTIPLE: u32 = 10;
const DATA_PATH: &'static str = "DATA_PATH";

pub type TransType = (u32, u32, MsgClass);
pub type PubType = (String, Vec<u8>);

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    Prevote,
    Precommit,
    ProposeWait,
    PrevoteWait,
    PrecommitWait,
    Commit,
    CommitWait,
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
            1u8 => Step::ProposeWait,
            2u8 => Step::Prevote,
            3u8 => Step::PrevoteWait,
            4u8 => Step::Precommit,
            5u8 => Step::PrecommitWait,
            6u8 => Step::Commit,
            7u8 => Step::CommitWait,
            _ => panic!("Invalid step."),
        }
    }
}

pub struct TenderMint {
    pub_sender: Sender<PubType>,
    pub_recver: Receiver<TransType>,

    timer_seter: Sender<TimeoutInfo>,
    timer_notity: Receiver<TimeoutInfo>,

    params: TendermintParams,
    height: usize,
    round: usize,
    step: Step,
    proof: TendermintProof,
    pre_hash: Option<H256>,
    votes: VoteCollector,
    proposals: ProposalCollector,
    proposal: Option<H256>,
    lock_round: Option<usize>,
    locked_vote: Option<VoteSet>,
    // lock_round set, locked block means itself,else means proposal's block
    locked_block: Option<Block>,
    //tx_pool: Pool,
    wal_log: Wal,
    send_filter: HashMap<Address, (usize, Step, Instant)>,
    last_commit_round: Option<usize>,
    // to be used for chain syncing
    //sync_ok :bool,
    dispatch: Arc<Dispatchtx>,

    htime: Instant,
}

impl TenderMint {
    pub fn new(s: Sender<PubType>, r: Receiver<TransType>, ts: Sender<TimeoutInfo>, rs: Receiver<TimeoutInfo>, params: TendermintParams, dispatch: Arc<Dispatchtx>) -> TenderMint {
        let proof = TendermintProof::default();
        if params.is_test {
            trace!("Run for test!");
        }
        let logpath = ::std::env::var(DATA_PATH).expect(format!("{} must be set", DATA_PATH).as_str()) + "/wal";

        trace!("tx pool size {}", params.tx_pool_size);
        TenderMint {
            pub_sender: s,
            pub_recver: r,
            timer_seter: ts,
            timer_notity: rs,

            //tx_pool:Pool::new(params.tx_filter_size,params.block_tx_limit),
            params: params,
            height: 0,
            round: INIT_ROUND,
            step: Step::Propose,
            proof: proof,
            pre_hash: None,
            votes: VoteCollector::new(),
            proposals: ProposalCollector::new(),
            proposal: None,
            lock_round: None,
            locked_vote: None,
            locked_block: None,
            wal_log: Wal::new(&*logpath).unwrap(),
            send_filter: HashMap::new(),
            last_commit_round: None,
            //To be used later
            //sync_ok : true,
            dispatch: dispatch,
            htime: Instant::now(),
        }
    }

    fn is_round_proposer(&self, height: usize, round: usize, address: &Address) -> Result<(), EngineError> {
        let ref p = self.params;
        let proposer_nonce = height + round;
        let proposer = p.authorities
                        .get(proposer_nonce % p.authority_n)
                        .expect("There are authority_n authorities; taking number modulo authority_n gives number in authority_n range; qed");
        if proposer == address {
            Ok(())
        } else {
            Err(EngineError::NotProposer(Mismatch {
                                             expected: proposer.clone().into(),
                                             found: address.clone().into(),
                                         }))
        }
    }

    /*    pub fn pub_transaction(&mut self, tx: &SignedTransaction) {
        let mut msg = communication::Message::new();
        msg.set_cmd_id(libproto::cmd_id(submodules::CONSENSUS, topics::NEW_TX));
        msg.set_field_type(communication::MsgType::TX);
        msg.set_content(tx.write_to_bytes().unwrap());
        MQWork::send2pub(&self.pub_sender ,("consensus.tx".to_string(), msg.write_to_bytes().unwrap()));
    }
*/
    pub fn pub_block(&self, block: &Block) {
        let mut msg = communication::Message::new();
        msg.set_cmd_id(libproto::cmd_id(submodules::CONSENSUS, topics::NEW_BLK));
        msg.set_field_type(communication::MsgType::BLOCK);
        msg.set_content(block.write_to_bytes().unwrap());
        self.pub_sender.send(("consensus.blk".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }

    pub fn pub_proposal(&self, proposal: &Proposal) -> Vec<u8> {
        let mut msg = communication::Message::new();
        msg.set_cmd_id(libproto::cmd_id(submodules::CONSENSUS, topics::NEW_PROPOSAL));
        msg.set_field_type(communication::MsgType::MSG);

        let message = serialize(&(self.height, self.round, proposal), Infinite).unwrap();
        let ref author = self.params.signer;
        let signature = Signature::sign(&author.keypair.privkey(), &message.crypt_hash().into()).unwrap();
        trace!("pub_proposal height {}, round {}, hash {}, signature {} ", self.height, self.round, message.crypt_hash(), signature);
        let bmsg = serialize(&(message, signature), Infinite).unwrap();
        msg.set_content(bmsg.clone());
        self.pub_sender.send(("consensus.msg".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        bmsg
    }

    fn pre_proc_prevote(&mut self) {
        let prop = self.proposal.clone();
        let height = self.height;
        let round = self.round;

        if prop.is_none() {
            self.proc_proposal(height, round);
        }
        info!("pre_proc_prevote height {},round {} hash {:?} locked_round {:?}", height, round, prop, self.lock_round);
        if self.lock_round.is_some() {
            //let hash = H256::from(self.locked_block.clone().unwrap().crypt_hash());
            self.pub_and_broadcast_message(height, round, Step::Prevote, prop);
        } else {
            if prop.is_some() {
                self.pub_and_broadcast_message(height, round, Step::Prevote, prop);
            } else {
                info!("pre_proc_prevote not have any thing in {} {}", height, round);
                self.pub_and_broadcast_message(height, round, Step::Prevote, Some(H256::default()));
            }
        }
        WaitTimer::set_timer(self.timer_seter.clone(),
                             TimeoutInfo {
                                 timeval: self.params.timer.prevote * TIMEOUT_RETRANSE_MULTIPLE,
                                 height: height,
                                 round: round,
                                 step: Step::Prevote,
                             });
    }

    fn proc_prevote(&mut self, height: usize, round: usize) -> bool {
        info!("proc_prevote begin height {}, round {} vs self {}, round {}", height, round, self.height, self.round);
        if height < self.height || (height == self.height && round < self.round) || (height == self.height && self.round == round && self.step > Step::PrevoteWait) {
            return false;
        }

        let vote_set = self.votes.get_voteset(height, round, Step::Prevote);
        trace!("proc_prevote vote_set {:?}", vote_set);
        if let Some(vote_set) = vote_set {
            if self.is_above_threshold(&vote_set.count) {
                let mut tv = self.params.timer.prevote;
                if self.is_all_vote(&vote_set.count) {
                    tv = ::std::time::Duration::new(0, 0);
                }

                for (hash, count) in &vote_set.votes_by_proposal {
                    if self.is_above_threshold(count) {
                        //we have lock block,and now polc  then unlock
                        if self.lock_round.is_some() {
                            if self.lock_round.unwrap() < round && round <= self.round {
                                //we see new lock block unlock mine
                                info!("unlock lock block height {:?}, hash {:?}", height, hash);
                                self.lock_round = None;
                                self.locked_vote = None;
                            }
                        }
                        tv = ::std::time::Duration::new(0, 0);
                        if hash.is_zero() {
                            self.clean_saved_info();
                        } else {
                            if self.proposal == Some(*hash) {
                                self.lock_round = Some(round);
                                self.locked_vote = Some(vote_set.clone());
                            } else {
                                let mut clean_flag = true;
                                let op = self.proposals.get_proposal(height, round);
                                if op.is_some() {
                                    let pro_block = parse_from_bytes::<Block>(&op.unwrap().block);
                                    if let Ok(block) = pro_block {
                                        let bhash: H256 = block.crypt_hash().into();
                                        if bhash == *hash {
                                            self.locked_block = Some(block);
                                            self.proposal = Some(*hash);
                                            self.locked_vote = Some(vote_set.clone());
                                            self.lock_round = Some(round);
                                            clean_flag = false;
                                        }
                                    }
                                }
                                if clean_flag {
                                    self.clean_saved_info();
                                }
                            }
                        }
                        //more than one hash have threahold is wrong !! do some check ??
                        break;
                    }
                }

                if self.step == Step::Prevote {
                    self.change_state_step(height, round, Step::PrevoteWait, false);

                    WaitTimer::set_timer(self.timer_seter.clone(),
                                         TimeoutInfo {
                                             timeval: tv,
                                             height: height,
                                             round: round,
                                             step: Step::PrevoteWait,
                                         });
                }
                return true;
            }
        }
        return false;
    }

    fn is_above_threshold(&self, n: &usize) -> bool {
        *n > self.params.authority_n * 2 / 3
    }

    fn is_all_vote(&self, n: &usize) -> bool {
        *n == self.params.authority_n
    }

    fn pre_proc_precommit(&mut self) {
        let height = self.height;
        let round = self.round;
        let proposal = self.proposal.clone();

        if let Some(lround) = self.lock_round {
            trace!("pre_proc_precommit locked round,{},self round {}", lround, round);
            if lround == round {
                self.pub_and_broadcast_message(height, round, Step::Precommit, proposal);
            } else {
                self.pub_and_broadcast_message(height, round, Step::Precommit, Some(H256::default()));
            }
        } else {
            trace!("pre_proc_precommit not locked ");
            self.pub_and_broadcast_message(height, round, Step::Precommit, Some(H256::default()));
        }

        //resend msg
        WaitTimer::set_timer(self.timer_seter.clone(),
                             TimeoutInfo {
                                 timeval: self.params.timer.precommit * TIMEOUT_RETRANSE_MULTIPLE,
                                 height: self.height,
                                 round: self.round,
                                 step: Step::Precommit,
                             });
    }

    fn retrans_vote(&mut self, height: usize, round: usize, step: Step) {
        self.pub_and_broadcast_message(height, round, step, Some(H256::default()));
    }

    fn proc_precommit(&mut self, height: usize, round: usize) -> bool {
        info!("proc_precommit begin {} {} vs self {} {}", height, round, self.height, self.round);
        if height < self.height || (height == self.height && round < self.round) || (height == self.height && self.round == round && self.step > Step::PrecommitWait) {
            return false;
        }

        let vote_set = self.votes.get_voteset(height, round, Step::Precommit);
        trace!("proc_precommit deal height {} round {} voteset {:?}", height, round, vote_set);
        if let Some(vote_set) = vote_set {
            if self.is_above_threshold(&vote_set.count) {
                trace!("proc_precommit is_above_threshold height {} round {}", height, round);

                let mut tv = self.params.timer.precommit;
                if self.is_all_vote(&vote_set.count) {
                    tv = ::std::time::Duration::new(0, 0);
                }
                for (hash, count) in vote_set.votes_by_proposal {
                    if self.is_above_threshold(&count) {
                        trace!("proc_precommit is_above_threshold hash {:?} {}", hash, count);
                        tv = ::std::time::Duration::new(0, 0);
                        if hash.is_zero() {
                            trace!("proc_precommit is zero");
                        //self.proposal = None;
                        } else {
                            if self.proposal.is_some() {
                                if hash != self.proposal.unwrap() {
                                    info!("proc_precommit why this hanppen self.proposql {:?} hash {:?}", self.proposal.unwrap(), hash);
                                    self.clean_saved_info();
                                    return false;
                                } else {
                                    self.proposal = Some(hash);
                                    self.last_commit_round = Some(round);
                                }
                            } else {
                                trace!("proc_precommit hash is ok,but self.propose is noe");
                                return false;
                            }
                        }
                        break;
                    }
                }

                if self.step == Step::Precommit {
                    self.change_state_step(height, round, Step::PrecommitWait, false);
                    WaitTimer::set_timer(self.timer_seter.clone(),
                                         TimeoutInfo {
                                             timeval: tv,
                                             height: height,
                                             round: round,
                                             step: Step::PrecommitWait,
                                         });
                }
                return true;
            }
        }
        return false;
    }

    fn pre_proc_commit(&mut self, h: usize, r: usize) -> bool {
        trace!("** pre_proc_commit now beginging {} {} self {} {} last_commit_round {:?} ", h, r, self.height, self.round, self.last_commit_round);
        if self.height == h && self.round == r {
            if let Some(cround) = self.last_commit_round {
                if cround == r && self.proposal.is_some() {
                    return self.commit_block();
                }
            }
        }
        trace!("** pre_proc_commit now false");
        false
    }

    fn save_wal_proof(&mut self) {
        //let msg = self.proof.clone();
        let bmsg = serialize(&self.proof, Infinite).unwrap();
        let _ = self.wal_log.save(LOG_TYPE_COMMITS, &bmsg);
    }

    fn proc_commit_after(&mut self, height: usize, round: usize) -> bool {
        let nowheight = self.height;

        info!("proc_commit after self height {},round {} in height {} round {} ", nowheight, self.round, height, round);
        if nowheight < height + 1 {
            self.change_state_step(height + 1, INIT_ROUND, Step::Propose, true);
            if let Some(hash) = self.pre_hash {
                let buf = hash.to_vec();
                let _ = self.wal_log.save(LOG_TYPE_PREV_HASH, &buf);

                if self.proof.height != nowheight && nowheight > 0 {
                    //panic!("******************");
                    let res = self.generate_proof(nowheight, round, hash);
                    if let Some(proof) = res {
                        self.proof = proof;
                    }
                }
            }

            if !self.proof.is_default() {
                if self.proof.height == nowheight && self.proof.round == round {
                    self.save_wal_proof();
                } else {
                    info!("save proof not ok height {} round {} now height {} ", nowheight, round, self.height);
                }
            }
            self.clean_saved_info();
            self.clean_filtr_info();
            return true;
        }
        false
    }

    fn generate_proof(&mut self, height: usize, round: usize, hash: H256) -> Option<TendermintProof> {
        let mut commits = HashMap::new();
        {
            let vote_set = self.votes.get_voteset(height, round, Step::Precommit);
            let mut num: usize = 0;
            if let Some(vote_set) = vote_set {
                for (sender, vote) in &vote_set.votes_by_sender {
                    if vote.proposal.is_none() {
                        continue;
                    }
                    if vote.proposal.unwrap() == hash {
                        num = num + 1;
                        commits.insert(*sender, vote.signature.clone());
                    }
                }
            }
            if !self.is_above_threshold(&num) {
                return None;
            }
        }
        let mut proof = TendermintProof::default();
        proof.height = height;
        proof.round = round;
        proof.proposal = hash;
        proof.commits = commits;
        return Some(proof);
    }

    fn commit_block(&mut self) -> bool {
        // Commit the block using a complete signature set.
        let height = self.height;
        let round = self.round;
        trace!("commit_block begining {} {} proposal {:?}", height, round, self.proposal);
        if let Some(hash) = self.proposal {
            if self.locked_block.is_some() {
                //generate proof
                let res = self.generate_proof(height, round, hash);
                if let Some(proof) = res {
                    self.proof = proof.clone();
                    self.save_wal_proof();
                /*{
                        self.locked_block.as_mut().unwrap().mut_header().set_proof1(proof.into());
                    }*/
                } else {
                    info!("commit_block proof not ok");
                    return false;
                }

                info!(" ######### height {} consensus time {:?} ", height, Instant::now() - self.htime);
                self.pub_block(self.locked_block.as_ref().unwrap());
                {
                    //update tx pool
                    let txs = self.locked_block.as_ref().unwrap().get_body().get_transactions();
                    //self.tx_pool.update(txs);
                    self.dispatch.del_txs_from_pool(txs.to_vec());
                }
                return true;
            }
        }
        //goto next round
        return false;
    }


    fn pub_message(&self, message: Vec<u8>) {
        let mut msg = communication::Message::new();
        msg.set_cmd_id(libproto::cmd_id(submodules::CONSENSUS, topics::CONSENSUS_MSG));
        msg.set_field_type(communication::MsgType::MSG);
        msg.set_content(message);
        self.pub_sender.send(("consensus.msg".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }

    fn pub_and_broadcast_message(&mut self, height: usize, round: usize, step: Step, hash: Option<H256>) {
        let ref author = self.params.signer;
        let msg = serialize(&(height, round, step, author.address.clone(), hash.clone()), Infinite).unwrap();
        let signature = Signature::sign(&author.keypair.privkey(), &msg.crypt_hash().into()).unwrap();
        let sig = signature.clone();
        let msg = serialize(&(msg, sig), Infinite).unwrap();

        trace!("pub_and_broadcast_message pub {},{},{:?} self {},{},{:?} ", height, round, step, self.height, self.round, self.step);
        self.pub_message(msg.clone());
        //self.wal_log.save(LOG_TYPE_VOTE,&msg).unwrap();

        if self.height >= height || (self.height == height && self.round >= round) {
            self.votes.add(height,
                           round,
                           step,
                           author.address.clone(),
                           VoteMessage {
                               proposal: hash.clone(),
                               signature: signature.into(),
                           });
        }
    }

    fn is_authority(&self, address: &Address) -> bool {
        self.params.authorities.contains(address.into())
    }

    fn change_state_step(&mut self, height: usize, round: usize, s: Step, newflag: bool) {
        self.height = height;
        self.round = round;
        self.step = s;

        if newflag {
            let _ = self.wal_log.set_height(height);
        }

        let message = serialize(&(height, round, s), Infinite).unwrap();
        self.wal_log.save(LOG_TYPE_STATE, &message).unwrap();
    }

    fn handle_state(&mut self, msg: Vec<u8>) {
        if let Ok(decoded) = deserialize(&msg[..]) {
            let (h, r, s) = decoded;
            self.height = h;
            self.round = r;
            self.step = s;
        }
    }

    fn handle_message(&mut self, message: Vec<u8>, wal_flag: bool) -> Result<(usize, usize, Step), EngineError> {
        trace!("handle_message beginning now !");
        let log_msg = message.clone();
        let res = deserialize(&message[..]);
        if let Ok(decoded) = res {
            let (message, signature): (Vec<u8>, &[u8]) = decoded;
            let signature = Signature::from(signature);
            if let Ok(pubkey) = signature.recover(&message.crypt_hash().into()) {
                let decoded = deserialize(&message[..]).unwrap();
                let (h, r, step, sender, hash) = decoded;
                trace!("handle_message  parse over sender:{:?}  h:{} r:{} s:{:?} vs self {} {} {:?}", sender, h, r, step, self.height, self.round, self.step);

                if h < self.height {
                    return Err(EngineError::UnexpectedMessage);
                }

                if self.is_authority(&sender) && pubkey_to_address(&pubkey) == sender {
                    let mut trans_flag = false;
                    let mut add_flag = false;
                    let now = ::std::time::Instant::now();

                    //deal with equal height,and round fall behind
                    if h == self.height && r < self.round {
                        let res = self.send_filter.get_mut(&sender);
                        if let Some(val) = res {
                            let (fround, fstep, ins) = *val;
                            if r > fround || (fround == r && step > fstep) {
                                add_flag = true;
                                //for re_transe msg for lag node
                                if r < self.round {
                                    trans_flag = true;
                                }
                            } else if fround == r && step == fstep {
                                if now - ins > self.params.timer.prevote * TIMEOUT_LOW_ROUND_MESSAGE_MULTIPLE {
                                    add_flag = true;
                                    trans_flag = true;
                                }
                            }
                        } else {
                            add_flag = true;
                        }
                    }

                    if add_flag {
                        self.send_filter.insert(sender, (r, step, now));
                    }
                    if trans_flag {
                        self.retrans_vote(h, r, step);
                        return Err(EngineError::UnexpectedMessage);
                    }

                    /*bellow commit content is suit for when chain not syncing ,but consensus need
                    process up */
                    if h > self.height || (h == self.height && r >= self.round) {
                        //if h == self.height && r >= self.round {
                        info!("handle_message get vote: height {:?}, round {:?}, step {:?}, sender {:?}, hash {:?}, signature {}", h, r, step, sender, hash, signature);
                        let ret = self.votes.add(h,
                                                 r,
                                                 step,
                                                 sender,
                                                 VoteMessage {
                                                     proposal: hash,
                                                     signature: signature.into(),
                                                 });
                        if ret {
                            info!("vote ok!");
                            if wal_flag {
                                self.wal_log.save(LOG_TYPE_VOTE, &log_msg).unwrap();
                            }
                            if h > self.height {
                                return Err(EngineError::VoteMsgForth(h));
                            }
                            return Ok((h, r, step));
                        }
                        return Err(EngineError::DoubleVote(sender.into()));
                    }
                }
            }
        }
        Err(EngineError::UnexpectedMessage)
    }

    fn proc_proposal(&mut self, height: usize, round: usize) -> bool {
        let proposal = self.proposals.get_proposal(height, round);
        if let Some(proposal) = proposal {
            trace!("proc proposal height {},round {} self {} {} ", height, round, self.height, self.round);
            //proposal check
            if !proposal.check(height, &self.params.authorities) {
                trace!("proc proposal check error");
                return false;
            }
            //height 1's block not have prehash
            if let Some(hash) = self.pre_hash {
                //prehash : self.prehash vs  proposal's block's prehash
                let block = parse_from_bytes::<Block>(&proposal.block).unwrap();
                let mut block_prehash = Vec::new();
                block_prehash.extend_from_slice(block.get_header().get_prevhash());
                {
                    if hash != H256::from(block_prehash.as_slice()).into() {
                        trace!("proc proposal pre_hash error");
                        return false;
                    }
                }

                //proof : self.params vs proposal's block's broof
                let block_proof = block.get_header().get_proof();
                let proof = TendermintProof::from(block_proof.clone());
                info!(" proof is {:?}  {} {}", proof, height, round);
                if !proof.check(height - 1, &self.params.authorities) {
                    return false;
                }
                if self.proof.height != height - 1 {
                    self.proof = proof;
                }
            } else {
                if height != INIT_HEIGHT {
                    return false;
                }
            }

            let proposal_lock_round = proposal.lock_round;
            //we have lock block,try unlock
            if self.lock_round.is_some() && proposal_lock_round.is_some() {
                if self.lock_round.unwrap() < proposal_lock_round.unwrap() && proposal_lock_round.unwrap() < round {
                    //we see new lock block unlock mine
                    info!("unlock lock block: height {:?}, proposal {:?}", height, self.proposal);
                    self.clean_saved_info();
                }
            }
            // still lock on a blk,next prevote it
            if self.lock_round.is_some() {
                let ref lock_block = self.locked_block.clone().unwrap();
                self.proposal = Some(lock_block.crypt_hash().into());
                trace!("still have lock block {} locked round {} {:?}", self.height, self.lock_round.unwrap(), self.proposal.unwrap());
            } else {
                // else use proposal block，self.lock_round is none
                let block = parse_from_bytes::<Block>(&proposal.block).unwrap();
                let block_hash = block.crypt_hash();
                self.proposal = Some(block_hash.into());
                info!("save the proposal's hash: height {:?}, round {}, proposal {:?}", self.height, self.round, self.proposal.unwrap());
                self.locked_block = Some(block);
            }
            return true;
        }
        return false;
    }

    fn handle_proposal(&mut self, msg: Vec<u8>, wal_flag: bool) -> Result<(usize, usize), EngineError> {
        let res = deserialize(&msg[..]);
        if let Ok(decoded) = res {
            let (message, signature): (Vec<u8>, &[u8]) = decoded;
            let signature = Signature::from(signature);
            trace!("handle proposal message {:?}", message.crypt_hash());

            if let Ok(pubkey) = signature.recover(&message.crypt_hash().into()) {
                let decoded = deserialize(&message[..]).unwrap();
                let (height, round, proposal) = decoded;
                trace!("handle_proposal height {:?}, round {:?} sender {:?}", height, round, pubkey_to_address(&pubkey));

                let ret = self.is_round_proposer(height, round, &pubkey_to_address(&pubkey));
                if ret.is_err() {
                    return Err(ret.err().unwrap());
                }

                if (height == self.height && round >= self.round) || height == self.height + 1 {
                    if wal_flag && height == self.height {
                        self.wal_log.save(LOG_TYPE_PROPOSE, &msg).unwrap();
                    }
                    info!("add proposal height {} round {}!", height, round);
                    self.proposals.add(height, round, proposal);

                    if height > self.height {
                        return Err(EngineError::VoteMsgForth(height));
                    }
                    return Ok((height, round));
                }
            }
        }
        return Err(EngineError::UnexpectedMessage);
    }

    fn clean_saved_info(&mut self) {
        self.proposal = None;
        self.lock_round = None;
        self.locked_vote = None;
        self.locked_block = None;
        self.last_commit_round = None;
        //self.sync_ok = true;
    }

    fn clean_filtr_info(&mut self) {
        self.send_filter.clear();
    }

    pub fn new_proposal(&mut self) {
        if let Some(lock_round) = self.lock_round {
            let ref lock_blk = self.locked_block;
            let ref lock_vote = self.locked_vote;
            let lock_blk = lock_blk.clone().unwrap();
            {
                let lock_blk_hash = H256::from(lock_blk.crypt_hash());
                info!("proposal lock block: height {:?}, block hash {:?}", self.height, lock_blk_hash);
                self.proposal = Some(lock_blk_hash);
            }
            let blk = lock_blk.write_to_bytes().unwrap();
            let proposal = Proposal {
                block: blk,
                lock_round: Some(lock_round),
                lock_votes: lock_vote.clone(),
            };
            trace!("pub proposal");
            let bmsg = self.pub_proposal(&proposal);
            self.wal_log.save(LOG_TYPE_PROPOSE, &bmsg).unwrap();
            trace!("proposor vote locked block: height {}, round {}", self.height, self.round);
            self.proposals.add(self.height, self.round, proposal);
            return;
        }
        // proposal new blk
        let mut block = Block::new();
        {
            if self.pre_hash.is_some() {
                block.mut_header().set_prevhash(self.pre_hash.unwrap().0.to_vec());
            } else {
                info!("in new_proposal,self.pre_hash is none: height {}, round {}", self.height, self.round);
                //block.mut_header().set_prevhash(H256::default().0.to_vec());
            }

            let proof = self.proof.clone();
            if proof.is_default() && self.height > INIT_HEIGHT {
                warn!("there is no proof height {} round {}", self.height, self.round);
                return;
            }
            if self.height > INIT_HEIGHT {
                if proof.height != self.height - 1 {
                    warn!("proof is old,proof height {}, round {}", proof.height, proof.round);
                    return;
                }
            }
            block.mut_header().set_proof(proof.into());
        }
        {
            //let txs: Vec<SignedTransaction> = self.tx_pool.package(self.height as u64);
            let txs: Vec<SignedTransaction> = self.dispatch.get_txs_from_pool(self.height as u64);
            trace!("new proposal height {:?} tx len {:?}", self.height, txs.len());
            block.mut_body().set_transactions(RepeatedField::from_slice(&txs[..]));
        }
        let block_time = unix_now();
        let transactions_root = block.get_body().transactions_root();
        block.mut_header().set_timestamp(block_time.as_millis());
        block.mut_header().set_height(self.height as u64);
        block.mut_header().set_transactions_root(transactions_root.to_vec());

        let bh = block.crypt_hash();
        info!("proposal new block: height {:?}, block hash {:?}", self.height, bh);
        let pro_hash = Some(bh);
        {
            self.proposal = pro_hash.map(|x| x.into());
            self.locked_block = Some(block.clone());
        }
        let blk = block.write_to_bytes().unwrap();
        let proposal = Proposal {
            block: blk,
            lock_round: None,
            lock_votes: None,
        };
        trace!("pub proposal in not locked");
        let bmsg = self.pub_proposal(&proposal);
        self.wal_log.save(LOG_TYPE_PROPOSE, &bmsg).unwrap();
        trace!("proposor vote myslef in not locked");
        self.proposals.add(self.height, self.round, proposal);
    }

    pub fn timeout_process(&mut self, tminfo: TimeoutInfo) {
        trace!("timeout_process {:?}", tminfo);
        if tminfo.height < self.height || (tminfo.height == self.height && tminfo.round < self.round) {
            return;
        }

        if tminfo.step == Step::ProposeWait {
            let pres = self.proc_proposal(tminfo.height, tminfo.round);
            if !pres {
                trace!("timeout_process proc_proposal res false height {} round {}", tminfo.height, tminfo.round);
            }
            self.change_state_step(tminfo.height, tminfo.round, Step::Prevote, false);
            self.pre_proc_prevote();
            //one node need this
            //if self.params.authorities.len() < 2
            {
                self.proc_prevote(tminfo.height, tminfo.round);
            }

        } else if tminfo.step == Step::Prevote {
            if tminfo.height == self.height && tminfo.round == self.round && tminfo.step == self.step {
                self.pre_proc_prevote();
            }
        } else if tminfo.step == Step::PrevoteWait {

            info!(" #########  height {} round {} prevote wait time {:?} ", tminfo.height, tminfo.round, Instant::now() - self.htime);
            self.change_state_step(tminfo.height, tminfo.round, Step::Precommit, false);
            self.pre_proc_precommit();

            //if self.params.authorities.len() < 2
            {
                self.proc_precommit(tminfo.height, tminfo.round);
            }
        } else if tminfo.step == Step::Precommit {
            if tminfo.height == self.height && tminfo.round == self.round && tminfo.step == self.step {
                /*in this case,need resend prevote : my net server can be connected but other node's
                server not connected when staring.  maybe my node recive enough vote(prevote),but others
                did not recive enough vote,so even if my node step precommit phase, i need resend prevote also.

                */
                self.pre_proc_prevote();
                self.pre_proc_precommit();
            }
        } else if tminfo.step == Step::PrecommitWait {
            info!(" ######### height {} round {} PrecommitWait time {:?} ", tminfo.height, tminfo.round, Instant::now() - self.htime);
            if self.pre_proc_commit(tminfo.height, tminfo.round) {
                /*wait for new status*/
                self.change_state_step(tminfo.height, tminfo.round, Step::Commit, false);
            } else {
                // clean the param if not locked
                if self.lock_round.is_none() {
                    self.clean_saved_info();
                }
                self.change_state_step(tminfo.height, tminfo.round + 1, Step::Propose, false);
                self.redo_work();
            }
        } else if tminfo.step == Step::CommitWait {
            let res = self.proc_commit_after(tminfo.height, tminfo.round);
            if res {
                self.redo_work();
            }
        }
    }

    pub fn process(&mut self, info: TransType) {
        let (id, cmd_id, content_ext) = info;
        let from_broadcast = id == submodules::NET;
        if from_broadcast {
            match cmd_id {
                ID_CONSENSUS_MSG => {
                    //trace!("net receive_new_consensus msg");
                    if let MsgClass::MSG(msg) = content_ext {
                        let res = self.handle_message(msg, true);

                        if let Ok((h, r, s)) = res {
                            if s == Step::Prevote {
                                self.proc_prevote(h, r);
                            } else {
                                self.proc_precommit(h, r);
                            }
                        }
                    }
                }

                ID_NEW_PROPOSAL => {
                    if let MsgClass::MSG(msg) = content_ext {
                        trace!("receive proposal");
                        let res = self.handle_proposal(msg, false);
                        if let Ok((h, r)) = res {
                            trace!("handle_proposal {:?}", (h, r));
                            if h == self.height && r == self.round && self.step < Step::PrevoteWait {
                                let pres = self.proc_proposal(h, r);
                                if !pres {
                                    trace!("proc_proposal res false height {}, round {}", h, r);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        } else {
            match content_ext {
                MsgClass::STATUS(status) => {
                    trace!("get new local status {:?}", status.height);
                    self.receive_new_status(status);
                }
                MsgClass::RICHSTATUS(rich_status) => {
                    info!("tendermint rich_status is {:?}", rich_status);
                }
                _ => {}
            }
        }
    }

    fn receive_new_status(&mut self, status: Status) {
        let status_height = status.height as usize;
        let height = self.height;
        let round = self.round;
        trace!("new_status new height {:?} self height {}", status_height + 1, height);
        if height > 0 && status_height < height - 1 {
            return;
        }
        let mut r = INIT_ROUND;

        if status_height == height || (height > 0 && status_height == height - 1) {
            let pre_hash = H256::from(H256::from_slice(&status.hash));
            {
                trace!("new_status hash is {:?}", pre_hash);
                self.pre_hash = Some(pre_hash);
            }
            if status_height != height {
                /*the statement judging if status height eq self height -1, is for the situation that
                the node lag behind others, jumping to the status_height need know the prev_hash. since the
                jumping ,the node may not save the prev_hash, so save it when chain resending new status*/
                return;
            }
            r = self.round;
        }

        // try my effor to save proof,when I skipping commit_blcok by the chain sending new status.
        if self.proof.height != height {
            if let Some(hash) = self.proposal {
                let res = self.generate_proof(height, round, hash);
                if let Some(proof) = res {
                    self.proof = proof;
                }
            }
        }

        let mut tv = self.params.timer.commit;
        if height > status_height {
            tv = ::std::time::Duration::new(0, 0);
        }

        info!(" ######### height {} round {} chain status return time {:?} ", status_height, self.round, Instant::now() - self.htime);
        WaitTimer::set_timer(self.timer_seter.clone(),
                             TimeoutInfo {
                                 timeval: tv,
                                 height: status_height,
                                 round: r,
                                 step: Step::CommitWait,
                             });
    }

    fn new_round_start(&mut self, height: usize, round: usize) {
        if round == INIT_ROUND {
            self.htime = Instant::now();
        }

        if self.is_round_proposer(height, round, &self.params.signer.address).is_ok() {
            self.new_proposal();
        }
        self.step = Step::ProposeWait;
        WaitTimer::set_timer(self.timer_seter.clone(),
                             TimeoutInfo {
                                 timeval: self.params.timer.propose * ((round + 1) as u32),
                                 height: height,
                                 round: round,
                                 step: Step::ProposeWait,
                             });
    }

    pub fn redo_work(&mut self) {
        let height = self.height;
        let round = self.round;

        trace!("redo work now {},{},{:?}", height, round, self.step);
        if self.step == Step::Propose || self.step == Step::ProposeWait {
            self.new_round_start(height, round);

        } else if self.step == Step::Prevote || self.step == Step::PrevoteWait {
            self.pre_proc_prevote();
            self.proc_prevote(height, round);

            if self.step == Step::PrevoteWait {
                WaitTimer::set_timer(self.timer_seter.clone(),
                                     TimeoutInfo {
                                         timeval: self.params.timer.prevote,
                                         height: height,
                                         round: round,
                                         step: Step::PrevoteWait,
                                     });
            }
        } else if self.step == Step::Precommit || self.step == Step::PrecommitWait {
            self.pre_proc_precommit();
            self.proc_precommit(height, round);
            if self.step == Step::PrecommitWait {
                WaitTimer::set_timer(self.timer_seter.clone(),
                                     TimeoutInfo {
                                         timeval: self.params.timer.precommit,
                                         height: height,
                                         round: round,
                                         step: Step::PrecommitWait,
                                     });
            }
        } else if self.step == Step::Commit {
            /*when rebooting ,we did not know chain if is ready
                if chain garantee that when I sent commit_block,
                it can always issue block, no need for this.
            */
            if !self.commit_block() {
                self.change_state_step(height, round + 1, Step::Propose, true);
                self.clean_saved_info();
                self.new_round_start(height, round + 1);
            }
        } else if self.step == Step::CommitWait {
            /*WaitTimer::set_timer(self.timer_seter.clone(),
                    TimeoutInfo{
                        timeval:self.params.timer.commit,
                        height:height,
                        round:round,
                        step:Step::CommitWait});*/
        }
    }

    pub fn start(&mut self) {
        let vec_buf = self.wal_log.load();
        for (mtype, vec_out) in vec_buf {
            trace!("******* wal_log type {}", mtype);
            if mtype == LOG_TYPE_PROPOSE {
                let res = self.handle_proposal(vec_out, false);
                if let Ok((h, r)) = res {
                    let pres = self.proc_proposal(h, r);
                    if !pres {
                        trace!("in start proc_proposal res false height {} round {}", h, r);
                    }
                }
            } else if mtype == LOG_TYPE_VOTE {
                let res = self.handle_message(vec_out, false);
                if let Ok((h, r, s)) = res {
                    if s == Step::Prevote {
                        self.proc_prevote(h, r);
                    } else {
                        self.proc_precommit(h, r);
                    }
                }
            } else if mtype == LOG_TYPE_STATE {
                self.handle_state(vec_out);
            } else if mtype == LOG_TYPE_PREV_HASH {
                let pre_hash = H256::from(H256::from_slice(&vec_out));
                self.pre_hash = Some(pre_hash);
            } else if mtype == LOG_TYPE_COMMITS {
                trace!(" wal proof begining!");
                if let Ok(proof) = deserialize(&vec_out) {
                    trace!(" wal proof here {:?}", proof);
                    self.proof = proof;
                }
            }
        }

        // TODO : broadcast some message, based on current state

        if self.height >= INIT_HEIGHT {
            self.redo_work();
        }

        loop {
            let mut gtm = Err(RecvError);
            let mut ginfo = Err(RecvError);

            {
                let tn = &self.timer_notity;
                let pn = &self.pub_recver;
                select!{
                    tm = tn.recv()=>{
                        gtm = tm;
                    },
                    info = pn.recv()=>{
                        ginfo = info;
                    }
                }
            }

            if let Ok(oktm) = gtm {
                //trace!("in select !height {},round {},step {:?}",oktm.height,oktm.round,oktm.step);
                self.timeout_process(oktm);
            }

            if let Ok(tinfo) = ginfo {
                self.process(tinfo);
            }
        }
    }
}
