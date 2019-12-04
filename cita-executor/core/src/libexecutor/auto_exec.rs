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

use crate::cita_executive::{
    build_evm_context, build_vm_exec_params, call as vm_call, ExecutiveParams,
};
use crate::cita_vm_helper::get_interpreter_conf;
use crate::contracts::tools::method as method_tools;
use crate::data_provider::Store as VMSubState;
use crate::libexecutor::block::EVMBlockDataProvider;
use crate::libexecutor::executor::CitaTrieDB;
use crate::types::context::Context;
use crate::types::reserved_addresses;
use cita_types::{Address, H160, U256};
use cita_vm::evm::InterpreterResult;
use cita_vm::state::State as CitaState;
use std::cell::RefCell;
use std::str::FromStr;
use std::sync::Arc;

const AUTO_EXEC: &[u8] = &*b"autoExec()";

lazy_static! {
    static ref AUTO_EXEC_ADDR: H160 = H160::from_str(reserved_addresses::AUTO_EXEC).unwrap();
    static ref AUTO_EXEC_HASH: Vec<u8> = method_tools::encode_to_vec(AUTO_EXEC);
}

pub fn auto_exec(
    state: Arc<RefCell<CitaState<CitaTrieDB>>>,
    auto_exec_quota_limit: u64,
    context: Context,
) {
    let hash = &*AUTO_EXEC_HASH;
    let params = ExecutiveParams {
        code_address: Some(*AUTO_EXEC_ADDR),
        sender: Address::from(0x0),
        to_address: Some(*AUTO_EXEC_ADDR),
        gas: U256::from(auto_exec_quota_limit),
        gas_price: U256::from(1),
        value: U256::from(0),
        nonce: U256::from(0),
        data: Some(hash.to_vec()),
    };
    let block_provider = EVMBlockDataProvider::new(context.clone());
    let vm_exec_params = build_vm_exec_params(&params, state.clone());
    let mut sub_state = VMSubState::default();

    sub_state.evm_context = build_evm_context(&context.clone());
    sub_state.evm_cfg = get_interpreter_conf();
    let sub_state = Arc::new(RefCell::new(sub_state));

    match vm_call(
        Arc::new(block_provider),
        state.clone(),
        sub_state.clone(),
        &vm_exec_params.into(),
    ) {
        Ok(res) => match res {
            InterpreterResult::Normal(_, _, _) => {
                trace!("Auto exec run succeed.");
            }
            InterpreterResult::Revert(_, _) => {
                info!("Auto exec run Revert!");
            }
            _ => {
                info!("Auto exec should not run as create");
            }
        },
        Err(e) => info!("Auto exec failed: {}", e),
    }
}
