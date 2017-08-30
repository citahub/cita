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
#![allow(deprecated)]
#![feature(plugin)]
#[macro_use]
extern crate log;
extern crate clap;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
extern crate rustc_serialize;
extern crate parking_lot;
extern crate libproto;
extern crate protobuf;
extern crate pubsub;
extern crate dotenv;
extern crate cita_log;
extern crate bytes;

pub mod config;
pub mod server;
pub mod connection;
pub mod citaprotocol;
pub mod msghandle;


use clap::{App, SubCommand};
use config::NetConfig;
use connection::{Connection, do_connect, start_client};
use dotenv::dotenv;
use log::LogLevelFilter;
use msghandle::{is_need_proc, handle_rpc};
use pubsub::start_pubsub;
use server::MySender;
use server::start_server;
use std::env;
use std::sync::Arc;
use std::sync::mpsc::channel;

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
        .version("0.1")
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

    let config = if is_test { NetConfig::test_config() } else { NetConfig::new(config_path) };

    // init pubsub
    let (ctx_sub, crx_sub) = channel();
    let (ctx_pub, crx_pub) = channel();

    start_pubsub("network", vec!["consensus.tx", "consensus.msg", "chain.status", "chain.blk", "chain.sync", "jsonrpc.net"], ctx_sub, crx_pub);

    // start server
    // This brings up our server.
    // all server recv msg directly publish to mq
    let mysender = MySender::new(ctx_pub.clone());
    start_server(&config, mysender);

    // connect peers
    let con = Connection::new(&config);
    do_connect(&con);
    let (ctx, crx) = channel();
    let con = Arc::new(con);
    start_client(con.clone(), crx);

    loop {
        // msg from mq need proc before broadcast
        let (key, body) = crx_sub.recv().unwrap();
        trace!("handle delivery id {:?} payload {:?}", key, body);
        if let (_, true, msg) = is_need_proc(body.as_ref()) {
            ctx.send(msg).unwrap();
        }
        handle_rpc(&con, &ctx_pub, body.as_ref());
    }
}
