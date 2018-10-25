// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

#[derive(Debug, Deserialize, Clone)]
pub struct NetConfig {
    pub id_card: Option<u32>,
    pub port: Option<u64>,
    pub peers: Option<Vec<PeerConfig>>,
    pub enable_tls: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PeerConfig {
    pub id_card: Option<u32>,
    pub ip: Option<String>,
    pub port: Option<u64>,
    pub common_name: Option<String>,
}

impl NetConfig {
    pub fn new(path: &str) -> Self {
        parse_config!(NetConfig, path)
    }
}

#[cfg(test)]
mod tests {
    use super::NetConfig;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn basics() {
        let toml_str = r#"
        port = 40000
        enable_tls = true
        [[peers]]
        ip = "127.0.0.1"
        port = 40001
        common_name = "test1.cita"
        [[peers]]
        ip = "127.0.0.1"
        port = 40002
        common_name = "test2.cita"
        "#;

        let mut tmpfile: NamedTempFile = NamedTempFile::new().unwrap();
        tmpfile.write_all(toml_str.as_bytes()).unwrap();
        let path = tmpfile.path().to_str().unwrap();
        let value = parse_config!(NetConfig, path);

        assert_eq!(value.port, Some(40000));
        assert_eq!(value.enable_tls, Some(true));
    }
}
