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

mod candidate_pool;
mod dispatch;
mod cmd;
use candidate_pool::*;

use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use std::sync::mpsc::Sender;
use libproto::MsgClass;
use amqp::{Consumer, Channel, protocol, Basic};
use pubsub::PubSub;
use libproto::key_to_id;
use log::LogLevelFilter;


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
        info!("handle delivery id {:?} payload {:?}",
              deliver.routing_key,
              body);
        dispatch::extract(&self.pool,
                          &self.tx,
                          key_to_id(deliver.routing_key.as_str()),
                          body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}


fn main() {
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "1");
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:txpool");
    let (tx, rx) = channel();
    let pool = threadpool::ThreadPool::new(2);
    let mut pubsub = PubSub::new();
    //TODO msg must rewrite
    pubsub.start_sub("consensus",
                     vec!["net.*",
                          "consensus_cmd.default",
                          "consensus.blk",
                          "chain.status",
                          "jsonrpc.new_tx"],
                     MyHandler::new(pool, tx));
    let mut _pub = pubsub.get_pub();

    let (height, hash) = dispatch::wait(&rx);
    let mut candidate_pool = CandidatePool::new(0);
    info!("Initialize height: {:?}, hash: {:?}", height, hash);
    candidate_pool.update_height(height);
    candidate_pool.update_hash(hash);
    candidate_pool.reflect_situation(&mut _pub);

    loop {
        dispatch::dispatch(&mut candidate_pool, &mut _pub, &rx);
    }
}
