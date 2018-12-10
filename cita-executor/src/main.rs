// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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
//! One of cita's main core components is to execute transaction,
//! create contracts, maintain world state trees, and send executed
//! result block to chain.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     | Queue    | PubModule | Message Type       |
//!     | -------- | --------- | ------------------ |
//!     | executor | Chain     | Request            |
//!     | executor | Chain     | Richstatus         |
//!     | executor | Chain     | StateSignal        |
//!     | executor | Chain     | LocalSync          |
//!     | executor | Consensus | BlockWithProof     |
//!     | executor | Consensus | SignedProposal     |
//!     | executor | Consensus | MiscellaneousReq   |
//!     | executor | Net       | SyncResponse       |
//!     | executor | Net       | SignedProposal     |
//!     | executor | Snapshot  | SnapshotReq        |
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

#![feature(try_from)]
#![feature(tool_lints)]
#![feature(mpsc_select)]

#[macro_use]
extern crate crossbeam_channel;
extern crate cita_types;
extern crate clap;
extern crate common_types as types;
extern crate core_executor as core;
extern crate dotenv;
extern crate error;
extern crate evm;
extern crate grpc;
extern crate jsonrpc_types;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate proof;
extern crate pubsub;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate util;

mod backlogs;
mod postman;
#[cfg(test)]
mod tests;

use clap::App;
use core::contracts::grpc::grpc_vm_adapter;
use core::libexecutor::executor::Executor;
use libproto::router::{MsgType, RoutingKey, SubModules};
use postman::Postman;
use pubsub::start_pubsub;
use std::sync::mpsc::channel;
use std::thread;
use util::datapath::DataPath;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

#[derive(Debug, PartialEq, Deserialize)]
pub struct Options {
    prooftype: u8,
    grpc_port: u16,
    journaldb_type: String,
    genesis_path: String,
    statedb_cache_size: usize,
}

impl Options {
    pub fn default() -> Self {
        Options {
            prooftype: 2,
            grpc_port: 5000,
            journaldb_type: String::from("archive"),
            genesis_path: String::from("genesis.json"),
            statedb_cache_size: 5 * 1024 * 1024,
        }
    }

    pub fn load(path: &str) -> Self {
        parse_config!(Options, path)
    }
}

fn main() {
    micro_service_init!("cita-executor", "CITA:executor");
    let matches = App::new("executor")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-c, --config=[FILE] 'Sets a switch config file'")
        .get_matches();
    let config_path = matches.value_of("config").unwrap_or("executor.toml");
    let options = Options::load(config_path);
    info!("Version: {}", get_build_info_str(true));
    info!("Config: {:?}", options);

    // start pubsub thread
    let (forward_req_sender, forward_req_receiver) = channel();
    let (forward_resp_sender, forward_resp_receiver) = channel();
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
            Net >> SignedProposal,
            Snapshot >> SnapshotReq,
            Auth >> MiscellaneousReq,
        ]),
        forward_req_sender,
        forward_resp_receiver,
    );

    // start threads to forward messages between mpsc::channel and crosebeam::channel
    thread::spawn(move || loop {
        match forward_req_receiver.recv() {
            Ok(message) => mq_req_sender.send(message),
            Err(_) => return,
        }
    });
    thread::spawn(move || loop {
        match mq_resp_receiver.recv() {
            Some(message) => forward_resp_sender.send(message).unwrap(),
            None => return,
        }
    });

    // start grpc server thread background
    let server = grpc_vm_adapter::vm_grpc_server(
        options.grpc_port,
        command_req_sender.clone(),
        command_resp_receiver.clone(),
    )
    .expect("failed to initialize grpc server");
    thread::spawn(move || {
        grpc_vm_adapter::serve(&server);
    });

    loop {
        // start executor thread
        // TODO consider to store `data_path` within executor.toml
        let data_path = DataPath::root_node_path();
        let mut executor = Executor::init(
            &options.genesis_path,
            &options.journaldb_type,
            options.statedb_cache_size,
            data_path,
            fsm_req_receiver.clone(),
            fsm_resp_sender.clone(),
            command_req_receiver.clone(),
            command_resp_sender.clone(),
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
