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

use super::{encode_contract_name, parse_output_to_addresses};
use super::ContractCallExt;
use libexecutor::executor::Executor;
use rustc_hex::ToHex;
use std::str::FromStr;
use util::*;

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

        let nodes: Vec<Address> = parse_output_to_addresses(&output);
        trace!("nodemanager nodes: {:?}", nodes);
        nodes
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate mktemp;

    use super::*;
    use tests::helpers::init_executor;
    use util::Address;

    #[test]
    fn test_node_manager_contract() {
        let executor = init_executor();
        let contract_address = Address::from(0x13241a2);
        let output = executor.call_contract_method(&contract_address, &*LIST_NODE_ENCODED.as_slice());
        let nodes: Vec<Address> = parse_output_to_addresses(&output);

        assert_eq!(
            nodes,
            vec![
                H160::from_str("fd1e96952de36d4dd1b702f34140bccc8749f0e1").unwrap(),
                H160::from_str("b17acb13539da59b067cad7d2f5cb314d25cee0d").unwrap(),
                H160::from_str("c0484735fc22ee452d469a8b79bbfa6121a1d661").unwrap(),
                H160::from_str("391f334bdeb109b11d57f62737554c6dbd282631").unwrap(),
            ]
        )
    }
}
