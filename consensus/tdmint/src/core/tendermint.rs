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

use authority_manage::AuthorityManage;
use bincode::{deserialize, serialize, Infinite};

use core::params::TendermintParams;
use core::voteset::{verify_tx, Proposal, ProposalCollector, VoteCollector, VoteMessage, VoteSet};

use core::votetime::TimeoutInfo;
use core::wal::Wal;

use crypto::{pubkey_to_address, CreateKey, Sign, Signature, SIGNATURE_BYTES_LEN};
use engine::{unix_now, AsMillis, EngineError, Mismatch};
use libproto::{auth, communication, factory, submodules, topics, MsgClass};
use libproto::blockchain::{Block, BlockTxs, BlockWithProof, RichStatus};
use libproto::consensus::{Proposal as ProtoProposal, SignedProposal, Vote as ProtoVote};
use proof::TendermintProof;
use protobuf::{Message, RepeatedField};
use protobuf::core::parse_from_bytes;
use std::collections::{HashMap, LinkedList};
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::time::{Duration, Instant};

use util::{Address, H256, Hashable};
use util::datapath::DataPath;

const INIT_HEIGHT: usize = 1;
const INIT_ROUND: usize = 0;

const LOG_TYPE_PROPOSE: u8 = 1;
const LOG_TYPE_VOTE: u8 = 2;
const LOG_TYPE_STATE: u8 = 3;
const LOG_TYPE_PREV_HASH: u8 = 4;
const LOG_TYPE_COMMITS: u8 = 5;
const LOG_TYPE_VERIFIED_PROPOSE: u8 = 6;
const LOG_TYPE_AUTH_TXS: u8 = 7;

const ID_CONSENSUS_MSG: u32 = (submodules::CONSENSUS << 16) + topics::CONSENSUS_MSG as u32;
const ID_NEW_PROPOSAL: u32 = (submodules::CONSENSUS << 16) + topics::NEW_PROPOSAL as u32;
//const ID_NEW_STATUS: u32 = (submodules::CHAIN << 16) + topics::NEW_STATUS as u32;

const TIMEOUT_RETRANSE_MULTIPLE: u32 = 15;
const TIMEOUT_LOW_ROUND_MESSAGE_MULTIPLE: u32 = 20;

pub type TransType = (u32, u32, MsgClass);
pub type PubType = (String, Vec<u8>);

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Eq, Clone, Copy, Hash)]
pub enum Step {
    Propose,
    ProposeWait,
    Prevote,
    PrevoteWait,
    PrecommitAuth,
    Precommit,
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
            4u8 => Step::PrecommitAuth,
            5u8 => Step::Precommit,
            6u8 => Step::PrecommitWait,
            7u8 => Step::Commit,
            8u8 => Step::CommitWait,
            _ => panic!("Invalid step."),
        }
    }
}

fn gen_reqid_from_idx(h: u64, r: u64) -> u64 {
    ((h & 0xffff_ffff_ffff) << 16) | r
}

fn get_idx_from_reqid(reqid: u64) -> (u64, u64) {
    (reqid >> 16, reqid & 0xffff)
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
    htime: Instant,
    auth_manage: AuthorityManage,
    consensus_power: bool,
    unverified_msg: HashMap<(usize, usize), communication::Message>,
    // VecDeque might work, Almost always it is better to use Vec or VecDeque instead of LinkedList
    block_txs: LinkedList<(usize, BlockTxs)>,
    block_proof: Option<(usize, BlockWithProof)>,
}

impl TenderMint {
    pub fn new(
        s: Sender<PubType>,
        r: Receiver<TransType>,
        ts: Sender<TimeoutInfo>,
        rs: Receiver<TimeoutInfo>,
        params: TendermintParams,
    ) -> TenderMint {
        let proof = TendermintProof::default();
        if params.is_test {
            trace!("Run for test!");
        }

        let logpath = DataPath::wal_path();
        TenderMint {
            pub_sender: s,
            pub_recver: r,
            timer_seter: ts,
            timer_notity: rs,

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
            htime: Instant::now(),
            auth_manage: AuthorityManage::new(),
            consensus_power: false,
            unverified_msg: HashMap::new(),
            block_txs: LinkedList::new(),
            block_proof: None,
        }
    }

    fn is_round_proposer(&self, height: usize, round: usize, address: &Address) -> Result<(), EngineError> {
        //let ref p = self.params;
        let p = &self.auth_manage;
        if p.authority_n == 0 {
            info!("authority_n is {}", p.authority_n);
            return Err(EngineError::NotAuthorized(Address::zero()));
        }
        let proposer_nonce = height + round;
        let proposer: &Address = p.authorities.get(proposer_nonce % p.authority_n).expect(
            "There are authority_n authorities; \
             taking number modulo authority_n gives number in authority_n range; qed",
        );
        if proposer == address {
            Ok(())
        } else {
            Err(EngineError::NotProposer(Mismatch {
                expected: *proposer,
                found: *address,
            }))
        }
    }

