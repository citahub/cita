// Copyrighttape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate rustc_serialize;
extern crate tempdir;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::UNIX_EPOCH;

use self::rustc_serialize::hex::FromHex;
use self::tempdir::TempDir;

use crate::libexecutor::block::{BlockBody, ClosedBlock, OpenBlock};
use crate::libexecutor::command;
use crate::libexecutor::executor::Executor;
use crate::types::header::OpenHeader;
use crate::types::transaction::SignedTransaction;

use cita_crypto::PrivKey;
use cita_types::traits::LowerHex;
use cita_types::{Address, U256};
use cita_vm::{state::MemoryDB, state::State};
use crossbeam_channel::{Receiver, Sender};
use libproto::blockchain;
use util::AsMillis;

const SCRIPTS_DIR: &str = "../../scripts";

pub fn get_temp_state() -> State<MemoryDB> {
    let db = Arc::new(MemoryDB::new(false));
    State::new(db).unwrap()
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

pub fn init_executor() -> Executor {
    let (_fsm_req_sender, fsm_req_receiver) = crossbeam_channel::unbounded();
    let (fsm_resp_sender, _fsm_resp_receiver) = crossbeam_channel::unbounded();
    let (_command_req_sender, command_req_receiver) = crossbeam_channel::bounded(0);
    let (command_resp_sender, _command_resp_receiver) = crossbeam_channel::bounded(0);
    init_executor2(
        fsm_req_receiver,
        fsm_resp_sender,
        command_req_receiver,
        command_resp_sender,
    )
}

pub fn init_executor2(
    fsm_req_receiver: Receiver<OpenBlock>,
    fsm_resp_sender: Sender<ClosedBlock>,
    command_req_receiver: Receiver<command::Command>,
    command_resp_sender: Sender<command::CommandResp>,
) -> Executor {
    // FIXME temp dir should be removed automatically, but at present it is not
    let tempdir = TempDir::new("init_executor").unwrap().into_path();
    let genesis_path = Path::new(SCRIPTS_DIR).join("config_tool/genesis/genesis.json");

    let mut data_path = tempdir.clone();
    data_path.push("data");
    env::set_var("DATA_PATH", data_path);
    let executor = Executor::init(
        genesis_path.to_str().unwrap(),
        tempdir.to_str().unwrap().to_string(),
        fsm_req_receiver,
        fsm_resp_sender,
        command_req_receiver,
        command_resp_sender,
        false,
    );
    executor
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
        let new_tx = SignedTransaction::create(&stx).unwrap();
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
