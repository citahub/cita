// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hyper::header::{HeaderMap, HeaderName, HeaderValue, InvalidHeaderValue};

pub const ORIGIN_ANY_STR: &str = "*";
pub const ORIGIN_NULL_STR: &str = "null";

// values come from hyper 0.11
pub const CONTENT_TYPE_PLAIN_TEXT_STR: &str = "text/plain; charset=utf-8";
pub const CONTENT_TYPE_JSON_STR: &str = "application/json";

pub const X_REQUESTED_WITH_STR: &str = "x-requested-with";

pub trait SafeHeaderValue {}

// NOTE: HeaderValue from these constants are checked, can directly unwrap the result.
impl SafeHeaderValue for hyper::Method {}
impl SafeHeaderValue for HeaderName {}

pub trait HeaderValueExt<T> {
    fn from_vec(values: Vec<T>) -> HeaderValue;
}

impl<T> HeaderValueExt<T> for HeaderValue
where
    T: AsRef<str> + SafeHeaderValue,
{
    fn from_vec(values: Vec<T>) -> HeaderValue {
        let joined_str = values
            .iter()
            .map(AsRef::<str>::as_ref)
            .collect::<Vec<&str>>()
            .join(", ");

        HeaderValue::from_str(&joined_str).unwrap()
    }
}

pub trait HeaderMapExt<T> {
    fn insert_vec(&mut self, name: HeaderName, values: Vec<T>) -> Option<HeaderValue>;
}

impl<T> HeaderMapExt<T> for HeaderMap
where
    T: AsRef<str> + SafeHeaderValue,
{
    fn insert_vec(&mut self, name: HeaderName, values: Vec<T>) -> Option<HeaderValue> {
        self.insert(name, HeaderValue::from_vec(values))
    }
}

pub struct Origin;

impl Origin {
    pub fn from_config(config: &Option<String>) -> Result<HeaderValue, InvalidHeaderValue> {
        match config.as_ref().map(|s| s.trim()) {
            Some("*") => Ok(Self::any()),
            None | Some("") | Some("null") => Ok(Self::null()),
            Some(origin) => Self::from_str(origin),
        }
    }

    pub fn from_str(value: &str) -> Result<HeaderValue, InvalidHeaderValue> {
        println!("{:?}", value);
        HeaderValue::from_str(value)
    }

    pub fn any() -> HeaderValue {
        HeaderValue::from_static(ORIGIN_ANY_STR)
    }

    pub fn null() -> HeaderValue {
        HeaderValue::from_static(ORIGIN_NULL_STR)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_content_type_plain_text() {
        assert_eq!(
            HeaderValue::from_static(CONTENT_TYPE_PLAIN_TEXT_STR)
                .to_str()
                .unwrap(),
            CONTENT_TYPE_PLAIN_TEXT_STR
        );
    }

    #[test]
    fn test_content_type_json() {
        assert_eq!(
            HeaderValue::from_static(CONTENT_TYPE_JSON_STR)
                .to_str()
                .unwrap(),
            CONTENT_TYPE_JSON_STR
        );
    }

    #[test]
    fn test_header_name_x_requested_with_str() {
        assert_eq!(
            HeaderName::from_static(X_REQUESTED_WITH_STR).as_str(),
            X_REQUESTED_WITH_STR
        );
    }

    #[test]
    fn test_header_value_ext_blanket_impl() {
        use hyper::{header::*, Method};

        let hv = HeaderValue::from_vec(vec![Method::POST, Method::OPTIONS]);
        assert_eq!(hv.to_str().unwrap(), "POST, OPTIONS");

        let x_requestd_with = HeaderName::from_static(X_REQUESTED_WITH_STR);
        let hv = HeaderValue::from_vec(vec![ORIGIN, x_requestd_with, ACCEPT]);
        assert_eq!(hv.to_str().unwrap(), "origin, x-requested-with, accept");

        let hv = HeaderValue::from_vec(Vec::<Method>::new());
        assert_eq!(hv.to_str().unwrap(), "");
    }

    #[test]
    fn test_header_map_ext_blanket_impl() {
        use hyper::{header::*, Method};

        let mut headers = HeaderMap::new();
        assert!(headers.is_empty());

        let allow_methods = vec![Method::POST, Method::GET];
        let ret = headers.insert_vec(ACCESS_CONTROL_ALLOW_METHODS, allow_methods.clone());
        assert_eq!(ret, None);
        assert_eq!(
            headers.get(ACCESS_CONTROL_ALLOW_METHODS),
            Some(&HeaderValue::from_vec(allow_methods.clone()))
        );

        let ret = headers.insert_vec(ACCESS_CONTROL_ALLOW_METHODS, vec![Method::PATCH]);
        assert_eq!(ret, Some(HeaderValue::from_vec(allow_methods)));
        assert_eq!(
            headers.get(ACCESS_CONTROL_ALLOW_METHODS),
            Some(&HeaderValue::from_vec(vec![Method::PATCH]))
        );

        let allow_headers = vec![ORIGIN, ACCEPT];
        let ret = headers.insert_vec(ACCESS_CONTROL_ALLOW_HEADERS, allow_headers.clone());
        assert_eq!(ret, None);
        assert_eq!(
            headers.get(ACCESS_CONTROL_ALLOW_HEADERS),
            Some(&HeaderValue::from_vec(allow_headers))
        );
    }

    #[test]
    fn test_origin_any() {
        assert_eq!(Origin::any(), HeaderValue::from_static("*"));
    }

    #[test]
    fn test_origin_null() {
        assert_eq!(Origin::null(), HeaderValue::from_static("null"));
    }

    #[test]
    fn test_origin_from_str() {
        assert_eq!(
            Origin::from_str("cyber").unwrap(),
            HeaderValue::from_static("cyber")
        );

        assert!(Origin::from_str("\n").is_err());
    }

    #[test]
    fn test_origin_from_config() {
        vec![
            (Some("*"), HeaderValue::from_static("*")),
            (Some(" * "), HeaderValue::from_static("*")),
            (None, HeaderValue::from_static("null")),
            (Some(""), HeaderValue::from_static("null")),
            (Some("   null "), HeaderValue::from_static("null")),
            (Some("abc"), HeaderValue::from_static("abc")),
            (Some(" xyz "), HeaderValue::from_static("xyz")),
        ]
        .into_iter()
        .for_each(|(origin, result)| {
            let origin = origin.map(String::from);
            assert_eq!(Origin::from_config(&origin).unwrap(), result);
        });

        let invalid = String::from_utf8(vec![127]).unwrap();
        assert!(Origin::from_config(&Some(invalid)).is_err());
    }
}
