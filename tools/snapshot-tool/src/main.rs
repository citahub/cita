// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;
#[macro_use]
extern crate util;

use crate::postman::Postman;
use clap::App;
use fs2::FileExt;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::channel;
use pubsub::start_pubsub;
use std::fs::{self, File, OpenOptions};
use util::set_panic_handler;

mod postman;

const SNAPSHOT_LOCK: &str = ".cita_snapshot";

fn main() {
    micro_service_init!("cita-snapshot", "CITA:snapshot", false);

    // 1. Aquire client's lock
    let locker = lock();

    // 2. Parse command-line options
    let matches = App::new("snapshot")
        .version("2.0")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-m, --cmd=[snapshot] 'snapshot or restore'")
        .arg_from_usage("-f, --file=[snapshot] 'the file of snapshot'") //snap file path
        .arg_from_usage("-s, --start_height=[0] 'start height'") //latest or valid ancient block_id
        .arg_from_usage("-e, --end_height=[1000] 'end height'") //todo remove
        .get_matches();
    let command = matches.value_of("cmd").expect("provide specific command");
    let command = cast_command(command);
    let file = matches.value_of("file").unwrap_or("snapshot");
    let file = file.to_owned();
    let start_height = matches.value_of("start_height").unwrap_or("0");
    let start_height = cast_height(start_height);
    let end_height = matches.value_of("end_height").unwrap_or("0");
    let end_height = cast_height(end_height);

    // 3. Start message-bus watcher in background
    let (mq_req_sender, mq_req_receiver) = channel::unbounded();
    let (mq_resp_sender, mq_resp_receiver) = channel::unbounded();
    start_pubsub(
        "snapshot",
        routing_key!([
            Net >> SnapshotResp,
            Consensus >> SnapshotResp,
            Executor >> SnapshotResp,
            Chain >> SnapshotResp,
            Auth >> SnapshotResp,
        ]),
        mq_req_sender,
        mq_resp_receiver,
    );

    // 4. Create a postman and start serving
    let mut postman = Postman::new(mq_req_receiver, mq_resp_sender, command, start_height, end_height, file);
    postman.clear_message_bus();
    match postman.serve() {
        Ok(()) => info!("successful to {}", postman.command),
        Err(err) => error!("failed to {}: {:?}", postman.command, err),
    }

    // 5. Release client's lock
    unlock(&locker);
}

fn lock() -> File {
    let locker = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(SNAPSHOT_LOCK)
        .unwrap_or_else(|_| panic!("failed to open lock-file {}", SNAPSHOT_LOCK));
    locker.try_lock_exclusive().expect("snapshot already started.");
    locker
}

fn unlock(locker: &File) {
    locker.unlock().unwrap();
    fs::remove_file(SNAPSHOT_LOCK).expect("failed to release lock-file");
}

fn cast_command(command: &str) -> String {
    assert!(
        command == "snapshot" || command == "restore",
        "given command is equal either snapshot or restore: {}",
        command,
    );
    command.to_string()
}

fn cast_height(s: &str) -> u64 {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16).unwrap()
    } else {
        u64::from_str_radix(s, 10).unwrap()
    }
}
