// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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
