// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

extern crate rustc_serialize;
extern crate tempdir;

use self::rustc_serialize::hex::FromHex;
use self::tempdir::TempDir;
use cita_crypto::PrivKey;
use cita_types::traits::LowerHex;
use cita_types::{Address, U256};
use core::libchain::chain;
use crossbeam_channel::{Receiver, Sender};
use db;
use journaldb;
use libexecutor::block::{Block, BlockBody, ClosedBlock, OpenBlock};
use libexecutor::command;
use libexecutor::executor::Executor;
use libexecutor::genesis::Genesis;
use libexecutor::genesis::Spec;
use libproto::blockchain;
use serde_json;
use state::State;
use state_db::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use types::header::OpenHeader;
use types::transaction::SignedTransaction;
use util::kvdb::{Database, DatabaseConfig};
use util::AsMillis;
use util::KeyValueDB;

const CHAIN_CONFIG: &str = "chain.toml";
const SCRIPTS_DIR: &str = "../../scripts";

pub fn get_temp_state() -> State<StateDB> {
    let state_db = get_temp_state_db();
    State::new(state_db, 0.into(), Default::default())
}

fn new_db() -> Arc<KeyValueDB> {
    Arc::new(::util::kvdb::in_memory(8))
}

pub fn get_temp_state_db() -> StateDB {
    let db = new_db();
    let journal_db = journaldb::new(db, journaldb::Algorithm::Archive, ::db::COL_STATE);
    StateDB::new(journal_db, 5 * 1024 * 1024)
}

pub fn solc(name: &str, source: &str) -> (Vec<u8>, Vec<u8>) {
    // input and output of solc command
    let output_dir = TempDir::new("solc_output").unwrap().into_path();
    let contract_file = output_dir.join("contract.sol");

    let deploy_code_file = output_dir.join([name, ".bin"].join(""));
    let runtime_code_file = output_dir.join([name, ".bin-runtime"].join(""));

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
    let deploy_code = content.as_str().from_hex().unwrap();

    // read runtime code
    let mut content = String::new();
    File::open(runtime_code_file)
        .expect("failed to open deploy code file!")
        .read_to_string(&mut content)
        .expect("failed to read binary");
    let runtime_code = content.from_hex().unwrap();
    (deploy_code, runtime_code)
}

pub fn init_executor(contract_arguments: Vec<(&str, &str)>) -> Executor {
    let (_fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
    let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
    let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
    let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
    init_executor2(
        contract_arguments,
        fsm_req_receiver,
        fsm_resp_sender,
        command_req_receiver,
        command_resp_sender,
    )
}

pub fn init_executor2(
    contract_arguments: Vec<(&str, &str)>,
    fsm_req_receiver: Receiver<OpenBlock>,
    fsm_resp_sender: Sender<ClosedBlock>,
    command_req_receiver: Receiver<command::Command>,
    command_resp_sender: Sender<command::CommandResp>,
) -> Executor {
    // FIXME temp dir should be removed automatically, but at present it is not
    let tempdir = TempDir::new("init_executor").unwrap().into_path();
    let create_init_data_py = Path::new(SCRIPTS_DIR).join("config_tool/create_init_data.py");
    let create_genesis_py = Path::new(SCRIPTS_DIR).join("config_tool/create_genesis.py");
    let contracts_dir = Path::new(SCRIPTS_DIR).join("contracts");
    let mut init_data_yml = tempdir.clone();
    init_data_yml.push("init_data.yml");
    let mut genesis_json = tempdir.clone();
    genesis_json.push("genesis.json");

    let contract_arguments = contract_arguments
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>();
    let mut init_data_args: Vec<&str> = vec![
        create_init_data_py.to_str().unwrap(),
        "--output",
        init_data_yml.to_str().unwrap(),
    ];
    if !contract_arguments.is_empty() {
        init_data_args.push("--contract_arguments");
        contract_arguments.iter().for_each(|arg| {
            init_data_args.push(arg);
        });
    }

    fn check_command_output(output: Output) {
        if !output.status.success() {
            panic!(
                "\n[stderr]: {}\n[stdout]: {}",
                String::from_utf8_lossy(output.stderr.as_slice()),
                String::from_utf8_lossy(output.stdout.as_slice()),
            );
        }
    }

    let output = Command::new("python3")
        .args(init_data_args.as_slice())
        .output()
        .expect("Failed to create init data");

    check_command_output(output);
    let output = Command::new("python3")
        .args(&[
            create_genesis_py.to_str().unwrap(),
            "--output",
            genesis_json.to_str().unwrap(),
            "--init_data_file",
            init_data_yml.to_str().unwrap(),
            "--contracts_dir",
            contracts_dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to create init data");
    check_command_output(output);

    // Load from genesis json file
    println!("genesis_json: {}", genesis_json.to_str().unwrap());
    let genesis_file = File::open(genesis_json.to_str().unwrap()).unwrap();
    let spec: Spec = serde_json::from_reader(genesis_file).expect("Failed to load genesis.");
    let _genesis = Genesis {
        spec,
        block: Block::default(),
    };

    let mut data_path = tempdir.clone();
    data_path.push("data");
    env::set_var("DATA_PATH", data_path);
    let executor = Executor::init(
        genesis_json.to_str().unwrap(),
        "archive",
        5 * 1024 * 1024,
        tempdir.to_str().unwrap().to_string(),
        fsm_req_receiver,
        fsm_resp_sender,
        command_req_receiver,
        command_resp_sender,
    );
    executor
}

pub fn init_chain() -> Arc<chain::Chain> {
    let tempdir = TempDir::new("solc_output").unwrap().into_path();
    let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
    let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
    let chain_config = chain::Config::new(CHAIN_CONFIG);
    Arc::new(chain::Chain::init_chain(Arc::new(db), &chain_config))
}

pub fn create_block(
    executor: &Executor,
    to: Address,
    data: &Vec<u8>,
    nonce: (u32, u32),
    privkey: &PrivKey,
) -> OpenBlock {
    let mut block = OpenBlock::default();

    block.set_parent_hash(executor.get_current_hash());
    block.set_timestamp(AsMillis::as_millis(&UNIX_EPOCH.elapsed().unwrap()));
    block.set_number(executor.get_current_height() + 1);
    // header.proof= ?;

    let mut body = BlockBody::default();
    let mut txs = Vec::new();

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

        let stx = tx.sign(*privkey);
        let new_tx = SignedTransaction::new(&stx).unwrap();
        txs.push(new_tx);
    }
    body.set_transactions(txs);
    block.set_body(body);
    block
}

pub fn generate_contract() -> Vec<u8> {
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
    data
}

pub fn generate_block_header() -> OpenHeader {
    OpenHeader::default()
}

pub fn generate_block_body() -> BlockBody {
    let mut stx = SignedTransaction::default();
    use types::transaction::SignedTransaction;
    stx.data = vec![1; 200];
    let transactions = vec![stx; 200];
    BlockBody { transactions }
}

pub fn generate_default_block() -> OpenBlock {
    let block_body = generate_block_body();
    let block_header = generate_block_header();
    OpenBlock {
        body: block_body,
        header: block_header,
    }
}
