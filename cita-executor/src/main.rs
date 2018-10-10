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
//! - `ExecutorInstance`: `executor_instance::ExecutorInstance`
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

extern crate cita_types;
extern crate clap;
extern crate common_types as types;
extern crate core_executor as core;
extern crate dotenv;
extern crate error;
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
extern crate util;

mod executor_instance;

use clap::App;
use core::libexecutor::vm_grpc_server;
use executor_instance::ExecutorInstance;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-executor", "CITA:executor");
    info!("Version: {}", get_build_info_str(true));

    let matches = App::new("executor")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-g, --genesis=[FILE] 'Sets a genesis config file")
        .arg_from_usage("-c, --config=[FILE] 'Sets a switch config file'")
        .get_matches();

    let genesis_path = matches.value_of("genesis").unwrap_or("genesis.json");

    let config_path = matches.value_of("config").unwrap_or("executor.toml");

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();
    let (write_sender, write_receiver) = channel();
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
        tx,
        crx_pub,
    );

    let ext_instance =
        ExecutorInstance::new(ctx_pub.clone(), write_sender, config_path, genesis_path);
    let mut distribute_ext = ext_instance.clone();

    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            distribute_ext.distribute_msg(&key, &msg);
        }
    });
    let mut server: Option<::grpc::Server> = None;
    let grpc_ext = ext_instance.clone();
    thread::spawn(move || loop {
        if server.is_none() {
            server = vm_grpc_server(grpc_ext.grpc_port, Arc::clone(&grpc_ext.ext));
        } else {
            thread::sleep(Duration::new(8, 0));
        }
    });

    let mut timeout_factor = 0u8;
    loop {
        if let Ok(number) = write_receiver
            .recv_timeout(Duration::new(18 * (2u64.pow(u32::from(timeout_factor))), 0))
        {
            ext_instance.execute_block(number);
            timeout_factor = 0;
        } else if !ext_instance.is_snapshot {
            info!("Executor enters the timeout");
            if timeout_factor < 6 {
                timeout_factor += 1
            }
        }
    }
}
