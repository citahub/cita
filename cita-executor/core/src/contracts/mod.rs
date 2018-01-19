// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

//! System contracts.

pub mod node_manager;
pub mod account_manager;
pub mod quota_manager;

pub use self::account_manager::AccountManager;
pub use self::node_manager::NodeManager;
pub use self::quota_manager::{AccountGasLimit, QuotaManager};

use libexecutor::call_request::CallRequest;
use libexecutor::executor::Executor;
use sha3::sha3_256;
use types::ids::BlockId;
use util::{Address, H160, U256};

/// Parse solidity return data `address[]` to rust `Vec<Address>`
pub fn parse_output_to_addresses(data: &Vec<u8>) -> Vec<Address> {
    let mut nodes = Vec::new();
    trace!("data.len is {:?}", data.len());
    if data.len() > 0 {
        let num = U256::from(&data[32..64]).as_u64() as usize;
        trace!("length of node list is {:?}", num);
        let bytes_per_keys = 32;
        for i in 0..num {
            let start = 64 + i * bytes_per_keys;
            let end = start + bytes_per_keys;
            let key = H160::from(&data[(start + 12)..end]);
            trace!("identity {:?}", key);
            nodes.push(key);
        }
    }
    nodes
}

// Should move to project top-level for code reuse.
trait ContractCallExt {
    fn call_contract_method(&self, address: &Address, encoded_method: &[u8]) -> Vec<u8>;
}

impl ContractCallExt for Executor {
    fn call_contract_method(&self, address: &Address, encoded_method: &[u8]) -> Vec<u8> {
        let call_request = CallRequest {
            from: None,
            to: *address,
            data: Some(encoded_method.to_vec()),
        };

        trace!("data: {:?}", call_request.data);
        self.eth_call(call_request, BlockId::Latest)
            .expect(&format!("eth call address: {}", address))
    }
}

// Should move to project top-level for code reuse.
pub fn encode_contract_name(method_name: &[u8]) -> Vec<u8> {
    let out: &mut [u8; 32] = &mut [0; 32];
    let outptr = out.as_mut_ptr();
    unsafe {
        sha3_256(outptr, 32, method_name.as_ptr(), method_name.len());
    }
    out[0..4].to_vec()
}
