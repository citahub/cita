// Copyright Cryptape Technologies LLC.
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

// FixMe: Rewrite
use cita_types::H256;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

#[derive(Debug)]
pub struct TransactionIndex {
    pub block_hash: H256,
    pub index: usize,
}

impl Decodable for TransactionIndex {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        let tx_index = TransactionIndex {
            block_hash: rlp.val_at(0)?,
            index: rlp.val_at(1)?,
        };

        Ok(tx_index)
    }
}

impl Encodable for TransactionIndex {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.block_hash);
        s.append(&self.index);
    }
}
