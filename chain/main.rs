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

#![allow(unused_must_use)]
extern crate core;
#[macro_use]
extern crate log;
extern crate libproto;
extern crate pubsub;
extern crate util;
extern crate clap;
extern crate dotenv;
extern crate logger;
extern crate jsonrpc_types;
extern crate common_types as types;
extern crate byteorder;
extern crate serde_json;
extern crate protobuf;
extern crate error;
extern crate proof;

mod forward;
mod synchronizer;

use clap::App;
use core::db;
use core::libchain;
use core::libchain::Genesis;
use core::libchain::submodules;
use forward::*;
use libproto::blockchain::Status;
use protobuf::Message;
use pubsub::start_pubsub;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time;
use std::time::Duration;
use synchronizer::Synchronizer;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};
use util::panichandler::set_panic_handler;

fn main() {
    dotenv::dotenv().ok();

    // Always print backtrace on panic.
    ::std::env::set_var("RUST_BACKTRACE", "full");

    //exit process when panic
    set_panic_handler();

    logger::init();
    info!("CITA:chain");
    let matches = App::new("chain")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-g, --genesis=[FILE] 'Sets a genesis config file")
        .arg_from_usage("-c, --config=[FILE] 'Sets a switch config file'")
        .get_matches();

    let mut genesis_path = "genesis";
    if let Some(ge) = matches.value_of("genesis") {
        trace!("Value for genesis: {}", ge);
        genesis_path = ge;
    }

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub("chain", vec!["net.blk", "net.status", "net.sync", "consensus.blk", "jsonrpc.request", "auth.blk_tx_hashs_req"], tx, crx_pub);

    let nosql_path = DataPath::nosql_path();
    trace!("nosql_path is {:?}", nosql_path);
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &nosql_path).unwrap();
    let genesis = Genesis::init(genesis_path);
    let (sync_tx, sync_rx) = channel();
    let config_file = File::open(config_path).unwrap();
    let (chain, st) = libchain::chain::Chain::init_chain(Arc::new(db), genesis, sync_tx, BufReader::new(config_file));

    let msg = factory::create_msg(submodules::CHAIN, topics::RICH_STATUS, communication::MsgType::RICH_STATUS, st.write_to_bytes().unwrap());
    info!("init status {:?}, {:?}", st.get_height(), st.get_hash());
    ctx_pub.send(("chain.richstatus".to_string(), msg.write_to_bytes().unwrap())).unwrap();

    let status: Status = st.into();
    let sync_msg = factory::create_msg(submodules::CHAIN, topics::NEW_STATUS, communication::MsgType::STATUS, status.write_to_bytes().unwrap());
    trace!("chain.status {:?}, {:?}", status.get_height(), status.get_hash());
    ctx_pub.send(("chain.status".to_string(), sync_msg.write_to_bytes().unwrap())).unwrap();

    let synchronizer = Synchronizer::new(chain.clone());
    let block_tx_hashes = chain.block_tx_hashes(status.get_height()).expect("shoud return current block tx hashes");
    chain.sync_block_tx_hashes(status.get_height(), block_tx_hashes, &ctx_pub);
    let chain1 = chain.clone();
    let ctx_pub1 = ctx_pub.clone();
    thread::spawn(move || loop {
                      let chain = chain1.clone();
                      forward::chain_result(chain, &rx, &ctx_pub1);
                  });

    thread::spawn(move || loop {
                      let notify = sync_rx.recv_timeout(Duration::new(8, 0));
                      if notify.is_ok() {
                          synchronizer.sync(&ctx_pub);
                      } else {
                          synchronizer.sync_status(&ctx_pub);
                      }
                  });
    //garbage collect
    let mut i: u32 = 0;
    loop {
        thread::sleep(time::Duration::from_millis(10000));
        if i > 100 {
            chain.collect_garbage();
            i = 0;
        }
        i += 1;
    }
}
