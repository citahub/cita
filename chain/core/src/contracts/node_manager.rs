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

// use ethabi::{Token, Param, Function, ParamType};
// use libchain::call_request::CallRequest;
// use libchain::chain::Chain;
// use libproto::*;
// use libproto::factory::*;
// use protobuf::{RepeatedField, Message};
// use rustc_hex::FromHex;
// use std::str::FromStr;
// use std::sync::Arc;
// use std::sync::mpsc::{channel, Sender};
// use types::ids::BlockId;

use authority_manage::AuthManageInfo;
use util::Address;
// use std::collections::HashSet;

// TODO: ethabi should be able to generate this.
// const METHOD_NAME: &'static [u8] = &*b"listNode()";

// lazy_static! {
// 	static ref METHOD_NAME_HASH: H256 = METHOD_NAME.crypt_hash();
//     static ref CONTRACT_ADDRESS: H160 = H160::from_str("0x00000000000000000000000000000000013241a2").unwrap();
//     static ref INTERFACE: Function = Function {
// 			name: "listNode".to_owned(),
// 			inputs: vec![],
// 			outputs: vec![Param {
// 				name: "".to_owned(),
// 				kind: ParamType::String,
// 			}],
// 			constant: true,
// 		};
// }

pub trait NodeManager {
    fn read(&self) -> AuthManageInfo;

    // fn add_node(&self, node: Address) -> Vec<Address>;
    // fn add_nodes(&self, nodes: HashSet<Address>) -> Vec<Address>;

    // fn remove_node(&self, node: Address) -> Vec<Address>;
    // fn remove_nodes(&self, nodes: HashSet<Address>) -> Vec<Address>;
}

// #[derive(PartialEq, Clone, Debug)]
// pub struct NodeManager {
//     list: Vec<Address>,
// }

// impl NodeManager {
//     pub fn new() -> Self {
//         NodeManager { list: vec![] }
//     }

//     fn pull(&mut self) {}

//     pub fn read(&self, chain: &Chain) -> Vec<Address> {
//         // let call_request = CallRequest {
//         //     from: None,
//         //     to: *CONTRACT_ADDRESS,
//         //     data: Some(METHOD_NAME_HASH.to_vec()),
//         // };
//         // let output = chain.cita_call(call_request, BlockId::Latest).unwrap();
//         // let decoded = INTERFACE.decode_output(output.as_slice());
//         // let list = decoded.unwrap().to_string().unwrap().as_bytes();
//         // let nodes = vec![];
//         // for (i, tx_hash) in (0..list/32) {
//         //     nodes.push(H160::from(list));
//         // }

//         vec![
//             H160::from_str("00000000000000000000000000000000013241a2").unwrap(),
//             H160::from_str("999325645d5c23b72af4fce6c512d752ccc8a354").unwrap(),
//         ]
//     }
// }
