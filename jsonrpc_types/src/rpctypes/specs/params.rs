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

use error::Error;
use serde::de::{DeserializeOwned, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{self, from_value, Value};
use std::fmt;

/// Request parameters
#[derive(Debug, PartialEq, Clone)]
pub enum Params {
    /// Array of values
    Array(Vec<Value>),
    /// Map of values
    Map(serde_json::Map<String, Value>),
    /// No parameters
    None,
}

impl Params {
    /// Parse incoming `Params` into expected types.
    pub fn parse<D>(self) -> Result<D, Error>
    where
        D: DeserializeOwned,
    {
        let value = match self {
            Params::Array(vec) => Value::Array(vec),
            Params::Map(map) => Value::Object(map),
            Params::None => Value::Null,
        };

        from_value(value).map_err(|e| Error::invalid_params(format!("Invalid params: {}.", e)))
    }

    pub fn len(&self) -> usize {
        match *self {
            Params::Array(ref vec) => vec.len(),
            Params::Map(ref map) => map.len(),
            Params::None => 0,
        }
    }
}

impl Serialize for Params {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Params::Array(ref vec) => vec.serialize(serializer),
            Params::Map(ref map) => map.serialize(serializer),
            Params::None => ([0u8; 0]).serialize(serializer),
        }
    }
}

struct ParamsVisitor;

impl<'a> Deserialize<'a> for Params {
    fn deserialize<D>(deserializer: D) -> Result<Params, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_any(ParamsVisitor)
    }
}

impl<'a> Visitor<'a> for ParamsVisitor {
    type Value = Params;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map or sequence")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'a>,
    {
        let mut values = Vec::new();

        while let Some(value) = visitor.next_element()? {
            values.push(value);
        }

        if values.is_empty() {
            Ok(Params::None)
        } else {
            Ok(Params::Array(values))
        }
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'a>,
    {
        let mut values = serde_json::Map::new();

        while let Some((key, value)) = visitor.next_entry()? {
            values.insert(key, value);
        }

        Ok(if values.is_empty() {
            Params::None
        } else {
            Params::Map(values)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Params;
    use error::Error;
    use rpctypes::Filter;
    use serde_json::{self, Map, Number, Value};

    #[test]
    fn params_deserialization() {
        let s = r#"[null, true, -1, 4, 2.3, "hello", [0], {"key": "value"}]"#;
        let deserialized: Params = serde_json::from_str(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));

        assert_eq!(
            Params::Array(vec![
                Value::Null,
                Value::Bool(true),
                Value::from(-1),
                Value::from(4),
                Value::from(2.3),
                Value::String("hello".to_string()),
                Value::Array(vec![Value::from(0)]),
                Value::Object(map),
            ]),
            deserialized
        );
    }

    #[test]
    fn should_return_error() {
        let s = r#"[1, true]"#;
        let params = || serde_json::from_str::<Params>(s).unwrap();
        let v1: Result<(Option<u8>, String), Error> = params().parse();
        let v2: Result<(u8, bool, String), Error> = params().parse();
        assert!(v1.is_err());
        assert!(v2.is_err());
    }

    #[test]
    fn should_parse() {
        let s = r#"["sdasda"]"#;
        let params = serde_json::from_str::<Params>(s).unwrap();
        let v1: Result<(String,), Error> = params.parse();
        assert_eq!(v1, Ok(("sdasda".to_string(),)));
    }

    #[test]
    fn should_parse_object() {
        let s = r#"[{"from":"foo", "to":"bar"}, 99]"#;
        let params = serde_json::from_str::<Params>(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("from".to_string(), Value::String("foo".to_string()));
        map.insert("to".to_string(), Value::String("bar".to_string()));
        let v1: Result<(Map<String, Value>, Value), Error> = params.parse();
        assert_eq!(v1, Ok((map, Value::Number(Number::from(99)))));
    }

    #[test]
    fn should_parse_as_array() {
        let s = r#"[{"from":"foo", "to":"bar"}]"#;
        let params: Params = serde_json::from_str::<Params>(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("from".to_string(), Value::String("foo".to_string()));
        map.insert("to".to_string(), Value::String("bar".to_string()));

        let v1: Value = params.parse().unwrap();
        let arr = v1.as_array().unwrap();

        assert_eq!(arr.get(0), Some(&Value::Object(map)));
        assert_eq!(arr.get(1), None);
        assert_eq!(arr.len(), 1);
    }

    #[test]
    fn should_parse_filter() {
        let s =
            "{\"topics\":[\"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\",\
             \"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\",\
             \"0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3\"]}";
        let deserialized: Filter = serde_json::from_str(s).unwrap();
        println!("deserialized = {:?}", deserialized);

        let filter_str =
            r#"{"topics":["0x8fb1356be6b2a4e49ee94447eb9dcb8783f51c41dcddfe7919f945017d163bf3"]}"#;
        let params = serde_json::from_str::<Params>(filter_str);
        println!("param = {:?}", params);

        let str_filter = serde_json::to_string(params.as_ref().unwrap());
        println!("str_filter = {:?}", str_filter);

        let filter: Filter = serde_json::from_str(filter_str).unwrap();
        println!("filter struct parse = {:?}", filter);

        //TODO
        let filter: Filter = params.unwrap().parse().unwrap();
        println!("params parse filter = {:?}", filter);
    }
}
