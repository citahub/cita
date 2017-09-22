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

#![allow(dead_code, unused_variables, unused_must_use, unused_imports, unused_extern_crates)]

extern crate ws;
extern crate url;
extern crate time;
#[macro_use]
extern crate env_logger;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate log;
extern crate clap;
extern crate num_cpus;
//extern crate parking_lot;
//extern crate threadpool;
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate jsonrpc_types;
extern crate protobuf;
extern crate rustc_hex;
extern crate util;

pub mod rpc_method;
pub mod connection;
pub mod config;
use clap::App;
use config::Config;
use connection::Connection;
use std::str::FromStr;
use std::thread;
use url::Url;
use ws::{Builder, Settings, Sender, CloseCode, Handler, Message, Handshake, Result};

fn main() {
    env_logger::init().unwrap();
    info!("CITA:bench_ws ");
    // todo load config
    let matches = App::new("bench_ws")
        .version("0.1")
        .author("Cryptape")
        .about("CITA bench_ws by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let mut config_path = "./bench_ws.json";
    if let Some(c) = matches.value_of("config") {
        info!("Value for config: {}", c);
        config_path = c;
    }

    let config: Config = config::read_user_from_file(config_path).unwrap_or(Config::default());
    info!("CITA:bench_ws config \n {:?}", serde_json::to_string_pretty(&config).unwrap());

    let start = time::precise_time_ns();
    println!("begin time. {} ", start);

    let mut handlers = vec![];
    //TODO
    for _ in 0..config.thread_number {
        let config = config.clone();
        let handler = thread::spawn(move || {
            let mut ws = Builder::new()
                .with_settings(Settings {
                                   max_connections: config.max_connections,
                                   queue_size: config.param.tx_num + 5,
                                   ..Settings::default()
                               })
                .build(|out| {
                    Connection {
                        out: out,
                        count: 0,
                        time: 0,
                        total: 0,
                        param: config.param.clone(),
                        success_count: 0,
                        failure_count: 0,
                        height: 0,
                    }
                })
                .unwrap();
            let url = Url::from_str(config.servers[0].as_str()).unwrap();
            for _ in 0..config.max_connections {
                ws.connect(url.clone()).unwrap();
            }
            ws.run().unwrap();
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join();
    }

    let end = (time::precise_time_ns() - start) / 1000000;
    info!("Total time. {} ms", end);
    println!("Total time. {} ms", end);
}
