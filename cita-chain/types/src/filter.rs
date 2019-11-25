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

use crate::block_number::BlockTag;
#[cfg(test)]
use crate::block_number::Tag;
use crate::log::Log;
use cita_types::traits::BloomTools;
use cita_types::{Address, Bloom, H256};
use jsonrpc_types::rpc_types::{Filter as RpcFilter, FilterAddress, Topic, VariadicValue};

/// Address Filter.
#[derive(Debug, PartialEq, Clone)]
pub struct AddressFilter {
    addresses: Option<Vec<Address>>,
}

impl AddressFilter {
    pub fn new(addresses: Option<Vec<Address>>) -> Self {
        AddressFilter { addresses }
    }

    pub fn blooms(&self) -> Vec<Bloom> {
        match self.addresses {
            Some(ref addresses) if !addresses.is_empty() => {
                addresses.iter().map(|ref address| Bloom::from_raw(address)).collect()
            }
            _ => vec![Bloom::default()],
        }
    }

    pub fn matches(&self, log: &Log) -> bool {
        match self.addresses {
            Some(ref addresses) if !addresses.is_empty() => addresses.iter().any(|address| &log.address == address),
            _ => true,
        }
    }
}

impl From<Option<FilterAddress>> for AddressFilter {
    fn from(addresses: Option<FilterAddress>) -> AddressFilter {
        let addresses = addresses.and_then(|address| match address {
            VariadicValue::Null => None,
            VariadicValue::Single(addr) => Some(vec![addr.into()]),
            VariadicValue::Multiple(addr) => Some(addr.into_iter().map(Into::into).collect()),
        });

        AddressFilter { addresses }
    }
}

impl Default for AddressFilter {
    fn default() -> Self {
        AddressFilter { addresses: None }
    }
}

/// Topic Filter.
#[derive(Debug, PartialEq)]
pub struct TopicFilter {
    topics: Vec<Option<Vec<H256>>>,
}

impl TopicFilter {
    pub fn new(topics: Vec<Option<Vec<H256>>>) -> Self {
        TopicFilter { topics }
    }

    pub fn zip_blooms(&self, blooms: Vec<Bloom>) -> Vec<Bloom> {
        self.topics.iter().fold(blooms, |bs, topic| match *topic {
            None => bs,
            Some(ref topics) => bs
                .into_iter()
                .flat_map(|bloom| {
                    topics
                        .iter()
                        .map(|topic| {
                            let mut bloom = bloom;
                            bloom.accrue_raw(topic);
                            bloom
                        })
                        .collect::<Vec<Bloom>>()
                })
                .collect(),
        })
    }

    pub fn matches(&self, log: &Log) -> bool {
        self.topics.iter().enumerate().all(|(index, topic)| match *topic {
            Some(ref topics) if !topics.is_empty() => topics.iter().any(|topic| log.topics.get(index) == Some(topic)),
            _ => true,
        })
    }
}

impl From<Option<Vec<Topic>>> for TopicFilter {
    fn from(topics: Option<Vec<Topic>>) -> TopicFilter {
        let mut iter = topics
            .map_or_else(Vec::new, |topics| {
                topics
                    .into_iter()
                    .take(4)
                    .map(|topic| match topic {
                        VariadicValue::Null => None,
                        VariadicValue::Single(t) => Some(vec![t.into()]),
                        VariadicValue::Multiple(t) => Some(t.into_iter().map(Into::into).collect()),
                    })
                    .collect()
            })
            .into_iter();

        let topics = vec![
            iter.next().unwrap_or(None),
            iter.next().unwrap_or(None),
            iter.next().unwrap_or(None),
            iter.next().unwrap_or(None),
        ];

        TopicFilter { topics }
    }
}

impl Clone for TopicFilter {
    fn clone(&self) -> Self {
        let mut topics = [None, None, None, None];
        topics[..4].clone_from_slice(&self.topics[..4]);

        TopicFilter { topics: topics.to_vec() }
    }
}

impl Default for TopicFilter {
    fn default() -> Self {
        let topics = vec![None, None, None, None];

        TopicFilter { topics }
    }
}

/// All filter.
#[derive(Debug, PartialEq)]
pub struct Filter {
    pub from_block: BlockTag,
    pub to_block: BlockTag,
    pub addresses: AddressFilter,
    pub topics: TopicFilter,
    pub limit: Option<usize>,
}

impl Filter {
    /// Zip blooms with address and topic.
    pub fn zip_blooms(&self) -> Vec<Bloom> {
        self.topics.zip_blooms(self.addresses.blooms())
    }

    /// Check the given log entry matches address or topic.
    pub fn matches(&self, log: &Log) -> bool {
        self.addresses.matches(log) && self.topics.matches(log)
    }

    // For test
    #[cfg(test)]
    pub fn new_with_address_and_topic(addresses: AddressFilter, topics: TopicFilter) -> Self {
        Filter {
            from_block: BlockTag::Tag(Tag::Earliest),
            to_block: BlockTag::Tag(Tag::Latest),
            addresses,
            topics,
            limit: None,
        }
    }
}

