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
