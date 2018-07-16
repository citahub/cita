// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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
use serde::de::Error;
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use serde_json::{from_value, Value};

use rpctypes::{BlockNumber, Data20, Data32, VariadicValue};

/// Filter Address
pub type FilterAddress = VariadicValue<Data20>;
/// Topic
pub type Topic = VariadicValue<Data32>;

/// Filter
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    /// From Block
    #[serde(rename = "fromBlock", default, skip_serializing_if = "BlockNumber::is_default")]
    pub from_block: BlockNumber,
    /// To Block
    #[serde(rename = "toBlock", default, skip_serializing_if = "BlockNumber::is_default")]
    pub to_block: BlockNumber,
    /// Address
    pub address: Option<FilterAddress>,
    /// Topics
    pub topics: Option<Vec<Topic>>,
    /// Limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

impl Filter {
    pub fn new(
        from_block: BlockNumber,
        to_block: BlockNumber,
        address: Option<FilterAddress>,
        topics: Option<Vec<Topic>>,
    ) -> Self {
        Filter {
            from_block,
            to_block,
            address,
            topics,
            limit: None,
        }
    }
}

// Results of the filter_changes RPC.
#[derive(Debug, PartialEq, Clone)]
pub enum FilterChanges {
    /// New logs.
    Logs(Vec<Log>),
    /// New hashes (block or transactions)
    Hashes(Vec<Data32>),
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
    use super::{BlockNumber, Data32, Filter, FilterChanges, Log, VariadicValue};
    use cita_types::{H160, H256, U256};
    use serde_json;
    use std::convert::Into;
    use std::str::FromStr;

    macro_rules! test_ser_and_de {
        ($type:tt, $json_params:tt, $value:tt) => {
            let data = $type::new$value;
            let serialized = serde_json::to_value(&data).unwrap();
            let jsonval = json!($json_params);
            assert_eq!(serialized, jsonval);
            let deserialized: $type = serde_json::from_str(&jsonval.to_string()).unwrap();
            assert_eq!(deserialized, data);
        };
    }

    #[test]
    fn serialize_and_deserialize() {
        test_ser_and_de!(
            Filter,
            {
                "fromBlock": "0xa",
                "address": "0x0000000000000000000000000000000000000010",
                "topics": [
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                    [
                        "0x0000000000000000000000000000000000000000000000000000000000000002",
                        "0x0000000000000000000000000000000000000000000000000000000000000003",
                    ],
                    null,
                ]
            },
            (
                BlockNumber::new(10u64.into()),
                BlockNumber::latest(),
                Some(VariadicValue::single(H160::from(16).into())),
                Some(
                    vec![
                        VariadicValue::single(H256::from(1).into()),
                        VariadicValue::multiple(
                            vec![
                                H256::from(2).into(),
                                H256::from(3).into(),
                            ]),
                        VariadicValue::null(),
                    ]
                ),
            )
        );
    }

    #[test]
    fn test_filter_changes_serde() {
        assert_eq!("[]", serde_json::to_string(&FilterChanges::Empty).unwrap());
        assert_eq!(FilterChanges::Empty, serde_json::from_str("[]").unwrap());

        assert_eq!(
            json!(["0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b"]),
            serde_json::to_value(FilterChanges::Hashes(vec![Data32::new(
                "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b".into(),
            )])).unwrap()
        );

        assert_eq!(
            FilterChanges::Hashes(vec![Data32::new(
                "000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b".into(),
            )]),
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
