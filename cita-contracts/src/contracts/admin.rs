use super::contract::Contract;
use super::error::ContractError;
use super::object::{InterpreterResult, VmExecParams};

use cita_types::Address;
// use cita_vm::evm::InterpreterResult;
use common_types::context::Context;
use core_executor::tools::method as method_tools;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminContract {
    admin_contract: BTreeMap<u64, String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Admin {
    admin: Address,
}

impl Contract for Admin {
    fn execute(
        &self,
        params: &VmExecParams,
        db: DB,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError> {
        let entry = Admin::default();
        if params.read_only {
            // read only
            let result =
                method_tools::extract_to_u32(&params.data[..]).and_then(
                    |signature| match signature {
                        0xf851a440 => entry.get_admin(),
                        0x24d7806c => entry.is_admin(params),
                        _ => Err(ContractError::Internal("Invalid function signature")),
                    },
                );
            // Todu hangle error, gas_left
            Ok(InterpreterResult::Normal(result.to_vec(), self.gas, vec![]))
        } else {
            let result = method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
                0x1c1b8772 => entry.update(params),
                 _ => Err(ContractError::Internal("Invalid function signature")),
            });
            // update contract storage
            // db udpate
            Ok(InterpreterResult::Normal(result.to_vec(), self.gas, vec![]))
        }
        return Err(ContractError::AdminError);
    }
}

impl Admin {
    fn init(admin: Address) -> Self {
        Admin {
            admin: admin,
        }
    }

    fn get_admin(&self) -> Address {
        self.admin
    }

    fn udpate(&mut self, admin: &Address) {
        // TODO, only admin can invoke
        self.admin = *admin;
    }

    fn is_admin(&self, admin: &Address) -> bool {
        self.admin == *admin
    }
}
