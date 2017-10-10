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
#[macro_use]
extern crate log;
extern crate clap;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate byteorder;
extern crate rustc_serialize;
extern crate libproto;
extern crate protobuf;
extern crate pubsub;
extern crate dotenv;
extern crate logger;
extern crate bytes;
extern crate notify;
extern crate util;

pub mod config;
pub mod server;
pub mod connection;
pub mod citaprotocol;
pub mod msghandle;


use clap::{App, SubCommand};
use config::NetConfig;
use connection::{Connection, do_connect as connect, start_client};
use dotenv::dotenv;
use msghandle::{is_need_proc, handle_rpc};
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use pubsub::start_pubsub;
use server::MySender;
use server::start_server;
use std::env;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use util::RwLock;
use util::panichandler::set_panic_handler;


pub fn do_connect(config_path: &str, con: Arc<RwLock<Connection>>) {

    let do_con_lock = con.clone();
    {
        let do_con = &mut *do_con_lock.as_ref().write();
        connect(do_con);
    }

    let config_file: String = "./".to_string() + config_path;
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(5)).unwrap();
    let _ = watcher.watch(config_file.clone(), RecursiveMode::Recursive).unwrap();

    thread::spawn(move || loop {
                      match rx.recv() {
                          Ok(_) => {
                              let config = NetConfig::new(&config_file);

                              let con = &mut *con.as_ref().write();
                              con.update(&config);
                              connect(&con);
                              //let con = &mut *con.as_ref().write();
                              con.del_peer();
                          }
                          Err(e) => info!("watch error: {:?}", e),
                      }
                  });
}


fn main() {
    dotenv().ok();
    // Always print backtrace on panic.
    env::set_var("RUST_BACKTRACE", "full");

    //exit process when panic
    set_panic_handler();

    // Init logger
    logger::init();
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

    start_pubsub("network", vec!["auth.tx", "consensus.msg", "chain.status", "chain.blk", "chain.sync", "jsonrpc.net"], ctx_sub, crx_pub);

    // start server
    // This brings up our server.
    // all server recv msg directly publish to mq
    let mysender = MySender::new(ctx_pub.clone());
    start_server(&config, mysender);

    // connect peers
    let con = Connection::new(&config);
    let con_lock = Arc::new(RwLock::new(con));
    do_connect(config_path, con_lock.clone());
    let (ctx, crx) = channel();
    start_client(con_lock.clone(), crx);

    loop {
        // msg from mq need proc before broadcast
        let (key, body) = crx_sub.recv().unwrap();
        trace!("handle delivery id {:?} payload {:?}", key, body);
        if let (_, true, msg) = is_need_proc(body.as_ref()) {
            ctx.send(msg).unwrap();
        }

        let con = &*con_lock.as_ref().read();
        handle_rpc(&con, &ctx_pub, body.as_ref());
    }
}
