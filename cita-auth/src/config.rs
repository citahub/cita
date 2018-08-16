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

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub count_per_batch: usize,
    pub buffer_duration: u64,
    pub tx_verify_thread_num: usize,
    pub tx_verify_cache_size: usize,
    pub tx_pool_limit: usize,
    pub wal_enable: bool,
    pub prof_start: u64,
    pub prof_duration: u64,
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
        prof_start = 0
        prof_duration = 0
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
        assert_eq!(0, value.prof_start);
        assert_eq!(0, value.prof_duration);
    }
}
