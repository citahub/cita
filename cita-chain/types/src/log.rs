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

use super::Bytes;
use crate::block_number::BlockNumber;
use cita_types::traits::BloomTools;
use cita_types::{Address, Bloom, H256};
use jsonrpc_types::rpc_types::Log as RpcLog;
use libproto::executor::LogEntry as ProtoLog;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
use std::ops::Deref;

type Topic = Vec<H256>;

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub address: Address,
    pub topics: Topic,
    pub data: Bytes,
}

impl Encodable for Log {
    fn rlp_append(&self, stream: &mut RlpStream) {
        stream.begin_list(3);
        stream.append(&self.address);
        stream.append_list(&self.topics);
        stream.append(&self.data);
    }
}

impl Decodable for Log {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(Log {
            address: rlp.val_at(0)?,
            topics: rlp.list_at(1)?,
            data: rlp.val_at(2)?,
        })
    }
}

impl Log {
    pub fn bloom(&self) -> Bloom {
        self.topics
            .iter()
            .fold(Bloom::from_raw(&self.address), |bloom, topic| {
                let mut bloom = bloom;
                bloom.accrue_raw(&topic);
                bloom
            })
    }

    pub fn protobuf(&self) -> ProtoLog {
        let mut proto_log = ProtoLog::new();

        proto_log.set_address(self.address.to_vec());
        proto_log.topics = self
            .topics
            .clone()
            .into_iter()
            .map(|topic| topic.to_vec())
            .collect();
        proto_log.set_data(self.data.clone());
        proto_log
    }
}

impl Into<RpcLog> for Log {
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

/// Log localized.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct LocalizedLog {
    pub log: Log,
    pub block_hash: H256,
    pub block_number: BlockNumber,
    pub transaction_hash: H256,
    pub transaction_index: usize,
    pub log_index: usize,
    pub transaction_log_index: usize,
}

impl Deref for LocalizedLog {
    type Target = Log;

    fn deref(&self) -> &Self::Target {
        &self.log
    }
}

impl Into<RpcLog> for LocalizedLog {
    fn into(self) -> RpcLog {
        RpcLog {
            address: self.log.address,
            topics: self.log.topics.into_iter().map(Into::into).collect(),
            data: self.log.data.into(),
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
    use super::{Bloom, Log};
    use cita_types::{Address, H256};

    #[test]
    fn test_address_log_bloom() {
        let address = Address::default();
        let log = Log {
            address,
            topics: vec![],
            data: vec![],
        };
        let bloom: Bloom = "
            0000000000000000008000000000000000000000000000000000000000000000
            0000000000000000000000000000000200000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000100000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000"
            .into();
        assert_eq!(log.bloom(), bloom);
    }

    #[test]
    fn test_address_and_topic_log_bloom() {
        let address = Address::default();
        let topics = vec![H256::zero()];
        let log = Log {
            address,
            topics,
            data: vec![],
        };
        let bloom: Bloom = "
            0000000000000000008000000000000000000000000000000000000000000000
            0000000000000000000000000000000200000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000002000000000000000000080000000000000000000000000000000000
            0000000000000000000000000000000100000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000002000
            0000000000000000000000000000000000000000000000000000000000000000"
            .into();
        assert_eq!(log.bloom(), bloom);
    }

    #[test]
    fn test_address_topic_and_data_log_bloom() {
        let address = Address::default();
        let topics = vec![H256::zero()];
        let data = b"test".to_vec();
        let log = Log {
            address,
            topics,
            data,
        };
        let bloom: Bloom = "
            0000000000000000008000000000000000000000000000000000000000000000
            0000000000000000000000000000000200000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000002000000000000000000080000000000000000000000000000000000
            0000000000000000000000000000000100000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000000000
            0000000000000000000000000000000000000000000000000000000000002000
            0000000000000000000000000000000000000000000000000000000000000000"
            .into();
        assert_eq!(log.bloom(), bloom);
    }
}
