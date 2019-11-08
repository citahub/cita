use super::error::ContractError;
use super::object::VmExecParams;

use cita_vm::evm::InterpreterResult;
use common_types::context::Context;
use rocksdb::DB;

pub trait Contract {
    // fn create(&self) {
    //     println!("This create a contract")
    // }

    fn execute(
        &self,
        params: &VmExecParams,
        db: DB,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError>;

    // fn commit(&self) {
    //     println!("This commit a contract")
    // }
}
