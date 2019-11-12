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

#[derive(Default)]
pub struct ContractsFactory {
    // contracts: HashMap<Address, Box<Contract>>,
    // sysconfig: Sysconfig,
    admin: Admin,
    // db: RocksDB,
}

// impl ContractsFactory {
//     fn get_contract(&self, address: Address) -> Option<Box<Contract>> {
//         if let Some(contract) = self.contracts.get(address) {
//             Some(contract::init())
//         } else {
//             None
//         }
//     }

//     fn register(&mut self, address: Address, contract: Box<Contract>) {
//         self.contracts.insert(address, contract);
//     }

//     fn unregister(address: Address) {}
// }

impl ContractsFactory {
    // fn init(&self, sysconfig: SysConfig, admin: Admin) {
    //     // factory.register(Address::from("0x1"), Box::new(Sysconfig::init()));
    //     factory.register(Address::from("0x1"), Box::new(Admin::init()));

    //     factory
    // }

    pub fn is_rs_contract(&self, param_address: &Address) -> bool {
        if param_address.to_vec() == Address::from(reserved_addresses::ADMIN).to_vec() {
            return true;
        }
        false
    }

    pub fn works(
        &self,
        params: &InterpreterParams,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError> {
        match params.contract.code_address {
            // SYS_CONFIG => self.sysconfig.execute(params, self.db, context),
            ADMIN => self.admin.execute(&params, context),
        }
    }
}
