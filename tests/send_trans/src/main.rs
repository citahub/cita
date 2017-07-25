extern crate util;
extern crate hyper;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate tabwriter;
extern crate protobuf;
extern crate uuid;
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate now_proto;
extern crate rustc_serialize;
extern crate serde_types;
extern crate cita_log;

mod config;
mod execute;
mod report;
mod transaction;
mod mission;

use config::{build_commandline, parse_arguments};
use execute::run_for_config;
use mission::get_mission;

fn main() {
    info!("Starting ...");
    let matches = build_commandline().get_matches();
    let config = parse_arguments(matches);
    let mission = get_mission(&config);
    let report = run_for_config(config, mission);
    report.print();
    info!("Finished.");
}
