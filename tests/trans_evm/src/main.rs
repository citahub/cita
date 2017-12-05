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
#![allow(unused_extern_crates)]
extern crate cita_crypto as crypto;
extern crate hyper;
extern crate jsonrpc_types;
extern crate libproto;
extern crate protobuf;
extern crate rustc_hex;
extern crate serde;
extern crate serde_json;
extern crate util;

extern crate clap;
#[macro_use]
extern crate serde_derive;

mod core;

use clap::App;
use core::param::Param;
use core::send_trans::Sendtx;
//use std::sync::mpsc;

fn main() {
    let matches = App::new("trans_evm")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("--config=[file] 'config file for params'")
        .get_matches();

    let mut filename = "".to_string();

    if let Some(file) = matches.value_of("config") {
        filename = file.parse::<String>().unwrap();
    }

    let p: Param = Param::load_from_file(&filename);
    let work = Sendtx::new(&p);
    match p.category {
        1 => work.start(1),
        2 => work.start(2),
        _ => work.start(3),
    }
}
