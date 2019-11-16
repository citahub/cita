use crate::contracts::admin::Admin;
// use crate::contracts::cons_error::ContractError;
use crate::contracts::contract::Contract;
use common_types::errors::ContractError;
// use crate::contracts::object::VmExecParams;
use crate::contracts::reserved_addresses;
use cita_vm::evm::InterpreterParams;
use cita_vm::evm::InterpreterResult;
// use crate::contracts::Sysconfig;

use cita_types::Address;
use common_types::context::Context;
use std::collections::HashMap;

use crate::storage::db_contracts::ContractsDB;
use std::sync::Arc;

use crate::contracts::admin::AdminContract;
use crate::contracts::price::PriceContract;

pub struct ContractsFactory {
    // contracts: HashMap<Address, Box<Contract>>,
    contracts_db: Arc<ContractsDB>,
    admin_contract: AdminContract,
    price_contract: PriceContract,
}

impl ContractsFactory {
    // pub fn get_contract(&self, addr: &Address) -> Option<Box<Contract>> {
    //     if let Some(contract) = self.contracts.get(addr) {
    //         Some(contract.create())
    //     } else {
    //         None
    //     }
    // }

    pub fn register(&mut self, address: Address, contract: String) {
        trace!(
            "System contracts register address {:?} contract {:?}",
            address,
            contract
        );
        let admin_address = Address::from(reserved_addresses::ADMIN);
        let price_address = Address::from(reserved_addresses::PRICE_MANAGEMENT);
        if address.to_vec() == admin_address.to_vec() {
            return self.admin_contract.init(contract, self.contracts_db.clone());
        } else if address.to_vec() == price_address.to_vec() {
            return self.price_contract.init(contract, self.contracts_db.clone());
        }

        // 匹配不了
        // match address {
        //     admin_address => self.admin_contract.init(contract, self.contracts_db.clone());
        //     price_address => self.price_contract.init(contract, self.contracts_db.clone());
        //     _ => println!("other contract"),
        // }
    }

    // pub fn unregister(&mut self, address: Address) {
    //     self.contracts.remove(&address);
    // }
}

impl ContractsFactory {
    pub fn new(db: Arc<ContractsDB>) -> Self {
        ContractsFactory {
            contracts_db: db,
            admin_contract: AdminContract::default(),
            price_contract: PriceContract::default(),
        }
    }

    pub fn is_rs_contract(&self, param_address: &Address) -> bool {
        let admin_address = Address::from(reserved_addresses::ADMIN);
        let price_address = Address::from(reserved_addresses::PRICE_MANAGEMENT);
        // trace!("===> lalala admin_address is {:?}", admin_address);
        // match param_address {
        //     admin_address => return true,
        //     _ => return false,
        // }
        if param_address.to_vec() == admin_address.to_vec() || param_address.to_vec() == price_address.to_vec() {
            return true;
        }
        false
    }

    pub fn works(
        &self,
        params: &InterpreterParams,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError> {
        let admin_address = Address::from(reserved_addresses::ADMIN);
        let price_address = Address::from(reserved_addresses::PRICE_MANAGEMENT);
        // match params.contract.code_address {
        //     // SYS_CONFIG => self.sysconfig.execute(params, self.db, context),
        //     admin_address => {
        //         // let contract = self.get_contract(&address).unwrap();
        //         self.admin_contract
        //             .execute(&params, context, self.contracts_db.clone())
        //         // contract.execute(&params, context, self.contracts_db.clone())
        //     }
        // }

        if params.contract.code_address.to_vec() == admin_address.to_vec() {
            return self.admin_contract.execute(&params, context, self.contracts_db.clone());
        } else if params.contract.code_address.to_vec() == price_address.to_vec() {
            return self.price_contract.execute(&params, context, self.contracts_db.clone());
        }
        return Err(ContractError::AdminError(String::from(
            "not a valid address",
        )));
    }
}
