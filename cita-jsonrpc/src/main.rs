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
//!
//! One of CITA's core components, the only external module that provides jsonrpc,
//! is used to facilitate user interaction with the chain and forward requests.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     |  Queue  | PubModule | Message Type |
//!     | ------- | --------- | ------------ |
//!     | jsonrpc | Auth      | Response     |
//!     | jsonrpc | Chain     | Response     |
//!     | jsonrpc | Executor  | Response     |
//!     | jsonrpc | Net       | Response     |
//!
//! 2. Publish channel
//!
//!     |  Queue  | PubModule | SubModule | Message Type      |
//!     | ------- | --------- | --------- | ----------------- |
//!     | jsonrpc | Jsonrpc   | Auth      | RequestNewTxBatch |
//!     | jsonrpc | Jsonrpc   | Chain     | Request           |
//!     | jsonrpc | Jsonrpc   | Net       | RequestNet        |
//!
//! ### Key behavior
//!
//! the key Struct:
//!
//! - `TransferType`: `helper::TransferType`
//! - `ReqInfo`: `helper::ReqInfo`
//!
//! The return message of the jsonrpc service is performed through this structure `responses`,
//! whether it is a Websocket or an Http interface.
//! Websocket and Http only write to this structure and write the internal transaction
//! uuid number and `TransferType`.
//!

#![feature(try_from)]
#![feature(tool_lints)]
#![feature(never_type)]

extern crate bytes;
extern crate clap;
extern crate cpuprofiler;
extern crate dotenv;
extern crate error;
extern crate futures;
extern crate http;
extern crate httparse;
extern crate hyper;
extern crate jsonrpc_types;
extern crate libc;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate logger;
extern crate net2;
extern crate num_cpus;
extern crate pubsub;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate threadpool;
extern crate time;
extern crate tokio_core;
extern crate tokio_io;
extern crate unicase;
#[macro_use]
extern crate util;
extern crate uuid;
extern crate ws;

mod config;
mod extractor;
mod fdlimit;
mod helper;
mod http_server;
mod mq_handler;
mod mq_publisher;
mod response;
mod service_error;
mod ws_handler;

use clap::App;
use config::{NewTxFlowConfig, ProfileConfig};
use cpuprofiler::PROFILER;
use fdlimit::set_fd_limit;
use http_server::Server;
use libproto::request::{self as reqlib, BatchRequest};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use pubsub::start_pubsub;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
use tokio_core::reactor::Core;
use util::{set_panic_handler, Mutex};
use uuid::Uuid;
use ws_handler::WsFactory;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-jsonrpc", "CITA:jsonrpc");
    info!("Version: {}", get_build_info_str(true));

    // todo load config
    let matches = App::new("JsonRpc")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA JSON-RPC by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("./jsonrpc.toml");

    let config = config::Config::new(config_path);
    info!("CITA:jsonrpc config \n {:?}", config);

    //enable HTTP or WebSocket server!
    if !config.ws_config.enable && !config.http_config.enable {
        error!("Please at least enable one of HTTP and WebSocket server!");
        std::process::exit(2);
    }

    start_profile(&config.profile_config);

    // set fd
    set_fd_limit();

    // init pubsub
    let (tx_sub, rx_sub) = channel();
    let (tx_pub, rx_pub) = channel();
    //used for buffer message
    let (tx_relay, rx_relay) = channel();
    start_pubsub(
        "jsonrpc",
        routing_key!([
            Auth >> Response,
            Chain >> Response,
            Executor >> Response,
            Net >> Response,
        ]),
        tx_sub,
        rx_pub,
    );

    let backlog_capacity = config.backlog_capacity;

    // type Arc<Mutex<HashMap<Uuid, TransferType>>>
    let responses = Arc::new(Mutex::new(HashMap::with_capacity(backlog_capacity)));
    let http_responses = Arc::clone(&responses);
    let ws_responses = Arc::clone(&responses);
    let mut mq_handle = mq_handler::MqHandler::new(responses);

    //dispatch
    let tx_flow_config = config.new_tx_flow_config;
    thread::spawn(move || {
        let mut new_tx_request_buffer = Vec::new();
        let mut time_stamp = SystemTime::now();
        loop {
            if let Ok(res) = rx_relay.try_recv() {
                let (topic, req): (String, reqlib::Request) = res;
                forward_service(
                    topic,
                    req,
                    &mut new_tx_request_buffer,
                    &mut time_stamp,
                    &tx_pub,
                    &tx_flow_config,
                );
            } else {
                if !new_tx_request_buffer.is_empty() {
                    batch_forward_new_tx(&mut new_tx_request_buffer, &mut time_stamp, &tx_pub);
                }
                thread::sleep(Duration::new(0, tx_flow_config.buffer_duration));
            }
        }
    });

    //ws
    if config.ws_config.enable {
        let ws_config = config.ws_config.clone();
        let tx = tx_relay.clone();
        thread::spawn(move || {
            let url =
                ws_config.listen_ip.clone() + ":" + &ws_config.listen_port.clone().to_string();
            //let factory = WsFactory::new(ws_responses, tx_pub, 0);
            let factory = WsFactory::new(ws_responses, tx, 0);
            info!("WebSocket Listening on {}", url);
            let mut ws_build = ws::Builder::new();
            ws_build.with_settings(ws_config.into());
            let ws_server = ws_build.build(factory).unwrap();
            let _ = ws_server.listen(url);
        });
    }

    if config.http_config.enable {
        let http_config = config.http_config.clone();
        let addr =
            http_config.listen_ip.clone() + ":" + &http_config.listen_port.clone().to_string();
        info!("Http Listening on {}", &addr);

        let threads: usize = config
            .http_config
            .thread_number
            .unwrap_or_else(num_cpus::get);

        for i in 0..threads {
            let addr = addr.clone().parse().unwrap();
            let tx = tx_relay.clone();
            let timeout = http_config.timeout;
            let http_responses = Arc::clone(&http_responses);
            let allow_origin = http_config.allow_origin.clone();
            let _ = thread::Builder::new()
                .name(format!("worker{}", i))
                .spawn(move || {
                    let core = Core::new().unwrap();
                    let handle = core.handle();
                    let timeout = Duration::from_secs(timeout);
                    let listener = http_server::listener(&addr, &handle).unwrap();
                    Server::start(core, listener, tx, http_responses, timeout, &allow_origin);
                })
                .unwrap();
        }
    }

    loop {
        let (key, msg) = rx_sub.recv().unwrap();
        let _ = mq_handle.handle(&key, &msg);
    }
}

