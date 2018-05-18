// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use super::Log;
use cita_types::{Address, H256};
use rpctypes::block_number::BlockNumber;
use serde::de::DeserializeOwned;
use serde::de::Error;
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use serde_json::{from_value, Value};
use types::filter::Filter as EthFilter;
use types::ids::BlockId;

/// Variadic value
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[serde(untagged)]
pub enum VariadicValue<T>
where
    T: DeserializeOwned + Serialize,
{
    /// Single
    Single(T),
    /// List
    Multiple(Vec<T>),
    /// None
    Null,
}

impl<'de, T> Deserialize<'de> for VariadicValue<T>
where
    T: DeserializeOwned + Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<VariadicValue<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;

        if v.is_null() {
            return Ok(VariadicValue::Null);
        }

        from_value(v.clone())
            .map(VariadicValue::Single)
            .or_else(|_| from_value(v).map(VariadicValue::Multiple))
            .map_err(|_| D::Error::custom("Invalid type."))
    }
}

/// Filter Address
pub type FilterAddress = VariadicValue<Address>;
/// Topic
pub type Topic = VariadicValue<H256>;

/// Filter
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    /// From Block
    #[serde(rename = "fromBlock")]
    pub from_block: Option<BlockNumber>,
    /// To Block
    #[serde(rename = "toBlock")]
    pub to_block: Option<BlockNumber>,
    /// Address
    pub address: Option<FilterAddress>,
    /// Topics
    pub topics: Option<Vec<Topic>>,
    /// Limit
    pub limit: Option<usize>,
}

