//! Lenient hash json deserialization for test json files.

use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use rustc_serialize::hex::ToHex;
use util::hash::{H64 as Hash64, H160 as Hash160, H256 as Hash256, H512 as Hash512, H520 as Hash520, H2048 as Hash2048};
use std::ops::Deref;

macro_rules! impl_hash {
    ($name: ident, $inner: ident) => {
        /// Lenient hash json deserialization for test json files.
        #[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
        pub struct $name(pub $inner);

        impl From<$name> for $inner {
            fn from(other: $name) -> $inner {
                other.0
            }
        }

        impl From<$inner> for $name {
            fn from(i: $inner) -> Self {
                $name(i)
            }
        }

        impl FromStr for $name {
            type Err = <$inner as FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $inner::from_str(s).map(|x| $name(x))
            }
        }

        impl Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Copy for $name {}

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                struct HashVisitor;

                impl<'de> Visitor<'de> for HashVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("struct Hash")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
                        let value = match value.len() {
                            0 => $inner::from(0),
                            2 if value == "0x" => $inner::from(0),
                            _ if value.starts_with("0x") => $inner::from_str(&value[2..]).map_err(|_| {
                                Error::custom(format!("Invalid hex value {}.", value).as_str())
                            })?,
                            _ => $inner::from_str(value).map_err(|_| {
                                Error::custom(format!("Invalid hex value {}.", value).as_str())
                            })?,
                        };

                        Ok($name(value))
                    }

                    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> where E: Error {
                        self.visit_str(value.as_ref())
                    }
                }

                deserializer.deserialize_str(HashVisitor)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut hex = "0x".to_owned();
                hex.push_str(&self.0.to_hex());
                serializer.serialize_str(&hex)
            }
        }
    }
}

impl_hash!(H64, Hash64);
impl_hash!(H160, Hash160);
impl_hash!(H256, Hash256);
impl_hash!(H512, Hash512);
impl_hash!(H520, Hash520);
impl_hash!(Bloom, Hash2048);

pub type Address = H160;

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use serde_json;
    use util::hash;
    use hash::H256;

    #[test]
    fn hash_deserialization() {
        let s = r#"["", "5a39ed1020c04d4d84539975b893a4e7c53eab6c2965db8bc3468093a31bc5ae"]"#;
        let deserialized: Vec<H256> = serde_json::from_str(s).unwrap();
        assert_eq!(deserialized, vec![
                   H256(hash::H256::from(0)),
                   H256(hash::H256::from_str("5a39ed1020c04d4d84539975b893a4e7c53eab6c2965db8bc3468093a31bc5ae").unwrap())
        ]);
    }

    #[test]
    fn hash_into() {
        assert_eq!(hash::H256::from(0), H256(hash::H256::from(0)).into());
    }
}
