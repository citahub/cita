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

// FixMe: Rewrite
use crate::receipt::Receipt;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

#[derive(Clone)]
pub struct BlockReceipts {
    pub receipts: Vec<Receipt>,
}

impl BlockReceipts {
    pub fn new(receipts: Vec<Receipt>) -> Self {
        BlockReceipts { receipts }
    }
}

impl Decodable for BlockReceipts {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(BlockReceipts {
            receipts: rlp.as_list()?,
        })
    }
}

impl Encodable for BlockReceipts {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.receipts);
    }
}

#[cfg(test)]
mod tests {
    use super::BlockReceipts;
    use rlp::*;

    #[test]
    fn encode_block_receipts() {
        let br = BlockReceipts::new(Vec::new());

        let mut s = RlpStream::new_list(2);
        s.append(&br);
        assert!(!s.is_finished(), "List shouldn't finished yet");
        s.append(&br);
        assert!(s.is_finished(), "List should be finished now");
        s.out();
    }
}
