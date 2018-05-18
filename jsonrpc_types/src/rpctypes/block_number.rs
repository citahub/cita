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

use cita_types::clean_0x;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use types::ids::BlockId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash, Eq)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "earliest")]
    Earliest,
}

/// Represents rpc api block height param.
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum BlockNumber {
    /// Latest block
    Tag(BlockTag),
    /// Height
    Height(u64),
}

impl Default for BlockNumber {
    fn default() -> Self {
        BlockNumber::Tag(BlockTag::Latest)
    }
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockNumber::Height(ref x) => serializer.serialize_str(&format!("{:#x}", x)),
            BlockNumber::Tag(ref tag) => serializer.serialize_some(tag),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    fn deserialize<D>(deserializer: D) -> Result<BlockNumber, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(BlockNumberVisitor)
    }
}

struct BlockNumberVisitor;

impl<'a> Visitor<'a> for BlockNumberVisitor {
    type Value = BlockNumber;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a hex block number or 'latest', 'earliest'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            "latest" => Ok(BlockNumber::Tag(BlockTag::Latest)),
            "earliest" => Ok(BlockNumber::Tag(BlockTag::Earliest)),
            _ => {
                let val = clean_0x(value);
                u64::from_str_radix(&val[0..], 16)
                    .map(BlockNumber::Height)
                    .map_err(|_| Error::custom("invalid hex block number"))
            }
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl Into<BlockId> for BlockNumber {
    fn into(self) -> BlockId {
        match self {
            BlockNumber::Height(n) => BlockId::Number(n),
            BlockNumber::Tag(BlockTag::Latest) => BlockId::Latest,
            BlockNumber::Tag(BlockTag::Earliest) => BlockId::Earliest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    #[test]
    fn block_height_deserialization() {
        let s3 = r#"[10, "latest"]"#;
        let deserialized: serde_json::Result<Vec<BlockNumber>> = serde_json::from_str(s3);
        assert!(deserialized.is_err())
    }

    #[test]
    fn lower_hex_number_deserialization() {
        let s3 = r#"["0xA", "latest"]"#;
        let deserialized3: Vec<BlockNumber> = serde_json::from_str(s3).unwrap();
        assert_eq!(
            deserialized3,
            vec![BlockNumber::Height(10), BlockNumber::Tag(BlockTag::Latest)]
        )
    }

    #[test]
    fn upper_hex_number_deserialization() {
        let s3 = r#"["0XA", "latest"]"#;
        let deserialized3: Result<Vec<BlockNumber>, serde_json::Error> = serde_json::from_str(s3);
        assert!(deserialized3.is_err())
    }

    #[test]
    fn hex_number_deserialization() {
        let s3 = r#"["10", "latest"]"#;
        let deserialized3: Vec<BlockNumber> = serde_json::from_str(s3).unwrap();
        assert_eq!(
            deserialized3,
            vec![BlockNumber::Height(16), BlockNumber::Tag(BlockTag::Latest)]
        )
    }

    #[test]
    fn hex_number_serialization() {
        let right = "[\"0x10\",\"latest\"]";
        let left = serde_json::to_string(&vec![
            BlockNumber::Height(16),
            BlockNumber::Tag(BlockTag::Latest),
        ]).unwrap();
        assert_eq!(left, right)
    }

    #[test]
    fn decimal_number_serialization() {
        let right = "[10,\"latest\"]";
        let left = serde_json::to_string(&vec![
            BlockNumber::Height(16),
            BlockNumber::Tag(BlockTag::Latest),
        ]).unwrap();
        assert_ne!(left, right)
    }

    #[test]
    fn decimal_hex_number_serialization() {
        let right = "[10,\"latest\"]";
        let left = serde_json::to_string(&vec![
            BlockNumber::Height(10),
            BlockNumber::Tag(BlockTag::Latest),
        ]).unwrap();
        assert_ne!(left, right)
    }

    #[test]
    fn decimal_into_number_serialization() {
        let right = "[0xa,\"latest\"]";
        let left = serde_json::to_string(&vec![
            BlockNumber::Height(10),
            BlockNumber::Tag(BlockTag::Latest),
        ]).unwrap();
        assert_ne!(left, right)
    }

    #[test]
    fn decimal_into_hex_serialization() {
        let right = "[\"0xa\",\"latest\"]";
        let left = serde_json::to_string(&vec![
            BlockNumber::Height(10),
            BlockNumber::Tag(BlockTag::Latest),
        ]).unwrap();
        assert_eq!(left, right);

        let right = "[\"0x10\",\"latest\"]";
        let left = serde_json::to_string(&vec![
            BlockNumber::Height(16),
            BlockNumber::Tag(BlockTag::Latest),
        ]).unwrap();
        assert_eq!(left, right)
    }
}
