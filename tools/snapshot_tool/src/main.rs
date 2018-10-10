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

#![feature(custom_attribute)]
#![feature(refcell_replace_swap, try_from)]
extern crate clap;
extern crate dotenv;
extern crate error;
extern crate fs2;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate pubsub;
#[macro_use]
extern crate util;

mod snapshot_tool;

use clap::App;
use fs2::FileExt;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use snapshot_tool::SnapShot;
use std::fs::{self, OpenOptions};
use std::sync::mpsc::channel;
use util::set_panic_handler;

const SNAPSHOT_FILE: &str = ".cita_snapshot";

fn main() {
    micro_service_init!("cita-snapshot", "CITA:snapshot");

    // Judge whether snapshot_tool have started.
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(SNAPSHOT_FILE)
        .expect("Failed to open lock file");
    f.try_lock_exclusive().expect("snapshot already started.");

    let matches = App::new("snapshot")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-m, --cmd=[snapshot] 'snapshot or restore'")
        .arg_from_usage("-f, --file=[snapshot] 'the file of snapshot'") //snap file path
        .arg_from_usage("-s, --start_height=[0] 'start height'") //latest or valid ancient block_id
        .arg_from_usage("-e, --end_height=[1000] 'end height'") //todo remove
        .get_matches();

    let cmd = matches.value_of("cmd").unwrap_or("snapshot");
    let file = matches.value_of("file").unwrap_or("snapshot");

    let s = matches.value_of("start_height").unwrap_or("0");
    let start_height = if s.starts_with("0x") | s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).unwrap()
    } else {
        u64::from_str_radix(s, 10).unwrap()
    };

    let e = matches.value_of("end_height").unwrap_or("0");
    let end_height = if e.starts_with("0x") | e.starts_with("0X") {
        u64::from_str_radix(&e[2..], 16).unwrap()
    } else {
        u64::from_str_radix(e, 10).unwrap()
    };

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();

    let mut snapshot_instance = SnapShot::new(ctx_pub, start_height, end_height, file.to_string());

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
            snapshot_instance.snapshot();
            println!("snapshot_tool send snapshot cmd");
        }
        "restore" => {
            snapshot_instance.begin();
            println!("snapshot_tool send restore cmd");
        }
        _ => {
            println!("snapshot_tool send error cmd");
            return;
        }
    }
    let mut exit = false;
    loop {
        if let Ok((key, msg)) = rx.recv() {
            info!("snapshot_tool receive ack key: {:?}", key);
            exit = snapshot_instance.parse_data(&key, &msg);
        }
        if exit {
            // Remove the file
            f.unlock().unwrap();
            let _ = fs::remove_file(SNAPSHOT_FILE);
            break;
        }
    }
}
