// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::data_provider::{BlockDataProvider, DataProvider, Store as VMSubState};
use cita_trie::DB;
use cita_vm::{
    evm::{self, InterpreterConf, InterpreterParams},
    native,
    state::State,
    Error as VMError,
};
use std::cell::RefCell;
use std::sync::Arc;

/// Function call_pure enters into the specific contract with no check or checkpoints.
pub fn call_pure<B: DB + 'static>(
    block_provider: Arc<dyn BlockDataProvider>,
    state_provider: Arc<RefCell<State<B>>>,
    store: Arc<RefCell<VMSubState>>,
    request: &InterpreterParams,
) -> Result<evm::InterpreterResult, VMError> {
    let evm_context = store.borrow().evm_context.clone();
    let evm_cfg = store.borrow().evm_cfg.clone();
    let evm_params = request.clone();
    let evm_data_provider = DataProvider::new(
        block_provider.clone(),
        state_provider.clone(),
        store.clone(),
    );
    // Transfer value
    if !request.disable_transfer_value {
        state_provider.borrow_mut().transfer_balance(
            &request.sender,
            &request.receiver,
            request.value,
        )?;
    }

    // Execute pre-compiled contracts.
    if native::contains(&request.contract.code_address) {
        let c = native::get(request.contract.code_address);
        let gas = c.required_gas(&request.input);
        if request.gas_limit < gas {
            return Err(VMError::Evm(evm::Error::OutOfGas));
        }
        let r = c.run(&request.input);
        match r {
            Ok(ok) => {
                return Ok(evm::InterpreterResult::Normal(
                    ok,
                    request.gas_limit - gas,
                    vec![],
                ));
            }
            Err(e) => return Err(e),
        }
    }

    // Run
    let mut evm_it = evm::Interpreter::new(
        evm_context,
        evm_cfg,
        Box::new(evm_data_provider),
        evm_params,
    );
    Ok(evm_it.run()?)
}

/// Returns the default interpreter configs for Constantinople.
pub fn get_interpreter_conf() -> InterpreterConf {
    let mut evm_cfg = InterpreterConf::default();
    evm_cfg.eip1283 = false;
    evm_cfg
}
