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

use crate::contracts::native::factory::Factory as NativeFactory;
use crate::engines::NullEngine;
use crate::executed::{Executed, ExecutionError};
use crate::executive::{Executive, TransactOptions};
use crate::libexecutor::economical_model::EconomicalModel;
use crate::libexecutor::sys_config::BlockSysConfig;
use crate::reserved_addresses;
use crate::state::State;
use crate::state::Substate;
use crate::state_db::StateDB;
use crate::tests::helpers::get_temp_state;
use crate::trace::{ExecutiveTracer, ExecutiveVMTracer};
use crate::types::transaction::Action;
use cita_types::{Address, H256, U256};
use core::transaction::Transaction;
use evm;
use evm::action_params::{ActionParams, ActionValue};
use evm::env_info::EnvInfo;
use evm::{Factory, VMType};
use util::{Bytes, BytesRef};

fn transact(
    state: &mut State<StateDB>,
    data: &Vec<u8>,
    value: U256,
    use_super_admin: bool,
) -> Result<Executed, ExecutionError> {
    let mut tx = Transaction::default();
    tx.action = Action::AmendData;
    tx.data = data.to_vec();
    tx.value = value;
    tx.gas = U256::from(100_000);
    let signed_tx = tx.fake_sign(Address::random());

    // TODO Refactor the executive as a param same as call_vm
    let factory = Factory::new(VMType::Interpreter, 1024 * 32);
    let native_factory = NativeFactory::default();
    let engine = NullEngine::default();
    let mut info = EnvInfo::default();
    info.gas_limit = U256::from(100_000);
    let mut ex = Executive::new(
        state,
        &info,
        &engine,
        &factory,
        &native_factory,
        false,
        EconomicalModel::default(),
        2,
    );
    let opts = TransactOptions::default();
    let mut block_config = BlockSysConfig::default();
    if use_super_admin {
        let sender = signed_tx.sender();
        block_config.super_admin_account = Some(*sender);
    }
    ex.transact(&signed_tx, opts, &block_config)
}

fn call_vm(
    state: &mut State<StateDB>,
    params: ActionParams,
) -> evm::Result<evm::FinalizationResult> {
    let factory = Factory::new(VMType::Interpreter, 1024 * 32);
    let native_factory = NativeFactory::default();
    let mut tracer = ExecutiveTracer::default();
    let mut vm_tracer = ExecutiveVMTracer::toplevel();
    let info = EnvInfo::default();
    let engine = NullEngine::default();
    let mut substate = Substate::new();
    let mut ex = Executive::new(
        state,
        &info,
        &engine,
        &factory,
        &native_factory,
        false,
        EconomicalModel::Quota,
        0,
    );
    let mut out = vec![];
    ex.call(
        &params,
        &mut substate,
        BytesRef::Fixed(&mut out),
        &mut tracer,
        &mut vm_tracer,
    )
}

#[test]
fn call_amend_data() {
    let mut state = get_temp_state();
    let key = H256::from(42);
    let value = H256::from(42);
    let storage_address: Address = "0000000000000000000000000000000000055555".into();
    let mut data: Bytes = storage_address.to_vec();
    data.append(&mut key.to_vec());
    data.append(&mut value.to_vec());
    // Sender is not super admin
    // `value=3` means the operation of amending kv
    let result = transact(&mut state, &data, U256::from(3), false);
    assert!(result.is_err());

    // Sender is super admin
    let result = transact(&mut state, &data, U256::from(3), true);
    assert!(result.is_ok());

    let mut data: Bytes = storage_address.to_vec();
    data.append(&mut key.to_vec());
    // Get value from key use transact interface
    // `value=4` means the operation of getting value from key.
    let result = transact(&mut state, &data, U256::from(4), true);
    assert!(result.is_ok());
    // TODO Add output of transact in executive.rs when amend get-value
    // println!("result: {:?}", result.clone().unwrap());

    // Get value from key use call interface
    let address: Address = reserved_addresses::AMEND_ADDRESS.into();
    let mut params = ActionParams::default();
    params.address = address;
    params.code_address = address;
    params.gas = U256::from(10_000);
    params.value = ActionValue::Apparent(4.into());
    params.data = Some(data);
    let result = call_vm(&mut state, params);
    assert!(result.is_ok());
    let return_data = &*(result.unwrap().return_data);
    let return_value: H256 = return_data.into();
    assert!(return_value == value)
}
