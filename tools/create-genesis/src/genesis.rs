// Copyright Cryptape Technologies LLC.
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

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use crate::common::string_2_bytes;
use crate::contracts::ContractsData;
use crate::miner::Miner;
use crate::params::InitData;
use crate::solc::Solc;

use cita_types::{clean_0x, Address, U256};
use ethabi::Contract;
use ethabi::Token;
use json;
use serde::{Deserialize, Serialize};

pub struct GenesisCreator<'a> {
    pub contract_dir: &'a str,
    pub contract_docs_dir: &'a str,
    pub genesis_path: &'a str,
    pub timestamp: u64,
    pub init_token: &'a str,
    pub prevhash: &'a str,
    pub contract_args: InitData,
    pub contract_list: ContractsData,
    pub accounts: BTreeMap<String, Account>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub nonce: U256,
    pub code: String,
    pub storage: BTreeMap<String, String>,
    pub value: U256,
}

#[derive(Serialize, Deserialize)]
pub struct Genesis {
    pub timestamp: u64,
    pub prevhash: String,
    pub alloc: BTreeMap<String, Account>,
}

impl Default for Genesis {
    fn default() -> Self {
        Genesis {
            timestamp: 0,
            prevhash: String::default(),
            alloc: BTreeMap::new(),
        }
    }
}

impl<'a> GenesisCreator<'a> {
    pub fn new(
        contract_dir: &'a str,
        contract_docs_dir: &'a str,
        params_path: &'a str,
        genesis_path: &'a str,
        timestamp: u64,
        init_token: &'a str,
        prevhash: &'a str,
    ) -> Self {
        let params = InitData::load_contract_args(params_path);
        let contracts_list = contract_dir.to_owned() + "/contracts.yml";
        let constracts = ContractsData::load_contract_list(&contracts_list);

        GenesisCreator {
            contract_dir,
            contract_docs_dir,
            genesis_path,
            timestamp,
            init_token,
            prevhash,
            contract_args: params,
            contract_list: constracts,
            accounts: BTreeMap::new(),
        }
    }

    pub fn create(&mut self) {
        // 1. Check compile exit or not
        if !Solc::compiler_version() {
            panic!("solc compiler not exit");
        }
        // 2. Init normal contracts
        self.init_normal_contracts();
        // 3. Init permission contracts
        self.init_permission_contracts();

        // 4. Save super admin
        let super_admin = self.contract_args.contracts.admin.admin.clone();
        self.set_account_value(
            &super_admin,
            U256::from_str(clean_0x(&self.init_token)).unwrap(),
        );
        // 5. Save genesis to file
        self.save_to_file();
        println!("Create genesis successfully !");
    }

