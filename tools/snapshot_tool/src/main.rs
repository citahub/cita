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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]
#![allow(deprecated, unused_must_use, unused_mut, unused_assignments)]
#![feature(refcell_replace_swap, try_from)]
extern crate clap;
extern crate dotenv;
extern crate error;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate proof;
extern crate protobuf;
extern crate pubsub;
extern crate serde_json;
#[macro_use]
extern crate util;

mod snapshot_tool;

use clap::App;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use snapshot_tool::SnapShot;
use std::sync::mpsc::channel;
use util::set_panic_handler;

fn main() {
    micro_service_init!("cita-snapshot", "CITA:snapshot");

    let matches = App::new("snapshot")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-m, --cmd=[snapshot] 'snapshot or restore")
        .arg_from_usage("-p, --file=[./snapshot] 'The file of snapshot'")  //snap file path
        .arg_from_usage("-s, --start_height=[0] 'start height'")  //latest or valid ancient block_id
        .arg_from_usage("-e, --end_height=[1000] 'end height'")  //todo remove
        .get_matches();

    let cmd = matches.value_of("cmd").unwrap_or("snapshot");
    let file = matches.value_of("file").unwrap_or("./snapshot");
    let start_height = matches
        .value_of("start_height")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap();
    let end_height = matches
        .value_of("end_height")
        .unwrap_or("1000")
        .parse::<u64>()
        .unwrap();

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();

    let snapshot_instance = SnapShot::new(ctx_pub, start_height, end_height, file.to_string());

    start_pubsub(
        "snapshot",
        routing_key!([
            Net >> SnapshotResp,
            Consensus >> SnapshotResp,
            Executor >> SnapshotResp,
            Chain >> SnapshotResp,
            Auth >> SnapshotResp,
        ]),
        tx,
        crx_pub,
    );

    match cmd {
        "snapshot" => {
            snapshot_instance.clone().snapshot();
            println!("snapshot_tool send snapshot cmd");
        }
        "restore" => {
            snapshot_instance.clone().begin();
            println!("snapshot_tool send restore cmd");
        }
        _ => println!("snapshot_tool send error cmd"),
    }
    let mut exit = false;
    loop {
        if let Ok((key, msg)) = rx.recv() {
            info!("snapshot_tool receive ack key: {:?}", key);
            exit = snapshot_instance.clone().parse_data(key, msg);
        }
        if exit {
            break;
        }
    }
}