    pub fn pub_block(&self, block: &BlockWithProof) {
        let msg = factory::create_msg(
            submodules::CONSENSUS,
            topics::NEW_PROOF_BLOCK,
            communication::MsgType::BLOCK_WITH_PROOF,
            block.write_to_bytes().unwrap(),
        );
        self.pub_sender
            .send(("consensus.blk".to_string(), msg.write_to_bytes().unwrap()))
            .unwrap();
    }

    pub fn pub_proposal(&self, proposal: &Proposal) -> Vec<u8> {
        let mut proto_proposal = ProtoProposal::new();
        let pro_block = parse_from_bytes::<Block>(&proposal.block).unwrap();
        proto_proposal.set_block(pro_block);
        proto_proposal.set_islock(proposal.lock_round.is_some());
        proto_proposal.set_round(self.round as u64);
        proto_proposal.set_height(self.height as u64);
        let is_lock = proposal.lock_round.is_some();
        if is_lock {
            proto_proposal.set_lock_round(proposal.lock_round.unwrap() as u64);

            let mut votes = Vec::new();
            for (sender, vote_message) in proposal.clone().lock_votes.unwrap().votes_by_sender {
                let mut vote = ProtoVote::new();
                if vote_message.proposal.is_none() {
                    continue;
                }
                vote.set_proposal(vote_message.proposal.unwrap().to_vec());
                vote.set_sender(sender.to_vec());
                vote.set_signature(vote_message.signature.to_vec());
                votes.push(vote);
            }

            proto_proposal.set_lock_votes(RepeatedField::from_slice(&votes[..]));
        }

        let message = proto_proposal.write_to_bytes().unwrap();
        let author = &self.params.signer;
        let signature = Signature::sign(author.keypair.privkey(), &message.crypt_hash().into()).unwrap();
        trace!(
            "pub_proposal height {}, round {}, hash {}, signature {} ",
            self.height,
            self.round,
            message.crypt_hash(),
            signature
        );
        let mut signed_proposal = SignedProposal::new();
        signed_proposal.set_proposal(proto_proposal);
        signed_proposal.set_signature(signature.to_vec());

        let bmsg = signed_proposal.write_to_bytes().unwrap();
        let msg = factory::create_msg(
            submodules::CONSENSUS,
            topics::NEW_PROPOSAL,
            communication::MsgType::MSG,
            bmsg.clone(),
        );
        self.pub_sender
            .send(("consensus.msg".to_string(), msg.write_to_bytes().unwrap()))
            .unwrap();
        bmsg
    }

    fn pre_proc_prevote(&mut self) {
        let prop = self.proposal;
        let height = self.height;
        let round = self.round;

        if prop.is_none() {
            self.proc_proposal(height, round);
        }
        info!(
            "pre_proc_prevote height {},round {} hash {:?} locked_round {:?}",
            height,
            round,
            prop,
            self.lock_round
        );
        if self.lock_round.is_some() || prop.is_some() {
            //let hash = H256::from(self.locked_block.clone().unwrap().crypt_hash());
            self.pub_and_broadcast_message(height, round, Step::Prevote, prop);
        } else {
            info!(
                "pre_proc_prevote not have any thing in {} {}",
                height,
                round
            );
            self.pub_and_broadcast_message(height, round, Step::Prevote, Some(H256::default()));
        }
        //this is for timeout resending votes
        self.timer_seter.send(TimeoutInfo {
            timeval: self.params.timer.prevote * TIMEOUT_RETRANSE_MULTIPLE,
            height: height,
            round: round,
            step: Step::Prevote,
        });
    }

