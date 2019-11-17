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
        trace!("Register system contract address {:?} contract {:?}", address, contract);
        if address == Address::from(reserved_addresses::ADMIN) {
            return self.admin_contract.init(contract, self.contracts_db.clone());
        } else if address == Address::from(reserved_addresses::PRICE_MANAGEMENT) {
            return self.price_contract.init(contract, self.contracts_db.clone());
        }
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

    pub fn is_rs_contract(&self, addr: &Address) -> bool {
        if *addr == Address::from(reserved_addresses::ADMIN) ||
            *addr == Address::from(reserved_addresses::PRICE_MANAGEMENT) {
            return true;
        }
        false
    }

    pub fn works(
        &self,
        params: &InterpreterParams,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError> {
        if params.contract.code_address == Address::from(reserved_addresses::ADMIN) {
            return self.admin_contract.execute(&params, context, self.contracts_db.clone());
        } else if params.contract.code_address == Address::from(reserved_addresses::PRICE_MANAGEMENT) {
            return self.price_contract.execute(&params, context, self.contracts_db.clone());
        }
        return Err(ContractError::AdminError(String::from(
            "not a valid address",
        )));
    }
}
