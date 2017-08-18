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

#![feature(plugin)]
#[allow(deprecated)]
#[allow(unused_assignments)]
#[allow(unused_must_use)]
extern crate futures;
extern crate hyper;
extern crate libproto;
extern crate protobuf;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate util;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rustc_serialize;
extern crate pubsub;
extern crate time;
extern crate proof;
extern crate docopt;
extern crate cpuprofiler;
extern crate jsonrpc_types;
extern crate dotenv;
extern crate transaction as cita_transaction;
extern crate cita_log;
extern crate threadpool;
extern crate num_cpus;
extern crate parking_lot;
extern crate ws;
extern crate clap;

pub mod http_handler;
pub mod mq_hanlder;
pub mod base_hanlder;
pub mod ws_handler;
pub mod config;

use base_hanlder::TransferType;
use clap::App;
use config::ProfileConfig;
use cpuprofiler::PROFILER;
use dotenv::dotenv;
use http_handler::RpcHandler;
use hyper::server::Server;
use jsonrpc_types::method;
use log::LogLevelFilter;
use parking_lot::{RwLock, Mutex};
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use ws_handler::WsFactory;


fn start_profile(config: &ProfileConfig) {
    if config.enable {
        if config.flag_prof_start != 0 && config.flag_prof_duration != 0 {
            let start = config.flag_prof_start;
            let duration = config.flag_prof_duration;
            thread::spawn(move || {
                              thread::sleep(Duration::new(start, 0));
                              PROFILER.lock().unwrap().start("./jsonrpc.profile").expect("Couldn't start");
                              thread::sleep(Duration::new(duration, 0));
                              PROFILER.lock().unwrap().stop().unwrap();
                          });
        }

    }
}



fn main() {
    dotenv().ok();
    ::std::env::set_var("RUST_BACKTRACE", "full");
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:jsonrpc ");

    // todo load config
    let matches = App::new("JsonRpc")
        .version("0.1")
        .author("Cryptape")
        .about("CITA JSON-RPC by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let mut config_path = "./jsonrpc.json";
    if let Some(c) = matches.value_of("config") {
        info!("Value for config: {}", c);
        config_path = c;
    }

    let config = config::read_user_from_file(config_path).expect("config error!");
    info!("CITA:jsonrpc config \n {:?}", serde_json::to_string_pretty(&config).unwrap());

    //TODO not enable both HTTP and WebSocket server
    if config.ws_config.enable == config.http_config.enable {
        error!("not enable both HTTP and WebSocket server!");
        std::process::exit(-1);
    }

    start_profile(&config.profile_config);

    // init pubsub
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    start_pubsub("jsonrpc", vec!["*.rpc"], tx_sub, rx_pub);

    //mq
    let mut new_subscriber = mq_hanlder::MqHandler::new();

    //http
    if config.http_config.enable {
        new_subscriber.set_http_or_ws(TransferType::HTTP, 0);
        let http_responses = Arc::new(RwLock::new(HashMap::with_capacity(1000)));
        let http_tx_responses = Arc::new(RwLock::new(HashMap::with_capacity(1000)));
        new_subscriber.set_http(http_tx_responses.clone(), http_responses.clone());

        let http_config = config.http_config.clone();
        let sender_mq_http = tx_pub.clone();
        thread::spawn(move || {
            let url = http_config.listen_ip.clone() + ":" + &http_config.listen_port.clone().to_string();
            let arc_tx = Arc::new(Mutex::new(sender_mq_http));
            info!("Http Listening on {}", url);
            let _ = Server::http(url).unwrap().handle_threads(RpcHandler {
                                                                  responses: http_responses,
                                                                  tx: arc_tx,
                                                                  tx_responses: http_tx_responses,
                                                                  sleep_duration: http_config.sleep_duration,
                                                                  timeout_count: http_config.timeout_count,
                                                                  method_handler: method::MethodHandler,
                                                              },
                                                              http_config.thread_number);
        });
    }

    //ws
    if config.ws_config.enable {
        new_subscriber.set_http_or_ws(TransferType::WEBSOCKET, 0);
        let ws_responses = Arc::new(Mutex::new(HashMap::with_capacity(1000)));
        let ws_tx_responses = Arc::new(Mutex::new(HashMap::with_capacity(1000)));
        new_subscriber.set_ws(ws_tx_responses.clone(), ws_responses.clone());

        let ws_config = config.ws_config.clone();
        thread::spawn(move || {
            let url = ws_config.listen_ip.clone() + ":" + &ws_config.listen_port.clone().to_string();
            let factory = WsFactory::new(ws_tx_responses, ws_responses, tx_pub, 0);
            info!("WebSocket Listening on {}", url);
            let mut ws_build = ws::Builder::new();
            ws_build.with_settings(ws_config.into());
            let ws_server = ws_build.build(factory).unwrap();
            let _ = ws_server.listen(url);
        });
    }

    loop {
        let (key, msg) = rx_sub.recv().unwrap();
        new_subscriber.handle(key, msg);
    }
}
