// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
