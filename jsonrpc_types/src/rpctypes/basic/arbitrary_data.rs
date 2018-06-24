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

use rustc_serialize::hex::FromHex;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use cita_types::traits::LowerHex;

/// Arbitrary length bytes (wrapper structure around vector of bytes).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Data(Vec<u8>);

impl Data {
    pub fn new(data: Vec<u8>) -> Data {
        Data(data)
    }
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.lower_hex_with_0x().as_ref())
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DataVisitor)
    }
}

struct DataVisitor;

impl<'de> Visitor<'de> for DataVisitor {
    type Value = Data;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Data")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.is_empty() {
            Ok(Data::new(Vec::new()))
        } else if value.len() >= 2
            && (&value[0..2] == "0x" || &value[0..2] == "0X")
            && value.len() & 1 == 0
        {
            let data = FromHex::from_hex(&value[2..]).map_err(|_| {
                if value.len() > 12 {
                    E::custom(format!(
                        "invalid hexadecimal string: [{}..{}]",
                        &value[..6],
                        &value[value.len() - 6..value.len()]
                    ))
                } else {
                    E::custom(format!("invalid hexadecimal string: [{}]", value))
                }
            })?;
            Ok(Data::new(data))
        } else {
            if value.len() > 12 {
                Err(E::custom(format!(
                    "invalid format: [{}..{}]",
                    &value[..6],
                    &value[value.len() - 6..value.len()]
                )))
            } else {
                Err(E::custom(format!("invalid format: [{}]", value)))
            }
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Data {
        Data::new(data)
    }
}

impl Into<Vec<u8>> for Data {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Data;
    use rustc_serialize::hex::FromHex;
    use serde_json;

    #[test]
    fn serialize() {
        let data = Data::new("0123456789abcdef".from_hex().unwrap());
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#""0x0123456789abcdef""#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""12""#, None),
            (r#""ab""#, None),
            (r#""0x123""#, None),
            (r#""0xabcdefgh""#, None),
            (r#""""#, Some(Data::new(vec![]))),
            (r#""0x""#, Some(Data::new(vec![]))),
            (r#""0x12""#, Some(Data::new(vec![0x12]))),
            (r#""0X12""#, Some(Data::new(vec![0x12]))),
            (r#""0x0123""#, Some(Data::new(vec![0x1, 0x23]))),
            (r#""0xabcdef""#, Some(Data::new(vec![0xab, 0xcd, 0xef]))),
            (r#""0XABCDEF""#, Some(Data::new(vec![0xab, 0xcd, 0xef]))),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Data, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
