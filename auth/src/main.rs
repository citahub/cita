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
extern crate cita_log;
extern crate protobuf;
#[macro_use]
extern crate log;
extern crate clap;
extern crate dotenv;
extern crate pubsub;
extern crate cpuprofiler;
extern crate libproto;
extern crate cita_crypto;

pub mod handler;
pub mod verifier;

use clap::App;
use cpuprofiler::PROFILER;
use dotenv::dotenv;
use handler::handle_msg;
use log::LogLevelFilter;
use pubsub::start_pubsub;
use std::env;
use std::sync::mpsc::channel;
use std::thread;

fn profifer(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
    let start = flag_prof_start;
    let duration = flag_prof_duration;
    thread::spawn(move || {
                      thread::sleep(std::time::Duration::new(start, 0));
                      PROFILER.lock().unwrap().start("./auth.profiler").expect("Couldn't start");
                      thread::sleep(std::time::Duration::new(duration, 0));
                      PROFILER.lock().unwrap().stop().unwrap();
                  });

}

fn main() {
    dotenv().ok();
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");

    // Init logger
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:auth");
    // init app
    let matches = App::new("auth")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("--prof-start=[0] 'Specify the start time of profiling, zero means no profiling'")
        .args_from_usage("--prof-duration=[0] 'Specify the duration for profiling, zero means no profiling'")
        .get_matches();

    let flag_prof_start = matches.value_of("prof-start").unwrap_or("0").parse::<u64>().unwrap();
    let flag_prof_duration = matches.value_of("prof-duration").unwrap_or("0").parse::<u64>().unwrap();

    profifer(flag_prof_start, flag_prof_duration);

    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("auth", vec!["*.verify_req", "*.verify_req_batch", "chain.status"], tx_sub, rx_pub);

    tx_pub.send(("auth.verify_req".to_string(), vec![0])).unwrap();


    loop {
        let (key, msg) = rx_sub.recv().unwrap();
        info!("get {} : {:?}", key, msg);
        handle_msg(msg,tx_pub );
    }
}
