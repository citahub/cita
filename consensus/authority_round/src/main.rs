extern crate serde;
extern crate serde_json;
extern crate libproto;
extern crate util;
extern crate threadpool;
extern crate rustc_serialize;
extern crate sha3;
extern crate protobuf;
#[macro_use]
extern crate log;
extern crate clap;
extern crate tx_pool;
extern crate cita_crypto as crypto;
extern crate proof;
extern crate amqp;
extern crate pubsub;
extern crate engine_json;
extern crate engine;
extern crate parking_lot;
extern crate cpuprofiler;
extern crate serde_types;
extern crate cita_log;
extern crate dotenv;

pub mod core;

use std::sync::mpsc::channel;
use std::thread;
use core::handler;
use core::Spec;
use libproto::*;
use clap::App;
use std::time::{Duration, Instant};
use amqp::{Consumer, Channel, protocol, Basic};
use pubsub::PubSub;
use threadpool::ThreadPool;
use std::sync::mpsc::Sender;
use log::LogLevelFilter;
use cpuprofiler::PROFILER;


pub struct MyHandler {
    pool: ThreadPool,
    tx: Sender<(u32, u32, u32, MsgClass)>,
}

impl MyHandler {
    pub fn new(pool: ThreadPool, tx: Sender<(u32, u32, u32, MsgClass)>) -> Self {
        MyHandler { pool: pool, tx: tx }
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self,
                       channel: &mut Channel,
                       deliver: protocol::basic::Deliver,
                       _: protocol::basic::BasicProperties,
                       body: Vec<u8>) {
        trace!("handle delivery id {:?} payload {:?}",
               deliver.routing_key,
               body);
        handler::receive(&self.pool, &self.tx, key_to_id(&deliver.routing_key), body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}

fn main() {
    dotenv::dotenv().ok();
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "1");

    cita_log::format(LogLevelFilter::Info);
    println!("CITA:consensus:poa");

    let matches = App::new("authority_round")
        .version("0.8")
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
    let mut pubsub = PubSub::new();
    pubsub.start_sub("consensus",
                     vec!["net.tx",
                          "net.blk",
                          "jsonrpc.new_tx",
                          "net.msg",
                          "chain.status"],
                     MyHandler::new(threadpool, tx));
    let mut _pub = pubsub.get_pub();


    let spec = Spec::new_test_round(config_path);
    let engine = spec.engine;
    let ready = spec.rx;

    let process = engine.clone();
    thread::spawn(move || loop {
                      let process = process.clone();
                      handler::process(process, &rx, &mut _pub);
                  });

    let mut _pub1 = pubsub.get_pub();
    let seal = engine.clone();
    let dur = engine.duration();
    let mut old_height = 0;
    let mut new_height = 0;
    thread::spawn(move || loop {
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
                      handler::seal(seal, &mut _pub1);
                      let elapsed = now.elapsed();
                      if let Some(dur1) = dur.checked_sub(elapsed) {
                          trace!("seal worker sleep !!!!!{:?}", dur1);
                          thread::sleep(dur1);
                      }
                  });

    loop {
        thread::sleep(Duration::from_millis(10000));
    }
}
