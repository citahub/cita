//! Blockchain DB extras.
use bloomchain;
use util::*;
use rlp::*;
use state::receipt::Receipt;
use state::db::Key;
use state::blooms::{GroupPosition, BloomGroup};

/// Represents index of extra data in database
#[derive(Copy, Debug, Hash, Eq, PartialEq, Clone)]
pub enum ExtrasIndex {
	/// Transaction address index
	TransactionAddress = 0,
	/// Block receipts index
	BlockReceipts = 1,
	/// Block blooms index
	BlocksBlooms = 2,
}

fn with_index(hash: &H256, i: ExtrasIndex) -> H264 {
	let mut result = H264::default();
	result[0] = i as u8;
	(*result)[1..].clone_from_slice(hash);
	result
}

impl Key<TransactionAddress> for H256 {
	type Target = H264;

	fn key(&self) -> H264 {
		with_index(self, ExtrasIndex::TransactionAddress)
	}
}

impl Key<BlockReceipts> for H256 {
	type Target = H264;

	fn key(&self) -> H264 {
		with_index(self, ExtrasIndex::BlockReceipts)
	}
}

pub struct LogGroupKey([u8; 6]);

impl Deref for LogGroupKey {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct LogGroupPosition(GroupPosition);

impl From<bloomchain::group::GroupPosition> for LogGroupPosition {
	fn from(position: bloomchain::group::GroupPosition) -> Self {
		LogGroupPosition(From::from(position))
	}
}

impl HeapSizeOf for LogGroupPosition {
	fn heap_size_of_children(&self) -> usize {
		self.0.heap_size_of_children()
	}
}

impl Key<BloomGroup> for LogGroupPosition {
	type Target = LogGroupKey;

	fn key(&self) -> Self::Target {
		let mut result = [0u8; 6];
		result[0] = ExtrasIndex::BlocksBlooms as u8;
		result[1] = self.0.level;
		result[2] = (self.0.index >> 24) as u8;
		result[3] = (self.0.index >> 16) as u8;
		result[4] = (self.0.index >> 8) as u8;
		result[5] = self.0.index as u8;
		LogGroupKey(result)
	}
}

/// Represents address of certain transaction within block
#[derive(Debug, PartialEq, Clone)]
pub struct TransactionAddress {
	/// Block hash
	pub block_hash: H256,
	/// Transaction index within the block
	pub index: usize
}

impl HeapSizeOf for TransactionAddress {
	fn heap_size_of_children(&self) -> usize { 0 }
}

impl Decodable for TransactionAddress {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		let d = decoder.as_rlp();
		let tx_address = TransactionAddress {
			block_hash: d.val_at(0)?,
			index: d.val_at(1)?,
		};

		Ok(tx_address)
	}
}

impl Encodable for TransactionAddress {
	fn rlp_append(&self, s: &mut RlpStream) {
		s.begin_list(2);
		s.append(&self.block_hash);
		s.append(&self.index);
	}
}

/// Contains all block receipts.
#[derive(Clone)]
pub struct BlockReceipts {
	pub receipts: Vec<Option<Receipt>>,
}

impl BlockReceipts {
	pub fn new(receipts: Vec<Option<Receipt>>) -> Self {
		BlockReceipts {
			receipts: receipts
		}
	}
}

impl Decodable for BlockReceipts {
	fn decode<D>(decoder: &D) -> Result<Self, DecoderError> where D: Decoder {
		Ok(BlockReceipts {
			receipts: Decodable::decode(decoder)?
		})
	}
}

impl Encodable for BlockReceipts {
	fn rlp_append(&self, s: &mut RlpStream) {
		Encodable::rlp_append(&self.receipts, s);
	}
}

impl HeapSizeOf for BlockReceipts {
	fn heap_size_of_children(&self) -> usize {
		self.receipts.heap_size_of_children()
	}
}

#[cfg(test)]
mod tests {
	use rlp::*;
	use super::BlockReceipts;

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
