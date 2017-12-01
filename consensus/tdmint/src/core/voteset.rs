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

use super::{Step, Address};
use bincode::{serialize, Infinite};
use crypto::{Sign, pubkey_to_address, Signature};
use libproto::blockchain::{Block, Transaction};
use lru_cache::LruCache;
use protobuf::core::parse_from_bytes;
use std::collections::HashMap;
use util::{H256, Hashable, BLOCKLIMIT};

// height -> round collector
#[derive(Debug)]
pub struct VoteCollector {
    pub votes: LruCache<usize, RoundCollector>,
}

impl VoteCollector {
    pub fn new() -> Self {
        VoteCollector { votes: LruCache::new(16) }
    }

    pub fn add(&mut self, height: usize, round: usize, step: Step, sender: Address, vote: VoteMessage) -> bool {
        if self.votes.contains_key(&height) {
            self.votes.get_mut(&height).unwrap().add(round, step, sender, vote)
        } else {
            let mut round_votes = RoundCollector::new();
            round_votes.add(round, step, sender, vote);
            self.votes.insert(height, round_votes);
            true
        }
    }

    pub fn get_voteset(&mut self, height: usize, round: usize, step: Step) -> Option<VoteSet> {
        if self.votes.contains_key(&height) {
            self.votes.get_mut(&height).unwrap().get_voteset(round, step)
        } else {
            None
        }
    }
}

//round -> step collector
#[derive(Debug)]
pub struct RoundCollector {
    pub round_votes: LruCache<usize, StepCollector>,
}

impl RoundCollector {
    pub fn new() -> Self {
        RoundCollector { round_votes: LruCache::new(16) }
    }

    pub fn add(&mut self, round: usize, step: Step, sender: Address, vote: VoteMessage) -> bool {
        if self.round_votes.contains_key(&round) {
            self.round_votes.get_mut(&round).unwrap().add(step, sender, vote)
        } else {
            let mut step_votes = StepCollector::new();
            step_votes.add(step, sender, vote);
            self.round_votes.insert(round, step_votes);
            true
        }
    }

    pub fn get_voteset(&mut self, round: usize, step: Step) -> Option<VoteSet> {
        if self.round_votes.contains_key(&round) {
            self.round_votes.get_mut(&round).unwrap().get_voteset(step)
        } else {
            None
        }
    }
}

//step -> voteset
#[derive(Debug)]
pub struct StepCollector {
    pub step_votes: HashMap<Step, VoteSet>,
}

impl StepCollector {
    pub fn new() -> Self {
        StepCollector { step_votes: HashMap::new() }
    }

    pub fn add(&mut self, step: Step, sender: Address, vote: VoteMessage) -> bool {
        if self.step_votes.contains_key(&step) {
            self.step_votes.get_mut(&step).unwrap().add(sender, vote)
        } else {
            let mut vote_set = VoteSet::new();
            vote_set.add(sender, vote);
            self.step_votes.insert(step, vote_set);
            true
        }
    }

    pub fn get_voteset(&self, step: Step) -> Option<VoteSet> {
        if self.step_votes.contains_key(&step) { Some((&self.step_votes[&step]).clone()) } else { None }
    }
}

//1. sender's votemessage 2. proposal'hash count
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoteSet {
    pub votes_by_sender: HashMap<Address, VoteMessage>,
    pub votes_by_proposal: HashMap<H256, usize>,
    pub count: usize,
}


impl VoteSet {
    pub fn new() -> Self {
        VoteSet {
            votes_by_sender: HashMap::new(),
            votes_by_proposal: HashMap::new(),
            count: 0,
        }
    }

    //just add ,not check
    pub fn add(&mut self, sender: Address, vote: VoteMessage) -> bool {
        if self.votes_by_sender.contains_key(&sender) {
            false
        } else {
            self.count += 1;
            self.votes_by_sender.insert(sender, vote.clone());
            let proposal = vote.proposal;
            let mut hash = H256::default();
            if let Some(h) = proposal {
                hash = h;
            }
            if let Some(c) = self.votes_by_proposal.insert(hash, 1) {
                self.votes_by_proposal.insert(hash, c + 1);
            }
            true
        }
    }

