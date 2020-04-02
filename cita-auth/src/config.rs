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

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub count_per_batch: usize,
    pub buffer_duration: u64,
    pub tx_verify_thread_num: usize,
    pub tx_verify_cache_size: usize,
    pub tx_pool_limit: usize,
    pub wal_enable: bool,
}

impl Config {
    pub fn new(path: &str) -> Self {
        parse_config!(Config, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    #[test]
    fn read_configure_file() {
        let toml_str = r#"
        count_per_batch = 30
        buffer_duration = 30
        tx_verify_thread_num = 4
        tx_verify_cache_size = 100000
        tx_pool_limit = 50000
        wal_enable = true
        "#;

        let mut tmpfile: NamedTempFile = NamedTempFile::new().unwrap();
        tmpfile.write_all(toml_str.as_bytes()).unwrap();
        let path = tmpfile.path().to_str().unwrap();
        let value: Config = parse_config!(Config, path);

        assert_eq!(30, value.count_per_batch);
        assert_eq!(30, value.buffer_duration);
        assert_eq!(4, value.tx_verify_thread_num);
        assert_eq!(100000, value.tx_verify_cache_size);
        assert_eq!(50000, value.tx_pool_limit);
        assert_eq!(true, value.wal_enable);
    }
}
