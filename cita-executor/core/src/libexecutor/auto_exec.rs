// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use cita_types::{Address, H160, U256};
use contracts::native::factory::Factory as NativeFactory;
use contracts::tools::method as method_tools;
use engines::NullEngine;
use evm::action_params::{ActionParams, ActionValue};
use evm::call_type::CallType;
use evm::env_info::EnvInfo;
use evm::{Factory, Finalize, VMType};
use externalities::{Externalities, OriginInfo, OutputPolicy};
use libexecutor::economical_model::EconomicalModel;
use state::State;
use state::Substate;
use state_db::StateDB;
use std::str::FromStr;
use trace::Tracer;
use trace::{NoopTracer, NoopVMTracer};
use types::reserved_addresses;
use util::BytesRef;

const AUTO_EXEC: &[u8] = &*b"autoExec()";

lazy_static! {
    static ref AUTO_EXEC_ADDR: H160 = H160::from_str(reserved_addresses::AUTO_EXEC).unwrap();
    static ref AUTO_EXEC_HASH: Vec<u8> = method_tools::encode_to_vec(AUTO_EXEC);
}

// pub fn auto_exec(state: &mut State<StateDB>) -> evm::Result<FinalizationResult> {
pub fn auto_exec(
    state: &mut State<StateDB>,
    auto_exec_quota_limit: u64,
    economical_model: EconomicalModel,
) {
    let hash = &*AUTO_EXEC_HASH;
    let params = ActionParams {
        code_address: *AUTO_EXEC_ADDR,
        address: *AUTO_EXEC_ADDR,
        sender: Address::from(0x0),
        origin: Address::from(0x0),
        gas: U256::from(auto_exec_quota_limit),
        gas_price: U256::from(1),
        value: ActionValue::Transfer(U256::from(0)),
        code: state.code(&*AUTO_EXEC_ADDR).unwrap(),
        code_hash: state.code_hash(&*AUTO_EXEC_ADDR).unwrap(),
        data: Some(hash.to_vec()),
        call_type: CallType::Call,
    };

    let mut substate = Substate::new();
    let mut tracer = NoopTracer;
    let mut out = vec![];
    let mut trace_output = tracer.prepare_trace_output();
    let output = OutputPolicy::Return(BytesRef::Flexible(&mut out), trace_output.as_mut());
    let factory = Factory::new(VMType::Interpreter, 1024 * 32);
    let env_info = EnvInfo::default();
    let engine = NullEngine::default();
    let native_factory = NativeFactory::default();
    let origin_info = OriginInfo::from(&params);
    let mut vm_tracer = NoopVMTracer;
    let mut ext = Externalities::new(
        state,
        &env_info,
        &engine,
        &factory,
        &native_factory,
        0,
        origin_info,
        &mut substate,
        output,
        &mut tracer,
        &mut vm_tracer,
        false,
        economical_model,
    );
    let res = {
        factory
            .create(params.gas)
            .exec(&params, &mut ext)
            .finalize(ext)
    };

    match res {
        Ok(res) => trace!("Auto exec succeed: {:?}", res),
        Err(e) => info!("Auto exec failed: {}", e),
    }
}
