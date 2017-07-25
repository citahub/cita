// In order to use Serde we need to enable these nightly features.
#![feature(plugin)]
#![feature(custom_derive)]
#![allow(unused_must_use)]

extern crate libraft; // <--- Kind of a big deal for this!
extern crate docopt;
extern crate serde_json;
extern crate rustc_serialize;
extern crate serde;
extern crate mio;
#[macro_use]
extern crate serde_derive;

extern crate threadpool;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate scoped_log;
extern crate pubsub;
extern crate amqp;

mod raft_server;
mod machine;
mod log_store;
mod dispatch;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use raft_server::*;
use docopt::Docopt;
use libproto::{parse_msg, MsgClass, key_to_id};
use amqp::{Consumer, Channel, protocol, Basic};
use pubsub::PubSub;


// Using docopt we define the overall usage of the application.
static USAGE: &'static str = "
A replicated mutable hashmap. Operations on the register have serializable
consistency, but no durability (once all register servers are terminated the
map is lost).

Each register server holds a replica of the map, and coordinates with its
peers to update the maps values according to client commands. The register
is available for reading and writing only if a majority of register servers are
available.


Commands:

  server  Starts a key server. Servers must be provided a unique ID and
          address (ip:port) at startup, along with the ID and address of all
          peer servers.

Usage:
  raft server <id> (<node-address>)...
  raft (-h | --help)

Options:
  -h --help   Show a help message.
";


pub struct MyHandler {
    tx: Sender<(u32, u32, MsgClass)>,
}

impl MyHandler {
    pub fn new(tx: Sender<(u32, u32, MsgClass)>) -> Self {
        MyHandler { tx: tx }
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
        let (cmd_id, _, content) = parse_msg(body.as_slice());
        self.tx
            .send((key_to_id(deliver.routing_key.as_str()), cmd_id, content))
            .unwrap();
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}


fn main() {
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    info!("CITA:raft");
    let (tx, rx) = channel();
    let mut pubsub = PubSub::new();
    pubsub.start_sub("consensus_cmd",
                     vec!["consensus.status", "consensus.msg"],
                     MyHandler::new(tx));
    let mut _pub = pubsub.get_pub();

    let (mut server, mut event_loop) = server(&args);
    let actions = server.consensus.init();
    server.execute_actions(&mut event_loop, actions);
    server.set_con(_pub);
    let eventloop_notifix = event_loop.channel();
    thread_handler(rx, eventloop_notifix);
    event_loop.run(&mut server);
}

fn thread_handler(rx: Receiver<(u32, u32, MsgClass)>,
                  notifix: mio::Sender<libraft::NotifyMessage>) {
    thread::spawn(move || loop {
                      dispatch::dispatch(&notifix, &rx);
                  });
}
