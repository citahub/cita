use cita_vm::evm::{InterpreterParams, InterpreterResult};
use cita_vm::state::State;
use common_types::errors::ContractError;
use common_types::context::Context;

use crate::rs_contracts::storage::db_contracts::ContractsDB;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use cita_trie::DB;
use std::cell::RefCell;

pub trait Contract<B>
where
    B: DB
{
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError>;
}
