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

extern crate mktemp;
extern crate env_logger;
extern crate rustc_serialize;

use self::mktemp::Temp;
use self::rustc_serialize::hex::FromHex;
use cita_crypto::KeyPair;
use db;

use journaldb;
use serde_json;
use libchain::block::{Block, BlockBody};
use libchain::chain::Chain;
use libchain::genesis::Genesis;
use libchain::genesis::Spec;
use libproto::blockchain;
use state::State;
use state_db::*;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::{UNIX_EPOCH, Instant};
use test::black_box;
use types::transaction::SignedTransaction;
use util::{U256, Address};
use util::KeyValueDB;
use util::crypto::CreateKey;
use util::kvdb::{Database, DatabaseConfig};
use std::io::BufReader;

pub fn get_temp_state() -> State<StateDB> {
    let journal_db = get_temp_state_db();
    State::new(journal_db, 0.into(), Default::default())
}

fn new_db() -> Arc<KeyValueDB> {
    Arc::new(::util::kvdb::in_memory(8))
}

pub fn get_temp_state_db() -> StateDB {
    let db = new_db();
    let journal_db = journaldb::new(db, journaldb::Algorithm::Archive, ::db::COL_STATE);
    StateDB::new(journal_db)
}


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

pub fn init_chain() -> Arc<Chain> {
    let _ = env_logger::init();
    let tempdir = mktemp::Temp::new_dir().unwrap().to_path_buf();
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
    // Load from genesis json file
    let genesis_file = File::open("genesis.json").unwrap();
    let fconfig = BufReader::new(genesis_file);
    let spec: Spec = serde_json::from_reader(fconfig).expect("Failed to load genesis.");
    let genesis = Genesis {
        spec: spec,
        block: Block::default(),
    };
    let (sync_tx, _) = channel();
    let path = "chain.json";
    let (chain, _) = Chain::init_chain(Arc::new(db), genesis, sync_tx, path);
    chain
}

pub fn create_block(chain: &Chain, to: Address, data: &Vec<u8>, nonce: (u32, u32)) -> Block {

    let mut block = Block::new();

    block.set_parent_hash(chain.get_current_hash());
    block.set_timestamp(UNIX_EPOCH.elapsed().unwrap().as_secs());
    block.set_number(chain.get_current_height() + 1);
    // header.proof= ?;

    let mut body = BlockBody::new();
    let mut txs = Vec::new();
    for i in nonce.0..nonce.1 {
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();

        let mut tx = blockchain::Transaction::new();
        if to == Address::from(0) {
            tx.set_to(String::from(""));
        } else {
            tx.set_to(to.hex());
        }
        tx.set_nonce(U256::from(i).to_hex());
        tx.set_data(data.clone());
        tx.set_valid_until_block(100);
        tx.set_quota(1844674);

        let stx = tx.sign(*privkey);
        let new_tx = SignedTransaction::new(&stx).unwrap();
        txs.push(new_tx);
    }
    body.set_transactions(txs);
    block.set_body(body);
    block
}

pub fn bench_chain(code: &Vec<u8>, data: &Vec<u8>, tpb: u32, native_address: Address) -> u64 {
    let chain = init_chain();

    // 1) deploy contract
    let block = create_block(&chain, Address::from(0), code, (0, 1));
    chain.set_block(block.clone());

    // 2) execute contract
    let mut nonce = 1;
    let txhash = block.body().transactions()[0].hash();
    let receipt = chain.localized_receipt(txhash).expect("no receipt found");
    let addr = if native_address == Address::zero() { receipt.contract_address.unwrap() } else { native_address };
    let bench = |to: Address, tpb: u32, nonce: u32, data: &Vec<u8>| -> u64 {
        let block = create_block(&chain, to, data, (nonce, tpb + nonce));
        let start = Instant::now();
        black_box(chain.set_block(block));
        let elapsed = start.elapsed();
        chain.collect_garbage();
        u64::from(tpb) * 1_000_000_000 / (elapsed.as_secs() * 1_000_000_000 + u64::from(elapsed.subsec_nanos()))
    };

    let blocks = 10;
    (0..blocks).fold(0, |total, _| {
        let tps = bench(addr, tpb, nonce, data);
        nonce += tpb;
        total + tps
    }) / blocks
}