    pub fn init_normal_contracts(&mut self) {
        let normal_params = self.contract_args.get_params();
        for (contract_name, contract_info) in self.contract_list.normal_contracts.list().iter() {
            let address = &contract_info.address;
            let data = self.get_data(contract_name, contract_info.file.clone());
            let input_data = string_2_bytes(data["bin"].clone());

            self.write_docs(contract_name, data);
            if let Some(constructor) = self.load_contract(contract_name.to_string()).constructor() {
                let params = normal_params
                    .get(*contract_name)
                    .map_or(Vec::new(), |p| (*p).clone());
                println!(
                    "Contract name {:?} address {:?} params is {:?}",
                    contract_name, address, params
                );
                let bytes = constructor.encode_input(input_data, &params).unwrap();

                if *contract_name == "Admin" || *contract_name == "QuotaManager" {
                    let mut param = BTreeMap::new();
                    let addr = match params.get(0) {
                        Some(Token::Address(addr)) => addr,
                        _ => unimplemented!(),
                    };
                    param.insert("admin".to_string(), addr.hex());
                    let admin_contract = Account {
                        nonce: U256::from(1),
                        code: "".to_string(),
                        storage: param,
                        value: U256::from(0),
                    };
                    self.accounts.insert((*address).clone(), admin_contract);
                } else if *contract_name == "PriceManager" {
                    let mut param = BTreeMap::new();
                    let quota_price = match params.get(0) {
                        Some(Token::Uint(price)) => price,
                        _ => unimplemented!(),
                    };
                    param.insert("quota_price".to_string(), quota_price.to_string());
                    let price_contract = Account {
                        nonce: U256::from(1),
                        code: "".to_string(),
                        storage: param,
                        value: U256::from(0),
                    };
                    self.accounts.insert((*address).clone(), price_contract);
                } else if *contract_name == "Authorization" {
                    let mut param = BTreeMap::new();
                    let addr = match params.get(0) {
                        Some(Token::Address(addr)) => addr,
                        _ => unimplemented!(),
                    };
                    param.insert("admin".to_string(), addr.hex());
                    let admin_contract = Account {
                        nonce: U256::from(1),
                        code: "".to_string(),
                        storage: param,
                        value: U256::from(0),
                    };
                    self.accounts.insert((*address).clone(), admin_contract);
                }
                //  else if *contract_name == "NodeManager" {
                //     match (params.get(0), params.get(1)) {
                //         (Some(Token::Array(n)), Some(Token::Array(s))) => {
                //             let nodes = n
                //                 .iter()
                //                 .map(|i| match i {
                //                     Token::Address(x) => Address::from_slice(x),
                //                     _ => unreachable!(),
                //                 })
                //                 .collect::<Vec<Address>>();

                //             let stakes: Vec<U256> = s
                //                 .iter()
                //                 .map(|i| match i {
                //                     Token::Uint(x) => *x,
                //                     _ => unreachable!(),
                //                 })
                //                 .collect::<Vec<_>>();

                //             let mut param = BTreeMap::new();
                //             for i in 0..nodes.len() {
                //                 param.insert(nodes[i].hex(), stakes[i].to_string());
                //             }

                //             let contract = Account {
                //                 nonce: U256::from(1),
                //                 code: "".to_string(),
                //                 storage: param,
                //                 value: U256::from(0),
                //             };
                //             self.accounts.insert((*address).clone(), contract);
                //         }
                //         _ => unimplemented!(),
                //     }
                // }
                else if *contract_name == "SysConfig" {
                    let mut param = BTreeMap::new();
                    let delay_block_number = match params.get(0) {
                        Some(Token::Uint(s)) => s,
                        _ => unimplemented!(),
                    };
                    let chain_owner = match params.get(1) {
                        Some(Token::Address(s)) => s,
                        _ => unimplemented!(),
                    };
                    let chain_name = match params.get(2) {
                        Some(Token::String(s)) => s,
                        _ => unimplemented!(),
                    };
                    let chain_id = match params.get(3) {
                        Some(Token::Uint(s)) => s,
                        _ => unimplemented!(),
                    };
                    let operator = match params.get(4) {
                        Some(Token::String(s)) => s,
                        _ => unimplemented!(),
                    };
                    let website = match params.get(5) {
                        Some(Token::String(s)) => s,
                        _ => unimplemented!(),
                    };
                    let block_interval = match params.get(6) {
                        Some(Token::Uint(s)) => s,
                        _ => unimplemented!(),
                    };
                    let economical_model = match params.get(7) {
                        Some(Token::Uint(s)) => s,
                        _ => unimplemented!(),
                    };
                    let name = match params.get(8) {
                        Some(Token::String(s)) => s,
                        _ => unimplemented!(),
                    };
                    let symbol = match params.get(9) {
                        Some(Token::String(s)) => s,
                        _ => unimplemented!(),
                    };
                    let avatar = match params.get(10) {
                        Some(Token::String(s)) => s,
                        _ => unimplemented!(),
                    };
                    let flags = match params.get(11) {
                        Some(Token::Array(s)) => s
                            .iter()
                            .map(|i| match i {
                                Token::Bool(b) => *b,
                                _ => panic!("should not be here"),
                            })
                            .collect::<Vec<bool>>(),
                        _ => panic!("should not be here"),
                    };

                    param.insert(
                        "delay_block_number".to_string(),
                        delay_block_number.to_hex(),
                    );
                    param.insert("chain_owner".to_string(), chain_owner.hex());
                    param.insert("chain_name".to_string(), chain_name.to_string());
                    param.insert("chain_id".to_string(), chain_id.to_hex());
                    param.insert("operator".to_string(), operator.to_string());
                    param.insert("website".to_string(), website.to_string());
                    param.insert("block_interval".to_string(), block_interval.to_hex());
                    param.insert("economical_model".to_string(), economical_model.to_hex());
                    param.insert("name".to_string(), name.to_string());
                    param.insert("symbol".to_string(), symbol.to_string());
                    param.insert("avatar".to_string(), avatar.to_string());
                    param.insert("check_call_permission".to_string(), flags[0].to_string());
                    param.insert("check_send_tx_permission".to_string(), flags[1].to_string());
                    param.insert(
                        "check_create_contract_permission".to_string(),
                        flags[2].to_string(),
                    );
                    param.insert("check_quota".to_string(), flags[3].to_string());
                    param.insert("check_fee_back_platform".to_string(), flags[4].to_string());
                    param.insert("auto_exec".to_string(), flags[5].to_string());

                    let contract = Account {
                        nonce: U256::from(1),
                        code: "".to_string(),
                        storage: param,
                        value: U256::from(0),
                    };
                    self.accounts.insert((*address).clone(), contract);
                } else {
                    if let Some(account) = Miner::mine(bytes) {
                        self.accounts.insert((*address).clone(), account);
                    }
                }
            } else if *contract_name == "EmergencyIntervention" {
                continue;
            } else if let Some(account) = Miner::mine(input_data) {
                self.accounts.insert((*address).clone(), account);
                println!("Normal contracts: {:?} {:?} is ok!", contract_name, address);
            }
        }
    }

