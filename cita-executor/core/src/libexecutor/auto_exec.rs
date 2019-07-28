// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::cita_executive::EnvInfo;
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
    env_info: EnvInfo,
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

    let evm_config = build_evm_config(auto_exec_quota_limit);
    let evm_context = build_evm_context(&env_info);

    let block_provider = EVMBlockDataProvider::new(env_info.clone());
    match cita_vm::exec(
        Arc::new(block_provider),
        state,
        evm_context,
        evm_config,
        evm_transaction,
    ) {
        Ok(res) => trace!("Auto exec succeed: {:?}", res),
        Err(e) => info!("Auto exec failed: {}", e),
    }
}
