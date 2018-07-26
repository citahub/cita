use cita_types::traits::LowerHex;
use cita_types::Address;
use grpc_contracts::contract_state::ContractState;
use std::collections::HashMap;
use std::sync::Mutex;
use util::RwLock;

lazy_static! {
    static ref SERVICE_MAP: Mutex<ServiceMap> = Mutex::new(ServiceMap::new());
}

pub fn register_contract(address: Address, ip: String, port: u16, height: u64) {
    let key = convert_address_to_key(address);
    SERVICE_MAP
        .lock()
        .unwrap()
        .insert_disable(key, ip, port, height);
}

pub fn find_contract(address: Address, enable: bool) -> Option<ContractState> {
    let key = convert_address_to_key(address);
    SERVICE_MAP.lock().unwrap().get(key, enable)
}

pub fn enable_contract(contract_address: Address) {
    let key = convert_address_to_key(contract_address);
    SERVICE_MAP.lock().unwrap().set_enable(key);
}

pub fn set_enable_contract_height(contract_address: Address, height: u64) {
    let key = convert_address_to_key(contract_address);
    SERVICE_MAP.lock().unwrap().set_enable_height(key, height);
}

fn convert_address_to_key(address: Address) -> String {
    address.lower_hex()
}

struct ServiceMap {
    disable: RwLock<HashMap<String, ContractState>>,
    enable: RwLock<HashMap<String, ContractState>>,
}

impl ServiceMap {
    pub fn new() -> Self {
        ServiceMap {
            disable: RwLock::new(HashMap::new()),
            enable: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_enable(&self, contract_address: String) {
        match self.disable.write().remove(&contract_address) {
            Some(value) => {
                self.enable.write().insert(contract_address, value);
            }
            None => {
                warn!(
                    "can't enable go contract address [{}] because it have not registed!",
                    contract_address
                );
            }
        }
    }

    pub fn set_enable_height(&self, contract_address: String, height: u64) {
        if let Some(value) = self.enable.write().get_mut(&contract_address) {
            value.height = height;
        }
    }

    pub fn insert_disable(&self, key: String, ip: String, port: u16, height: u64) {
        self.disable
            .write()
            .insert(key, ContractState::new(ip, port, "".to_string(), height));
    }

    //    pub fn contains_key(&self, key: String) -> bool {
    //        self.enable.write().contains_key(&key)
    //    }

    pub fn get(&self, key: String, enable: bool) -> Option<ContractState> {
        if enable {
            if let Some(value) = self.enable.write().get(&key) {
                Some(value.clone())
            } else {
                None
            }
        } else {
            if let Some(value) = self.disable.write().get(&key) {
                Some(value.clone())
            } else {
                None
            }
        }
    }
}
