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

#[macro_use]
extern crate serde_derive;
extern crate serde;
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
extern crate bincode;
extern crate parking_lot;
extern crate time;
extern crate engine_json;
extern crate engine;
extern crate lru_cache;
extern crate serde_types;
extern crate dotenv;
extern crate cita_log;

mod core;

use std::sync::mpsc::channel;
use std::thread;
use core::Spec;
use core::handler;
use libproto::*;
use clap::App;
use std::time::{Duration, Instant};
use amqp::{Consumer, Channel, protocol, Basic};
use pubsub::PubSub;
use threadpool::ThreadPool;
use std::sync::mpsc::Sender;
use log::LogLevelFilter;
use serde_types::hash::{H256};

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
        trace!("handle delivery id {:?}", deliver.routing_key);
        handler::receive(&self.pool, &self.tx, key_to_id(&deliver.routing_key), body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}

fn main() {
    dotenv::dotenv().ok();
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "full");

    cita_log::format(LogLevelFilter::Info);
    info!("CITA:consensus:tendermint");

    let matches = App::new("tendermint")
        .version("0.8")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let threadpool = threadpool::ThreadPool::new(10);
    let (tx, rx) = channel();
    let mut pubsub = PubSub::new();
    pubsub.start_sub("consensus",
                     vec!["net.tx", "net.blk", "jsonrpc.new_tx", "net.msg", "chain.status"],
                     MyHandler::new(threadpool, tx));
    let mut _pub = pubsub.get_pub();

    let spec = Spec::new_test_tendermint(config_path);
    let engine = spec.engine;

    let process = engine.clone();
    let _ = thread::Builder::new()
        .name("handler".to_string())
        .spawn(move || loop {
                   let process = process.clone();
                   handler::process(process, &rx, &mut _pub);
               });

    let ready = spec.rx;

    let mut _pub = pubsub.get_pub();
    let seal = engine.clone();
    let dur = seal.duration();
    let mut old_height = 0;
    let mut new_height = 0;
    let mut pre_hash = H256::default();
    let mut try_recv_cnt = 0;
    let _ = thread::Builder::new()
        .name("main_loop".to_string())
        .spawn(move || loop {
                   let seal = seal.clone();
                   trace!("-----------seal worker beigin-----------");
                   loop {
                       let new_status = ready.recv().unwrap();
                       new_height = new_status.0;
                       pre_hash = new_status.1;
                       loop {
                           if let Ok((another_new_height, another_pre_hash)) = ready.try_recv() {
                               if another_new_height > new_height {
                                   trace!("---------update the height from {:?} to {:?}------", new_height, another_new_height);
                                   new_height = another_new_height;
                                   pre_hash = another_pre_hash;
                               }
                               try_recv_cnt = try_recv_cnt + 1;
                           } else {
                               trace!("---------Get tryRecv empty ------");
                               if try_recv_cnt > 1 {
                                   info!("---------try_recv tried {} times to flush the channel to get the latest height", try_recv_cnt);
                               }
                               try_recv_cnt = 0;
                               break;
                           }
                       }

                       if new_height > old_height {
                           old_height = new_height;
                           break;
                       }
                   }
                   seal.set_new_status(new_height, pre_hash);
                   info!("-----------seal worker go next {}-----------", new_height);

                   let now = Instant::now();
                   seal.new_block(&mut _pub);
                   let elapsed = now.elapsed();
                   trace!("seal worker next elapsed {:?}", elapsed);
                   if new_height != 0 {
                       if let Some(dur1) = dur.checked_sub(elapsed) {
                           trace!("-----------seal worker sleep {:?}-----------", dur1);
                           thread::sleep(dur1);
                       }
                   }
               });

    loop {
        thread::sleep(Duration::from_millis(10000));
    }
}
