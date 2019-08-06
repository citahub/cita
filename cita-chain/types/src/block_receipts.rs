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
