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

//! Blockchain filter

use crate::block_number::BlockTag;
use crate::log_entry::{LogBloom, LogEntry};
use cita_types::traits::BloomTools;
use cita_types::{Address, H256};
use jsonrpc_types::rpc_types::{Filter as RpcFilter, VariadicValue};

/// Blockchain Filter.
#[derive(Debug, PartialEq)]
pub struct Filter {
    /// Blockchain will be searched from this block.
    pub from_block: BlockTag,

    /// Till this block.
    pub to_block: BlockTag,

    /// Search addresses.
    ///
    /// If None, match all.
    /// If specified, log must be produced by one of these addresses.
    pub address: Option<Vec<Address>>,

    /// Search topics.
    ///
    /// If None, match all.
    /// If specified, log must contain one of these topics.
    pub topics: Vec<Option<Vec<H256>>>,

    /// Logs limit
    ///
    /// If None, return all logs
    /// If specified, should only return *last* `n` logs.
    pub limit: Option<usize>,
}

impl Clone for Filter {
    fn clone(&self) -> Self {
        let mut topics = [None, None, None, None];
        topics[..4].clone_from_slice(&self.topics[..4]);

        Filter {
            from_block: self.from_block,
            to_block: self.to_block,
            address: self.address.clone(),
            topics: topics[..].to_vec(),
            limit: self.limit,
        }
    }
}

impl Filter {
    /// Returns combinations of each address and topic.
    pub fn bloom_possibilities(&self) -> Vec<LogBloom> {
        let blooms = match self.address {
            Some(ref addresses) if !addresses.is_empty() => addresses
                .iter()
                .map(|ref address| LogBloom::from_raw(address))
                .collect(),
            _ => vec![LogBloom::default()],
        };

        self.topics.iter().fold(blooms, |bs, topic| match *topic {
            None => bs,
            Some(ref topics) => bs
                .into_iter()
                .flat_map(|bloom| {
                    topics
                        .iter()
                        .map(|topic| {
                            let mut b = bloom;
                            b.accrue_raw(topic);
                            b
                        })
                        .collect::<Vec<LogBloom>>()
                })
                .collect(),
        })
    }

    /// Returns true if given log entry matches filter.
    pub fn matches(&self, log: &LogEntry) -> bool {
        let matches = match self.address {
            Some(ref addresses) if !addresses.is_empty() => {
                addresses.iter().any(|address| &log.address == address)
            }
            _ => true,
        };

        matches
            && self
                .topics
                .iter()
                .enumerate()
                .all(|(i, topic)| match *topic {
                    Some(ref topics) if !topics.is_empty() => {
                        topics.iter().any(|topic| log.topics.get(i) == Some(topic))
                    }
                    _ => true,
                })
    }
}

