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

#![feature(try_from)]
#![feature(plugin)]
extern crate futures;
extern crate hyper;
extern crate libproto;
extern crate protobuf;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate sha3;
extern crate util;
extern crate serde_json;
extern crate serde;
extern crate rustc_serialize;
extern crate amqp;
extern crate pubsub;
extern crate time;
extern crate proof;
extern crate docopt;
extern crate cpuprofiler;
extern crate jsonrpc_types;
extern crate dotenv;
extern crate transaction as cita_transaction;
extern crate cita_log;

pub mod rpc_handler;
pub mod subscriber;
pub mod publisher;

use hyper::server::Server;
use std::thread;
use rpc_handler::RpcHandler;
use std::sync::mpsc::{channel, Sender, Receiver};
use libproto::TopicMessage;
use std::sync::{Arc, Mutex};
use pubsub::PubSub;
use log::LogLevelFilter;
use docopt::Docopt;
use cpuprofiler::PROFILER;
use std::time::Duration;
use dotenv::dotenv;


const USAGE: &'static str = r#"
CITA jsonrpc client.

Usage:
  jsonrpc [options]
  jsonrpc --help

Options:
  --jsonrpc-port PORT      Specify the port portion of the JSONRPC API server
                           [default: 1337].
  --thread-num NUM         Specify the thread number of the JSONRPC API server
                           [default: 200].
  --sleep-duration MSEC    Specify the duration wait for response each time in loop
                           [default: 1].
  --timeout-count COUNT    Specify the loop count for wait response
                           [default: 3000].
  --prof-start SEC         Specify the start time of profiling, zero means no profiling
                           [default: 0].
  --prof-duration SEC      Specify the duration for profiling, zero means no profiling
                           [default: 0].
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_jsonrpc_port: u16,
    flag_thread_num: usize,
    flag_sleep_duration: u32,
    flag_timeout_count: u32,
    flag_prof_start: u64,
    flag_prof_duration: u64,
}

struct Configuration {
    args: Args,
}

impl Configuration {
    fn parse() -> Self {
        Configuration {
            args: Docopt::new(USAGE)
                .and_then(|d| d.decode())
                .unwrap_or_else(|e| e.exit()),
        }
    }

    fn execute(&self) {
        {
            //start profiling
            if self.args.flag_prof_start != 0 && self.args.flag_prof_duration != 0 {
                let start = self.args.flag_prof_start;
                let duration = self.args.flag_prof_duration;
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
            let url = format!("{}:{}", "0.0.0.0", self.args.flag_jsonrpc_port);
            start_http(url.as_str(),
                       self.args.flag_thread_num,
                       self.args.flag_sleep_duration,
                       self.args.flag_timeout_count);
        }
    }
}

fn start_http(url: &str, thread_num: usize, sleep_duration: u32, timeout_count: u32) {
    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "full");
    dotenv().ok();

    cita_log::format(LogLevelFilter::Info);

    info!("CITA:jsonrpc");

    let new_subscriber = subscriber::MyHandler::new();
    let responses = new_subscriber.responses.clone();
    let tx_responses = new_subscriber.tx_responses.clone();
    let mut pubsub = PubSub::new();
    pubsub.start_sub("jsonrpc", vec!["*.rpc"], new_subscriber);

    let (tx, rx): (Sender<TopicMessage>, Receiver<TopicMessage>) = channel();
    let mut _pub = pubsub.get_pub();
    let new_publisher = publisher::Publisher::new(rx);
    thread::spawn(move || { new_publisher.run(&mut _pub); });

    let arc_tx = Arc::new(Mutex::new(tx));


    info!("Listening on {}", url);
    Server::http(url)
        .unwrap()
        .handle_threads(RpcHandler {
                            responses: responses,
                            tx: arc_tx,
                            tx_responses: tx_responses,
                            sleep_duration: sleep_duration,
                            timeout_count: timeout_count,
                        },
                        thread_num)
        .unwrap();
}

fn main() {
    dotenv().ok();
    Configuration::parse().execute();
}