impl From<RpcFilter> for Filter {
    fn from(v: RpcFilter) -> Filter {
        Filter {
            from_block: v.from_block.into(),
            to_block: v.to_block.into(),
            addresses: v.address.into(),
            topics: v.topics.into(),
            limit: v.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::filter::{AddressFilter, Filter, TopicFilter};
    use crate::log::Log;
    use cita_types::{Address, Bloom, H256};

    #[test]
    fn test_zip_blooms_none() {
        let none_filter = Filter::new_with_address_and_topic(Default::default(), Default::default());

        let blooms = none_filter.zip_blooms();
        assert_eq!(blooms.len(), 1);
        assert!(blooms[0].is_zero())
    }

    #[test]
    fn test_zip_blooms_single_address_and_single_topic() {
        let topics = vec![Some(vec![H256::zero()]), None, None, None];
        let addresses = Some(vec![Address::default()]);

        let filter = Filter::new_with_address_and_topic(AddressFilter::new(addresses), TopicFilter::new(topics));

        let zip_blooms = filter.zip_blooms();
        let blooms: Vec<Bloom> = vec!["0000000000000000008000000000000000000000000000000000000000000000
             0000000000000000000000000000000200000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000002000000000000000000080000000000000000000000000000000000
             0000000000000000000000000000000100000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000002000
             0000000000000000000000000000000000000000000000000000000000000000"
            .into()];
        assert_eq!(zip_blooms, blooms);
    }

    #[test]
    fn test_zip_bloom_single_address_and_mul_topics() {
        let topics = vec![
            Some(vec![H256::zero()]),
            Some(vec![H256::zero()]),
            Some(vec![H256::zero()]),
            Some(vec![H256::zero()]),
        ];
        let addresses = Some(vec![Address::default()]);

        let filter = Filter::new_with_address_and_topic(AddressFilter::new(addresses), TopicFilter::new(topics));

        let zip_blooms = filter.zip_blooms();
        let blooms: Vec<Bloom> = vec!["0000000000000000008000000000000000000000000000000000000000000000
             0000000000000000000000000000000200000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000002000000000000000000080000000000000000000000000000000000
             0000000000000000000000000000000100000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000002000
             0000000000000000000000000000000000000000000000000000000000000000"
            .into()];
        assert_eq!(zip_blooms, blooms);
    }

    #[test]
    fn test_zip_blooms_mul_addresses_and_mul_topics() {
        let topics = vec![
            Some(vec![H256::zero(), H256::zero()]),
            Some(vec![H256::zero()]),
            Some(vec![H256::zero()]),
            Some(vec![H256::zero()]),
        ];
        let addresses = Some(vec![Address::default(), Address::default()]);

        let filter = Filter::new_with_address_and_topic(AddressFilter::new(addresses), TopicFilter::new(topics));

        let zip_blooms = filter.zip_blooms();
        let blooms: Vec<Bloom> = vec![
            "0000000000000000008000000000000000000000000000000000000000000000
             0000000000000000000000000000000200000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000002000000000000000000080000000000000000000000000000000000
             0000000000000000000000000000000100000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000002000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000008000000000000000000000000000000000000000000000
             0000000000000000000000000000000200000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000002000000000000000000080000000000000000000000000000000000
             0000000000000000000000000000000100000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000002000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000008000000000000000000000000000000000000000000000
             0000000000000000000000000000000200000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000002000000000000000000080000000000000000000000000000000000
             0000000000000000000000000000000100000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000002000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000008000000000000000000000000000000000000000000000
             0000000000000000000000000000000200000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000002000000000000000000080000000000000000000000000000000000
             0000000000000000000000000000000100000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000002000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
        ];
        assert_eq!(zip_blooms, blooms);
    }

    #[test]
    fn test_matches() {
        let topics = vec![Some(vec![H256::zero()]), Some(vec![H256::zero()]), None, None];
        let addresses = Some(vec![Address::default()]);

        let filter = Filter::new_with_address_and_topic(AddressFilter::new(addresses), TopicFilter::new(topics));

        let entry0 = Log {
            address: Address::default(),
            topics: vec![H256::zero(), H256::zero(), H256::random()],
            data: Default::default(),
        };

        let entry1 = Log {
            address: Address::default(),
            topics: vec![H256::zero(), H256::random(), H256::random()],
            data: Default::default(),
        };

        let entry2 = Log {
            address: Address::random(),
            topics: vec![H256::random(), H256::random(), H256::random()],
            data: Default::default(),
        };

        // Filter matches
        assert_eq!(filter.matches(&entry0), true);
        assert_eq!(filter.matches(&entry1), false);
        assert_eq!(filter.matches(&entry2), false);
        // Topic filter matches
        assert_eq!(filter.topics.matches(&entry0), true);
        assert_eq!(filter.topics.matches(&entry1), false);
        assert_eq!(filter.topics.matches(&entry2), false);
        // Address filter matches
        assert_eq!(filter.addresses.matches(&entry0), true);
        assert_eq!(filter.addresses.matches(&entry1), true);
        assert_eq!(filter.addresses.matches(&entry2), false);
    }
}
