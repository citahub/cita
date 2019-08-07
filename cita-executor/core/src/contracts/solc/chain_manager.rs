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
