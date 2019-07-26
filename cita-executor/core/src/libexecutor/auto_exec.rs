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
use crate::libexecutor::economical_model::EconomicalModel;
use crate::types::reserved_addresses;
use cita_types::H160;
// use evm::{Factory, VMType};
use std::str::FromStr;
// use util::BytesRef;

const AUTO_EXEC: &[u8] = &*b"autoExec()";

lazy_static! {
    static ref AUTO_EXEC_ADDR: H160 = H160::from_str(reserved_addresses::AUTO_EXEC).unwrap();
    static ref AUTO_EXEC_HASH: Vec<u8> = method_tools::encode_to_vec(AUTO_EXEC);
}

// FIXME
pub fn auto_exec(
    // state: &mut State<StateDB>,
    _auto_exec_quota_limit: u64,
    _economical_model: EconomicalModel,
    _env_info: EnvInfo,
    _chain_version: u32,
) {
    unimplemented!();
    // let _hash = &*AUTO_EXEC_HASH;
    // let params = ActionParams {
    //     code_address: *AUTO_EXEC_ADDR,
    //     address: *AUTO_EXEC_ADDR,
    //     sender: Address::from(0x0),
    //     origin: Address::from(0x0),
    //     gas: U256::from(auto_exec_quota_limit),
    //     gas_price: U256::from(1),
    //     value: ActionValue::Transfer(U256::from(0)),
    //     code: state.code(&*AUTO_EXEC_ADDR).unwrap(),
    //     code_hash: state.code_hash(&*AUTO_EXEC_ADDR).unwrap(),
    //     data: Some(hash.to_vec()),
    //     call_type: CallType::Call,
    // };

    // let mut out = vec![];
    // let _output = OutputPolicy::Return(BytesRef::Flexible(&mut out), None);
    // let _factory = Factory::new(VMType::Interpreter, 1024 * 32);

    // let _engine = NullEngine::default();
    // let _native_factory = NativeFactory::default();
    // let _origin_info = OriginInfo::from(&params);
    //    let mut ext = Externalities::new(
    //        state,
    //        &env_info,
    //        &engine,
    //        &factory,
    //        &native_factory,
    //        0,
    //        origin_info,
    //        &mut substate,
    //        output,
    //        false,
    //        economical_model,
    //        chain_version,
    //    );
    //    let res = {
    //        factory
    //            .create(params.gas)
    //            .exec(&params, &mut ext)
    //            .finalize(ext)
    //    };
    //
    //    match res {
    //        Ok(res) => trace!("Auto exec succeed: {:?}", res),
    //        Err(e) => info!("Auto exec failed: {}", e),
    //    }
}
