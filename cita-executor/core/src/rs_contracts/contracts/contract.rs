use cita_vm::evm::{InterpreterParams, InterpreterResult};
use cita_vm::state::State;
use common_types::context::Context;
use common_types::errors::ContractError;

use crate::rs_contracts::storage::db_contracts::ContractsDB;
use cita_trie::DB;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::Arc;

pub trait Contract<B>
where
    B: DB,
{
    fn execute(
        &self,
        params: &InterpreterParams,
        context: &Context,
        contracts_db: Arc<ContractsDB>,
        state: Arc<RefCell<State<B>>>,
    ) -> Result<InterpreterResult, ContractError>;
}
