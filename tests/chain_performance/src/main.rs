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

extern crate cita_crypto as crypto;
extern crate libproto;
extern crate protobuf;
extern crate util;
extern crate rustc_serialize;
extern crate proof;
extern crate clap;
extern crate core;
extern crate dotenv;
extern crate bincode;
extern crate cpuprofiler;
extern crate common_types;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate cita_log;

mod generate_block;
mod call_chain;

use call_chain::Callchain;
use clap::App;
use core::db;
use core::libchain::Genesis;
use cpuprofiler::PROFILER;
use generate_block::Generateblock;
use log::LogLevelFilter;
use std::{env, time, thread};
use std::sync::Arc;
use std::sync::mpsc::channel;
use util::H256;
use util::kvdb::{Database, DatabaseConfig};

pub const DATA_PATH: &'static str = "DATA_PATH";

//创建合约交易性能
fn create_contract(block_tx_num: i32, call: Callchain, pre_hash: H256, flag_prof_start: u64, flag_prof_duration: u64, flag: i32) {
    let code = "60606040523415600e57600080fd5b5b5b5b60948061001f6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603d575b600080fd5b3415604757600080fd5b605b6004808035906020019091905050605d565b005b806000819055505b505600a165627a7a72305820c471b4376626da2540b2374e8b4110501051c426ff46814a6170ce9e219e49a80029";
    let mut contract_address = "".to_string();
    if flag != 0 {
        contract_address = "ffffffffffffffffffffffffffffffffffffffff".to_string();
    }
    let mut txs = Vec::new();
    for _ in 0..block_tx_num - 1 {
        let tx = Generateblock::generate_tx(code.clone(), contract_address.clone());
        txs.push(tx);
    }
    let tx = Generateblock::generate_tx(code, contract_address.clone());
    let hash = tx.hash();
    txs.push(tx);

    //构造block
    let h = call.get_height() + 1;
    let (send_data, block) = Generateblock::build_block(txs, pre_hash, h as u64);

    info!("===============start add block===============");
    profifer(flag_prof_start, flag_prof_duration);
    let sys_time = time::SystemTime::now();
    call.add_block(block.clone());
    info!("===============end add block===============");
    let duration = sys_time.elapsed().unwrap();
    let secs = duration.as_secs() * 1000000 + (duration.subsec_nanos() / 1000) as u64;
    info!("tx_num = {}, size = {} , use time = {} μs", block_tx_num, send_data.len(), secs);
    info!("receipt = {:?}", call.get_receipt(hash));
}


//发送合约交易性能
#[allow(unused_assignments)]
fn send_contract_tx(block_tx_num: i32, call: Callchain, pre_hash: H256, flag_prof_start: u64, flag_prof_duration: u64) {
    //构造创建合约的交易交易
    let mut code = "60606040523415600e57600080fd5b5b5b5b60948061001f6000396000f30060606040526000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680635524107714603d575b600080fd5b3415604757600080fd5b605b6004808035906020019091905050605d565b005b806000819055505b505600a165627a7a72305820c471b4376626da2540b2374e8b4110501051c426ff46814a6170ce9e219e49a80029";
    let mut contract_address = "".to_string();
    let mut txs = Vec::new();
    let mut hash = H256::default();
    let tx = Generateblock::generate_tx(code.clone(), contract_address.clone());
    hash = tx.hash();
    txs.push(tx);

    //构造block
    {
        let h = call.get_height() + 1;
        let (_, block) = Generateblock::build_block(txs.clone(), pre_hash, h as u64);
        call.add_block(block.clone());
    }

    let addr = call.get_contract_address(hash);
    contract_address = format!("{:?}", addr);
    code = "552410770000000000000000000000000000000000000000000000000000000012345678";
    txs.clear();
    for _ in 0..block_tx_num - 1 {
        let tx = Generateblock::generate_tx(code.clone(), contract_address.clone());
        txs.push(tx);
    }
    let tx = Generateblock::generate_tx(code.clone(), contract_address.clone());
    hash = tx.hash();
    txs.push(tx);

    let h = call.get_height() + 1;
    let pre_hash = call.get_pre_hash();
    let (send_data, block) = Generateblock::build_block(txs.clone(), pre_hash, h as u64);
    info!("===============start add block===============");
    profifer(flag_prof_start, flag_prof_duration);
    let sys_time = time::SystemTime::now();
    call.add_block(block.clone());
    let duration = sys_time.elapsed().unwrap();
    info!("===============end add block===============");
    let secs = duration.as_secs() * 1000000 + (duration.subsec_nanos() / 1000) as u64;
    info!("tx_num = {}, size = {} , use time = {} μs", txs.len(), send_data.len(), secs);
    info!("receipt = {:?}", call.get_receipt(hash));
}


fn profifer(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
    let start = flag_prof_start;
    let duration = flag_prof_duration;
    thread::spawn(move || {
                      thread::sleep(time::Duration::new(start, 0));
                      PROFILER.lock().unwrap().start("./chain_performance.profile").expect("Couldn't start");
                      thread::sleep(time::Duration::new(duration, 0));
                      PROFILER.lock().unwrap().stop().unwrap();
                  });

}


fn main() {
    dotenv::dotenv().ok();
    cita_log::format(LogLevelFilter::Info);
    info!("CITA:chain_performance");
    let matches = App::new("generate_block")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("--tx_num=[4000] 'transation num in block'")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .args_from_usage("-m, --method=[method] 'create | call | store'")
        .args_from_usage("--flag_prof_start=[1] 'prof start time'")
        .args_from_usage("--flag_prof_duration=[1] 'prof run time'")
        .get_matches();

    let block_tx_num = matches.value_of("tx_num").unwrap_or("4000").parse::<i32>().unwrap();
    let config_path = matches.value_of("config").unwrap_or("genesis.json");
    let method = matches.value_of("method").unwrap_or("create");
    let flag_prof_start = matches.value_of("flag_prof_start").unwrap_or("0").parse::<u64>().unwrap();
    let flag_prof_duration = matches.value_of("flag_prof_duration").unwrap_or("0").parse::<u64>().unwrap();

    //数据库配置
    let nosql_path = env::var(DATA_PATH).expect(format!("{} must be set", DATA_PATH).as_str()) + "/nosql";
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &nosql_path).unwrap();
    let genesis = Genesis::init(config_path);

    //chain初始化
    let (sync_tx, _) = channel();
    let call = Callchain::new(Arc::new(db), genesis, sync_tx);
    let pre_hash = call.get_pre_hash();
    match method {
        "create" => create_contract(block_tx_num, call.clone(), pre_hash, flag_prof_start, flag_prof_duration, 0),
        "store" => create_contract(block_tx_num, call.clone(), pre_hash, flag_prof_start, flag_prof_duration, 1),
        "call" | _ => send_contract_tx(block_tx_num, call.clone(), pre_hash, flag_prof_start, flag_prof_duration),
    }
}
