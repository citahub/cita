// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Log entry type definition.


use BlockNumber;
use bloomable::Bloomable;
use rlp::*;
use std::ops::Deref;
use util::{H256, Address, Bytes, HeapSizeOf, Hashable};
// use ethjson;

pub type LogBloom = ::util::H2048;

/// A record of execution for a `LOG` operation.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    /// The address of the contract executing at the point of the `LOG` operation.
    pub address: Address,
    /// The topics associated with the `LOG` operation.
    pub topics: Vec<H256>,
    /// The data associated with the `LOG` operation.
    pub data: Bytes,
}

impl Encodable for LogEntry {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data);
    }
}

impl Decodable for LogEntry {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        let entry = LogEntry {
            address: rlp.val_at(0)?,
            topics: rlp.list_at(1)?,
            data: rlp.val_at(2)?,
        };
        Ok(entry)
    }
}

impl HeapSizeOf for LogEntry {
    fn heap_size_of_children(&self) -> usize {
        self.topics.heap_size_of_children() + self.data.heap_size_of_children()
    }
}

impl LogEntry {
    /// Calculates the bloom of this log entry.
    pub fn bloom(&self) -> LogBloom {
        self.topics
            .iter()
            .fold(LogBloom::from_bloomed(&self.address.crypt_hash()), |b, t| b.with_bloomed(&t.crypt_hash()))
    }
}

// impl From<ethjson::state::Log> for LogEntry {
// 	fn from(l: ethjson::state::Log) -> Self {
// 		LogEntry {
// 			address: l.address.into(),
// 			topics: l.topics.into_iter().map(Into::into).collect(),
// 			data: l.data.into(),
// 		}
// 	}
// }

/// Log localized in a blockchain.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LocalizedLogEntry {
    /// Plain log entry.
    pub entry: LogEntry,
    /// Block in which this log was created.
    pub block_hash: H256,
    /// Block number.
    pub block_number: BlockNumber,
    /// Hash of transaction in which this log was created.
    pub transaction_hash: H256,
    /// Index of transaction within block.
    pub transaction_index: usize,
    /// Log position in the block.
    pub log_index: usize,
    /// Log position in the transaction.
    pub transaction_log_index: usize,
}

impl Deref for LocalizedLogEntry {
    type Target = LogEntry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

#[cfg(test)]
mod tests {
    use super::{LogEntry, LogBloom};
    use util::*;
    use util::hashable::HASH_NAME;

    #[test]
    fn test_empty_log_bloom() {
        let mut bloom = LogBloom::default();
        if HASH_NAME == "sha3" {
            bloom = "00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .parse::<H2048>()
                .unwrap();
        }
        if HASH_NAME == "blake2b" {
            bloom = "00000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .parse::<H2048>()
                .unwrap();
        }
        let address = "0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6".parse::<Address>().unwrap();
        let log = LogEntry {
            address: address,
            topics: vec![],
            data: vec![],
        };
        assert_eq!(log.bloom(), bloom);
    }
}
