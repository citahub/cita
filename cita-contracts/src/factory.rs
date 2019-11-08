use crate::contracts::Contract;
use cita_types::Address;
use std::collections::HashMap;

use crate::contracts::addresses::*;
use crate::contracts::Sysconfig;

pub struct ContractsFactory {
    contracts: HashMap<Address, Box<Contract>>,
    sysconfig: Sysconfig,
    admin: Admin,
    db: RocksDB,
}

impl ContractsFactory {
    fn get_contract(&self, address: Address) -> Option<Box<Contract>> {
        if let Some(contract) = self.contracts.get(address) {
            Some(contract::init())
        } else {
            None
        }
    }

    fn register(&mut self, address: Address, contract: Box<Contract>) {
        self.contracts.insert(address, contract);
    }

    fn unregister(address: Address) {}
}

impl ContractsFactory {
    fn init(&self, sysconfig: SysConfig, admin: Admin) -> Factory {
        factory.register(Address::from("0x1"), Box::new(Sysconfig::init()));
        factory.register(Address::from("0x1"), Box::new(Admin::init()));

        factory
    }

    fn get_contract(&self, address: &Address) -> bool {
        if address == SYS_CONFIG || ADMIN {
            true
        }
        false
    }

    fn works(
        &self,
        params: VmExecParams,
        context: Context,
    ) -> Result<InterpreterResult, NativeError> {
        match params.contract.code_address {
            SYS_CONFIG => self.sysconfig.execute(params, self.db, context),
            ADMIN => self.admin.execute(params, db, context),
        }
    }
}
