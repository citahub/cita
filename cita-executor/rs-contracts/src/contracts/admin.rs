use super::contract::Contract;
use super::object::VmExecParams;
use super::utils::extract_to_u32;

use cita_types::{Address, H256, U256};
use cita_vm::evm::{InterpreterParams, InterpreterResult};
// use cita_vm::evm::InterpreterResult;
use common_types::context::Context;
use common_types::errors::ContractError;
use rocksdb::DB;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminContract {
    admin_contract: BTreeMap<u64, Option<String>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Admin {
    admin: Address,
}

impl Contract for Admin {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError> {
        let mut entry = Admin::default();
        if params.read_only {
            // read only
            let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                0xf851a440 => entry.get_admin(),
                0x24d7806c => entry.is_admin(params),
                _ => Err(ContractError::Internal(
                    "Invalid function signature".to_owned(),
                )),
            });
            // Todu hangle error, gas_left
            return result;
        } else {
            let result = extract_to_u32(&params.input[..]).and_then(|signature| match signature {
                0x1c1b8772 => entry.update(params),
                _ => Err(ContractError::Internal(
                    "Invalid function signature".to_owned(),
                )),
            });
            // update contract storage
            // db udpate
            return result;
        }
        return Err(ContractError::AdminError);
    }
}

impl Admin {
    pub fn init(admin: Address) -> Self {
        Admin { admin: admin }
    }

    fn get_admin(&self) -> Result<InterpreterResult, ContractError> {
        return Ok(InterpreterResult::Normal(
            H256::from(self.admin).0.to_vec(),
            0,
            vec![],
        ));
    }

    fn update(&mut self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        // TODO, only admin can invoke

        let param_address = Address::from_slice(&params.input[16..36]);
        self.admin = param_address;
        return Ok(InterpreterResult::Normal(
            H256::from(1).0.to_vec(),
            params.gas_limit,
            vec![],
        ));
    }

    fn is_admin(&self, params: &InterpreterParams) -> Result<InterpreterResult, ContractError> {
        let param_address = Address::from_slice(&params.input[16..36]);
        if param_address == self.admin {
            return Ok(InterpreterResult::Normal(
                H256::from(1).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        } else {
            return Ok(InterpreterResult::Normal(
                H256::from(0).0.to_vec(),
                params.gas_limit,
                vec![],
            ));
        }
    }
}
