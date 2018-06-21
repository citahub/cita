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

use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use cita_types::traits::LowerHex;
use cita_types::U256;

/// A big unsigned integer (wrapper structure around U256).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Quantity(U256);

impl Quantity {
    pub fn new(data: U256) -> Quantity {
        Quantity(data)
    }
}

impl Serialize for Quantity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.lower_hex_with_0x().as_ref())
    }
}

impl<'de> Deserialize<'de> for Quantity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(QuantityVisitor)
    }
}

struct QuantityVisitor;

impl<'de> Visitor<'de> for QuantityVisitor {
    type Value = Quantity;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Quantity")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value.len() > 2 && (&value[0..2] == "0x" || &value[0..2] == "0X") {
            let data = U256::from_str(&value[2..]).map_err(|_| {
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
            Ok(Quantity::new(data))
        } else if value.len() > 0 {
            let data = U256::from_dec_str(&value[..]).map_err(|_| {
                if value.len() > 12 {
                    E::custom(format!(
                        "invalid decimal string: [{}..{}]",
                        &value[..6],
                        &value[value.len() - 6..value.len()]
                    ))
                } else {
                    E::custom(format!("invalid decimal string: [{}]", value))
                }
            })?;
            Ok(Quantity::new(data))
        } else {
            Err(E::custom(format!("invalid input: string is empty")))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(value.as_ref())
    }
}

impl<'a> From<&'a [u8]> for Quantity {
    fn from(data: &[u8]) -> Self {
        U256::from_big_endian(data).into()
    }
}

impl From<Vec<u8>> for Quantity {
    fn from(data: Vec<u8>) -> Self {
        Self::from(&data[..])
    }
}

impl Into<Vec<u8>> for Quantity {
    fn into(self) -> Vec<u8> {
        let mut bytes = [0u8; 32];
        self.0.to_big_endian(&mut bytes);
        bytes.to_vec()
    }
}

impl From<U256> for Quantity {
    fn from(data: U256) -> Self {
        Self::new(data)
    }
}

impl Into<U256> for Quantity {
    fn into(self) -> U256 {
        self.0
    }
}

macro_rules! impl_from_and_into_for_small_uint {
    ($utype:ty) => {
        impl From<$utype> for Quantity {
            fn from(data: $utype) -> Quantity {
                Quantity::new(U256::from(data))
            }
        }

        impl Into<$utype> for Quantity {
            fn into(self) -> $utype {
                self.0.low_u64() as $utype
            }
        }
    };
}

impl_from_and_into_for_small_uint!(u64);
impl_from_and_into_for_small_uint!(u32);
impl_from_and_into_for_small_uint!(u16);
impl_from_and_into_for_small_uint!(u8);

#[cfg(test)]
mod tests {
    use super::Quantity;
    use cita_types::U256;
    use serde_json;

    #[test]
    fn serialize() {
        let data = Quantity::new(U256::from(1311768467463790320u64));
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#""0x123456789abcdef0""#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""""#, None),
            (r#""0x""#, None),
            (r#""a""#, None),
            (r#""0xg""#, None),
            (r#""0""#, Some(0u64)),
            (r#""0x0""#, Some(0u64)),
            (r#""10""#, Some(10u64)),
            (r#""0xa""#, Some(10u64)),
            (r#""0xA""#, Some(10u64)),
            (r#""0Xa""#, Some(10u64)),
            (r#""0XA""#, Some(10u64)),
            (r#""010""#, Some(10u64)),
            (r#""0x0a""#, Some(10u64)),
            (r#""00010""#, Some(10u64)),
            (r#""0x000a""#, Some(10u64)),
            (r#""0xabcdef""#, Some(11259375u64)),
            (r#""0XABCDEF""#, Some(11259375u64)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Quantity, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), Quantity::new(U256::from(expected)));
            } else {
                assert!(result.is_err());
            }
        }
    }
}
