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

use rpctypes::{BlockTag, Quantity};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum BlockNumber {
    /// Block Tag
    Tag(BlockTag),
    /// Height
    Height(Quantity),
}

impl BlockNumber {
    pub fn new(height: Quantity) -> Self {
        BlockNumber::Height(height)
    }
    pub fn latest() -> Self {
        BlockNumber::Tag(BlockTag::Latest)
    }
    pub fn earliest() -> Self {
        BlockNumber::Tag(BlockTag::Earliest)
    }
    pub fn is_default(&self) -> bool {
        *self == BlockNumber::Tag(BlockTag::Latest)
    }
}

impl Default for BlockNumber {
    fn default() -> Self {
        BlockNumber::Tag(BlockTag::Latest)
    }
}

#[cfg(test)]
mod tests {
    use super::BlockNumber;
    use serde_json;

    #[test]
    fn serialize() {
        let testdata = vec![
            (BlockNumber::new(10u64.into()), Some(r#""0xa""#)),
            (BlockNumber::new(16u64.into()), Some(r#""0x10""#)),
            (BlockNumber::latest(), Some(r#""latest""#)),
            (BlockNumber::earliest(), Some(r#""earliest""#)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result = serde_json::to_string(&data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""a""#, None),
            (r#""0xg""#, None),
            (r#"0xa"#, None),
            (r#"10"#, None),
            (r#"latest"#, None),
            (r#"earliest"#, None),
            (r#""latest""#, Some(BlockNumber::latest())),
            (r#""earliest""#, Some(BlockNumber::earliest())),
            (r#""10""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0x10""#, Some(BlockNumber::new(16u64.into()))),
            (r#""0xa""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0xA""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0Xa""#, Some(BlockNumber::new(10u64.into()))),
            (r#""0XA""#, Some(BlockNumber::new(10u64.into()))),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<BlockNumber, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
