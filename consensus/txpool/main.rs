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
extern crate threadpool;
extern crate tx_pool;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libproto;
extern crate util;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate pubsub;
extern crate amqp;
extern crate cita_log;
extern crate engine;
extern crate dotenv;

mod candidate_pool;
mod dispatch;
mod cmd;
use amqp::{Consumer, Channel, protocol, Basic};
use candidate_pool::*;
use libproto::MsgClass;
use libproto::key_to_id;
use log::LogLevelFilter;
use pubsub::PubSub;
use std::sync::mpsc::Sender;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;

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
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, _: protocol::basic::BasicProperties, body: Vec<u8>) {
        dispatch::extract(&self.pool, &self.tx, key_to_id(deliver.routing_key.as_str()), body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}


fn main() {
    dotenv::dotenv().ok();
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "1");
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:txpool");
    let (tx, rx) = channel();
    let pool = threadpool::ThreadPool::new(2);
    let mut pubsub = PubSub::new();
    //TODO msg must rewrite
    pubsub.start_sub(
        "consensus",
        vec![
            "net.*",
            "consensus_cmd.default",
            "consensus.blk",
            "chain.status",
            "jsonrpc.new_tx",
        ],
        MyHandler::new(pool, tx)
    );
    let mut _pub = pubsub.get_pub();
    let mut candidate_pool = CandidatePool::new(0);
    loop {
        dispatch::dispatch(&mut candidate_pool, &mut _pub, &rx);
    }
}
