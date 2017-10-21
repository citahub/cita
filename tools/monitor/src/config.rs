extern crate toml;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, RustcDecodable, Clone, Default)]
pub struct MonitorConfig {
    pub name: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub pidfile: Option<String>,
    pub logfile: Option<String>,
    pub errfile: Option<String>,
    pub process: Option<Vec<ProcessConfig>>,
}

#[derive(Debug, RustcDecodable, Clone, Default)]
pub struct ProcessConfig {
    pub name: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub pidfile: Option<String>,
    pub logfile: Option<String>,
    pub errfile: Option<String>,
    pub respawn: Option<u32>,
    pub pid: Option<u32>,
    pub respawns: Option<u32>,
}

impl MonitorConfig {
    pub fn new(path: &str) -> Self {
        let config_file = File::open(path).unwrap();
        let mut fconfig = BufReader::new(config_file);
        let mut content = String::new();
        fconfig.read_to_string(&mut content).unwrap();
        toml::decode_str(&content).unwrap()
    }
}
