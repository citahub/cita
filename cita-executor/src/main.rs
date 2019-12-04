// Copyright Rivtower Technologies LLC.
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

//! ## Summary
//! One of cita's main core components is to execute transaction,
//! create contracts, maintain world state trees, and send executed
//! result block to chain.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     | Queue    | PubModule | Message Type               |
//!     | -------- | --------- | ------------------         |
//!     | executor | Chain     | Request                    |
//!     | executor | Chain     | Richstatus                 |
//!     | executor | Chain     | StateSignal                |
//!     | executor | Chain     | LocalSync                  |
//!     | executor | Consensus | BlockWithProof             |
//!     | executor | Consensus | SignedProposal             |
//!     | executor | Consensus | MiscellaneousReq           |
//!     | executor | Net       | SyncResponse               |
//!     | executor | Net       | SignedProposal             |
//!     | executor | Snapshot  | SnapshotReq                |
//!
//! 2. Publish channel
//!
//!     | Queue    | PubModule | SubModule | Message Type   |
//!     | -------- | --------- | --------- | -------------- |
//!     | executor | Executor  | Snapshot  | SnapshotResp   |
//!     | executor | Executor  | Jsonrpc   | Response       |
//!     | executor | Executor  | Chain     | ExecutedResult |
//!     | executor | Executor  | Auth      | Miscellaneous  |
//!     | executor | Executor  | Auth      | BlackList      |
//!     | executor | Executor  | Chain     | StateSignal    |
//!
//! ### Key behavior
//!
//! key struct:
//!
//! - `Postman`: `postman::Postman`
//! - [`Executor`]
//! - [`GlobalSysConfig`]
//! - [`Genesis`]
//! - [`Contract`]
//! - [`Account`]
//! - `AccountEntry`: `core_executor::state::AccountEntry`
//! - [`State`]
//! - [`StateDB`]
//!
//! This is currently the most complex module that maintains the current state of
//! the entire chain and caches some data, keeps the hash values of the last 256
//! blocks and the information of each block (gas_limit/quota, etc.) in memory,
//! holds the current block map(heigh, block).
//!
//! Of course there is an evm interface in this module.
//!
//! The contract/transaction submission is first cached in memory before being committed
//! to the stateDB (disk).
//!
//! [`Executor`]: ../core_executor/libexecutor/executor/struct.Executor.html
//! [`GlobalSysConfig`]: ../core_executor/libexecutor/executor/struct.GlobalSysConfig.html
//! [`Genesis`]: ../core_executor/libexecutor/genesis/struct.Genesis.html
//! [`Contract`]: ../core_executor/libexecutor/genesis/struct.Contract.html
//! [`Account`]: ../core_executor/state/account/struct.Account.html
//! [`State`]: ../core_executor/state/struct.State.html
//! [`StateDB`]: ../core_executor/state_db/struct.StateDB.html
//!

#[cfg(test)]
extern crate cita_crypto;
extern crate common_types as types;
extern crate core_executor as core;
#[macro_use]
extern crate crossbeam_channel;
extern crate cita_database as cita_db;
#[cfg(test)]
extern crate hashable;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate util;

use crate::core::libexecutor::executor::Executor;
use crate::postman::Postman;
use cita_directories::DataPath;
use clap::App;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::channel;
use pubsub::start_pubsub;
use std::thread;
use util::set_panic_handler;

mod backlogs;
mod postman;
#[cfg(test)]
mod tests;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

#[derive(Debug, PartialEq, Deserialize)]
pub struct Options {
    prooftype: u8,
    journaldb_type: String,
    genesis_path: String,
    statedb_cache_size: usize,
    eth_compatibility: bool,
}

impl Options {
    pub fn default() -> Self {
        Options {
            prooftype: 2,
            journaldb_type: String::from("archive"),
            genesis_path: String::from("genesis.json"),
            statedb_cache_size: 5 * 1024 * 1024,
            eth_compatibility: false,
        }
    }

    pub fn load(path: &str) -> Self {
        parse_config!(Options, path)
    }
}

fn main() {
    let matches = App::new("executor")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Rivtower")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a switch config file'
                          -s, --stdout 'Log to console'",
        )
        .get_matches();

    let stdout = matches.is_present("stdout");
    micro_service_init!("cita-executor", "CITA:executor", stdout);

    let config_path = matches.value_of("config").unwrap_or("executor.toml");
    let options = Options::load(config_path);
    info!("Version: {}", get_build_info_str(true));
    info!("Config: {:?}", options);

    // start pubsub thread
    let (forward_req_sender, forward_req_receiver) = channel::unbounded();
    let (forward_resp_sender, forward_resp_receiver) = channel::unbounded();
    let (mq_req_sender, mq_req_receiver) = crossbeam_channel::unbounded();
    let (mq_resp_sender, mq_resp_receiver) = crossbeam_channel::unbounded();
    let (fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
    let (fsm_resp_sender, fsm_resp_receiver) = crossbeam_channel::unbounded();
    let (command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
    let (command_resp_sender, command_resp_receiver) = crossbeam_channel::bounded(0);
    start_pubsub(
        "executor",
        routing_key!([
            Chain >> Request,
            Chain >> RichStatus,
            Chain >> StateSignal,
            Chain >> LocalSync,
            Consensus >> BlockWithProof,
            Consensus >> SignedProposal,
            Net >> SyncResponse,
            Snapshot >> SnapshotReq,
            Auth >> MiscellaneousReq,
        ]),
        forward_req_sender,
        forward_resp_receiver,
    );

    // start threads to forward messages between mpsc::channel and crosebeam::channel
    thread::spawn(move || loop {
        match forward_req_receiver.recv() {
            Ok(message) => {
                let _ = mq_req_sender.send(message);
            }
            Err(_) => return,
        };
    });
    thread::spawn(move || loop {
        match mq_resp_receiver.recv() {
            Ok(message) => {
                forward_resp_sender.send(message).unwrap();
            }
            Err(_) => return,
        }
    });

    loop {
        // start executor thread
        // TODO consider to store `data_path` within executor.toml
        let data_path = DataPath::root_node_path();
        let mut executor = Executor::init(
            &options.genesis_path,
            data_path,
            fsm_req_receiver.clone(),
            fsm_resp_sender.clone(),
            command_req_receiver.clone(),
            command_resp_sender.clone(),
            options.eth_compatibility,
        );
        let current_height = executor.get_current_height();
        let current_hash = executor.get_current_hash();
        let handle = thread::spawn(move || {
            executor.do_loop();
        });

        // start postman thread
        let mut postman = Postman::new(
            current_height,
            current_hash,
            mq_req_receiver.clone(),
            mq_resp_sender.clone(),
            fsm_req_sender.clone(),
            fsm_resp_receiver.clone(),
            command_req_sender.clone(),
            command_resp_receiver.clone(),
        );
        postman.do_loop();

        handle.join().expect(
            "
            Executor exit cause Command::Exit was sent by postman inside.

            When postman roll back the whole cita-chain to an old height,
            it would tell executor thread to reset the `CURRNENT_HASH` to the
            target height, and then exit, both with postman. Main thread would
            re-run postman and executor inside this loop statement.
        ",
        );
    }
}
