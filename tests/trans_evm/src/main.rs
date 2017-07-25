#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
extern crate cita_crypto as crypto;
extern crate libproto;
extern crate protobuf;
extern crate util;
extern crate uuid;
extern crate serde_types;
extern crate rustc_serialize;
extern crate hyper;
extern crate serde_json;
extern crate jsonrpc_types;

#[macro_use]
extern crate serde_derive;
extern crate clap;

mod core;

use core::param::Param;
use clap::App;
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

    let p:Param = Param::load_from_file(&filename);
    let work = Sendtx::new(&p);
    match p.category{
        1 => work.start(1),
        _ => work.start(2),
    }


}
