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

//! Node manager.

use super::{encode_contract_name, to_address_vec};
use super::ContractCallExt;
use libexecutor::executor::Executor;
use rustc_hex::ToHex;
use std::str::FromStr;
use util::{Address, H160};

const LIST_NODE: &'static [u8] = &*b"listNode()";

lazy_static! {
    static ref LIST_NODE_ENCODED: Vec<u8> = encode_contract_name(LIST_NODE);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("00000000000000000000000000000000013241a2").unwrap();
}

pub struct NodeManager;

impl NodeManager {
    pub fn read(executor: &Executor) -> Vec<Address> {
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*LIST_NODE_ENCODED.as_slice());
        trace!("nodemanager output: {:?}", ToHex::to_hex(output.as_slice()));

        let nodes: Vec<Address> = to_address_vec(&output);
        trace!("nodemanager nodes: {:?}", nodes);
        nodes
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::NodeManager;
    use std::str::FromStr;
    use tests::helpers::init_executor;
    use util::H160;

    #[test]
    fn test_node_manager_contract() {
        let executor = init_executor();
        let nodes = NodeManager::read(&executor);

        assert_eq!(
            nodes,
            vec![
                H160::from_str("50ad2b9d6946d9c75ae978534043e3021ee1bfb1").unwrap(),
                H160::from_str("eeb3a71c4046f63a941013f826fccc503be26b77").unwrap(),
                H160::from_str("a2bbb65d4f8c3ada29f7471abe416e18061127f3").unwrap(),
                H160::from_str("72eb1e258c9cdccebb7b62930a35cfb6ef4cd24b").unwrap(),
            ]
        )
    }
}
