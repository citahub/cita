use cita_types::{Address, H256, U256};
//    use core_executor::tests::helpers::{get_temp_state};
use super::helpers::*;
use evm;
use evm::action_params::{ActionParams, ActionValue};
use executive::Executive;
use reserved_addresses;
use state::State;
use state_db::*;
use util::{Bytes, BytesRef};

fn call_vm(
    state: &mut State<StateDB>,
    params: ActionParams,
) -> evm::Result<evm::FinalizationResult> {
    use engines::NullEngine;
    use evm::env_info::EnvInfo;
    use evm::{Factory, VMType};
    use libexecutor::executor::EconomicalModel;
    use native::factory::Factory as NativeFactory;
    use state::Substate;
    use trace::{ExecutiveTracer, ExecutiveVMTracer};
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
        false,
        Address::from(0),
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
    let super_admin_address: Address = "0000000000000000000000000000000000012345".into();
    state.super_admin_account = Some(super_admin_address.clone());
    let key = H256::from(42);
    let value = H256::from(42);
    let storage_address: Address = "0000000000000000000000000000000000055555".into();
    let mut data: Bytes = storage_address.to_vec();
    data.append(&mut key.to_vec());
    data.append(&mut value.to_vec());
    // non admin sender
    let sender = Address::default();
    let address: Address = reserved_addresses::AMEND_ADDRESS.into();
    let mut params = ActionParams::default();
    params.address = address;
    params.code_address = address;
    params.sender = sender.clone();
    params.origin = sender.clone();
    params.gas = U256::from(10000);
    params.value = ActionValue::Apparent(3.into());
    params.data = Some(data.clone());
    let result = call_vm(&mut state, params);
    assert!(result.is_err());

    // sender is admin
    let mut params = ActionParams::default();
    let sender: Address = super_admin_address.clone();
    params.address = address;
    params.code_address = address;
    params.sender = sender.clone();
    params.origin = sender.clone();
    params.gas = U256::from(10000);
    params.value = ActionValue::Apparent(3.into());
    params.data = Some(data.clone());
    let result = call_vm(&mut state, params);
    assert!(result.is_ok());

    // get value from key
    let mut data: Bytes = storage_address.to_vec();
    data.append(&mut key.to_vec());
    let mut params = ActionParams::default();
    let sender: Address = super_admin_address.clone();
    params.address = address;
    params.code_address = address;
    params.sender = sender.clone();
    params.origin = sender.clone();
    params.gas = U256::from(10000);
    params.value = ActionValue::Apparent(4.into());
    params.data = Some(data.clone());
    let result = call_vm(&mut state, params);
    assert!(result.is_ok());
    let return_data = &*(result.unwrap().return_data);
    let return_value: H256 = return_data.into();
    assert!(return_value == value)
}
