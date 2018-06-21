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

use serde::de::{self, DeserializeOwned};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{self, Value};

/// Variadic value.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
#[serde(untagged)]
pub enum VariadicValue<T>
where
    T: DeserializeOwned + Serialize,
{
    /// None
    Null,
    /// Single
    Single(T),
    /// Multiple
    Multiple(Vec<T>),
}

impl<T> VariadicValue<T>
where
    T: DeserializeOwned + Serialize,
{
    pub fn null() -> Self {
        VariadicValue::Null
    }
    pub fn single(data: T) -> Self {
        VariadicValue::Single(data)
    }
    pub fn multiple(data: Vec<T>) -> Self {
        VariadicValue::Multiple(data)
    }
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

        serde_json::from_value(v.clone())
            .map(VariadicValue::Single)
            .or_else(|_| serde_json::from_value(v).map(VariadicValue::Multiple))
            .map_err(|_| de::Error::custom("invalid type"))
    }
}

#[cfg(test)]
mod tests {
    use super::VariadicValue;
    use serde_json;

    type VariadicU64 = VariadicValue<u64>;

    macro_rules! test_ser_and_de {
        ($type:ty, $json_params:tt, $value:expr) => {
            let data = $value;
            let serialized = serde_json::to_string(&data).unwrap();
            let jsonstr = json!($json_params).to_string();
            assert_eq!(serialized, jsonstr);
            let deserialized: $type = serde_json::from_str(&jsonstr).unwrap();
            assert_eq!(deserialized, data);
        };
    }

    #[test]
    fn serialize_and_deserialize() {
        test_ser_and_de!(VariadicU64, null, VariadicU64::null());
        test_ser_and_de!(VariadicU64, 123, VariadicU64::single(123));
        test_ser_and_de!(VariadicU64, [123], VariadicU64::multiple(vec![123]));
        test_ser_and_de!(VariadicU64, [1, 2, 3], VariadicU64::multiple(vec![1, 2, 3]));
    }
}
