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

use libchain::call_request::CallRequest;
use libchain::chain::Chain;
use rustc_hex::ToHex;
use sha3::sha3_256;
use types::ids::BlockId;
use util::{Address, H160, U256};


/// Parse solidity return data `String` to rust `Vec<Address>`
pub fn parse_string_to_addresses(data: &[u8]) -> Vec<Address> {
    let mut nodes = Vec::new();
    trace!("data.len is {:?}", data.len());
    if !data.is_empty() {
        let len_len = U256::from(&data[0..32]).as_u64() as usize;
        trace!("len_len is {:?}", len_len);
        if len_len <= 32 {
            let len = U256::from(&data[32..32 + len_len]).as_u64() as usize;
            let num = len / 20;
            let mut iter = data[32 + len_len..].chunks(20);
            for _i in 0..num {
                let node = H160::from(iter.next().expect("string cann't parse to addresses"));
                trace!("node {:?}", node);
                if node != H160::default() {
                    nodes.push(node);
                }
            }
        }
    }
    nodes
}

/// parse quota
pub fn parse_string_to_quota(data: &[u8]) -> Vec<u64> {
    let mut quotas = Vec::new();
    trace!("parse_string_to_quota data.len is {:?}", data.len());
    if !data.is_empty() {
        let len_len = U256::from(&data[0..32]).as_u64() as usize;
        trace!("parse_string_to_quota len_len is {:?}", len_len);
        if len_len <= 32 {
            let len = U256::from(&data[32..32 + len_len]).as_u64() as usize;
            let num = len / 4;
            let mut iter = data[32 + len_len..].chunks(4);
            for _i in 0..num {
                let quota = ToHex::to_hex(iter.next().expect("string cann't parse to addresses"));
                trace!("parse_string_to_addresses quota {:?}", quota);
                if !quota.is_empty() {
                    let q = u64::from_str_radix(&*quota, 16).unwrap();
                    quotas.push(q);
                }
            }
        }
    }
    quotas
}

// Should move to project top-level for code reuse.
trait ContractCallExt {
    fn call_contract_method(&self, address: &Address, encoded_method: &[u8]) -> Vec<u8>;
}

impl ContractCallExt for Chain {
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
