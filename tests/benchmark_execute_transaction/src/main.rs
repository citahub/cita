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

extern crate cita_crypto;
extern crate cita_types;
extern crate clap;
extern crate common_types as types;
extern crate core_executor;
extern crate dotenv;
extern crate libproto;
#[macro_use]
extern crate log;
extern crate logger;
extern crate mktemp;
extern crate rustc_serialize;
extern crate serde_json;
extern crate util;

use cita_crypto::KeyPair;
use cita_types::traits::LowerHex;
use core_executor::libexecutor::block::{Block, BlockBody, Drain, OpenBlock};
use mktemp::Temp;
use rustc_serialize::hex::FromHex;
//use core_executor::libexecutor::block::{Block, BlockBody};
use core_executor::db;
use core_executor::db::*;
use core_executor::env_info::LastHashes;
use core_executor::libexecutor::executor::{Config, Executor};
use core_executor::libexecutor::genesis::Genesis;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
//use std::sync::mpsc::channel;
//use std::thread;
use cita_types::{Address, H256, U256};
use clap::App;
use libproto::blockchain;
use std::time::Instant;
use types::transaction::SignedTransaction;
use util::AsMillis;
use util::crypto::CreateKey;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig};

pub fn solc(name: &str, source: &str) -> (Vec<u8>, Vec<u8>) {
    // input and output of solc command
    let contract_file = Temp::new_file().unwrap().to_path_buf();
    let output_dir = Temp::new_dir().unwrap().to_path_buf();
    let deploy_code_file = output_dir.clone().join([name, ".bin"].join(""));
    let runtime_code_file = output_dir.clone().join([name, ".bin-runtime"].join(""));

    // prepare contract file
    let mut file = File::create(contract_file.clone()).unwrap();
    let mut content = String::new();
    file.write_all(source.as_ref()).expect("failed to write");

    // execute solc command
    Command::new("solc")
        .arg(contract_file.clone())
        .arg("--bin")
        .arg("--bin-runtime")
        .arg("-o")
        .arg(output_dir)
        .output()
        .expect("failed to execute solc");

    // read deploy code
    File::open(deploy_code_file)
        .expect("failed to open deploy code file!")
        .read_to_string(&mut content)
        .expect("failed to read binary");
    trace!("deploy code: {}", content);
    let deploy_code = content.as_str().from_hex().unwrap();

    // read runtime code
    let mut content = String::new();
    File::open(runtime_code_file)
        .expect("failed to open deploy code file!")
        .read_to_string(&mut content)
        .expect("failed to read binary");
    trace!("runtime code: {}", content);
    let runtime_code = content.from_hex().unwrap();
    (deploy_code, runtime_code)
}

pub struct CurrentHash;

impl Key<H256> for CurrentHash {
    type Target = H256;

    fn key(&self) -> H256 {
        H256::from("7cabfb7709b29c16d9e876e876c9988d03f9c3414e1d3ff77ec1de2d0ee59f66")
    }
}

pub fn create_block(executor: &Executor, to: Address, data: &Vec<u8>, nonce: (u32, u32), is_change_pv: bool) -> Block {
    let mut block = Block::new();

    block.set_parent_hash(executor.get_current_hash());
    block.set_timestamp(UNIX_EPOCH.elapsed().unwrap().as_millis());
    block.set_number(executor.get_current_height() + 1);
    // header.proof= ?;

    let mut body = BlockBody::new();
    let mut txs = Vec::new();
    let mut keypair = KeyPair::gen_keypair();
    let mut privkey = keypair.privkey().clone();

    for i in nonce.0..nonce.1 {
        let mut tx = blockchain::Transaction::new();
        if to == Address::from(0) {
            tx.set_to(String::from(""));
        } else {
            tx.set_to(to.lower_hex());
        }
        tx.set_nonce(U256::from(i).lower_hex());
        tx.set_data(data.clone());
        tx.set_valid_until_block(100);
        tx.set_quota(1844674);

        let stx = tx.sign(privkey);
        let new_tx = SignedTransaction::new(&stx).unwrap();
        txs.push(new_tx);
        if is_change_pv {
            keypair = KeyPair::gen_keypair();
            privkey = keypair.privkey().clone();
        }
    }
    body.set_transactions(txs);
    block.set_body(body);
    block
}

pub fn init_executor(config_path: &str, genesis_path: &str) -> Arc<Executor> {
    let state_path = DataPath::state_path();
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &state_path).unwrap();

    let executor_config = Config::new(config_path);
    let genesis = Genesis::init(genesis_path);
    Arc::new(Executor::init_executor(
        Arc::new(db),
        genesis,
        executor_config,
    ))
}

