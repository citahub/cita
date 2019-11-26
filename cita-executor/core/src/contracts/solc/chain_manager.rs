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

//! Chain manager.

use std::str::FromStr;

use crate::contracts::tools::{decode as decode_tools, method as method_tools};
use crate::types::reserved_addresses;

use cita_types::{Address, H160, H256, U256};
use cita_vm::evm::{DataProvider, InterpreterParams, InterpreterResult, OpCode};

const CHAIN_ID: &[u8] = &*b"getChainId()";
const AUTHORITIES: &[u8] = &*b"getAuthorities(uint256)";

lazy_static! {
    static ref CHAIN_ID_ENCODED: Vec<u8> = method_tools::encode_to_vec(CHAIN_ID);
    static ref AUTHORITIES_ENCODED: Vec<u8> = method_tools::encode_to_vec(AUTHORITIES);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(reserved_addresses::CHAIN_MANAGER).unwrap();
}

pub struct ChainManagement;

impl ChainManagement {
    pub fn ext_chain_id(
        data_provider: &mut DataProvider,
        gas: &U256,
        sender: &Address,
    ) -> Option<(U256, U256)> {
        trace!("Call chainManagement ext_chain_id()");
        let mut params = InterpreterParams::default();
        params.receiver = *CONTRACT_ADDRESS;
        params.sender = *sender;
        params.gas_limit = gas.low_u64();
        params.input = CHAIN_ID_ENCODED.to_vec();

        match data_provider.call(OpCode::CALL, params) {
            Ok(InterpreterResult::Normal(output, gas_left, _)) => {
                decode_tools::to_u256(&output).map(|x| (U256::from(gas_left), x))
            }
            _ => None,
        }
    }

    pub fn ext_authorities(
        data_provider: &mut DataProvider,
        gas: &U256,
        sender: &Address,
        chain_id: U256,
    ) -> Option<(U256, Vec<Address>)> {
        trace!("call chainManagement ext_authorities({})", chain_id);
        let mut params = InterpreterParams::default();
        params.receiver = *CONTRACT_ADDRESS;
        params.sender = *sender;
        params.gas_limit = gas.low_u64();
        let mut tx_data = AUTHORITIES_ENCODED.to_vec();
        tx_data.extend(H256::from(chain_id).to_vec());

        match data_provider.call(OpCode::CALL, params) {
            Ok(InterpreterResult::Normal(output, gas_left, _logs)) => {
                trace!("chainManagement ext_authorities() return [{:?}]", output);
                decode_tools::to_address_vec(&output).map(|addrs| (U256::from(gas_left), addrs))
            }
            _ => None,
        }
    }
}
