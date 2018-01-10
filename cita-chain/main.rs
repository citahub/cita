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

#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(unused_must_use)]
#![feature(custom_attribute)]
#![feature(refcell_replace_swap)]
extern crate byteorder;
extern crate clap;
extern crate common_types as types;
extern crate core;
extern crate dotenv;
extern crate error;
extern crate jsonrpc_types;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate proof;
extern crate protobuf;
extern crate pubsub;
extern crate serde_json;
#[macro_use]
extern crate util;

mod forward;
mod block_processor;

use block_processor::BlockProcessor;
use clap::App;
use core::db;
use core::libchain;
//use core::libchain::Genesis;
use forward::Forward;
use pubsub::start_pubsub;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::time;
use std::time::Duration;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};
use util::set_panic_handler;

fn main() {
    micro_service_init!("cita-chain", "CITA:chain");
    let matches = App::new("chain")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-g, --genesis=[FILE] 'Sets a genesis config file")
        .arg_from_usage("-c, --config=[FILE] 'Sets a switch config file'")
        .get_matches();

    //let mut genesis_path = "genesis";
    if let Some(ge) = matches.value_of("genesis") {
        trace!("Value for genesis: {}", ge);
        // genesis_path = ge;
    }

    let mut config_path = "config";
    if let Some(c) = matches.value_of("config") {
        trace!("Value for config: {}", c);
        config_path = c;
    }

    let (tx, rx) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub(
        "chain",
        vec![
            "net.blk",
            "net.sync",
            "consensus.blk",
            "jsonrpc.request",
            "auth.blk_tx_hashs_req",
            "executor.result",
        ],
        tx,
        crx_pub,
    );

    let nosql_path = DataPath::nosql_path();
    trace!("nosql_path is {:?}", nosql_path);
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &nosql_path).unwrap();
    //let genesis = Genesis::init(genesis_path);
    let config_file = File::open(config_path).unwrap();
    let chain = Arc::new(libchain::chain::Chain::init_chain(
        Arc::new(db),
        BufReader::new(config_file),
    ));

    let (write_sender, write_receiver) = channel();
    let forward = Forward::new(Arc::clone(&chain), ctx_pub.clone(), write_sender);

    let block_processor = BlockProcessor::new(Arc::clone(&chain), ctx_pub);
    block_processor.broadcast_current_status();

    //chain 读写分离
    //chain 读数据 => 查询数据
    thread::spawn(move || loop {
        if let Ok((key, msg)) = rx.recv() {
            forward.dispatch_msg(&key, &msg);
        }
    });

    //chain 写数据 => 添加块
    thread::spawn(move || {
        loop {
            if let Ok(einfo) = write_receiver.recv_timeout(Duration::new(18, 0)) {
                block_processor.set_excuted_result(einfo);
            } else {
                //here maybe need send blockbody when max_store_height > max_height
                block_processor.broadcast_current_block();
            }
        }
    });

    //garbage collect
    let mut i: u32 = 0;
    loop {
        thread::sleep(time::Duration::from_millis(10_000));
        if i > 100 {
            chain.collect_garbage();
            i = 0;
        }
        i += 1;
    }
}
