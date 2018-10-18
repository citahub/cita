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

//! ## Summary
//!
//! One of the CITA's core components is used to implement the peer-to-peer network
//! and provide point-to-point connection interaction.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     |       Queue       | PubModule | Message Type   |
//!     | ----------------- | --------- | -------------- |
//!     | network_tx        | Auth      | Request        |
//!     | network_consensus | Consensus | SignedProposal |
//!     | network_consensus | Consensus | RawBytes       |
//!     | network           | Chain     | Status         |
//!     | network           | Chain     | syncResponse   |
//!     | network           | Jonsonrpc | RequestNet     |
//!
//! 2. Publish channel
//!
//!     |       Queue       | PubModule | SubModule           | Message Type   |
//!     | ----------------- | --------- | ------------------- | -------------- |
//!     | network           | Net       | Chain, Executor     | SyncResponse   |
//!     | network           | Net       | Snapshot            | SnapshotResp   |
//!     | network           | Net       | Jsonrpc             | Response       |
//!     | network_tx        | Net       | Auth                | Request        |
//!     | network_consensus | Net       | Executor, Consensus | SignedProposal |
//!     | network_consensus | Net       | Consensus           | RawBytes       |
//!
//! ### p2p binary protocol
//! | Start      | Full length | Key length | Key value      | Message value    |
//! | ---------- | ----------- | ---------- | -------------- | ---------------- |
//! | \xDEADBEEF | u32         | u8(byte)   | bytes of a str | a serialize data |
//!
//! full_len = 1 + key_len + body_len
//!
//! ### Key behavoir
//!
//! the key struct:
//!
//! - [`Connection`]
//! - [`NetWork`]
//! - [`Synchronizer`]
//!
//! In addition to the `tokio_server`, there is an `Arc<Connection>` for
//! this structure in almost all the threads of this module to confirm that the node is alive,
//! increase or decrease nodes, consensus message broadcasts, authentication message broadcasts,
//! node status broadcasts, synchronization node blocks Height and so on.
//!
//! About binary protocol encoding and decoding, please look at module `citaprotocol`, the fuction
//! [`pubsub_message_to_network_message`] and [`network_message_to_pubsub_message`].
//!
//! [`Connection`]: ./connection/struct.Connection.html
//! [`NetWork`]: ./network/struct.NetWork.html
//! [`Synchronizer`]: ./synchronizer/struct.Synchronizer.html
//! [`pubsub_message_to_network_message`]: ./citaprotocol/fn.pubsub_message_to_network_message.html
//! [`network_message_to_pubsub_message`]: ./citaprotocol/fn.network_message_to_pubsub_message.html
//!

#![feature(try_from)]
#![feature(tool_lints)]

extern crate byteorder;
extern crate bytes;
extern crate clap;
extern crate dotenv;
extern crate futures;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate notify;
extern crate pubsub;
extern crate rand;
#[cfg(test)]
extern crate tempfile;
extern crate tokio;
#[macro_use]
extern crate util;

#[macro_use]
extern crate serde_derive;

pub mod citaprotocol;
pub mod config;
pub mod connection;
pub mod netserver;
pub mod synchronizer;
//pub mod sync_vec;
pub mod network;

use clap::App;
use config::NetConfig;
use connection::{manage_connect, Connections, Task};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use netserver::NetServer;
use network::NetWork;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use pubsub::start_pubsub;
use std::convert::TryFrom;
use std::net::SocketAddr;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use synchronizer::Synchronizer;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-network", "CITA:network");
    info!("Version: {}", get_build_info_str(true));

    // init app
    // todo load config
    let matches = App::new("network")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config");

    let config = NetConfig::new(config_path);

    // init pubsub

    // split new_tx with other msg
    let (ctx_sub_tx, crx_sub_tx) = channel();
    let (ctx_pub_tx, crx_pub_tx) = channel();
    start_pubsub(
        "network_tx",
        routing_key!([Auth >> Request]),
        ctx_sub_tx,
        crx_pub_tx,
    );

    let (ctx_sub_consensus, crx_sub_consensus) = channel();
    let (ctx_pub_consensus, crx_pub_consensus) = channel();
    start_pubsub(
        "network_consensus",
        routing_key!([Consensus >> SignedProposal, Consensus >> RawBytes]),
        ctx_sub_consensus,
        crx_pub_consensus,
    );

    let (ctx_sub, crx_sub) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub(
        "network",
        routing_key!([
            Chain >> Status,
            Chain >> SyncResponse,
            Jsonrpc >> RequestNet,
            Snapshot >> SnapshotReq,
        ]),
        ctx_sub,
        crx_pub,
    );

    let (net_work_tx, net_work_rx) = channel();
    // start server
    // This brings up our server.
    // all server recv msg directly publish to mq
    let address_str = format!("0.0.0.0:{}", config.port.unwrap());
    let address = address_str.parse::<SocketAddr>().unwrap();
    let net_server = NetServer::new(net_work_tx.clone());

    //network server listener
    thread::spawn(move || net_server.server(address));

    //connections manage to loop
    let (tx, rx) = channel();
    let (mut con, task_sender) = Connections::new(&config);
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();
    let _ = watcher.watch(".", RecursiveMode::NonRecursive);

    let (sync_tx, sync_rx) = channel();
    let net_work = NetWork::new(
        task_sender.clone(),
        ctx_pub.clone(),
        sync_tx,
        ctx_pub_tx,
        ctx_pub_consensus,
        con.is_pause.clone(),
        con.connect_number.clone(),
    );
    manage_connect(config_path, rx, task_sender.clone());

    thread::spawn(move || con.run());

    // loop deal data
    thread::spawn(move || loop {
        if let Ok((source, cita_req)) = net_work_rx.recv() {
            net_work.receiver(source, cita_req);
        }
    });

    // Sync loop
    let mut synchronizer = Synchronizer::new(ctx_pub, task_sender.clone());
    thread::spawn(move || loop {
        if let Ok((source, payload)) = sync_rx.recv() {
            synchronizer.receive(source, payload);
        }
    });

    // Subscribe Auth Tx
    let tx_task_sender = task_sender.clone();
    thread::spawn(move || loop {
        let (key, body) = crx_sub_tx.recv().unwrap();
        let msg = Message::try_from(&body).unwrap();
        trace!("Auth Tx from Local");
        tx_task_sender.send(Task::Broadcast((key, msg))).unwrap();
    });

    // Subscribe Consensus Msg
    thread::spawn(move || loop {
        let (key, body) = crx_sub_consensus.recv().unwrap();
        let msg = Message::try_from(&body).unwrap();
        trace!("Consensus Msg from Local");
        task_sender.send(Task::Broadcast((key, msg))).unwrap();
    });

    loop {
        // Msg from MQ need proc before broadcast
        let (key, body) = crx_sub.recv().unwrap();
        trace!("handle delivery from {} payload {:?}", key, body);
        net_work_tx.send((Source::LOCAL, (key, body))).unwrap();
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Source {
    LOCAL,
    REMOTE,
}
