use crate::contracts::Contract;
use cita_types::Address;
use std::collections::HashMap;

use crate::contracts::Sysconfig;

pub struct Factory {
    contracts: HashMap<Address, Box<Contract>>,
}

impl Factory {
    fn get_contract() {}

    fn register(&mut self, address: Address, contract: Box<Contract>) {
        self.contracts.insert(address, contract);
    }

    fn unregister(address: Address) {}
}

impl Factory {
    fn default(&mut self) -> Factory {
        self.register(Address::from("0x1"), Box::new(Sysconfig::init()));

        Factory {
            contracts: HashMap::default(),
        }
    }
}
