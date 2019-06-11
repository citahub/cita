use crate::common::{clean_0x, string_2_bytes};
use crate::contracts::ContractsData;
use crate::miner::Miner;
use crate::params::InitData;
use crate::solc::Solc;
use ethabi::Contract;
use evm::cita_types::U256;
use json;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

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
        for (contract_name, contract_info) in self.contract_list.normal_contracts.list().iter() {
            let address = &contract_info.address;
            let sol_path = self.contract_dir.to_owned() + "/src/" + &contract_info.file;
            let abi_path =
                self.contract_dir.to_owned() + "/interaction/abi/" + contract_name + ".abi";
            let data = Solc::get_contracts_data(sol_path, contract_name);

            let input_data = string_2_bytes(data["bin"].clone());
            self.write_docs(contract_name, data);

            let abi_file = File::open(abi_path).expect("failed to open abi file.");
            let contract = Contract::load(abi_file).unwrap();

            if let Some(constructor) = contract.constructor() {
                let mut params = Vec::new();
                match *contract_name {
                    "SysConfig" => {
                        params = self.contract_args.contracts.sys_config.as_params();
                    }
                    "QuotaManager" => {
                        params = self.contract_args.contracts.quota_manager.as_params();
                    }
                    "NodeManager" => {
                        params = self.contract_args.contracts.node_manager.as_params();
                    }
                    "ChainManager" => {
                        params = self.contract_args.contracts.chain_manager.as_params();
                    }
                    "Authorization" => {
                        params = self.contract_args.contracts.authorization.as_params();
                    }
                    "Group" => {
                        params = self.contract_args.contracts.group.as_params();
                    }
                    "Admin" => {
                        params = self.contract_args.contracts.admin.as_params();
                    }
                    "VersionManager" => {
                        params = self.contract_args.contracts.version_manager.as_params();
                    }
                    "PriceManager" => {
                        params = self.contract_args.contracts.price_manager.as_params();
                    }
                    _ => (),
                }
                let bytes = constructor.encode_input(input_data, &params).unwrap();
                let account = Miner::mine(bytes);
                self.accounts.insert((*address).clone(), account);
            } else {
                let account = Miner::mine(input_data);
                self.accounts.insert((*address).clone(), account);
            }
            println!("Normal contracts: {:?} {:?} is ok!", contract_name, address);
        }
    }

    pub fn init_permission_contracts(&mut self) {
        let normal_contracts = self.contract_list.normal_contracts.clone();
        let perm_contracts = self.contract_list.permission_contracts.clone();

        let contract_name: &'static str = "Permission";
        let perm_path = self.contract_dir.to_owned() + "/src/" + &perm_contracts.file;
        let abi_path = self.contract_dir.to_owned() + "/interaction/abi/" + contract_name + ".abi";

        let data = Solc::get_contracts_data(perm_path, contract_name);
        let input_data = string_2_bytes(data["bin"].clone());
        self.write_docs(&contract_name, data);

        let abi_file = File::open(abi_path).expect("failed to open abi file.");
        let contract = Contract::load(abi_file).unwrap();
        let constructor = contract.constructor().unwrap();

        for (name, info) in perm_contracts.basic.list().iter() {
            let address = &info.address;
            let params = self
                .contract_list
                .permission_contracts
                .basic
                .as_params(name, info);

            let bytes = constructor
                .encode_input(input_data.clone(), &params)
                .unwrap();
            let account = Miner::mine(bytes);
            self.accounts.insert(address.clone(), account);
            println!("Permission contracts: {:?} {:?} is ok!", name, address);
        }

        for (name, info) in perm_contracts.contracts.list().iter() {
            let perm_address = &info.address;
            let params = self.contract_list.permission_contracts.contracts.as_params(
                &normal_contracts,
                name,
                info,
            );

            let bytes = constructor
                .encode_input(input_data.clone(), &params)
                .unwrap();
            let account = Miner::mine(bytes);
            self.accounts.insert((*perm_address).clone(), account);
            println!("Permission contracts: {:?} {:?} is ok!", name, perm_address);
        }
    }

    pub fn write_docs(&self, name: &str, data: BTreeMap<&'static str, String>) {
        for doc_type in ["hashes", "userdoc", "devdoc"].iter() {
            let file_path =
                self.contract_docs_dir.to_owned() + "/" + name + "-" + doc_type + ".json";
            let path = Path::new(&file_path);
            let json = json::stringify_pretty(data[doc_type].clone(), 4);
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
}
