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

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A unsigned integer (wrapper structure around bool).
#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
pub struct Boolean(bool);

impl Boolean {
    pub fn new(data: bool) -> Boolean {
        Boolean(data)
    }
}

impl Serialize for Boolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bool(self.0)
    }
}

impl<'de> Deserialize<'de> for Boolean {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bool(BooleanVisitor)
    }
}

struct BooleanVisitor;

impl<'de> Visitor<'de> for BooleanVisitor {
    type Value = Boolean;

    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        formatter.write_str("Boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Boolean::new(value))
    }
}

impl From<bool> for Boolean {
    fn from(data: bool) -> Boolean {
        Boolean::new(data)
    }
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;
    use serde_json;

    #[test]
    fn serialize() {
        let data = Boolean::new(true);
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"true"#);
        let data = Boolean::new(false);
        let serialized = serde_json::to_string(&data).unwrap();
        assert_eq!(serialized, r#"false"#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""""#, None),
            (r#""0""#, None),
            (r#""10""#, None),
            (r#""#, None),
            (r#"a"#, None),
            (r#""True""#, None),
            (r#""False""#, None),
            (r#""TRUE""#, None),
            (r#""FALSE""#, None),
            (r#"true"#, Some(true)),
            (r#"false"#, Some(false)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<Boolean, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), Boolean::new(expected));
            } else {
                assert!(result.is_err());
            }
        }
    }
}
