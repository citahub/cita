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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum BlockTag {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "earliest")]
    Earliest,
}

#[cfg(test)]
mod tests_blocktag {
    use super::BlockTag;
    use serde_json;

    #[test]
    fn serialize() {
        let serialized = serde_json::to_string(&BlockTag::Latest).unwrap();
        assert_eq!(serialized, r#""latest""#);
        let serialized = serde_json::to_string(&BlockTag::Earliest).unwrap();
        assert_eq!(serialized, r#""earliest""#);
    }

    #[test]
    fn deserialize() {
        let testdata = vec![
            (r#""#, None),
            (r#""""#, None),
            (r#"latest"#, None),
            (r#"earliest"#, None),
            (r#""latest ""#, None),
            (r#"" latest""#, None),
            (r#""lat est""#, None),
            (r#""Latest""#, None),
            (r#""Earliest""#, None),
            (r#""LATEST""#, None),
            (r#""EARLIEST""#, None),
            (r#""latest""#, Some(BlockTag::Latest)),
            (r#""earliest""#, Some(BlockTag::Earliest)),
        ];
        for (data, expected_opt) in testdata.into_iter() {
            let result: Result<BlockTag, serde_json::Error> = serde_json::from_str(data);
            if let Some(expected) = expected_opt {
                assert_eq!(result.unwrap(), expected);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