fn batch_forward_new_tx(
    new_tx_request_buffer: &mut Vec<reqlib::Request>,
    time_stamp: &mut SystemTime,
    tx_pub: &Sender<(String, Vec<u8>)>,
) {
    trace!(
        "Going to send new tx batch to auth with {} new tx and buffer time cost is {:?} ",
        new_tx_request_buffer.len(),
        time_stamp.elapsed().unwrap()
    );
    let mut batch_request = BatchRequest::new();
    batch_request.set_new_tx_requests(new_tx_request_buffer.clone().into());

    let request_id = Uuid::new_v4().as_bytes().to_vec();
    let mut request = reqlib::Request::new();
    request.set_batch_req(batch_request);
    request.set_request_id(request_id);

    let data: Message = request.into();
    tx_pub
        .send((
            routing_key!(Jsonrpc >> RequestNewTxBatch).into(),
            data.try_into().unwrap(),
        ))
        .unwrap();
    *time_stamp = SystemTime::now();
    new_tx_request_buffer.clear();
}

fn forward_service(
    topic: String,
    req: reqlib::Request,
    new_tx_request_buffer: &mut Vec<reqlib::Request>,
    time_stamp: &mut SystemTime,
    tx_pub: &Sender<(String, Vec<u8>)>,
    config: &NewTxFlowConfig,
) {
    if RoutingKey::from(&topic) != routing_key!(Jsonrpc >> RequestNewTx) {
        let data: Message = req.into();
        tx_pub.send((topic, data.try_into().unwrap())).unwrap();
    } else {
        new_tx_request_buffer.push(req);
        trace!(
            "New tx is pushed and has {} new tx and buffer time cost is {:?}",
            new_tx_request_buffer.len(),
            time_stamp.elapsed().unwrap()
        );
        if new_tx_request_buffer.len() > config.count_per_batch
            || time_stamp.elapsed().unwrap().subsec_nanos() > config.buffer_duration
        {
            batch_forward_new_tx(new_tx_request_buffer, time_stamp, tx_pub);
        }
    }
}

fn start_profile(config: &ProfileConfig) {
    if config.enable && config.flag_prof_start != 0 && config.flag_prof_duration != 0 {
        let start = config.flag_prof_start;
        let duration = config.flag_prof_duration;
        thread::spawn(move || {
            thread::sleep(Duration::new(start, 0));
            PROFILER
                .lock()
                .unwrap()
                .start("./jsonrpc.profile")
                .expect("Couldn't start");
            thread::sleep(Duration::new(duration, 0));
            PROFILER.lock().unwrap().stop().unwrap();
        });
    }
}
