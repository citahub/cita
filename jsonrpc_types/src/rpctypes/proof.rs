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

use bincode::deserialize;
use cita_types::{Address, H256};
use libproto::blockchain::{Proof as ProtoProof, ProofType};
use proof::{AuthorityRoundProof as AProof, TendermintProof as TProof};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Proof {
    AuthorityRound(AuthorityRoundProof),
    Raft,
    Tendermint(TendermintProof),
}

impl From<ProtoProof> for Proof {
    fn from(p: ProtoProof) -> Self {
        match p.get_field_type() {
            ProofType::AuthorityRound => Proof::AuthorityRound(AuthorityRoundProof::from(p)),
            ProofType::Raft => Proof::Raft,
            ProofType::Tendermint => Proof::Tendermint(TendermintProof::from(p)),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TendermintProof {
    pub proposal: H256,
    pub height: usize,
    pub round: usize,
    pub commits: HashMap<Address, String>,
}

impl From<ProtoProof> for TendermintProof {
    fn from(p: ProtoProof) -> Self {
        let decoded: TProof = deserialize(&p.get_content()[..]).unwrap();
        let mut commits: HashMap<Address, String> = HashMap::new();
        let str_0x = "0x".to_string();
        for (addr, sign) in decoded.commits {
            commits.insert(addr, str_0x.clone() + &String::from(sign));
        }
        TendermintProof {
            proposal: decoded.proposal,
            height: decoded.height,
            round: decoded.round,
            commits: commits,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuthorityRoundProof {
    pub signature: String,
    pub step: u64,
}

impl From<ProtoProof> for AuthorityRoundProof {
    fn from(p: ProtoProof) -> Self {
        let decoded: AProof = deserialize(&p.get_content()[..]).unwrap();
        let signature: String = decoded.signature.into();
        AuthorityRoundProof {
            signature: signature,
            step: decoded.step,
        }
    }
}