fn bench_execute_trans(config_path: &str, genesis_path: &str, trans_num: u32, is_change_pv: bool, max_h: u32) {
    let source = r#"
        pragma solidity ^0.4.8;
        contract ConstructSol {
            uint a;
            event LogCreate(address contractAddr);
            event A(uint);
            function ConstructSol(){
                LogCreate(this);
            }

            function set(uint _a) {
                a = _a;
                A(a);
            }

            function get() returns (uint) {
                return a;
            }
        }
    "#;
    let (data, _) = solc("ConstructSol", source);

    let mut result = format!("height,execute,state_root,db write(state),db write(header CurrentHash)\n");

    let path = Path::new("analysis_result.csv");
    let mut file = match File::create(&path) {
        Err(_) => panic!("create fail"),
        Ok(file) => file,
    };
    match file.write_all(result.as_bytes()) {
        Err(_) => info!("write fail"),
        Ok(_) => (),
    }

    let mut exit: u32 = 0;

    let ext = init_executor(config_path, genesis_path);
    loop {
        if exit > max_h {
            break;
        }
        let mut block = create_block(&ext, Address::from(0), &data, (0, trans_num), is_change_pv);
        let current_height = ext.get_current_height();
        info!("current height: {}", current_height);
        let conf = ext.get_sys_config(current_height);
        let check_permission = conf.check_permission;
        let check_quota = conf.check_quota;
        let current_state_root = ext.current_state_root();
        let last_hashes = LastHashes::from(ext.last_hashes.read().clone());

        let mut open_block = OpenBlock::new(
            ext.factories.clone(),
            conf,
            false,
            block.clone(),
            ext.state_db.boxed_clone(),
            current_state_root,
            last_hashes.into(),
        ).unwrap();

        //execute transactions
        let now = Instant::now();
        let mut transactions = Vec::with_capacity(block.body.transactions.len());
        for (_, mut t) in block.body.transactions.clone().into_iter().enumerate() {
            // Apply transaction and set account nonce
            open_block.apply_transaction(&mut t, check_permission, check_quota);
            transactions.push(t);
        }
        let new_now = Instant::now();
        let execute_duration = new_now.duration_since(now);
        info!("execute transactions use {:?}", execute_duration);
        block.body.set_transactions(transactions);

        //generate mpt， get state root
        let now = Instant::now();
        open_block.state.commit().expect("commit trie error");
        let new_now = Instant::now();
        let state_duration = new_now.duration_since(now);
        info!("state root use {:?}", state_duration);
        let gas_used = open_block.current_gas_used;
        open_block.set_gas_used(gas_used);

        //save data to db
        let closed_block = open_block.close();

        //write state root
        let now = Instant::now();
        let height = closed_block.number();
        let hash = closed_block.hash();
        let mut batch = ext.db.transaction();

        let header = closed_block.header().clone();
        {
            *ext.current_header.write() = header;
        }

        let mut state = closed_block.drain();
        state
            .journal_under(&mut batch, height, &hash)
            .expect("DB commit failed");
        ext.db.write(batch).expect("DB write failed.");
        let new_now = Instant::now();
        let db_write_duration = new_now.duration_since(now);
        info!("db write use {:?}", db_write_duration);

        //write header CurrentHash
        let now = Instant::now();
        let mut batch = ext.db.transaction();
        batch.write(db::COL_HEADERS, &hash, block.header());
        batch.write(db::COL_EXTRA, &CurrentHash, &hash);
        batch.write(db::COL_EXTRA, &height, &hash);
        ext.db.write(batch).expect("DB write failed.");
        let new_now = Instant::now();
        let db_write2_duration = new_now.duration_since(now);
        info!("db write2 use {:?}", db_write2_duration);

        {
            let mut hashes = ext.last_hashes.write();
            if hashes.len() > 255 {
                hashes.pop_back();
            }
            hashes.push_front(ext.get_current_hash());
        }

        let ext_secs = execute_duration.as_secs() as f64 + execute_duration.subsec_nanos() as f64 * 1e-9;
        let state_secs = state_duration.as_secs() as f64 + state_duration.subsec_nanos() as f64 * 1e-9;
        let db_secs = db_write_duration.as_secs() as f64 + db_write_duration.subsec_nanos() as f64 * 1e-9;
        let db2_secs = db_write2_duration.as_secs() as f64 + db_write2_duration.subsec_nanos() as f64 * 1e-9;

        result = format!(
            "{},{},{},{},{}\n",
            current_height + 1,
            ext_secs,
            state_secs,
            db_secs,
            db2_secs
        );
        match file.write_all(result.as_bytes()) {
            Err(_) => info!("write fail"),
            Ok(_) => (),
        }
        exit = exit + 1;
    }
}

fn main() {
    dotenv::dotenv().ok();
    logger::init();
    info!("CITA:benchmark_execute_transaction");
    let matches = App::new("generate_block")
        .version("0.1")
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .arg_from_usage("-n, --tx_num=[4000] 'transation num in block'")
        .arg_from_usage("-c, --config=[FILE] 'Sets a check config file'")
        .arg_from_usage("-g, --genesis=[FILE] 'Sets a custom config file'")
        .arg_from_usage("-b, --is_change_pv=[true] 'pv is or is‘t change'")
        .arg_from_usage("-h, --max_height=[30] 'Sets a custom config file'")
        .get_matches();

    let block_tx_num = matches
        .value_of("tx_num")
        .unwrap_or("30000")
        .parse::<u32>()
        .unwrap();

    let max_h = matches
        .value_of("max_height")
        .unwrap_or("30")
        .parse::<u32>()
        .unwrap();

    let is_change_pv = matches
        .value_of("is_change_pv")
        .unwrap_or("true")
        .parse::<bool>()
        .unwrap();

    let genesis_path = matches.value_of("genesis").unwrap_or("genesis.json");
    let config_path = matches.value_of("config").unwrap_or("executor.json");
    bench_execute_trans(config_path, genesis_path, block_tx_num, is_change_pv, max_h);
}
