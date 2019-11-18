// use crate::contracts::cons_error::ContractError;
// use crate::contracts::object::InterpreterResult;
// use crate::contracts::object::VmExecParams;
use cita_vm::evm::{InterpreterParams, InterpreterResult};
use common_types::errors::ContractError;

use common_types::context::Context;
// use rocksdb::DB;
use crate::rs_contracts::storage::db_contracts::ContractsDB;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub trait Contract {
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
    ) -> Result<InterpreterResult, ContractError>;

    // fn create(&self) -> Box<Contract>;
}