    fn proc_prevote(&mut self, height: usize, round: usize) -> bool {
        info!(
            "proc_prevote begin height {}, round {} vs self {}, round {}",
            height,
            round,
            self.height,
            self.round
        );
        if height < self.height || (height == self.height && round < self.round)
            || (height == self.height && self.round == round && self.step > Step::PrevoteWait)
        {
            return false;
        }

        let vote_set = self.votes.get_voteset(height, round, Step::Prevote);
        trace!("proc_prevote vote_set {:?}", vote_set);
        if let Some(vote_set) = vote_set {
            if self.is_above_threshold(&vote_set.count) {
                let mut tv = if self.is_all_vote(&vote_set.count) {
                    Duration::new(0, 0)
                } else {
                    self.params.timer.prevote
                };

                for (hash, count) in &vote_set.votes_by_proposal {
                    if self.is_above_threshold(count) {
                        //we have lock block,and now polc  then unlock
                        if self.lock_round.is_some() && self.lock_round.unwrap() < round && round <= self.round {
                            //we see new lock block unlock mine
                            info!("unlock lock block height {:?}, hash {:?}", height, hash);
                            self.lock_round = None;
                            self.locked_vote = None;
                        }

                        if hash.is_zero() {
                            self.clean_saved_info();
                            tv = Duration::new(0, 0);
                        } else if self.proposal == Some(*hash) {
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
                        //more than one hash have threahold is wrong !! do some check ??
                        break;
                    }
                }

                if self.step == Step::Prevote {
                    self.change_state_step(height, round, Step::PrevoteWait, false);
                    self.timer_seter.send(TimeoutInfo {
                        timeval: tv,
                        height: height,
                        round: round,
                        step: Step::PrevoteWait,
                    });
                }
                return true;
            }
        }
        false
    }

    fn is_above_threshold(&self, n: &usize) -> bool {
        *n * 3 > self.auth_manage.authority_n * 2
    }

    fn is_all_vote(&self, n: &usize) -> bool {
        *n == self.auth_manage.authority_n
    }

    fn pre_proc_precommit(&mut self) -> bool {
        let height = self.height;
        let round = self.round;
        let proposal = self.proposal;
        let mut verify_ok = false;
        let mut lock_ok = false;

        if !self.unverified_msg.contains_key(&(height, round)) {
            verify_ok = true;
        }
        if let Some(lround) = self.lock_round {
            trace!(
                "pre_proc_precommit locked round,{},self round {}",
                lround,
                round
            );
            if lround == round {
                lock_ok = true;
            }
        }
        //polc is ok,but not verified , not send precommit
        if lock_ok && !verify_ok {
            return false;
        }

        if lock_ok && verify_ok {
            self.pub_and_broadcast_message(height, round, Step::Precommit, proposal);
        } else {
            self.pub_and_broadcast_message(height, round, Step::Precommit, Some(H256::default()));
        }

        //timeout for resending vote msg
        self.timer_seter.send(TimeoutInfo {
            timeval: self.params.timer.precommit * TIMEOUT_RETRANSE_MULTIPLE,
            height: self.height,
            round: self.round,
            step: Step::Precommit,
        });
        true
    }

    fn retrans_vote(&mut self, height: usize, round: usize, step: Step) {
        self.pub_and_broadcast_message(height, round, step, Some(H256::default()));
    }

    fn proc_precommit(&mut self, height: usize, round: usize) -> bool {
        info!(
            "proc_precommit begin {} {} vs self {} {}",
            height,
            round,
            self.height,
            self.round
        );
        if height < self.height || (height == self.height && round < self.round)
            || (height == self.height && self.round == round && self.step > Step::PrecommitWait)
        {
            return false;
        }

        let vote_set = self.votes.get_voteset(height, round, Step::Precommit);
        trace!(
            "proc_precommit deal height {} round {} voteset {:?}",
            height,
            round,
            vote_set
        );
        if let Some(vote_set) = vote_set {
            if self.is_above_threshold(&vote_set.count) {
                trace!(
                    "proc_precommit is_above_threshold height {} round {}",
                    height,
                    round
                );

                let mut tv = if self.is_all_vote(&vote_set.count) {
                    Duration::new(0, 0)
                } else {
                    self.params.timer.precommit
                };

                for (hash, count) in vote_set.votes_by_proposal {
                    if self.is_above_threshold(&count) {
                        trace!(
                            "proc_precommit is_above_threshold hash {:?} {}",
                            hash,
                            count
                        );
                        if hash.is_zero() {
                            tv = Duration::new(0, 0);
                            trace!("proc_precommit is zero");
                        //self.proposal = None;
                        } else if self.proposal.is_some() {
                            if hash != self.proposal.unwrap() {
                                info!(
                                    "proc_precommit why this hanppen self.proposql {:?} hash {:?}",
                                    self.proposal.unwrap(),
                                    hash
                                );
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
                        break;
                    }
                }

                if self.step == Step::Precommit {
                    self.change_state_step(height, round, Step::PrecommitWait, false);
                    self.timer_seter.send(TimeoutInfo {
                        timeval: tv,
                        height: height,
                        round: round,
                        step: Step::PrecommitWait,
                    });
                }
                return true;
            }
        }
        false
    }

    fn pre_proc_commit(&mut self, h: usize, r: usize) -> bool {
        trace!(
            "** pre_proc_commit now beginging {} {} self {} {} last_commit_round {:?} ",
            h,
            r,
            self.height,
            self.round,
            self.last_commit_round
        );
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
        let now_height = self.height;
        info!(
            "proc_commit after self height {},round {} in height {} round {} ",
            now_height,
            self.round,
            height,
            round
        );
        if now_height < height + 1 {
            self.change_state_step(height + 1, INIT_ROUND, Step::Propose, true);
            if let Some(hash) = self.pre_hash {
                let buf = hash.to_vec();
                let _ = self.wal_log.save(LOG_TYPE_PREV_HASH, &buf);
            }

            if self.proof.height != now_height && now_height > 0 {
                if let Some(phash) = self.proposal {
                    let mut res = self.last_commit_round
                        .and_then(|cround| self.generate_proof(now_height, cround, phash));
                    if res.is_none() {
                        res = self.lock_round
                            .and_then(|cround| self.generate_proof(now_height, cround, phash));
                    }
                    if let Some(proof) = res {
                        self.proof = proof;
                    }
                }
            }
            if !self.proof.is_default() {
                if self.proof.height == now_height {
                    self.save_wal_proof();
                } else {
                    info!(
                        "try my best to save proof not ok,at height {} round {} now height {}",
                        now_height,
                        round,
                        self.height
                    );
                }
            }
            self.clean_saved_info();
            self.clean_filter_info();
            self.clean_block_txs();
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
                        num += 1;
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
        Some(proof)
    }

    fn commit_block(&mut self) -> bool {
        // Commit the block using a complete signature set.
        let height = self.height;
        let round = self.round;

        //to be optimize
        self.clean_verified_info();
        trace!("commit_block begining {} {}", height, round);
        if let Some(hash) = self.proposal {
            if self.locked_block.is_some() {
                //generate proof
                trace!("commit_block proof is {:?}", self.proof);
                let mut get_proof = Some(self.proof.clone());

                let gen_flag = if self.proof.height != height {
                    get_proof = self.generate_proof(height, round, hash);
                    true
                } else {
                    false
                };

                if let Some(proof) = get_proof {
                    if gen_flag {
                        self.proof = proof.clone();
                    }
                    self.save_wal_proof();

                    let mut proof_blk = BlockWithProof::new();
                    let blk = self.locked_block.clone();
                    proof_blk.set_blk(blk.unwrap());
                    proof_blk.set_proof(proof.into());

                    // statement with no effect, bug here?
                    self.block_proof == Some((height, proof_blk.clone()));
                    info!(
                        " ######### height {} consensus time {:?} ",
                        height,
                        Instant::now() - self.htime
                    );
                    self.pub_block(&proof_blk);
                    return true;
                } else {
                    info!(
                        "commit_block proof not ok height {},round {}",
                        height,
                        round
                    );
                    return false;
                }
            }
        }
        //goto next round
        false
    }


    fn pub_message(&self, message: Vec<u8>) {
        let msg = factory::create_msg(
            submodules::CONSENSUS,
            topics::CONSENSUS_MSG,
            communication::MsgType::MSG,
            message,
        );
        self.pub_sender
            .send(("consensus.msg".to_string(), msg.write_to_bytes().unwrap()))
            .unwrap();
    }

    fn pub_and_broadcast_message(&mut self, height: usize, round: usize, step: Step, hash: Option<H256>) {
        let author = &self.params.signer;
        let msg = serialize(&(height, round, step, author.address, hash), Infinite).unwrap();
        let signature = Signature::sign(author.keypair.privkey(), &msg.crypt_hash().into()).unwrap();
        let sig = signature.clone();
        let msg = serialize(&(msg, sig), Infinite).unwrap();

        trace!(
            "pub_and_broadcast_message pub {},{},{:?} self {},{},{:?} ",
            height,
            round,
            step,
            self.height,
            self.round,
            self.step
        );
        self.pub_message(msg.clone());
        //self.wal_log.save(LOG_TYPE_VOTE,&msg).unwrap();

        if self.height >= height || (self.height == height && self.round >= round) {
            self.votes.add(
                height,
                round,
                step,
                author.address,
                VoteMessage {
                    proposal: hash,
                    signature,
                },
            );
        }
    }

    fn is_authority(&self, address: &Address) -> bool {
        //self.params.authorities.contains(address.into())
        self.auth_manage.authorities.contains(address.into())
    }

    fn change_state_step(&mut self, height: usize, round: usize, s: Step, newflag: bool) {
        self.height = height;
        self.round = round;
        self.step = s;

        if newflag {
            let _ = self.wal_log.set_height(height);
        }

        let message = serialize(&(height, round, s), Infinite).unwrap();
        let _ = self.wal_log.save(LOG_TYPE_STATE, &message);
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
            if signature.len() != SIGNATURE_BYTES_LEN {
                return Err(EngineError::InvalidSignature);
            }
            let signature = Signature::from(signature);
            if let Ok(pubkey) = signature.recover(&message.crypt_hash().into()) {
                let decoded = deserialize(&message[..]).unwrap();
                let (h, r, step, sender, hash) = decoded;
                trace!(
                    "handle_message  parse over sender:{:?}  h:{} r:{} s:{:?} vs self {} {} {:?}",
                    sender,
                    h,
                    r,
                    step,
                    self.height,
                    self.round,
                    self.step
                );

                if h < self.height {
                    return Err(EngineError::UnexpectedMessage);
                }

                if self.is_authority(&sender) && pubkey_to_address(&pubkey) == sender {
                    let mut trans_flag = false;
                    let mut add_flag = false;
                    let now = Instant::now();

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
                            } else if fround == r && step == fstep
                                && now - ins > self.params.timer.prevote * TIMEOUT_LOW_ROUND_MESSAGE_MULTIPLE
                            {
                                add_flag = true;
                                trans_flag = true;
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
                        info!(
                            "handle_message get vote: \
                             height {:?}, \
                             round {:?}, \
                             step {:?}, \
                             sender {:?}, \
                             hash {:?}, \
                             signature {} ",
                            h,
                            r,
                            step,
                            sender,
                            hash,
                            signature.clone()
                        );
                        let ret = self.votes.add(
                            h,
                            r,
                            step,
                            sender,
                            VoteMessage {
                                proposal: hash,
                                signature: signature,
                            },
                        );
                        if ret {
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
            trace!(
                "proc proposal height {},round {} self {} {} ",
                height,
                round,
                self.height,
                self.round
            );
            if !proposal.check(height, &self.auth_manage.authorities) {
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
                        trace!(
                            "proc proposal pre_hash error height {} round {} self height {} round {}",
                            height,
                            round,
                            self.height,
                            self.round
                        );
                        return false;
                    }
                }

                //proof : self.params vs proposal's block's broof
                let block_proof = block.get_header().get_proof();
                let proof = TendermintProof::from(block_proof.clone());
                info!(" proof is {:?}  {} {}", proof, height, round);
                if self.auth_manage.authority_h_old == height - 1 {
                    if !proof.check(height - 1, &self.auth_manage.authorities_old) {
                        return false;
                    }
                } else if !proof.check(height - 1, &self.auth_manage.authorities) {
                    return false;
                }

                if self.proof.height != height - 1 {
                    self.proof = proof;
                }
            } else if height != INIT_HEIGHT {
                return false;
            }

            let proposal_lock_round = proposal.lock_round;
            //we have lock block,try unlock
            if self.lock_round.is_some() && proposal_lock_round.is_some()
                && self.lock_round.unwrap() < proposal_lock_round.unwrap()
                && proposal_lock_round.unwrap() < round
            {
                //we see new lock block unlock mine
                info!(
                    "unlock lock block: height {:?}, proposal {:?}",
                    height,
                    self.proposal
                );
                self.clean_saved_info();
            }
            // still lock on a blk,next prevote it
            if self.lock_round.is_some() {
                let lock_block = &self.locked_block.clone().unwrap();
                self.proposal = Some(lock_block.crypt_hash().into());
                info!(
                    "still have lock block {} locked round {} {:?}",
                    self.height,
                    self.lock_round.unwrap(),
                    self.proposal.unwrap()
                );
            } else {
                // else use proposal block，self.lock_round is none
                let block = parse_from_bytes::<Block>(&proposal.block).unwrap();
                let block_hash = block.crypt_hash();
                self.proposal = Some(block_hash.into());
                info!(
                    "save the proposal's hash: height {:?}, round {}, proposal {:?}",
                    self.height,
                    self.round,
                    self.proposal.unwrap()
                );
                self.locked_block = Some(block);
            }
            return true;
        }
        false
    }

    fn verify_req(&mut self, block: Block, vheight: usize, vround: usize) -> bool {
        let transactions = block.get_body().get_transactions();
        let len = transactions.len();
        let verify_ok = transactions.into_iter().all(|tx| {
            let result = verify_tx(tx.get_transaction(), vheight as u64);
            if !result {
                let raw_tx = tx.get_transaction();
                info!(
                    "verify tx in proposal failed, tx nonce: {}, tx valid_until_block: {}, proposal height: {}",
                    raw_tx.get_nonce(),
                    raw_tx.get_valid_until_block(),
                    vheight
                );
            }
            result
        });
        if (len > 0) && verify_ok {
            trace!("Going to send block verify request to auth");
            let reqid = gen_reqid_from_idx(vheight as u64, vround as u64);
            let verify_req = block.block_verify_req(reqid);
            trace!(
                "verify_req with {} txs with block verify request id: {} and height:{} round {} ",
                len,
                reqid,
                vheight,
                vround
            );
            let msg = factory::create_msg(
                submodules::CONSENSUS,
                topics::VERIFY_BLK_REQ,
                communication::MsgType::VERIFY_BLK_REQ,
                verify_req.write_to_bytes().unwrap(),
            );
            self.pub_sender
                .send((
                    "consensus.verify_blk_req".to_string(),
                    msg.write_to_bytes().unwrap(),
                ))
                .unwrap();
            self.unverified_msg.insert((vheight, vround), msg);
        }
        verify_ok
    }

    fn handle_proposal(
        &mut self,
        msg: Vec<u8>,
        wal_flag: bool,
        need_verify: bool,
    ) -> Result<(usize, usize), EngineError> {
        trace!(
            "handle_proposal params wal_flag {}, need_verify {}",
            wal_flag,
            need_verify
        );
        let signed_proposal = parse_from_bytes::<SignedProposal>(&msg);
        trace!(
            "handle proposal here self height {} round {} step {:?}",
            self.height,
            self.round,
            self.step
        );
        if let Ok(signed_proposal) = signed_proposal {
            let signature = signed_proposal.get_signature();
            if signature.len() != SIGNATURE_BYTES_LEN {
                return Err(EngineError::InvalidSignature);
            }
            let signature = Signature::from(signature);

            let proto_proposal = signed_proposal.get_proposal();
            let message = proto_proposal.write_to_bytes().unwrap();
            trace!("handle proposal message {:?}", message.crypt_hash());
            if let Ok(pubkey) = signature.recover(&message.crypt_hash().into()) {
                let height = proto_proposal.get_height() as usize;
                let round = proto_proposal.get_round() as usize;
                if !(height == self.height && round == self.round && self.step < Step::Prevote) {
                    info!(
                        "handle proposal get old proposal now height {} round {} step {:?}",
                        self.height,
                        self.round,
                        self.step
                    );
                    return Err(EngineError::VoteMsgDelay(height));
                }

                let block = proto_proposal.clone().take_block();
                trace!(
                    "handle_proposal height {:?}, round {:?} sender {:?}",
                    height,
                    round,
                    pubkey_to_address(&pubkey)
                );

                if need_verify && !self.verify_req(block.clone(), height, round) {
                    trace!("handle_proposal verify_req is error");
                    return Err(EngineError::InvalidTxInProposal);
                }

                let ret = self.is_round_proposer(height, round, &pubkey_to_address(&pubkey));
                if ret.is_err() {
                    trace!("handle_proposal is_round_proposer {:?}", ret);
                    return Err(ret.err().unwrap());
                }

                if (height == self.height && round >= self.round) || height > self.height {
                    if wal_flag && height == self.height {
                        self.wal_log.save(LOG_TYPE_PROPOSE, &msg).unwrap();
                    }
                    info!("add proposal height {} round {}!", height, round);
                    let blk = block.write_to_bytes().unwrap();
                    let mut lock_round = None;
                    let lock_votes = if proto_proposal.get_islock() {
                        lock_round = Some(proto_proposal.get_lock_round() as usize);
                        let mut vote_set = VoteSet::new();
                        for vote in proto_proposal.get_lock_votes() {
                            vote_set.add(
                                Address::from_slice(vote.get_sender()),
                                VoteMessage {
                                    proposal: Some(H256::from_slice(vote.get_proposal())),
                                    signature: Signature::from(vote.get_signature()),
                                },
                            );
                        }
                        Some(vote_set)
                    } else {
                        None
                    };

                    let proposal = Proposal {
                        block: blk,
                        lock_round: lock_round,
                        lock_votes: lock_votes,
                    };

                    self.proposals.add(height, round, proposal);

                    if height > self.height {
                        return Err(EngineError::VoteMsgForth(height));
                    }
                    return Ok((height, round));
                }
            }
        }
        Err(EngineError::UnexpectedMessage)
    }

    fn clean_saved_info(&mut self) {
        self.proposal = None;
        self.lock_round = None;
        self.locked_vote = None;
        self.locked_block = None;
        self.last_commit_round = None;
    }

    fn clean_verified_info(&mut self) {
        self.unverified_msg.clear();
    }

    fn clean_block_txs(&mut self) {
        let height = self.height - 1;
        self.block_txs = self.block_txs
            .clone()
            .into_iter()
            .filter(|&(hi, _)| hi >= height)
            .collect();
    }

    fn clean_filter_info(&mut self) {
        self.send_filter.clear();
    }

    pub fn new_proposal(&mut self) {
        if let Some(lock_round) = self.lock_round {
            let lock_blk = &self.locked_block;
            let lock_vote = &self.locked_vote;
            let lock_blk = lock_blk.clone().unwrap();
            {
                let lock_blk_hash = H256::from(lock_blk.crypt_hash());
                info!(
                    "proposal lock block: height {:?}, block hash {:?}",
                    self.height,
                    lock_blk_hash
                );
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
            trace!(
                "proposor vote locked block: height {}, round {}",
                self.height,
                self.round
            );
            self.proposals.add(self.height, self.round, proposal);
        } else {
            // proposal new blk
            let mut block = Block::new();
            let mut flag = false;

            for (height, ref blocktxs) in self.block_txs.clone() {
                trace!(
                    "BLOCKTXS get height {}, self height {}",
                    height,
                    self.height
                );
                if height == self.height - 1 {
                    flag = true;
                    block.set_body(blocktxs.get_body().clone());
                }
            }
            if !flag && self.height > INIT_HEIGHT {
                return;
            }

            if self.pre_hash.is_some() {
                block
                    .mut_header()
                    .set_prevhash(self.pre_hash.unwrap().0.to_vec());
            } else {
                info!(
                    "in new_proposal,self.pre_hash is none: height {}, round {}",
                    self.height,
                    self.round
                );
                //block.mut_header().set_prevhash(H256::default().0.to_vec());
            }

            let proof = self.proof.clone();
            if proof.is_default() && self.height > INIT_HEIGHT {
                warn!(
                    "there is no proof height {} round {}",
                    self.height,
                    self.round
                );
                return;
            }
            if self.height > INIT_HEIGHT && proof.height != self.height - 1 {
                warn!(
                    "proof is old,proof height {}, round {}",
                    proof.height,
                    proof.round
                );
                return;
            }
            block.mut_header().set_proof(proof.into());

            let block_time = unix_now();
            let transactions_root = block.get_body().transactions_root();
            block.mut_header().set_timestamp(block_time.as_millis());
            block.mut_header().set_height(self.height as u64);
            block
                .mut_header()
                .set_transactions_root(transactions_root.to_vec());

            let bh = block.crypt_hash();
            info!(
                "proposal new block: height {:?}, block hash {:?}",
                self.height,
                bh
            );
            {
                self.proposal = Some(bh);
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
    }

    pub fn timeout_process(&mut self, tminfo: TimeoutInfo) {
        trace!(
            "timeout_process {:?} now height {},round {}，step {:?}",
            tminfo,
            self.height,
            self.round,
            self.step
        );
        if tminfo.height < self.height {
            return;
        }
        if tminfo.height == self.height && tminfo.round < self.round && tminfo.step != Step::CommitWait {
            return;
        }
        if tminfo.height == self.height && tminfo.round == self.round && tminfo.step != self.step
            && tminfo.step != Step::CommitWait
        {
            return;
        }
        if tminfo.step == Step::ProposeWait {
            let pres = self.proc_proposal(tminfo.height, tminfo.round);
            if !pres {
                trace!(
                    "timeout_process proc_proposal res false height {} round {}",
                    tminfo.height,
                    tminfo.round
                );
            }
            self.pre_proc_prevote();
            self.change_state_step(tminfo.height, tminfo.round, Step::Prevote, false);
            //one node need this
            {
                self.proc_prevote(tminfo.height, tminfo.round);
            }
        } else if tminfo.step == Step::Prevote {
            self.pre_proc_prevote();
        } else if tminfo.step == Step::PrevoteWait {
            info!(
                " #########  height {} round {} prevote wait time {:?} ",
                tminfo.height,
                tminfo.round,
                Instant::now() - self.htime
            );
            if self.pre_proc_precommit() {
                self.change_state_step(tminfo.height, tminfo.round, Step::Precommit, false);
                self.proc_precommit(tminfo.height, tminfo.round);
            } else {
                self.change_state_step(tminfo.height, tminfo.round, Step::PrecommitAuth, false);
                self.timer_seter.send(TimeoutInfo {
                    timeval: self.params.timer.prevote * TIMEOUT_RETRANSE_MULTIPLE,
                    height: tminfo.height,
                    round: tminfo.round,
                    step: Step::PrecommitAuth,
                });
            }
        } else if tminfo.step == Step::PrecommitAuth {
            let msg = self.unverified_msg.get(&(tminfo.height, tminfo.round));
            if let Some(msg) = msg {
                self.pub_sender
                    .send((
                        "consensus.verify_blk_req".to_string(),
                        msg.write_to_bytes().unwrap(),
                    ))
                    .unwrap();
            }
        } else if tminfo.step == Step::Precommit {
            /*in this case,need resend prevote : my net server can be connected but other node's
            server not connected when staring.  maybe my node recive enough vote(prevote),but others
            did not recive enough vote,so even if my node step precommit phase, i need resend prevote also.
            */
            self.pre_proc_prevote();
            self.pre_proc_precommit();
        } else if tminfo.step == Step::PrecommitWait {
            info!(
                " ######### height {} round {} PrecommitWait time {:?} ",
                tminfo.height,
                tminfo.round,
                Instant::now() - self.htime
            );
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
                self.htime = Instant::now();
                self.redo_work();
            }
        }
    }

    pub fn process(&mut self, info: TransType) {
        let (id, cmd_id, content_ext) = info;
        let from_broadcast = id == submodules::NET;
        if from_broadcast && self.consensus_power {
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
                        let res = self.handle_proposal(msg, true, true);
                        if let Ok((h, r)) = res {
                            trace!(
                                "recive handle_proposal {:?} self height {} round {} step {:?}",
                                (h, r),
                                self.height,
                                self.round,
                                self.step
                            );
                            if h == self.height && r == self.round && self.step < Step::Prevote {
                                self.step = Step::ProposeWait;
                                self.timer_seter.send(TimeoutInfo {
                                    timeval: Duration::new(0, 0),
                                    height: h,
                                    round: r,
                                    step: Step::ProposeWait,
                                });
                            }
                        } else {
                            trace!(" fail handle_proposal {}", res.err().unwrap());
                        }
                    }
                }
                _ => {}
            }
        } else {
            match content_ext {
                //接受chain发送的 authorities_list
                MsgClass::RICHSTATUS(rich_status) => {
                    trace!("get new local status {:?}", rich_status.height);
                    self.receive_new_status(rich_status.clone());
                    let authorities: Vec<Address> = rich_status
                        .get_nodes()
                        .into_iter()
                        .map(|node| Address::from_slice(node))
                        .collect();
                    trace!("authorities: [{:?}]", authorities);
                    if authorities.contains(&self.params.signer.address) {
                        self.consensus_power = true;
                    } else {
                        trace!(
                            "address[{:?}] is not consensus power !",
                            self.params.signer.address
                        );
                        self.consensus_power = false;
                    }
                    self.auth_manage
                        .receive_authorities_list(self.height, authorities);
                }

                MsgClass::VERIFYBLKRESP(resp) => {
                    let verify_id = resp.get_id();
                    let (vheight, vround) = get_idx_from_reqid(verify_id);
                    let vheight = vheight as usize;
                    let vround = vround as usize;
                    let mut verify_ok = false;

                    if self.unverified_msg.contains_key(&(vheight, vround)) {
                        if resp.get_ret() == auth::Ret::Ok {
                            verify_ok = true;
                        }
                        let msg = serialize(&(vheight, vround, verify_ok), Infinite).unwrap();
                        let _ = self.wal_log.save(LOG_TYPE_VERIFIED_PROPOSE, &msg);
                        self.unverified_msg
                            .remove(&(vheight as usize, vround as usize));
                    }
                    info!(
                        "recive VERIFYBLKRESP verify_id {} height {} round {} ok {}",
                        verify_id,
                        vheight,
                        vround,
                        verify_ok
                    );
                    if vheight == self.height && vround == self.round && self.step == Step::PrecommitAuth {
                        if !verify_ok {
                            //verify not ok,so clean the proposal info
                            self.clean_saved_info();
                        }
                        if self.pre_proc_precommit() {
                            self.change_state_step(vheight, vround, Step::Precommit, false);
                            self.proc_precommit(vheight, vround);
                        }
                    }
                }

                MsgClass::BLOCKTXS(block_txs) => {
                    info!(
                        "recive blocktxs height {} self height {}",
                        block_txs.get_height(),
                        self.height
                    );
                    let height = block_txs.get_height() as usize;
                    let msg = block_txs.write_to_bytes().unwrap();
                    self.block_txs.push_back((height, block_txs));
                    let _ = self.wal_log.save(LOG_TYPE_AUTH_TXS, &msg);
                    let now_height = self.height;
                    let now_round = self.round;
                    let now_step = self.step;
                    if now_height == height + 1
                        && self.is_round_proposer(now_height, now_round, &self.params.signer.address)
                            .is_ok() && now_step == Step::ProposeWait && self.proposal.is_none()
                    {
                        self.new_proposal();
                        self.timer_seter.send(TimeoutInfo {
                            timeval: Duration::new(0, 0),
                            height: now_height,
                            round: now_round,
                            step: Step::ProposeWait,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    fn receive_new_status(&mut self, status: RichStatus) {
        let status_height = status.height as usize;
        let height = self.height;
        let round = self.round;
        let step = self.step;
        trace!(
            "new_status new height {:?} self height {}",
            status_height + 1,
            height
        );
        if height > 0 && status_height + 1 < height {
            return;
        }

        let pre_hash = H256::from(H256::from_slice(&status.hash));
        if height > 0 && status_height + 1 == height {
            // try efforts to save previous hash,when current block is not commit to chain
            if step < Step::CommitWait {
                self.pre_hash = Some(pre_hash);
            }

            // commit timeout since pub block to chain,so resending the block
            if step >= Step::Commit {
                if let Some((hi, ref bproof)) = self.block_proof {
                    if hi == height {
                        self.pub_block(bproof);
                    }
                }
            }
            return;
        }
        let r = if status_height == height {
            self.pre_hash = Some(pre_hash);
            self.round
        } else {
            INIT_ROUND
        };
        // try my effor to save proof,when I skipping commit_blcok by the chain sending new status.
        if self.proof.height != height {
            if let Some(hash) = self.proposal {
                let res = self.generate_proof(height, round, hash);
                if let Some(proof) = res {
                    self.proof = proof;
                }
            }
        }

        let cost_time = Instant::now() - self.htime;
        let mut tv = self.params.timer.commit;
        if height > status_height
        //|| self.is_round_proposer(status_height+1,INIT_ROUND,&self.params.signer.address).is_ok()
        {
            tv = Duration::new(0, 0);
        } else if cost_time < self.params.duration {
            tv = self.params.duration - cost_time;
        }

        self.change_state_step(status_height, r, Step::CommitWait, false);
        info!(
            " ######### height {} round {} chain status return time {:?} ",
            status_height,
            self.round,
            cost_time
        );
        self.timer_seter.send(TimeoutInfo {
            timeval: tv,
            height: status_height,
            round: r,
            step: Step::CommitWait,
        });
    }

    fn new_round_start(&mut self, height: usize, round: usize) {
        let mut tv = self.params.timer.propose * ((round + 1) as u32);
        if self.proposals.get_proposal(height, round).is_some() {
            tv = Duration::new(0, 0);
        } else if self.is_round_proposer(height, round, &self.params.signer.address)
            .is_ok()
        {
            self.new_proposal();
            tv = Duration::new(0, 0);
        }
        //if is proposal,enter prevote stage immedietly
        self.step = Step::ProposeWait;
        self.timer_seter.send(TimeoutInfo {
            timeval: tv,
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
                self.timer_seter.send(TimeoutInfo {
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
                self.timer_seter.send(TimeoutInfo {
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
                if self.lock_round.is_none() {
                    self.clean_saved_info();
                }
                self.change_state_step(height, round + 1, Step::Propose, true);
                self.new_round_start(height, round + 1);
            }
        } else if self.step == Step::CommitWait {
            /*self.timer_seter.send(
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
                let res = self.handle_proposal(vec_out, false, true);
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
            } else if mtype == LOG_TYPE_VERIFIED_PROPOSE {
                trace!(" LOG_TYPE_VERIFIED_PROPOSE begining!");
                if let Ok(decode) = deserialize(&vec_out) {
                    let (vheight, vround, verified): (usize, usize, bool) = decode;
                    if !verified {
                        self.clean_saved_info();
                    } else {
                        self.unverified_msg.remove(&(vheight, vround));
                    }
                }
            } else if mtype == LOG_TYPE_AUTH_TXS {
                trace!(" LOG_TYPE_AUTH_TXS begining!");
                let blocktxs = parse_from_bytes::<BlockTxs>(&vec_out);
                if let Ok(blocktxs) = blocktxs {
                    let height = blocktxs.get_height() as usize;
                    trace!(" LOG_TYPE_AUTH_TXS add height {}!", height);
                    self.block_txs.push_back((height, blocktxs));
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
