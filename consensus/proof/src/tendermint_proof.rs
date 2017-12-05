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

use bincode::{deserialize, serialize, Infinite};
use crypto::{pubkey_to_address, Sign, Signature};
use libproto::blockchain::{Proof, ProofType};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::usize::MAX;
use util::{Address, H256};
use util::Hashable;
use util::datapath::DataPath;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TendermintProof {
    pub proposal: H256,
    // Prev height
    pub height: usize,
    pub round: usize,
    pub commits: HashMap<Address, Signature>,
}

impl TendermintProof {
    pub fn new(height: usize, round: usize, proposal: H256, commits: HashMap<Address, Signature>) -> TendermintProof {
        TendermintProof {
            height: height,
            round: round,
            proposal: proposal,
            commits: commits,
        }
    }

    pub fn default() -> Self {
        TendermintProof {
            height: MAX,
            round: MAX,
            proposal: H256::default(),
            commits: HashMap::new(),
        }
    }

    pub fn store(&self) {
        let proof_path = DataPath::proof_bin_path();
        let mut file = File::create(&proof_path).unwrap();
        let encoded_proof: Vec<u8> = serialize(&self, Infinite).unwrap();
        file.write_all(&encoded_proof).unwrap();
        let _ = file.sync_all();
    }

    pub fn load(&mut self) {
        let proof_path = DataPath::proof_bin_path();
        if let Ok(mut file) = File::open(&proof_path) {
            let mut content = Vec::new();
            if file.read_to_end(&mut content).is_ok() {
                if let Ok(decoded) = deserialize(&content[..]) {
                    //self.round = decoded.round;
                    //self.proposal = decoded.proposal;
                    //self.commits = decoded.commits;
                    *self = decoded;
                }
            }
        }
    }

    pub fn is_default(&self) -> bool {
        if self.round == MAX {
            return true;
        }
        return false;
    }

    // Check proof commits
    pub fn check(&self, h: usize, authorities: &[Address]) -> bool {
        if h == 0 {
            return true;
        }
        if h != self.height {
            return false;
        }
        if 2 * authorities.len() >= 3 * self.commits.len() {
            return false;
        }
        self.commits.iter().all(|(sender, sig)| {
            if authorities.contains(sender) {
                let msg = serialize(
                    &(
                        h,
                        self.round,
                        Step::Precommit,
                        sender,
                        Some(self.proposal.clone()),
                    ),
                    Infinite,
                ).unwrap();
                let signature = Signature(sig.0.into());
                if let Ok(pubkey) = signature.recover(&msg.crypt_hash().into()) {
                    return pubkey_to_address(&pubkey) == sender.clone().into();
                }
            }
            false
        })
    }
}

impl From<Proof> for TendermintProof {
    fn from(p: Proof) -> Self {
        let decoded: TendermintProof = deserialize(&p.get_content()[..]).unwrap();
        decoded
    }
}

impl Into<Proof> for TendermintProof {
    fn into(self) -> Proof {
        let mut proof = Proof::new();
        let encoded_proof: Vec<u8> = serialize(&self, Infinite).unwrap();
        proof.set_content(encoded_proof);
        proof.set_field_type(ProofType::Tendermint);
        proof
    }
}

#[cfg(test)]
mod tests {
    use super::{H256, TendermintProof};
    use libproto::blockchain::Proof;
    use std::collections::HashMap;

    #[test]
    fn proof_convert() {
        let o_proof = TendermintProof::new(0, 1, H256::default(), HashMap::new());
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: TendermintProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}