    pub fn init_permission_contracts(&mut self) {
        let normal_contracts = self.contract_list.normal_contracts.clone();
        let perm_contracts = self.contract_list.permission_contracts.clone();
        let contract_name = "Permission".to_string();
        let data = self.get_data(&contract_name, perm_contracts.file);
        let input_data = string_2_bytes(data["bin"].clone());

        self.write_docs(&contract_name, data);
        if let Some(constructor) = self.load_contract(contract_name.clone()).constructor() {
            for (name, info) in perm_contracts.basic.list().iter() {
                let address = &info.address;
                let params = self
                    .contract_list
                    .permission_contracts
                    .basic
                    .as_params(name);
                println!(
                    "Contract name {:?} name {:?} params is {:?}",
                    contract_name, name, params
                );

                // let bytes = constructor
                //     .encode_input(input_data.clone(), &params)
                //     .unwrap();
                // if let Some(account) = Miner::mine(bytes) {
                //     self.accounts.insert(address.clone(), account);
                //     println!("Permission contracts: {:?} {:?} is ok!", name, address);
                // }
                let address = &info.address;
                let mut params = BTreeMap::new();
                params.insert("".to_string(), info.address.clone());
                params.insert("perm_name".to_string(), name.to_string());

                let account = Account {
                    nonce: U256::from(1),
                    code: "".to_string(),
                    storage: params,
                    value: U256::from(0),
                };
                self.accounts.insert(address.clone(), account);
            }

            for (name, info) in perm_contracts.contracts.list().iter() {
                let perm_address = &info.address;
                let params = self
                    .contract_list
                    .permission_contracts
                    .contracts
                    .as_params(&normal_contracts, name);
                //     println!("Contract name {:?} name {:?} params is {:?}", contract_name, name,  params);

                // let bytes = constructor
                //     .encode_input(input_data.clone(), &params)
                //     .unwrap();
                // if let Some(account) = Miner::mine(bytes) {
                //     self.accounts.insert((*perm_address).clone(), account);
                //     println!("Permission contracts: {:?} {:?} is ok!", name, perm_address);
                // }
                // let perm_address = &info.address;
                // let mut params = BTreeMap::new();
                // for i in 0..info.contracts.len() {
                //     params.insert(info.contracts.get(i).unwrap().to_string(), info.functions.get(i).unwrap().to_string());
                // }
                // params.insert("name".to_string(), name.to_string());

                let account = Account {
                    nonce: U256::from(1),
                    code: "".to_string(),
                    storage: params,
                    value: U256::from(0),
                };
                self.accounts.insert((*perm_address).clone(), account);
            }
        }
    }

    pub fn write_docs(&self, name: &str, data: BTreeMap<String, String>) {
        for doc_type in ["hashes", "userdoc", "devdoc"].iter() {
            let file_path =
                self.contract_docs_dir.to_owned() + "/" + name + "-" + doc_type + ".json";
            let path = Path::new(&file_path);
            let json = json::stringify_pretty(data[*doc_type].clone(), 4);
            let mut f = File::create(path).expect("failed to write docs.");
            let _ = f.write_all(&json.as_bytes());
        }
    }

    pub fn set_account_value(&mut self, address: &str, value: U256) {
        let account = Account {
            nonce: U256::one(),
            code: String::from(""),
            storage: BTreeMap::new(),
            value,
        };
        self.accounts.insert(address.to_owned(), account);
    }

    pub fn save_to_file(&mut self) {
        let mut genesis = Genesis::default();
        genesis.timestamp = self.timestamp;
        genesis.prevhash = self.prevhash.to_owned();
        genesis.alloc = self.accounts.clone();
        let f = File::create(self.genesis_path.to_owned()).expect("failed to create genesis.json.");
        let _ = serde_json::to_writer_pretty(f, &genesis);
    }

    pub fn get_data(&self, contract_name: &str, file_path: String) -> BTreeMap<String, String> {
        let path = self.contract_dir.to_owned() + "/src/" + &file_path;
        Solc::get_contracts_data(path, contract_name)
    }

    pub fn load_contract(&self, contract_name: String) -> Contract {
        let abi_path = self.contract_dir.to_owned() + "/interaction/abi/" + &contract_name + ".abi";
        let abi_file = File::open(abi_path).expect("failed to open abi file.");
        Contract::load(abi_file).unwrap()
    }
}
