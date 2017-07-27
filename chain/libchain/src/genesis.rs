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

use serde_types::hash::{H256, H512};
use libproto::blockchain::{Block, Commit};
use std::io::BufReader;
use serde_json;
use std::fs::File;
use util::SHA3_NULL_RLP;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Spec {
    pub prevhash: H256,
    pub timestamp: u64,
    pub admin: Admin,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Admin {
    pub pubkey: H512,
    pub crypto: String,
    pub identifier: String,
}

#[derive(Debug, PartialEq)]
pub struct Genesis {
    pub spec: Spec,
    pub block: Block,
    pub hash: H256,
}

pub trait GenesisCommitProvider {
    fn get(&self, spec: &Spec) -> Option<Commit>;
}

impl Genesis {
    pub fn init(path: &str) -> Genesis {
        let config_file = File::open(path).unwrap();
        let fconfig = BufReader::new(config_file);
        let spec: Spec = serde_json::from_reader(fconfig).expect("Failed to load genesis.");
        Genesis {
            spec: spec,
            block: Block::default(),
            hash: H256::default(),
        }
    }

    pub fn lazy_execute(&mut self) -> Result<(), &str>
    {
        self.block.set_version(0);
        self.block
            .mut_header()
            .set_prevhash(self.spec.prevhash.0.to_vec());
        self.block.mut_header().set_timestamp(self.spec.timestamp);
        self.block.mut_header().set_height(0);
        trace!("prepare genesis commit");
        
        let mut commit = Commit::new();
        let root = SHA3_NULL_RLP;
        commit.set_state_root(root.to_vec());
        info!("genesis state {:?}", root);

        self.block
            .mut_header()
            .set_commit(commit);
        self.hash = self.block.sha3().into();
        Ok(())
    }
}
