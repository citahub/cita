//! Basic account type -- the decoded RLP from the state trie.

use rlp::*;
use util::{U256, H256};

/// Basic account type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicAccount {
    // Nonce of the account.
    pub nonce: U256,
    /// Storage root of the account.
    pub storage_root: H256,
    /// Code hash of the account.
    pub code_hash: H256,
}

impl Encodable for BasicAccount {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3)
            .append(&self.nonce)
            .append(&self.storage_root)
            .append(&self.code_hash);
    }
}

impl Decodable for BasicAccount {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		let rlp = decoder.as_rlp();
		Ok(BasicAccount {
			nonce: rlp.val_at(0)?,
			storage_root: rlp.val_at(1)?,
			code_hash: rlp.val_at(2)?,
		})
	}
}
