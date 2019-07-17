// FIXME: This file will replace ../cita-chain/types/Receipt.rs
use crate::core_types::{Bloom, Hash};
use cita_types::Address;
use rlp::{Encodable, RlpStream};

#[derive(Default, Debug, Clone)]
pub struct Receipt {
    pub state_root: Hash,
    pub transaction_hash: Hash,
    pub quota_used: u64,
    pub logs_bloom: Bloom,
    pub logs: Vec<LogEntry>,
    pub receipt_error: String,
    pub contract_address: Option<Address>,
}

impl Receipt {
    /// Calculate the receipt hash. To maintain consistency we use RLP
    /// serialization.
    pub fn hash(&self) -> Hash {
        let rlp_data = rlp::encode(self);
        Hash::digest(&rlp_data)
    }
}

/// Structure encodable to RLP
impl Encodable for Receipt {
    /// Append a value to the stream
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.state_root);
        s.append(&self.transaction_hash);
        s.append(&self.quota_used);
        s.append(&self.logs_bloom.as_bytes());
        s.append_list(&self.logs);
        s.append(&self.receipt_error);
        s.append(&self.contract_address);
    }
}

#[derive(Default, Debug, Clone)]
pub struct LogEntry {
    pub address: Address,
    pub topics: Vec<Hash>,
    pub data: Vec<u8>,
}

/// Structure encodable to RLP
impl Encodable for LogEntry {
    /// Append a value to the stream
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data);
    }
}
