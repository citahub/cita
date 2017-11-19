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

use libproto::blockchain::RichStatus as ProtoRichStatus;
use protobuf::RepeatedField;
use util::{H256, Address};

#[derive(PartialEq, Clone, Debug)]
pub struct RichStatus {
    number: u64,
    hash: H256,
    nodes: Vec<Address>,
}

impl RichStatus {
    pub fn new() -> Self {
        RichStatus {
            number: 0,
            hash: H256::default(),
            nodes: vec![],
        }
    }

    pub fn hash(&self) -> &H256 {
        &self.hash
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    pub fn set_hash(&mut self, h: H256) {
        self.hash = h;
    }

    pub fn set_number(&mut self, n: u64) {
        self.number = n;
    }

    pub fn set_nodes(&mut self, nodes: Vec<Address>) {
        self.nodes = nodes
    }

    pub fn protobuf(&self) -> ProtoRichStatus {
        let mut ps = ProtoRichStatus::new();
        ps.set_height(self.number());
        ps.set_hash(self.hash().to_vec());
        let node_list = self.nodes.clone().into_iter().map(|address| address.to_vec()).collect();
        ps.set_nodes(RepeatedField::from_vec(node_list));
        ps
    }
}
