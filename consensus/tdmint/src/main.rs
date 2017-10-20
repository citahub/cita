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

#![feature(mpsc_select)]
#[macro_use]
extern crate serde_derive;
extern crate libproto;
extern crate util;
extern crate threadpool;
extern crate protobuf;
#[macro_use]
extern crate log;
extern crate clap;
extern crate cita_crypto as crypto;
extern crate proof;
extern crate pubsub;
extern crate bincode;
extern crate engine;
extern crate lru_cache;
extern crate dotenv;
extern crate logger;
extern crate cpuprofiler;
extern crate authority_manage;
extern crate engine_json;

use clap::App;
use std::sync::mpsc::channel;
use std::thread;

mod core;
use core::spec::Spec;
use core::tendermint::TenderMint;
use core::votetime::WaitTimer;
use cpuprofiler::PROFILER;
use libproto::{parse_msg, key_to_id};
use pubsub::start_pubsub;
use util::panichandler::set_panic_handler;

const THREAD_POOL_NUM: usize = 10;

fn profifer(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
    let start = flag_prof_start;
    let duration = flag_prof_duration;
    thread::spawn(move || {
                      thread::sleep(std::time::Duration::new(start, 0));
                      PROFILER.lock().unwrap().start("./tdmint.profiler").expect("Couldn't start");
                      thread::sleep(std::time::Duration::new(duration, 0));
                      PROFILER.lock().unwrap().stop().unwrap();
                  });

}

fn main() {
    dotenv::dotenv().ok();
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "full");

    //exit process when panic
    set_panic_handler();

    logger::init();
    info!("CITA:consensus:tendermint");

    let matches = App::new("tendermint")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .args_from_usage("--prof-start=[0] 'Specify the start time of profiling, zero means no profiling'")
        .args_from_usage("--prof-duration=[0] 'Specify the duration for profiling, zero means no profiling'")
        .get_matches();

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let flag_prof_start = matches.value_of("prof-start").unwrap_or("0").parse::<u64>().unwrap();
    let flag_prof_duration = matches.value_of("prof-duration").unwrap_or("0").parse::<u64>().unwrap();

    profifer(flag_prof_start, flag_prof_duration);
    // timer module
    let (main2timer, timer4main) = channel();
    let (timer2main, main4timer) = channel();
    let timethd = thread::spawn(move || {
                                    let wt = WaitTimer::new(timer2main, timer4main);
                                    wt.start();
                                });

    //mq pubsub module
    let threadpool = threadpool::ThreadPool::new(THREAD_POOL_NUM);
    let (mq2main, main4mq) = channel();
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("consensus", vec!["net.msg", "chain.richstatus", "auth.block_txs", "verify_blk_consensus"], tx_sub, rx_pub);
    thread::spawn(move || loop {
                      let (key, body) = rx_sub.recv().unwrap();
                      let tx = mq2main.clone();
                      let pool = threadpool.clone();
                      pool.execute(move || {
                                       let (cmd_id, _, content) = parse_msg(body.as_slice());
                                       tx.send((key_to_id(&key), cmd_id, content)).unwrap();
                                   });
                  });

    //main tendermint loop module
    let spec = Spec::new_test_tendermint(config_path);
    info!("main loop start **** ");
    let mainthd = thread::spawn(move || {
                                    let mut engine = TenderMint::new(tx_pub, main4mq, main2timer, main4timer, spec.params);
                                    engine.start();
                                });

    /*let mut log = Wal::new("./yubo").unwrap();
    log.save("abcdefgh".to_string().into_bytes()).unwrap();
    log.save("1234567890".to_string().into_bytes()).unwrap();
    log.load();*/

    mainthd.join().unwrap();
    timethd.join().unwrap();
}
