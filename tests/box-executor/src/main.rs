// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cita_crypto as crypto;
#[macro_use]
extern crate libproto;
#[macro_use]
extern crate cita_logger as logger;

#[macro_use]
extern crate serde_derive;

mod config;
mod generate_block;
mod runner;

use crate::config::Config;
use clap::App;
use std::env;

fn main() {
    dotenv::dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
    logger::init_config(&logger::LogFavour::File("box_executor"));
    let matches = App::new("mock-consensus")
        .arg(
            clap::Arg::with_name("mock-data")
                .short("m")
                .long("mock-data")
                .required(true)
                .takes_value(true)
                .help(".yaml which contains blocks data"),
        )
        .get_matches();
    let path = matches.value_of("mock-data").unwrap();
    info!("mock-data-path={}", path);
    info!("AMQP_URL={}", env::var("AMQP_URL").expect("AMQP_URL empty"));

    let config = Config::init(path);
    runner::run(config);
}
