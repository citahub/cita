#![feature(plugin)]
#[macro_use]
extern crate log;
extern crate clap;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
extern crate rustc_serialize;
extern crate parking_lot;
extern crate amqp;
extern crate libproto;
extern crate protobuf;
extern crate pubsub;
extern crate util;
extern crate dotenv;
extern crate cita_log;

pub mod config;
pub mod server;
pub mod connection;
pub mod citaprotocol;
pub mod msghandle;

use std::sync::mpsc::channel;
use std::env;

use clap::{App, SubCommand};
use server::start_server;
use config::NetConfig;
use connection::{Connection, do_connect};
use pubsub::PubSub;
use msghandle::MyHandler;
use server::MySender;
use log::LogLevelFilter;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");

    // Init logger
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:network");
    // init app
    // todo load config
    let matches = App::new("network")
        .version("0.8")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .subcommand(SubCommand::with_name("test").about("does testing things"))
        .get_matches();

    let mut config_path = "config";

    if let Some(c) = matches.value_of("config") {
        info!("Value for config: {}", c);
        config_path = c;
    }

    // check for the existence of subcommands
    let is_test = matches.is_present("test");

    let config = if is_test {
        NetConfig::test_config()
    } else {
        NetConfig::new(config_path)
    };

    let (tx, rx) = channel();

    // start server
    // This brings up our server.
    let mysender = MySender::new(tx);
    start_server(&config, mysender);

    // connect peers
    let con = Connection::new(&config);
    do_connect(&con);

    // init pubsub
    let mut pubsub = PubSub::new();
    let mut _pub2 = pubsub.get_pub();
    pubsub.start_sub("network",
                     vec!["consensus.tx",
                          "consensus.msg",
                          "chain.status",
                          "chain.blk",
                          "chain.sync",
                          "jsonrpc.net"],
                     MyHandler::new(con, _pub2));

    let mut _pub = pubsub.get_pub();
    loop {
        let msg = rx.recv().unwrap();
        _pub.publish(&msg.0, msg.1);
    }
}
