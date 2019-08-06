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
