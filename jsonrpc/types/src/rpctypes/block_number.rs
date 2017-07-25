use state::ids::BlockId;
use serde_json;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Visitor;
use std::fmt;
use serde::de::Error;


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
            BlockNumber::Height(ref x) => serializer.serialize_str(&format!("0x{:x}", x)),
            BlockNumber::Tag(ref tag) => {
                serializer.serialize_str(serde_json::to_string(tag).unwrap().as_str())
            }
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    fn deserialize<D>(deserializer: D) -> Result<BlockNumber, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_unit(BlockNumberVisitor)
    }
}

struct BlockNumberVisitor;

impl<'a> Visitor<'a> for BlockNumberVisitor {
    type Value = BlockNumber;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a block number or 'latest', 'earliest'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            "latest" => Ok(BlockNumber::Tag(BlockTag::Latest)),
            "earliest" => Ok(BlockNumber::Tag(BlockTag::Earliest)),
            _ if value.starts_with("0x") => {
                u64::from_str_radix(&value[2..], 16)
                    .map(BlockNumber::Height)
                    .map_err(|_| Error::custom("invalid block number"))
            }
            _ => {
                value.parse::<u64>().map(BlockNumber::Height).map_err(|_| {
                    Error::custom("invalid block number")
                })
            }
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(value.as_ref())
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value {
            _ => Ok(BlockNumber::Height(value)),
        }
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
        let deserialized3: Vec<BlockNumber> = serde_json::from_str(s3).unwrap();
        assert_eq!(
            deserialized3,
            vec![BlockNumber::Height(10), BlockNumber::Tag(BlockTag::Latest)]
        )
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
    fn decimal_number_deserialization() {
        let s3 = r#"["10", "latest"]"#;
        let deserialized3: Vec<BlockNumber> = serde_json::from_str(s3).unwrap();
        assert_eq!(
            deserialized3,
            vec![BlockNumber::Height(10), BlockNumber::Tag(BlockTag::Latest)]
        )
    }
}
