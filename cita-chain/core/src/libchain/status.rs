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

use cita_types::H256;
use libproto::blockchain::Status as ProtoStatus;

#[derive(PartialEq, Clone, Debug, Default)]
pub struct Status {
    number: u64,
    hash: H256,
}

impl Status {
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

    pub fn protobuf(&self) -> ProtoStatus {
        let mut ps = ProtoStatus::new();
        ps.set_height(self.number());
        ps.set_hash(self.hash().to_vec());
        ps
    }
}
