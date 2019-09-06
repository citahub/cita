// Copyright Cryptape Technologies LLC.
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

// use crate::contracts::native::factory::Factory as NativeFactory;
use crate::contracts::tools::method as method_tools;
// use crate::engines::NullEngine;
use crate::libexecutor::block::EVMBlockDataProvider;
use crate::types::reserved_addresses;
use cita_types::{Address, H160, U256};
// use evm::{Factory, VMType};
use std::str::FromStr;
// use util::BytesRef;
use crate::cita_executive::{build_evm_config, build_evm_context};
use crate::libexecutor::executor::CitaTrieDB;
use crate::types::context::Context;
use cita_vm::evm::InterpreterResult;
use cita_vm::state::State as CitaState;
use cita_vm::Transaction as EVMTransaction;
use std::cell::RefCell;
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
    let evm_transaction = EVMTransaction {
        from: Address::from(0x0),
        value: U256::from(0),
        gas_limit: auto_exec_quota_limit,
        gas_price: U256::from(1),
        input: hash.to_vec(),
        to: Some(*AUTO_EXEC_ADDR),
        nonce: U256::from(0),
    };

    let mut evm_config = build_evm_config(auto_exec_quota_limit);

    // Do not check nonce and balance for auto exec
    evm_config.check_nonce = false;
    evm_config.check_balance = false;
    let evm_context = build_evm_context(&context);

    let block_provider = EVMBlockDataProvider::new(context.clone());
    match cita_vm::exec(
        Arc::new(block_provider),
        state,
        evm_context,
        evm_config,
        evm_transaction,
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
