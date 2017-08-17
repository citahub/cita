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

#![allow(unused_must_use)]
extern crate core;
extern crate threadpool;
#[macro_use]
extern crate log;
extern crate libproto;
extern crate amqp;
extern crate pubsub;
extern crate util;
extern crate clap;
extern crate dotenv;
extern crate cita_log;
extern crate jsonrpc_types;
extern crate common_types as types;
extern crate byteorder;
extern crate serde_json;
extern crate protobuf;

mod forward;
mod synchronizer;

use amqp::{Consumer, Channel, protocol, Basic};
use clap::App;
use core::db;
use core::libchain;
use core::libchain::{submodules, key_to_id};
use core::libchain::Genesis;
use forward::*;
use log::LogLevelFilter;
use pubsub::PubSub;
use std::env;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;
use std::time;
use std::time::Duration;
use synchronizer::Synchronizer;
use threadpool::ThreadPool;
use util::kvdb::{Database, DatabaseConfig};

pub const DATA_PATH: &'static str = "DATA_PATH";

pub struct MyHandler {
    pool: ThreadPool,
    tx: Sender<(u32, u32, u32, MsgClass)>,
}

impl MyHandler {
    pub fn new(pool: ThreadPool, tx: Sender<(u32, u32, u32, MsgClass)>) -> Self {
        MyHandler { pool: pool, tx: tx }
    }
}

// TODO: Remove Pool?
impl Consumer for MyHandler {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, _: protocol::basic::BasicProperties, body: Vec<u8>) {
        forward::chain_pool(&self.pool, &self.tx, key_to_id(&deliver.routing_key), body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}

fn main() {
    dotenv::dotenv().ok();

    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "full");
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:chain");
    let matches = App::new("chain")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let (tx, rx) = channel();
    let pool = threadpool::ThreadPool::new(10);
    let mut pubsub = PubSub::new();
    pubsub.start_sub("chain", vec!["net.blk", "net.status", "net.sync", "consensus.blk", "jsonrpc.request"], MyHandler::new(pool, tx));
    let mut _pub = pubsub.get_pub();
    let nosql_path = env::var(DATA_PATH).expect(format!("{} must be set", DATA_PATH).as_str()) + "/nosql";
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &nosql_path).unwrap();
    let genesis = Genesis::init(config_path);
    let (sync_tx, sync_rx) = channel();
    let (chain, st) = libchain::chain::Chain::init_chain(Arc::new(db), genesis, sync_tx);
    let msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, st.write_to_bytes().unwrap());

    info!("init status {:?}, {:?}", st.height, st.hash);
    _pub.publish("chain.status", msg.write_to_bytes().unwrap());
    let synchronizer = Synchronizer::new(chain.clone());
    let chain1 = chain.clone();
    thread::spawn(move || loop {
                      let chain = chain1.clone();
                      forward::chain_result(chain, &rx, &mut _pub);
                  });

    let mut _pub1 = pubsub.get_pub();
    thread::spawn(move || loop {
                      let notify = sync_rx.recv_timeout(Duration::new(8, 0));
                      if notify.is_ok() {
                          synchronizer.sync(&mut _pub1);
                      } else {
                          synchronizer.sync_status(&mut _pub1);
                      }
                  });
    //garbage collect
    let mut i: u32 = 0;
    loop {
        thread::sleep(time::Duration::from_millis(10000));
        if i > 100 {
            chain.collect_garbage();
            i = 0;
        }
        i += 1;
    }
}
