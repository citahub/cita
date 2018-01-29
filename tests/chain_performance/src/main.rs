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
extern crate bincode;
extern crate cita_crypto as crypto;
extern crate clap;
extern crate common_types;
extern crate core;
extern crate core_executor;
extern crate cpuprofiler;
extern crate dotenv;
extern crate libproto;
extern crate proof;
extern crate rustc_serialize;
extern crate util;

#[macro_use]
extern crate log;
extern crate logger;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod generate_block;
mod call_exet;

use call_exet::Callexet;
use clap::App;
use core::libchain::*;
use core::libchain::block::Block as ChainBlock;
use core_executor::db;
use core_executor::libexecutor::Genesis;
use cpuprofiler::PROFILER;
use generate_block::Generateblock;
use std::{thread, time};
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::mpsc::channel;
use util::H256;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};

//创建合约交易性能
fn create_contract(
    block_tx_num: i32,
    call: Callexet,
    pre_hash: H256,
    flag_prof_start: u64,
    flag_prof_duration: u64,
    flag: i32,
) {
    let code = "60606040523415600e57600080fd5b5b5b5b60948061001f\
                6000396000f30060606040526000357c0100000000000000\
                000000000000000000000000000000000000000000900463\
                ffffffff1680635524107714603d575b600080fd5b341560\
                4757600080fd5b605b600480803590602001909190505060\
                5d565b005b806000819055505b505600a165627a7a723058\
                20c471b4376626da2540b2374e8b4110501051c426ff4681\
                4a6170ce9e219e49a80029";
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
    profiler(flag_prof_start, flag_prof_duration);
    let sys_time = time::SystemTime::now();
    let (ctx_pub, recv) = channel::<(String, Vec<u8>)>();
    let inblock = block.clone();
    let inchain = call.chain.clone();
    thread::spawn(move || {
        loop {
            if let Ok((_, msg_vec)) = recv.recv() {
                let mut msg = Message::try_from(&msg_vec).unwrap();
                match msg.take_content() {
                    MsgClass::EXECUTED(info) => {
                        //info!("**** get excuted info {:?}", info);
                        let pro = inblock.protobuf();
                        let chan_block = ChainBlock::from(pro);
                        inchain.set_block_body(h, &chan_block);
                        inchain.set_db_result(&info, &chan_block);
                    }

                    _ => {}
                }
            } else {
                break;
            }
        }
    });
    call.add_block(block.clone(), &ctx_pub);
    info!("===============end add block===============");
    let duration = sys_time.elapsed().unwrap();
    let secs = duration.as_secs() * 1000000 + (duration.subsec_nanos() / 1000) as u64;
    info!(
        "tx_num = {}, size = {} , use time = {} μs",
        block_tx_num,
        send_data.len(),
        secs
    );

    std::thread::sleep(std::time::Duration::new(5, 0));
    info!("receipt = {:?}", call.get_receipt(hash));
}

