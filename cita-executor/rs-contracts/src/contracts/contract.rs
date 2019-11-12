// use crate::contracts::cons_error::ContractError;
// use crate::contracts::object::InterpreterResult;
// use crate::contracts::object::VmExecParams;
use cita_vm::evm::{InterpreterParams, InterpreterResult};
use common_types::errors::ContractError;

use common_types::context::Context;
use rocksdb::DB;

pub trait Contract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError>;
}
