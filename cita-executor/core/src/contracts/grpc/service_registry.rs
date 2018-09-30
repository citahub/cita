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

use cita_types::traits::LowerHex;
use cita_types::Address;
use contracts::grpc::contract_state::ContractState;
use std::collections::HashMap;
use std::sync::Mutex;
use util::RwLock;

lazy_static! {
    static ref SERVICE_MAP: Mutex<ServiceMap> = Mutex::new(ServiceMap::new());
}

pub fn register_contract(address: Address, ip: &str, port: u16, height: u64) {
    let key = convert_address_to_key(address);
    SERVICE_MAP
        .lock()
        .unwrap()
        .insert_disable(&key, &ip, port, height);
}

pub fn find_contract(address: Address, enable: bool) -> Option<ContractState> {
    let key = convert_address_to_key(address);
    SERVICE_MAP.lock().unwrap().get(&key, enable)
}

pub fn enable_contract(contract_address: Address) {
    let key = convert_address_to_key(contract_address);
    SERVICE_MAP.lock().unwrap().set_enable(&key);
}

pub fn set_enable_contract_height(contract_address: Address, height: u64) {
    let key = convert_address_to_key(contract_address);
    SERVICE_MAP.lock().unwrap().set_enable_height(&key, height);
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

    pub fn set_enable(&self, contract_address: &str) {
        match self.disable.write().remove(contract_address) {
            Some(value) => {
                self.enable
                    .write()
                    .insert(contract_address.to_owned(), value);
            }
            None => {
                warn!(
                    "can't enable go contract address [{}] because it have not registed!",
                    contract_address
                );
            }
        }
    }

    pub fn set_enable_height(&self, contract_address: &str, height: u64) {
        if let Some(value) = self.enable.write().get_mut(contract_address) {
            value.height = height;
        }
    }

    pub fn insert_disable(&self, key: &str, ip: &str, port: u16, height: u64) {
        self.disable.write().insert(
            key.to_owned(),
            ContractState::new(ip.to_owned(), port, "".to_string(), height),
        );
    }

    //    pub fn contains_key(&self, key: String) -> bool {
    //        self.enable.write().contains_key(&key)
    //    }

    pub fn get(&self, key: &str, enable: bool) -> Option<ContractState> {
        if enable { &self.enable } else { &self.disable }
            .write()
            .get(key)
            .map(::std::clone::Clone::clone)
    }
}
