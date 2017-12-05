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

extern crate authority_manage;
extern crate cita_crypto as crypto;
extern crate clap;
extern crate cpuprofiler;
extern crate dotenv;
extern crate engine;
extern crate engine_json;
extern crate error;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate proof;
extern crate protobuf;
extern crate pubsub;
extern crate rustc_serialize;
extern crate serde_json;
extern crate threadpool;
extern crate tx_pool;
extern crate util;

pub mod core;

use clap::App;
use core::Spec;
use core::handler;
use cpuprofiler::PROFILER;
use libproto::*;
use pubsub::start_pubsub;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};
use util::panichandler::set_panic_handler;

fn main() {
    dotenv::dotenv().ok();
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "full");

    //exit process when panic
    set_panic_handler();

    logger::init_config("cita-consensus");
    println!("CITA:consensus:poa");

    let matches = App::new("authority_round")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .args_from_usage("--prof-start=[TIME] 'Sets profiling start time (second from app start)'")
        .args_from_usage("--prof-duration=[DURATION] 'Sets duration(second) of profiling'")
        .get_matches();

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    //start profiling
    let mut prof_start: u64 = 0;
    let mut prof_duration: u64 = 0;
    if let Some(start) = matches.value_of("prof-start") {
        trace!("Value for prof-start: {}", start);
        prof_start = start.parse::<u64>().unwrap();
    }
    if let Some(duration) = matches.value_of("prof-duration") {
        trace!("Value for prof-duration: {}", duration);
        prof_duration = duration.parse::<u64>().unwrap();
    }
    if prof_start != 0 && prof_duration != 0 {
        thread::spawn(move || {
            thread::sleep(Duration::new(prof_start, 0));
            println!("******Profiling Start******");
            PROFILER
                .lock()
                .unwrap()
                .start("./consensus_poa.profile")
                .expect("Couldn't start");
            thread::sleep(Duration::new(prof_duration, 0));
            println!("******Profiling Stop******");
            PROFILER.lock().unwrap().stop().unwrap();
        });
    }

    let threadpool = threadpool::ThreadPool::new(2);
    let (tx, rx) = channel();
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub(
        "consensus",
        vec!["net.tx", "jsonrpc.new_tx", "net.msg", "chain.richstatus"],
        tx_sub,
        rx_pub,
    );
    thread::spawn(move || {
        loop {
            let (key, body) = rx_sub.recv().unwrap();
            let tx = tx.clone();
            handler::receive(&threadpool, &tx, key_to_id(&key), body);
        }
    });
    let spec = Spec::new_test_round(config_path);
    let engine = spec.engine;
    let ready = spec.rx;

    let process = engine.clone();
    let tx_pub1 = tx_pub.clone();
    thread::spawn(move || {
        loop {
            let process = process.clone();
            handler::process(process, &rx, tx_pub1.clone());
        }
    });

    let seal = engine.clone();
    let dur = engine.duration();
    let mut old_height = 0;
    let mut new_height = 0;
    let tx_pub = tx_pub.clone();
    thread::spawn(move || {
        loop {
            let seal = seal.clone();
            trace!("seal worker lock!");
            loop {
                new_height = ready.recv().unwrap();
                if new_height > old_height {
                    old_height = new_height;
                    break;
                }
            }
            trace!("seal worker go {}!!!", new_height);
            let now = Instant::now();
            trace!("seal worker ready!");
            handler::seal(seal, tx_pub.clone());
            let elapsed = now.elapsed();
            if let Some(dur1) = dur.checked_sub(elapsed) {
                trace!("seal worker sleep !!!!!{:?}", dur1);
                thread::sleep(dur1);
            }
        }
    });

    loop {
        thread::sleep(Duration::from_millis(10000));
    }
}
