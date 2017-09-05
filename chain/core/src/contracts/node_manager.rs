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

use ethabi::{Param, Function, ParamType};
use libchain::chain::Chain;
use libproto::blockchain::{Nodes, Nodes_Node};
use protobuf::{RepeatedField, Message};
use std::str::FromStr;
use util::*;
use libproto::*;
use std::sync::mpsc::{Sender};
use std::sync::Arc;

// TODO: ethabi should be able to generate this.
const METHOD_NAME: &'static [u8] = &*b"listNode()";

#[allow(dead_code)]
lazy_static! {
	static ref METHOD_NAME_HASH: H256 = METHOD_NAME.crypt_hash();
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("0x00000000000000000000000000000000013241a2").unwrap();
    static ref INTERFACE: Function = Function {
			name: "listNode".to_owned(),
			inputs: vec![],
			outputs: vec![Param {
				name: "".to_owned(),
				kind: ParamType::String,
			}],
			constant: true,
		};
}

pub fn send_nodes(chain: Arc<Chain>, sender: Sender<(String, Vec<u8>)>) {
    let node_manager = NodeManager::new();
    let address_list = node_manager.read(chain);
    let node_list: Vec<Nodes_Node> = address_list.into_iter()
                                               .map(|address| {
                                                        let mut node = Nodes_Node::new();
                                                        node.set_address(address.to_vec());
                                                        node
                                                    })
                                               .collect();
    let mut proto_nodes: Nodes = Nodes::new();
    proto_nodes.set_nodes(RepeatedField::from_slice(&node_list[..]));
    let msg = factory::create_msg(submodules::CHAIN, topics::NODES_CHANGE, communication::MsgType::NODES, proto_nodes.write_to_bytes().unwrap());

    sender.send(("chain.nodes".to_string(), msg.write_to_bytes().unwrap())).unwrap();
}

#[allow(dead_code, unused_variables)]
struct NodeManager {
    list: Vec<H160>,
}

pub struct Authorities {
    pub authorities: Vec<Address>,
}

#[allow(dead_code, unused_variables)]
impl NodeManager {
    pub fn new() -> Self {
        NodeManager {list: vec![]}
    }

    fn pull(&mut self) {}

    pub fn read(&self, chain: Arc<Chain>) -> Vec<Address> {
        // let call_request = CallRequest {
        //     from: None,
        //     to: *CONTRACT_ADDRESS,
        //     data: Some(METHOD_NAME_HASH.to_vec()),
        // };
        // let output = chain.cita_call(call_request, BlockId::Latest).unwrap();
        // let decoded = INTERFACE.decode_output(output.as_slice());
        // let list = decoded.unwrap().to_string().unwrap().as_bytes();
        // let nodes = vec![];
        // for (i, tx_hash) in (0..list/32) {
        //     nodes.push(H160::from(list));
        // }
        let config_file = File::open("./").unwrap();
        let fconfig = BufReader::new(config_file);
        let auth: Authorities = serde_json::from_reader(fconfig).expect(concat!("json is invalid."));

        vec![
            H160::from_str("18e7c55c9b555d58e2af1de1241e09109b89a864").unwrap(),
            H160::from_str("8130b68cecf1c14bdf9407272221fd9b5782535f").unwrap(),
            H160::from_str("adbb5c46b5919bebefb0f7654e7c60ecbd3dc543").unwrap(),
            H160::from_str("7715d7b7ebe835dabe57dee9aa26ce039caf97a3").unwrap(),
        ]
    }
}
