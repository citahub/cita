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

// use crate::contracts::tools::decode as decode_tools;
use crate::contracts::tools::method as method_tools;
use crate::types::reserved_addresses;
use cita_types::{Address, H160, H256, U256};
// use evm::call_type::CallType;
// use evm::ext::{Ext, MessageCallResult};
use std::str::FromStr;

const CHAIN_ID: &[u8] = &*b"getChainId()";
const AUTHORITIES: &[u8] = &*b"getAuthorities(uint256)";

lazy_static! {
    static ref CHAIN_ID_ENCODED: Vec<u8> = method_tools::encode_to_vec(CHAIN_ID);
    static ref AUTHORITIES_ENCODED: Vec<u8> = method_tools::encode_to_vec(AUTHORITIES);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(reserved_addresses::CHAIN_MANAGER).unwrap();
}

pub struct ChainManagement;

use cita_vm::evm::DataProvider;

impl ChainManagement {
    pub fn ext_chain_id(
        _ext: &mut DataProvider,
        _gas: &U256,
        _sender: &Address,
    ) -> Option<(U256, U256)> {
        trace!("call system contract ChainManagement.ext_chain_id()");
        let _contract = &*CONTRACT_ADDRESS;
        let tx_data = CHAIN_ID_ENCODED.to_vec();
        let _data = &tx_data.as_slice();
        let _output = Vec::<u8>::new();
        // match ext.call(
        //     gas,
        //     sender,
        //     contract,
        //     None,
        //     data,
        //     contract,
        //     &mut output,
        //     CallType::Call,
        // ) {
        //     MessageCallResult::Success(gas_left, return_data) => {
        //         decode_tools::to_u256(&*return_data).map(|x| (gas_left, x))
        //     }
        //     MessageCallResult::Reverted(..) | MessageCallResult::Failed => None,
        // }
        None
    }

    pub fn ext_authorities(
        _ext: &mut DataProvider,
        _gas: &U256,
        _sender: &Address,
        chain_id: U256,
    ) -> Option<(U256, Vec<Address>)> {
        trace!(
            "call system contract ChainManagement.ext_authorities({})",
            chain_id
        );
        let _contract = &*CONTRACT_ADDRESS;
        let mut tx_data = AUTHORITIES_ENCODED.to_vec();
        let param = H256::from(chain_id);
        tx_data.extend(param.to_vec());
        let _data = &tx_data.as_slice();
        let _output = Vec::<u8>::new();
        // match ext.call(
        //     gas,
        //     sender,
        //     contract,
        //     None,
        //     data,
        //     contract,
        //     &mut output,
        //     CallType::Call,
        // ) {
        //     MessageCallResult::Success(gas_left, return_data) => {
        //         trace!(
        //             "call system contract ChainManagement.ext_authorities() return [{:?}]",
        //             return_data
        //         );
        //         decode_tools::to_address_vec(&*return_data).map(|addrs| (gas_left, addrs))
        //     }
        //     MessageCallResult::Reverted(..) | MessageCallResult::Failed => None,
        // }
        None
    }
}
