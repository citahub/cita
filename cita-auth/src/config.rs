use serde_json;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub count_per_batch: usize,
    pub buffer_duration: u32,
    pub tx_verify_thread_num: usize,
    pub tx_verify_num_per_thread: usize,
    pub proposal_tx_verify_num_per_thread: usize,
    pub tx_pool_limit: usize,
    pub block_packet_tx_limit: usize,
    pub prof_start: u64,
    pub prof_duration: u64,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let config_file = File::open(path).unwrap();
        let fconfig = BufReader::new(config_file);
        let config: Config = serde_json::from_reader(fconfig).expect("Failed to load auth config.");
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_configure_file() {
        let json = r#"{
          "count_per_batch": 30,
          "buffer_duration": 3000000,
          "tx_verify_thread_num": 10,
          "tx_verify_num_per_thread": 300,
          "proposal_tx_verify_num_per_thread": 30,
          "tx_pool_limit": 50000,
          "block_packet_tx_limit": 30000,
          "prof_start": 0,
          "prof_duration": 0
        }"#;

        let value: Config = serde_json::from_str(json).expect("read Error");
        println!("{:?}", value);
        assert_eq!(30, value.count_per_batch);
        assert_eq!(3000000, value.buffer_duration);
        assert_eq!(10, value.tx_verify_thread_num);
        assert_eq!(300, value.tx_verify_num_per_thread);
        assert_eq!(30, value.proposal_tx_verify_num_per_thread);
        assert_eq!(50000, value.tx_pool_limit);
        assert_eq!(30000, value.block_packet_tx_limit);
        assert_eq!(0, value.prof_start);
        assert_eq!(0, value.prof_duration);
    }
}
