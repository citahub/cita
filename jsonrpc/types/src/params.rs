use std::fmt;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{Visitor, SeqAccess, MapAccess};
use serde_json;
use serde_json::value::from_value;
use super::RpcError;

use super::Value;

#[derive(Debug, PartialEq, Clone)]
pub enum Params {
    Array(Vec<Value>),
    Map(serde_json::Map<String, Value>),
    None,
}

impl Params {
    /// lazy parse into expected types.
    pub fn parse<D>(self) -> Result<D, RpcError>
        where for<'de> D: Deserialize<'de>
    {
        let value = match self {
            Params::Array(vec) => Value::Array(vec),
            Params::Map(map) => Value::Object(map),
            Params::None => Value::Null,
        };
        from_value(value).map_err(|_| RpcError::InvalidParams)
    }
}

impl Serialize for Params {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            Params::Array(ref vec) => vec.serialize(serializer),
            Params::Map(ref map) => map.serialize(serializer),
            Params::None => ([0u8; 0]).serialize(serializer),
        }
    }
}

struct ParamsVisitor;

impl<'de> Deserialize<'de> for Params {
    fn deserialize<D>(deserializer: D) -> Result<Params, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_any(ParamsVisitor)
    }
}

impl<'de> Visitor<'de> for ParamsVisitor {
    type Value = Params;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a map or sequence")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: SeqAccess<'de>
    {
        let mut values = Vec::new();
        while let Some(value) = visitor.next_element()? {
            values.push(value);
        }

        Ok(if values.is_empty() {
               Params::None
           } else {
               Params::Array(values)
           })
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where V: MapAccess<'de>
    {
        let mut values = serde_json::Map::with_capacity(visitor.size_hint().unwrap());

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
    use serde_json;
    use super::Params;
    use {Value, RpcError};
    use serde_json::Number;
    use serde_json::Map;

    #[test]
    fn params_deserialization() {

        let s = r#"[null, true, -1, 4, 2.3, "hello", [0], {"key": "value"}]"#;
        let deserialized: Params = serde_json::from_str(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));

        assert_eq!(Params::Array(vec![Value::Null,
                                      Value::Bool(true),
                                      Value::from(-1),
                                      Value::from(4),
                                      Value::from(2.3),
                                      Value::String("hello".to_string()),
                                      Value::Array(vec![Value::from(0)]),
                                      Value::Object(map)]),
                   deserialized);
    }

    #[test]
    fn should_return_error() {
        let s = r#"[1, true]"#;
        let params = || serde_json::from_str::<Params>(s).unwrap();

        let v1: Result<(Option<u8>, String), RpcError> = params().parse();
        let v2: Result<(u8, bool, String), RpcError> = params().parse();
        let err1 = v1.unwrap_err();
        let err2 = v2.unwrap_err();
        assert_eq!(err1, RpcError::InvalidParams);
        assert_eq!(err2, RpcError::InvalidParams);
    }


    #[test]
    fn should_parse() {
        let s = r#"["sdasda"]"#;
        let params = serde_json::from_str::<Params>(s).unwrap();
        let v1: Result<(String,), RpcError> = params.parse();
        assert_eq!(v1, Ok(("sdasda".to_string(),)));
    }

    #[test]
    fn should_parse_object() {
        let s = r#"[{"from":"foo", "to":"bar"}, 99]"#;
        let params = serde_json::from_str::<Params>(s).unwrap();

        let mut map = serde_json::Map::new();
        map.insert("from".to_string(), Value::String("foo".to_string()));
        map.insert("to".to_string(), Value::String("bar".to_string()));

        let v1: Result<(Map<String, Value>, Value), RpcError> = params.parse();
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
}
