// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Log entry type definition.

use BlockNumber;
use rlp::*;
use std::ops::Deref;
use cita_types::{Address, Bloom, H256};
use cita_types::traits::BloomTools;
use jsonrpc_types::rpctypes::Log as RpcLog;
use util::{Bytes, HeapSizeOf};
use libproto::executor::LogEntry as ProtoLogEntry;

pub type LogBloom = Bloom;

/// A record of execution for a `LOG` operation.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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
        self.topics.iter().fold(
            LogBloom::from_raw(&self.address),
            |b, t| {
                let mut b = b;
                b.accrue_raw(&t);
                b
            }
        )
    }

    pub fn protobuf(&self) -> ProtoLogEntry {
        let mut proto_log_entry = ProtoLogEntry::new();

        proto_log_entry.set_address(self.address.to_vec());
        proto_log_entry.topics = self.topics
            .clone()
            .into_iter()
            .map(|topic| topic.to_vec())
            .collect();
        proto_log_entry.set_data(self.data.clone());
        proto_log_entry
    }
}

impl Into<RpcLog> for LogEntry {
    fn into(self) -> RpcLog {
        RpcLog {
            address: self.address,
            topics: self.topics.into_iter().map(Into::into).collect(),
            data: self.data.into(),
            block_hash: None,
            block_number: None,
            transaction_hash: None,
            transaction_index: None,
            log_index: None,
            transaction_log_index: None,
        }
    }
}

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

impl Into<RpcLog> for LocalizedLogEntry {
    fn into(self) -> RpcLog {
        RpcLog {
            address: self.entry.address,
            topics: self.entry.topics.into_iter().map(Into::into).collect(),
            data: self.entry.data.into(),
            block_hash: Some(self.block_hash),
            block_number: Some(self.block_number.into()),
            transaction_hash: Some(self.transaction_hash),
            transaction_index: Some(self.transaction_index.into()),
            log_index: Some(self.log_index.into()),
            transaction_log_index: Some(self.transaction_log_index.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LogEntry, LogBloom};
    use cita_types::Address;

    #[test]
    fn test_empty_log_bloom() {
        let bloom = "0000000000000000000000000000000000000000000000000000000000000000\
                     0000000000000000000000000000000000000000000000000000000000000000\
                     0000000000000000000000000000000000000000000000000000088000000000\
                     0000000000000000000000000000000000000000000000000000000000000000\
                     0000000000000000000000000000000000000000000000000000000000000000\
                     0000000000000000000000000000000000000000000000000000000000000000\
                     0000000000000000000080000000000000000000000000000000000000000000\
                     0000000000000000000000000000000000000000000000000000000000000000"
            .parse::<LogBloom>()
            .unwrap();
        let address = "0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6".parse::<Address>().unwrap();
        let log = LogEntry {
            address,
            topics: vec![],
            data: vec![],
        };
        assert_eq!(log.bloom(), bloom);
    }
}
