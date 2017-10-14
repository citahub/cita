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

#![allow(dead_code, unused_variables, unused_must_use, unused_mut, unused_imports, unused_extern_crates)]
#![feature(test)]
extern crate test;
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
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate jsonrpc_types;
extern crate protobuf;
extern crate rustc_hex;
extern crate util;
extern crate uuid;
extern crate rand;
extern crate pubsub;

pub mod method;
pub mod connection;
pub mod config;
pub mod worker;
pub mod client;

use clap::App;
use client::*;
use config::Config;
use connection::*;
use rand::{thread_rng, ThreadRng, Rng};
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use url::Url;
use util::RwLock;
use worker::*;
use ws::{Builder, Settings};

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
    println!("CITA:bench_ws config \n {:?}", serde_json::to_string_pretty(&config).unwrap());


    let (tx, rx) = mpsc::channel();
    let ws_senders = Arc::new(RwLock::new(vec![]));
    let fac_con = FactoryConnection::new(ws_senders.clone(), tx);
    let mut ws = Builder::new()
        .with_settings(Settings {
                           max_connections: config.max_connections,
                           queue_size: config.param.number + 5,
                           ..Settings::default()
                       })
        .build(fac_con)
        .unwrap();

    let mut rand = thread_rng();
    for i in 0..config.max_connections {
        let index = rand.gen_range(0, config.servers.len());
        let url = Url::from_str(config.servers[index].as_str()).unwrap();
        println!("conection{} url = {:?}", i, url);
        ws.connect(url).unwrap();
    }

    thread::spawn(move || {
        let mut worker = Worker::new(ws_senders, config.param.clone());
        thread::sleep(Duration::new(3, 0));
        if config.param.peer_param.enable {
            worker.bench_peer_count();
        }

        if config.param.tx_param.enable {
            worker.heart_beat_height();
            thread::sleep(Duration::new(10, 0));
        }
        worker.recive(rx);
    });

    let _ = ws.run();
}
