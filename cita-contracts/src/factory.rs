use crate::contracts::Contract;
use cita_types::Address;
use std::collections::HashMap;

pub struct Factory {
    contracts: HashMap<Address, Box<Contract>>,
}

impl Factory {
    fn get_contract() {}

    fn insert_contract() {}

    fn remove_contract() {}
}

impl Factory {
    fn default() -> Factory {}
}