    pub fn check(&self, h: usize, r: usize, step: Step, authorities: &[Address]) -> Result<Option<H256>, &str> {
        let mut votes_by_proposal = HashMap::new();
        for (sender, vote) in &self.votes_by_sender {
            if authorities.contains(sender) {
                let msg = serialize(&(h, r, step, sender, vote.proposal), Infinite).unwrap();
                let signature = &vote.signature;
                if let Ok(pubkey) = signature.recover(&msg.crypt_hash().into()) {
                    if pubkey_to_address(&pubkey) == *sender {
                        let mut hash = H256::default();
                        if let Some(h) = vote.proposal {
                            hash = h;
                        }
                        // inc the count of vote for hash
                        if let Some(c) = votes_by_proposal.insert(hash, 1) {
                            votes_by_proposal.insert(hash, c + 1);
                        }
                    }
                }
            }
        }
        for (hash, count) in &votes_by_proposal {
            if *count * 3 > authorities.len() * 2 {
                if hash.is_zero() {
                    return Ok(None);
                } else {
                    return Ok(Some(*hash));
                }
            }

        }
        Err("vote set check error!")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VoteMessage {
    pub proposal: Option<H256>,
    pub signature: Signature,
}


#[derive(Debug)]
pub struct ProposalCollector {
    pub proposals: LruCache<usize, ProposalRoundCollector>,
}

impl ProposalCollector {
    pub fn new() -> Self {
        ProposalCollector { proposals: LruCache::new(16) }
    }

    pub fn add(&mut self, height: usize, round: usize, proposal: Proposal) -> bool {
        if self.proposals.contains_key(&height) {
            self.proposals.get_mut(&height).unwrap().add(round, proposal)
        } else {
            let mut round_proposals = ProposalRoundCollector::new();
            round_proposals.add(round, proposal);
            self.proposals.insert(height, round_proposals);
            true
        }
    }

    pub fn get_proposal(&mut self, height: usize, round: usize) -> Option<Proposal> {
        if self.proposals.contains_key(&height) {
            self.proposals.get_mut(&height).unwrap().get_proposal(round)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ProposalRoundCollector {
    pub round_proposals: LruCache<usize, Proposal>,
}

impl ProposalRoundCollector {
    pub fn new() -> Self {
        ProposalRoundCollector { round_proposals: LruCache::new(16) }
    }

    pub fn add(&mut self, round: usize, proposal: Proposal) -> bool {
        if self.round_proposals.contains_key(&round) {
            false
        } else {
            self.round_proposals.insert(round, proposal);
            true
        }
    }

    pub fn get_proposal(&mut self, round: usize) -> Option<Proposal> {
        if self.round_proposals.contains_key(&round) {
            Some(self.round_proposals.get_mut(&round).unwrap().clone())
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Proposal {
    pub block: Vec<u8>,
    pub lock_round: Option<usize>,
    pub lock_votes: Option<VoteSet>,
}


// verify tx nonce and valid_until_block
pub fn verify_tx(tx: &Transaction, height: u64) -> bool {
    let nonce = tx.get_nonce();
    let valid_until_block = tx.get_valid_until_block();
    (nonce.len() <= 128) && (height <= valid_until_block && valid_until_block < (height + BLOCKLIMIT))
}

impl Proposal {
    pub fn check(&self, h: usize, authorities: &[Address]) -> bool {
        if self.lock_round.is_none() && self.lock_votes.is_none() {
            true
        } else {
            let round = self.lock_round.unwrap();

            let ret = self.lock_votes.as_ref().unwrap().check(h, round, Step::Prevote, authorities);
            if ret.is_err() {
                return false;
            }

            if let Some(p) = ret.unwrap() {
                let block = parse_from_bytes::<Block>(&self.block).unwrap();

                let hash = block.crypt_hash().into();
                if p == hash {
                    return true;
                }
            }
            false
        }
    }
}