impl From<RpcFilter> for Filter {
    fn from(v: RpcFilter) -> Filter {
        Filter {
            from_block: v.from_block.into(),
            to_block: v.to_block.into(),
            address: v.address.and_then(|address| match address {
                VariadicValue::Null => None,
                VariadicValue::Single(a) => Some(vec![a.into()]),
                VariadicValue::Multiple(a) => Some(a.into_iter().map(Into::into).collect()),
            }),
            topics: {
                let mut iter = v
                    .topics
                    .map_or_else(Vec::new, |topics| {
                        topics
                            .into_iter()
                            .take(4)
                            .map(|topic| match topic {
                                VariadicValue::Null => None,
                                VariadicValue::Single(t) => Some(vec![t.into()]),
                                VariadicValue::Multiple(t) => {
                                    Some(t.into_iter().map(Into::into).collect())
                                }
                            })
                            .collect()
                    })
                    .into_iter();

                vec![
                    iter.next().unwrap_or(None),
                    iter.next().unwrap_or(None),
                    iter.next().unwrap_or(None),
                    iter.next().unwrap_or(None),
                ]
            },
            limit: v.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::block_number::{BlockTag, Tag};
    use crate::filter::Filter;
    use crate::log_entry::{LogBloom, LogEntry};

    #[test]
    fn test_bloom_possibilities_none() {
        let none_filter = Filter {
            from_block: BlockTag::Tag(Tag::Earliest),
            to_block: BlockTag::Tag(Tag::Latest),
            address: None,
            topics: vec![None, None, None, None],
            limit: None,
        };

        let possibilities = none_filter.bloom_possibilities();
        assert_eq!(possibilities.len(), 1);
        assert!(possibilities[0].is_zero())
    }

    // block 399849
    #[test]
    fn test_bloom_possibilities_single_address_and_topic() {
        let filter = Filter {
            from_block: BlockTag::Tag(Tag::Earliest),
            to_block: BlockTag::Tag(Tag::Latest),
            address: Some(vec!["b372018f3be9e171df0581136b59d2faf73a7d5d".into()]),
            topics: vec![
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                None,
                None,
                None,
            ],
            limit: None,
        };
        let possibilities = filter.bloom_possibilities();
        let blooms: Vec<LogBloom> = vec![
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
        ];
        assert_eq!(possibilities, blooms);
    }

    #[test]
    fn test_bloom_possibilities_single_address_and_many_topics() {
        let filter = Filter {
            from_block: BlockTag::Tag(Tag::Earliest),
            to_block: BlockTag::Tag(Tag::Latest),
            address: Some(vec!["b372018f3be9e171df0581136b59d2faf73a7d5d".into()]),
            topics: vec![
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                None,
                None,
            ],
            limit: None,
        };
        let possibilities = filter.bloom_possibilities();
        let blooms: Vec<LogBloom> = vec![
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
        ];
        assert_eq!(possibilities, blooms);
    }

    #[test]
    fn test_bloom_possibilites_multiple_addresses_and_topics() {
        let filter = Filter {
            from_block: BlockTag::Tag(Tag::Earliest),
            to_block: BlockTag::Tag(Tag::Latest),
            address: Some(vec![
                "b372018f3be9e171df0581136b59d2faf73a7d5d".into(),
                "b372018f3be9e171df0581136b59d2faf73a7d5d".into(),
            ]),
            topics: vec![
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                None,
            ],
            limit: None,
        };

        // number of possibilites should be equal 2 * 2 * 2 * 1 = 8
        let possibilities = filter.bloom_possibilities();
        let blooms: Vec<LogBloom> = vec![
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
            "0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000008
             0000000000000000000000000000000000000000800000000000000000000000
             0000000000000000000000000000000000000000000000400000000400000000
             0000000000000000000000000000000000000000020000000000000000000000
             0000000000000000000000000000000000000000000000000000000000000000
             0000000000000000000000000000000000000000000000000000040000000000
             0000000000000000000000000000000000000000000000000000000000000000"
                .into(),
        ];
        assert_eq!(possibilities, blooms);
    }

    #[test]
    fn test_filter_matches() {
        let filter = Filter {
            from_block: BlockTag::Tag(Tag::Earliest),
            to_block: BlockTag::Tag(Tag::Latest),
            address: Some(vec!["b372018f3be9e171df0581136b59d2faf73a7d5d".into()]),
            topics: vec![
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                ]),
                Some(vec![
                    "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23fa".into(),
                ]),
                None,
                None,
            ],
            limit: None,
        };

        let entry0 = LogEntry {
            address: "b372018f3be9e171df0581136b59d2faf73a7d5d".into(),
            topics: vec![
                "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23fa".into(),
                "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
            ],
            data: vec![],
        };

        let entry1 = LogEntry {
            address: "b372018f3be9e171df0581136b59d2faf73a7d5e".into(),
            topics: vec![
                "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
                "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23fa".into(),
                "ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into(),
            ],
            data: vec![],
        };

        let entry2 = LogEntry {
            address: "b372018f3be9e171df0581136b59d2faf73a7d5d".into(),
            topics: vec!["ff74e91598aed6ae5d2fdcf8b24cd2c7be49a0808112a305069355b7160f23f9".into()],
            data: vec![],
        };

        assert_eq!(filter.matches(&entry0), true);
        assert_eq!(filter.matches(&entry1), false);
        assert_eq!(filter.matches(&entry2), false);
    }
}
