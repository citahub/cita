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
use executor_instance::ExecutorInstance;
use libproto::router::{MsgType, RoutingKey, SubModules};
use pubsub::start_pubsub;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use util::set_panic_handler;

fn main() {
    micro_service_init!("cita-executor", "CITA:executor");

    let matches = App::new("executor")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-g, --genesis=[FILE] 'Sets a genesis config file")
        .arg_from_usage("-c, --config=[FILE] 'Sets a switch config file'")
        .get_matches();

    let mut genesis_path = "genesis.json";
    if let Some(ge) = matches.value_of("genesis") {
        trace!("Value for genesis: {}", ge);
        genesis_path = ge;
    }

    let mut config_path = "executor.toml";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let (tx, rx) = channel();
    let (write_sender, write_receiver) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub(
        "executor",
        routing_key!([
            Chain >> SyncResponse,
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

    let mut ext_instance = ExecutorInstance::new(ctx_pub.clone(), write_sender, config_path, genesis_path);
    let distribute_ext = ext_instance.clone();

    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            distribute_ext.distribute_msg(&key, &msg);
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
