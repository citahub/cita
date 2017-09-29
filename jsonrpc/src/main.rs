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

#![allow(unused_assignments, unused_must_use, deprecated, unused_extern_crates)]
extern crate hyper;
extern crate libproto;
extern crate protobuf;
#[macro_use]
extern crate log;
extern crate util;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate pubsub;
extern crate cpuprofiler;
extern crate jsonrpc_types;
extern crate dotenv;
extern crate cita_log;
extern crate threadpool;
extern crate num_cpus;
extern crate parking_lot;
extern crate ws;
extern crate clap;
extern crate uuid;

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
use http_handler::HttpHandler;
use hyper::server::Server;
use jsonrpc_types::method;

use libproto::communication::Message as commMsg;
use libproto::request as reqlib;
use libproto::request::BatchRequest;
use log::LogLevelFilter;
use parking_lot::{RwLock, Mutex};
use protobuf::Message;
use protobuf::RepeatedField;
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use uuid::Uuid;
use ws_handler::WsFactory;

pub const TOPIC_NEW_TX: &str = "jsonrpc.new_tx";
pub const TOPIC_NEW_TX_BATCH: &str = "jsonrpc.new_tx_batch";

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
    //used for buffer message
    let (tx_relay, rx_relay) = channel();
    start_pubsub("jsonrpc", vec!["*.rpc"], tx_sub, rx_pub);

    //mq
    let mut mq_handle = mq_hanlder::MqHandler::new();

    //http
    if config.http_config.enable {
        mq_handle.set_http_or_ws(TransferType::HTTP, 0);
        let http_responses = Arc::new(RwLock::new(HashMap::with_capacity(1000)));
        mq_handle.set_http(http_responses.clone());

        let http_config = config.http_config.clone();
        //let sender_mq_http = tx_pub.clone();
        let sender_mq_http = tx_relay.clone();
        thread::spawn(move || {
            let url = http_config.listen_ip.clone() + ":" + &http_config.listen_port.clone().to_string();
            let arc_tx = Arc::new(Mutex::new(sender_mq_http));
            info!("Http Listening on {}", url);
            let _ = Server::http(url).unwrap().handle_threads(HttpHandler {
                                                                  responses: http_responses,
                                                                  tx: arc_tx,
                                                                  sleep_duration: http_config.sleep_duration,
                                                                  timeout_count: http_config.timeout_count,
                                                                  method_handler: method::MethodHandler,
                                                              },
                                                              http_config.thread_number);
        });
    }

    //ws
    if config.ws_config.enable {
        mq_handle.set_http_or_ws(TransferType::WEBSOCKET, 0);
        let ws_responses = Arc::new(Mutex::new(HashMap::with_capacity(1000)));
        mq_handle.set_ws(ws_responses.clone());
        let ws_config = config.ws_config.clone();
        thread::spawn(move || {
            let url = ws_config.listen_ip.clone() + ":" + &ws_config.listen_port.clone().to_string();
            //let factory = WsFactory::new(ws_responses, tx_pub, 0);
            let factory = WsFactory::new(ws_responses, tx_relay, 0);
            info!("WebSocket Listening on {}", url);
            let mut ws_build = ws::Builder::new();
            ws_build.with_settings(ws_config.into());
            let ws_server = ws_build.build(factory).unwrap();
            let _ = ws_server.listen(url);
        });
    }

    thread::spawn(move || {
        let mut new_tx_request_buffer = Vec::new();
        let mut time_stamp = SystemTime::now();
        loop {
            let (topic, req): (String, reqlib::Request) = rx_relay.recv().unwrap();
            if topic.as_str() != TOPIC_NEW_TX {
                let data: commMsg = req.into();
                tx_pub.send((topic, data.write_to_bytes().unwrap())).unwrap();
            } else {
                new_tx_request_buffer.push(req);
                if new_tx_request_buffer.len() > config.new_tx_flow_config.count_per_batch || time_stamp.elapsed().unwrap().subsec_nanos() > config.new_tx_flow_config.buffer_durtation {
                    trace!("Going to send new tx batch to auth with {} new tx and buffer {} ns", new_tx_request_buffer.len(), time_stamp.elapsed().unwrap().subsec_nanos());

                    let mut batch_request = BatchRequest::new();
                    batch_request.set_new_tx_requests(RepeatedField::from_slice(&new_tx_request_buffer[..]));

                    let request_id = Uuid::new_v4().as_bytes().to_vec();
                    let mut request = reqlib::Request::new();
                    request.set_batch_req(batch_request);
                    request.set_request_id(request_id);

                    let data: commMsg = request.into();
                    tx_pub.send((String::from(TOPIC_NEW_TX_BATCH), data.write_to_bytes().unwrap())).unwrap();
                    time_stamp = SystemTime::now();
                    new_tx_request_buffer = Vec::new();
                }
            }
        }
    });

    loop {
        let (key, msg) = rx_sub.recv().unwrap();
        mq_handle.handle(key, msg);
    }
}
