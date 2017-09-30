#![allow(unused_extern_crates)]
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate protobuf;
extern crate util;
extern crate serde;
extern crate hyper;
extern crate serde_json;
extern crate jsonrpc_types;
extern crate rustc_hex;

#[macro_use]
extern crate serde_derive;
extern crate clap;
#[macro_use]
extern crate log;
extern crate logger;


pub mod param;
pub mod trans;
pub mod send_trans;

use clap::App;
use param::Param;
use send_trans::Sendtx;

fn main() {
    logger::init();
    info!("CITA:jsonrpc_performance");
    let matches = App::new("jsonrpc_performance")
        .version("0.1")
        .author("Cryptape")
        .about("CITA jsonrpc performance testing")
        .args_from_usage("--config=[file] 'config file for params'")
        .args_from_usage("--start_h=[1] 'block chain height'")
        .args_from_usage("--analysis=[false] 'is analysis info ?'")
        .get_matches();

    let file_name = matches.value_of("config").unwrap_or("config_test.json");

    let start_h = matches.value_of("start_h").unwrap_or("1").parse::<u64>().unwrap();
    let analysis = matches.value_of("analysis").unwrap_or("false").parse::<bool>().unwrap();
    let p: Param = Param::load_from_file(&file_name);
    let mut work = Sendtx::new(&p, start_h, analysis);
    work.start();

}
