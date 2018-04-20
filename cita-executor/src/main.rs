//! ## Summary
//! One of cita's main core components is to execute transaction,
//! create contracts, maintain world state trees, and send executed
//! result block to chain.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     | Queue    | SubModule | Message Type   |
//!     | -------- | --------- | -------------- |
//!     | executor | Chain     | LocalSync      |
//!     | executor | Chain     | Request        |
//!     | executor | Consensus | BlockWithProof |
//!     | executor | Consensus | SignedProposal |
//!     | executor | Consensus | RawBytes       |
//!     | executor | Net       | SyncResponse   |
//!     | executor | Net       | SignedProposal |
//!     | executor | Net       | RawBytes       |
//!     | executor | Snapshot  | SnapshotReq    |
//!
//! 2. Publish channel
//!
//!     | Queue    | SubModule | Message Type   |
//!     | -------- | --------- | -------------- |
//!     | executor | Executor  | SnapshotResp   |
//!     | executor | Executor  | Response       |
//!     | executor | Executor  | ExecutedResult |
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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![feature(custom_attribute)]
#![allow(deprecated, unused_must_use, unused_mut, unused_assignments)]
#![feature(refcell_replace_swap)]
#![feature(try_from)]
extern crate bincode;
extern crate clap;
extern crate core_executor as core;
extern crate dotenv;
extern crate error;
extern crate grpc;
extern crate jsonrpc_types;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate proof;
extern crate pubsub;
extern crate serde_json;
#[macro_use]
extern crate util;

mod executor_instance;

use clap::App;
use core::libexecutor::{vm_grpc_server, ServiceMap};
use executor_instance::ExecutorInstance;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-executor", "CITA:executor");

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
    let (write_sender, write_receiver) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub(
        "executor",
        routing_key!([
            Chain >> LocalSync,
            Net >> SyncResponse,
            Consensus >> BlockWithProof,
            Chain >> Request,
            Consensus >> SignedProposal,
            Consensus >> RawBytes,
            Net >> SignedProposal,
            Net >> RawBytes,
            Snapshot >> SnapshotReq,
        ]),
        tx,
        crx_pub,
    );

    let service_map = Arc::new(ServiceMap::new());
    let mut ext_instance = ExecutorInstance::new(
        ctx_pub.clone(),
        write_sender,
        config_path,
        genesis_path,
        Arc::clone(&service_map),
    );
    let distribute_ext = ext_instance.clone();

    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            distribute_ext.distribute_msg(&key, &msg);
        }
    });
    let mut server: Option<::grpc::Server> = None;
    let grpc_ext = ext_instance.clone();
    thread::spawn(move || loop {
        if server.is_none() {
            server = vm_grpc_server(
                grpc_ext.grpc_port,
                Arc::clone(&service_map),
                Arc::clone(&grpc_ext.ext),
            );
        } else {
            thread::sleep(Duration::new(8, 0));
        }
    });

    loop {
        if let Ok(number) = write_receiver.recv_timeout(Duration::new(8, 0)) {
            ext_instance.execute_block(number);
        } else {
            ext_instance.ext.send_executed_info_to_chain(&ctx_pub);
        }
    }
}