impl Into<EthFilter> for Filter {
    fn into(self) -> EthFilter {
        EthFilter {
            from_block: self.from_block.map_or_else(|| BlockId::Latest, Into::into),
            to_block: self.to_block.map_or_else(|| BlockId::Latest, Into::into),
            address: self.address.and_then(|address| match address {
                VariadicValue::Null => None,
                VariadicValue::Single(a) => Some(vec![a]),
                VariadicValue::Multiple(a) => Some(a.into_iter().map(Into::into).collect()),
            }),
            topics: {
                let mut iter = self
                    .topics
                    .map_or_else(Vec::new, |topics| {
                        topics
                            .into_iter()
                            .take(4)
                            .map(|topic| match topic {
                                VariadicValue::Null => None,
                                VariadicValue::Single(t) => Some(vec![t]),
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
            limit: self.limit,
        }
    }
}

// Results of the filter_changes RPC.
#[derive(Debug, PartialEq, Clone)]
pub enum FilterChanges {
    /// New logs.
    Logs(Vec<Log>),
    /// New hashes (block or transactions)
    Hashes(Vec<H256>),
    /// Empty result,
    Empty,
}

impl Serialize for FilterChanges {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            FilterChanges::Logs(ref logs) => logs.serialize(s),
            FilterChanges::Hashes(ref hashes) => hashes.serialize(s),
            FilterChanges::Empty => (&[] as &[Value]).serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for FilterChanges {
    fn deserialize<D>(deserializer: D) -> Result<FilterChanges, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        match v.clone() {
            Value::Array(filter_change) => {
                if filter_change.len() == 0 {
                    Ok(FilterChanges::Empty)
                } else {
                    from_value(v.clone())
                        .map(FilterChanges::Logs)
                        .or_else(|_| from_value(v).map(FilterChanges::Hashes))
                        .map_err(|_| D::Error::custom("Invalid type."))
                }
            }
            Value::Null => Ok(FilterChanges::Empty),
            _ => Err(D::Error::custom("Invalid type.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Filter, FilterChanges, Log, Topic, VariadicValue};
    use cita_types::{H160, H256, U256};
    use rpctypes::block_number::{BlockNumber, BlockTag};
    use serde_json;
    use std::str::FromStr;
    use types::filter::Filter as EthFilter;
    use types::ids::BlockId;

    #[test]
    fn topic_deserialization() {
        let value = json!([
            "0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b",
            null,
            [
                "0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b",
                "0x0000000000000000000000000aff3454fce5edbc8cca8697c15331677e6ebccc"
            ]
        ]);
        let deserialized: Vec<Topic> = serde_json::from_value(value).unwrap();
        assert_eq!(
            deserialized,
            vec![
                VariadicValue::Single(
                    H256::from_str(
                        "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b",
                    ).unwrap()
                        .into(),
                ),
                VariadicValue::Null,
                VariadicValue::Multiple(vec![
                    H256::from_str(
                        "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b",
                    ).unwrap()
                        .into(),
                    H256::from_str(
                        "0000000000000000000000000aff3454fce5edbc8cca8697c15331677e6ebccc",
                    ).unwrap()
                        .into(),
                ]),
            ]
        );
    }

    #[test]
    fn filter_deserialization() {
        let value = json!({
            "fromBlock":"earliest",
            "toBlock":"latest"
        });
        let deserialized: Filter = serde_json::from_value(value).unwrap();
        assert_eq!(
            deserialized,
            Filter {
                from_block: Some(BlockNumber::Tag(BlockTag::Earliest)),
                to_block: Some(BlockNumber::Tag(BlockTag::Latest)),
                address: None,
                topics: None,
                limit: None,
            }
        );
    }

    #[test]
    fn filter_deserialization2() {
        let value = json!({
            "topics":["0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"]
        });
        let deserialized: Filter = serde_json::from_value(value).unwrap();
        assert_eq!(
            deserialized,
            Filter {
                from_block: None,
                to_block: None,
                address: None,
                topics: Some(vec![VariadicValue::Single(
                    H256::from_str(
                        "8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3",
                    ).unwrap()
                        .into(),
                )]),
                limit: None,
            }
        );
    }

    #[test]
    fn filter_conversion() {
        let filter = Filter {
            from_block: Some(BlockNumber::Tag(BlockTag::Earliest)),
            to_block: Some(BlockNumber::Tag(BlockTag::Latest)),
            address: Some(VariadicValue::Multiple(vec![])),
            topics: Some(vec![
                VariadicValue::Null,
                VariadicValue::Single(
                    H256::from_str(
                        "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b",
                    ).unwrap()
                        .into(),
                ),
                VariadicValue::Null,
            ]),
            limit: None,
        };

        let eth_filter: EthFilter = filter.into();
        assert_eq!(
            eth_filter,
            EthFilter {
                from_block: BlockId::Earliest,
                to_block: BlockId::Latest,
                address: Some(vec![]),
                topics: vec![
                    None,
                    Some(vec![
                        "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b".into(),
                    ]),
                    None,
                    None,
                ],
                limit: None,
            }
        );
    }

    #[test]
    fn test_filter_changes_serde() {
        assert_eq!("[]", serde_json::to_string(&FilterChanges::Empty).unwrap());
        assert_eq!(FilterChanges::Empty, serde_json::from_str("[]").unwrap());

        assert_eq!(
            json!(["0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b"]),
            serde_json::to_value(FilterChanges::Hashes(vec![
                "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b".into(),
            ])).unwrap()
        );

        assert_eq!(
            FilterChanges::Hashes(vec![
                "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b".into(),
            ]),
            serde_json::from_value(json!([
                "0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b"
            ])).unwrap()
        );

        let value = json!([{
            "address":"0x33990122638b9132ca29c723bdf037f1a891a70c",
            "topics":[
                "0xa6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc",
                "0x4861736852656700000000000000000000000000000000000000000000000000"
            ],
            "data":"0x",
            "blockHash":"0xed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5",
            "blockNumber":"0x4510c",
            "transactionHash":"0x0000000000000000000000000000000000000000000000000000000000000000",
            "transactionIndex":"0x0",
            "logIndex":"0x1",
            "transactionLogIndex":"0x1"
        }]);

        let logs = FilterChanges::Logs(vec![Log {
            address: H160::from_str("33990122638b9132ca29c723bdf037f1a891a70c").unwrap(),
            topics: vec![
                H256::from_str("a6697e974e6a320f454390be03f74955e8978f1a6971ea6730542e37b66179bc")
                    .unwrap(),
                H256::from_str("4861736852656700000000000000000000000000000000000000000000000000")
                    .unwrap(),
            ],
            data: vec![].into(),
            block_hash: Some(
                H256::from_str("ed76641c68a1c641aee09a94b3b471f4dc0316efe5ac19cf488e2674cf8d05b5")
                    .unwrap(),
            ),
            block_number: Some(U256::from(0x4510c)),
            transaction_hash: Some(H256::default()),
            transaction_index: Some(U256::default()),
            transaction_log_index: Some(1.into()),
            log_index: Some(U256::from(1)),
        }]);

        let serialized = serde_json::to_value(logs.clone()).unwrap();
        assert_eq!(serialized, value);
        assert_eq!(
            serde_json::from_value::<FilterChanges>(value).unwrap(),
            logs
        );
    }
}