//发送合约交易性能
#[allow(unused_assignments)]
fn send_contract_tx(block_tx_num: i32, call: Callexet, pre_hash: H256, flag_prof_start: u64, flag_prof_duration: u64) {
    //构造创建合约的交易交易
    let mut code = "60606040523415600e57600080fd5b5b5b5b609480\
                    61001f6000396000f30060606040526000357c0100\
                    000000000000000000000000000000000000000000\
                    000000000000900463ffffffff1680635524107714\
                    603d575b600080fd5b3415604757600080fd5b605b\
                    6004808035906020019091905050605d565b005b80\
                    6000819055505b505600a165627a7a72305820c471\
                    b4376626da2540b2374e8b4110501051c426ff4681\
                    4a6170ce9e219e49a80029";
    let mut contract_address = "".to_string();
    let mut txs = Vec::new();
    let mut hash = H256::default();
    let tx = Generateblock::generate_tx(code.clone(), contract_address.clone());
    hash = tx.hash();
    txs.push(tx);

    let (ctx_pub, recv) = channel::<(String, Vec<u8>)>();

    let h = call.get_height() + 1;
    let (_, block) = Generateblock::build_block(txs.clone(), pre_hash, h as u64);
    let inblock = block.clone();
    let inchain = call.chain.clone();
    thread::spawn(move || loop {
        if let Ok((_, msg_vec)) = recv.recv() {
            let mut msg = Message::try_from(&msg_vec).unwrap();
            match msg.take_content() {
                MsgClass::EXECUTED(info) => {
                    let pro = inblock.protobuf();
                    let chan_block = ChainBlock::from(pro);
                    inchain.set_block_body(h, &chan_block);
                    inchain.set_db_result(&info, &chan_block);
                }

                _ => {}
            }
        } else {
            break;
        }
    });

    //构造block
    {
        call.add_block(block.clone(), &ctx_pub);
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
    profiler(flag_prof_start, flag_prof_duration);
    let sys_time = time::SystemTime::now();
    call.add_block(block.clone(), &ctx_pub);
    let duration = sys_time.elapsed().unwrap();
    info!("===============end add block===============");
    let secs = duration.as_secs() * 1000000 + (duration.subsec_nanos() / 1000) as u64;
    info!(
        "tx_num = {}, size = {} , use time = {} μs",
        txs.len(),
        send_data.len(),
        secs
    );

    std::thread::sleep(std::time::Duration::new(5, 0));
    info!("receipt = {:?}", call.get_receipt(hash));
}

fn profiler(flag_prof_start: u64, flag_prof_duration: u64) {
    //start profiling
    let start = flag_prof_start;
    let duration = flag_prof_duration;
    thread::spawn(move || {
        thread::sleep(time::Duration::new(start, 0));
        PROFILER
            .lock()
            .unwrap()
            .start("./chain_performance.profile")
            .expect("Couldn't start");
        thread::sleep(time::Duration::new(duration, 0));
        PROFILER.lock().unwrap().stop().unwrap();
    });
}

fn main() {
    dotenv::dotenv().ok();
    logger::init();
    info!("CITA:chain_performance");
    let matches = App::new("generate_block")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("--tx_num=[4000] 'transation num in block'")
        .arg_from_usage("-g, --genesis=[FILE] 'Sets a custom config file'")
        .arg_from_usage("-m, --method=[method] 'create | call | store'")
        .arg_from_usage("--flag_prof_start=[1] 'prof start time'")
        .arg_from_usage("--flag_prof_duration=[1] 'prof run time'")
        .arg_from_usage("-c, --config=[FILE] 'Sets a check config file'")
        .get_matches();

    let block_tx_num = matches
        .value_of("tx_num")
        .unwrap_or("4000")
        .parse::<i32>()
        .unwrap();
    let genesis_path = matches.value_of("genesis").unwrap_or("genesis.json");
    let config_path = matches.value_of("config").unwrap_or("chain.toml");
    let method = matches.value_of("method").unwrap_or("create");
    let flag_prof_start = matches
        .value_of("flag_prof_start")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap();
    let flag_prof_duration = matches
        .value_of("flag_prof_duration")
        .unwrap_or("0")
        .parse::<u64>()
        .unwrap();

    //数据库配置
    let nosql_path = DataPath::nosql_path();
    let state_path = DataPath::state_path();
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &state_path).unwrap();
    let chain_db = Database::open(&config, &nosql_path).unwrap();
    let genesis = Genesis::init(genesis_path);

    let chain_config = chain::Config::new(config_path);
    let chain = Arc::new(chain::Chain::init_chain(Arc::new(chain_db), chain_config));

    //chain初始化
    let call = Callexet::new(Arc::new(db), chain, genesis, config_path);
    let pre_hash = call.get_pre_hash();
    match method {
        "create" => create_contract(
            block_tx_num,
            call.clone(),
            pre_hash,
            flag_prof_start,
            flag_prof_duration,
            0,
        ),
        "store" => create_contract(
            block_tx_num,
            call.clone(),
            pre_hash,
            flag_prof_start,
            flag_prof_duration,
            1,
        ),
        "call" | _ => send_contract_tx(
            block_tx_num,
            call.clone(),
            pre_hash,
            flag_prof_start,
            flag_prof_duration,
        ),
    }
}
