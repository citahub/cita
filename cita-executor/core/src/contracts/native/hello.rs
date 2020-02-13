use super::factory::Contract;
use crate::cita_executive::VmExecParams;
use crate::context::Context;
use crate::contracts::tools::method as method_tools;
use crate::storage::Scalar;
use crate::types::errors::NativeError;
use cita_types::{H256, U256};
use cita_vm::evm::DataProvider;
use cita_vm::evm::InterpreterResult;

#[derive(Clone)]
pub struct HelloWorld {
    balance: Scalar,
    output: Vec<u8>,
}

impl Contract for HelloWorld {
    fn exec(
        &mut self,
        params: &VmExecParams,
        _context: &Context,
        data_provider: &mut dyn DataProvider,
    ) -> Result<InterpreterResult, NativeError> {
        method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
            0 => self.init(params, data_provider),
            // Register function
            0x832b_4580 => self.balance_get(params, data_provider),
            0xaa91_543e => self.update(params, data_provider),
            _ => Err(NativeError::Internal("out of gas".to_string())),
        })
    }
    fn create(&self) -> Box<dyn Contract> {
        Box::new(HelloWorld::default())
    }
}

impl Default for HelloWorld {
    fn default() -> Self {
        HelloWorld {
            output: Vec::new(),
            balance: Scalar::new(H256::from(0)),
        }
    }
}

impl HelloWorld {
    fn init(
        &mut self,
        _params: &VmExecParams,
        _data_provider: &mut dyn DataProvider,
    ) -> Result<InterpreterResult, NativeError> {
        Ok(InterpreterResult::Normal(vec![], 100, vec![]))
    }

    fn update(
        &mut self,
        params: &VmExecParams,
        data_provider: &mut dyn DataProvider,
    ) -> Result<InterpreterResult, NativeError> {
        self.output.resize(32, 0);

        // Get the params of`update`
        let amount = U256::from(params.data.get(4..36).expect("no enough data"));
        let new_balance = self
            .balance
            .get(data_provider, &params.storage_address)?
            .saturating_add(amount);

        self.balance
            .set(data_provider, &params.storage_address, new_balance)?;

        Ok(InterpreterResult::Normal(self.output.clone(), 100, vec![]))
    }

    fn balance_get(
        &mut self,
        params: &VmExecParams,
        data_provider: &mut dyn DataProvider,
    ) -> Result<InterpreterResult, NativeError> {
        self.output.resize(32, 0);
        self.balance
            .get(data_provider, &params.code_address)?
            .to_big_endian(self.output.as_mut_slice());
        Ok(InterpreterResult::Normal(self.output.clone(), 100, vec![]))
    }
}