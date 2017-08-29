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

// In order to use Serde we need to enable these nightly features.
#![feature(plugin)]
#![feature(custom_derive)]
#![allow(unused_must_use)]

extern crate libraft; // <--- Kind of a big deal for this!
extern crate docopt;
extern crate serde_json;
extern crate rustc_serialize;
extern crate mio;
#[macro_use]
extern crate serde_derive;
extern crate libproto;
#[macro_use]
extern crate log;
#[macro_use]
extern crate scoped_log;
extern crate pubsub;
extern crate dotenv;
extern crate cita_log;

mod raft_server;
mod machine;
mod log_store;
mod dispatch;

use docopt::Docopt;
use libproto::{parse_msg, MsgClass, key_to_id};
use log::LogLevelFilter;
use pubsub::start_pubsub;
use raft_server::*;
use std::sync::mpsc::{channel, Receiver};
use std::thread;


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

fn main() {
    dotenv::dotenv().ok();
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "1");
    cita_log::format(LogLevelFilter::Info);
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    info!("CITA:raft");
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    let (tx, rx) = channel();
    start_pubsub("consensus_cmd", vec!["chain.status", "consensus.default"], tx_sub, rx_pub);
    thread::spawn(move || loop {
                      let (key, body) = rx_sub.recv().unwrap();
                      let (cmd_id, _, content) = parse_msg(body.as_slice());
                      tx.send((key_to_id(&key), cmd_id, content)).unwrap();
                  });

    let (mut server, mut event_loop) = server(&args);
    let actions = server.consensus.init();
    server.execute_actions(&mut event_loop, actions);
    server.set_pub(tx_pub);
    let eventloop_notifix = event_loop.channel();
    thread_handler(rx, eventloop_notifix);
    event_loop.run(&mut server);
}

fn thread_handler(rx: Receiver<(u32, u32, MsgClass)>, notifix: mio::Sender<libraft::NotifyMessage>) {
    thread::spawn(move || loop {
                      dispatch::dispatch(&notifix, &rx);
                  });
}
