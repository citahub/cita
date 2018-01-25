use std::fs::File;
use std::io::Read;
use toml;

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
        let mut config_file = File::open(path).unwrap();
        let mut buffer = String::new();
        config_file
            .read_to_string(&mut buffer)
            .expect("Failed to load auth config.");
        toml::from_str(&buffer).unwrap()
    }
}
