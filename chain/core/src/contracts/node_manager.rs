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

use super::parse_string_to_addresses;
use libchain::call_request::CallRequest;
use libchain::chain::Chain;
use sha3::sha3_256;
use std::str::FromStr;
use types::ids::BlockId;
use util::*;

const METHOD_NAME: &'static [u8] = &*b"listNode()";

lazy_static! {
	static ref METHOD_NAME_HASH: Vec<u8> = {
        let out :&mut[u8;32] = &mut [0;32];
        let outptr = out.as_mut_ptr();
        unsafe {
            sha3_256(outptr, 32, METHOD_NAME.as_ptr(), METHOD_NAME.len());
        }
        let func = out[0..4].to_vec();
        func
	};
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("00000000000000000000000000000000013241a2").unwrap();
}

pub struct NodeManager;

impl NodeManager {
    pub fn read(chain: &Chain) -> Vec<Address> {
        let call_request = CallRequest {
            from: None,
            to: *CONTRACT_ADDRESS,
            data: Some(METHOD_NAME_HASH.to_vec()),
        };

        trace!("data: {:?}", call_request.data);
        let output = chain.eth_call(call_request, BlockId::Latest).expect("load nodes eth call");
        trace!("nodemanager output: {:?}", output);
        let nodes: Vec<Address> = parse_string_to_addresses(&output);
        trace!("nodemanager nodes: {:?}", nodes);
        nodes
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;
    extern crate mktemp;
    use self::Chain;
    use super::*;
    use cita_crypto::{PrivKey, SIGNATURE_NAME};
    use db;
    use libchain::block::{Block, BlockBody};
    use libchain::genesis::{Spec, Genesis};
    use libproto::blockchain;
    use rustc_serialize::hex::FromHex;
    use serde_json;

    use std::fs::File;
    use std::io::BufReader;
    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use std::time::UNIX_EPOCH;
    use types::transaction::SignedTransaction;
    use util::{U256, Address};
    use util::kvdb::{Database, DatabaseConfig};

    fn init_chain() -> Arc<Chain> {

        // Load from genesis json file
        let genesis_file = File::open("genesis.json").unwrap();
        let fconfig = BufReader::new(genesis_file);
        let spec: Spec = serde_json::from_reader(fconfig).expect("Failed to load genesis.");
        let genesis = Genesis {
            spec: spec,
            block: Block::default(),
        };

        let _ = env_logger::init();
        let tempdir = mktemp::Temp::new_dir().unwrap().to_path_buf();
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
        let (sync_tx, _) = channel();
        let (chain, _) = Chain::init_chain(Arc::new(db), genesis, sync_tx);
        chain
    }

    fn create_block(chain: &Chain, privkey: &PrivKey, to: Address, data: Vec<u8>, nonce: (u32, u32)) -> Block {
        let mut block = Block::new();

        block.set_parent_hash(chain.get_current_hash());
        block.set_timestamp(UNIX_EPOCH.elapsed().unwrap().as_secs());
        block.set_number(chain.get_current_height() + 1);

        let mut body = BlockBody::new();
        let mut txs = Vec::new();
        for i in nonce.0..nonce.1 {
            let mut tx = blockchain::Transaction::new();
            if to == Address::from(0) {
                tx.set_to(String::from(""));
            } else {
                tx.set_to(to.hex());
            }
            tx.set_nonce(U256::from(i).to_hex());
            tx.set_data(data.clone());
            tx.set_valid_until_block(100);
            tx.set_quota(999999);
            let stx = tx.sign(*privkey);

            let new_tx = SignedTransaction::new(&stx).unwrap();
            txs.push(new_tx);

        }
        body.set_transactions(txs);
        block.set_body(body);
        block
    }

    #[test]
    fn test_node_manager_contract() {
        let privkey = if SIGNATURE_NAME == "ed25519" {
            // TODO: fix this privkey
            PrivKey::from("fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcfd978661983b8af4bc115d2d66")
        } else if SIGNATURE_NAME == "secp256k1" {
            PrivKey::from("352416e1c910e413768c51390dfd791b414212b7b4fe6b1a18f58007fa894214")
        } else {
            panic!("unexcepted signature algorithm");
        };
        println!("privkey: {:?}", privkey);
        let chain = init_chain();
        println!("init chain finish");
        let data = "6060604052341561000f57600080fd5b604051610b2d380380610b2d8339810160405280805190910190505b60005b81518110156100e957600260008084848151811061004857fe5b90602001906020020151600160a060020a031681526020810191909152604001600020805460ff1916600183600281111561007f57fe5b02179055506001805480820161009583826100f1565b916000526020600020900160005b8484815181106100af57fe5b90602001906020020151909190916101000a815481600160a060020a030219169083600160a060020a03160217905550505b60010161002e565b5b505061013c565b8154818355818115116101155760008381526020902061011591810190830161011b565b5b505050565b61013991905b808211156101355760008155600101610121565b5090565b90565b6109e28061014b6000396000f300606060405236156100755763ffffffff7c01000000000000000000000000000000000000000000000000000000006000350416632d4ede93811461007a57806330ccebb5146100ad578063609df32f146100e2578063645b8b1b1461016d578063dd4c97a0146101b0578063ddad2ffe146101e3575b600080fd5b341561008557600080fd5b610099600160a060020a0360043516610216565b604051901515815260200160405180910390f35b34156100b857600080fd5b6100cc600160a060020a0360043516610387565b60405160ff909116815260200160405180910390f35b34156100ed57600080fd5b6100f56103b6565b60405160208082528190810183818151815260200191508051906020019080838360005b838110156101325780820151818401525b602001610119565b50505050905090810190601f16801561015f5780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b341561017857600080fd5b61018c600160a060020a0360043516610427565b6040518082600281111561019c57fe5b60ff16815260200191505060405180910390f35b34156101bb57600080fd5b610099600160a060020a036004351661043c565b604051901515815260200160405180910390f35b34156101ee57600080fd5b610099600160a060020a0360043516610561565b604051901515815260200160405180910390f35b600060025b600160a060020a03831660009081526020819052604090205460ff16600281111561024257fe5b1461028c577f74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d582604051600160a060020a03909116815260200160405180910390a1506000610382565b600160a060020a0382166000908152602081905260408120805460ff19166001835b02179055506001546102bf8361067d565b141561030a577f74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d582604051600160a060020a03909116815260200160405180910390a1506000610382565b60016103158361067d565b8154811061031f57fe5b906000526020600020900160005b6101000a815490600160a060020a0302191690557f74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d582604051600160a060020a03909116815260200160405180910390a15060015b919050565b600160a060020a03811660009081526020819052604081205460ff1660028111156103ae57fe5b90505b919050565b6103be610930565b610421600180548060200260200160405190810160405280929190818152602001828054801561041757602002820191906000526020600020905b8154600160a060020a031681526001909101906020018083116103f9575b50505050506106ed565b90505b90565b60006020819052908152604090205460ff1681565b600060015b600160a060020a03831660009081526020819052604090205460ff16600281111561046857fe5b146104b2577f5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb82604051600160a060020a03909116815260200160405180910390a1506000610382565b600160a060020a038216600090815260208190526040902080546002919060ff19166001835b0217905550600180548082016104ee8382610942565b916000526020600020900160005b8154600160a060020a038087166101009390930a92830292021916179055507f5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb82604051600160a060020a03909116815260200160405180910390a15060015b919050565b600060015b600160a060020a03831660009081526020819052604090205460ff16600281111561058d57fe5b14156105d8577ffd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c182604051600160a060020a03909116815260200160405180910390a1506000610382565b60025b600160a060020a03831660009081526020819052604090205460ff16600281111561060257fe5b141561060d57600080fd5b600160a060020a038216600090815260208190526040902080546001919060ff191682805b02179055507ffd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c182604051600160a060020a03909116815260200160405180910390a15060015b919050565b6000805b6001548110156106e357600180548290811061069957fe5b906000526020600020900160005b9054906101000a9004600160a060020a0316600160a060020a031683600160a060020a031614156106da578091506106e7565b5b600101610681565b8091505b50919050565b6106f5610930565b600080835111156107225761071f8360008151811061071057fe5b9060200190602002015161077f565b91505b5060015b82518110156106e75761076d61075861075385848151811061071057fe5b9060200190602002015161077f565b610852565b61076184610852565b9063ffffffff61087c16565b91505b600101610726565b5b50919050565b610787610930565b61078f610930565b600060146040518059106107a05750595b908082528060200260200182016040525b509150600090505b6014811015610847578060130360080260020a84600160a060020a03168115156107df57fe5b047f01000000000000000000000000000000000000000000000000000000000000000282828151811061080e57fe5b9060200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916908160001a9053505b6001016107b9565b8192505b5050919050565b61085a61097e565b6020820160408051908101604052808451815260200182905291505b50919050565b610884610930565b61088c610930565b600083518551016040518059106108a05750595b908082528060200260200182016040525b5091506020820190506108ca81866020015187516108e9565b6108dd85518201856020015186516108e9565b8192505b505092915050565b60005b6020821061091057825184526020840193506020830192505b6020820391506108ec565b6001826020036101000a0390508019835116818551161784525b50505050565b60206040519081016040526000815290565b81548183558181151161096657600083815260209020610966918101908301610995565b5b505050565b60206040519081016040526000815290565b604080519081016040526000808252602082015290565b61042491905b808211156109af576000815560010161099b565b5090565b905600a165627a7a7230582023e21606774bda0917386bff3b95ba7218b21b36d0532f2dbc666818f76db583002900000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000004000000000000000000000000dd19c4f035c847d49aef6fc35f7fc3b3c801f3f7000000000000000000000000553d3c7059f118dbf6379971941050fb2757db3400000000000000000000000094b3a300a1e294d1bb11b4ad76080417a1377ff70000000000000000000000005f26faf1e9cab2c647f76ac6e9eb2d8743e065c8"
            .from_hex()
            .unwrap();

        let block = create_block(&chain, &privkey, Address::from(0), data, (0, 1));
        chain.set_block(block.clone());

        let txhash = block.body().transactions()[0].hash();
        let receipt = chain.localized_receipt(txhash).unwrap();

        println!("{:?}", receipt);

        let contract_address = receipt.contract_address.unwrap();
        let call_request = CallRequest {
            from: None,
            to: contract_address,
            data: Some(METHOD_NAME_HASH.to_vec()),
        };

        println!("call_request: {:?}", call_request);
        let output = chain.eth_call(call_request, BlockId::Latest);
        let mut nodes = Vec::new();
        if let Ok(output) = output {
            println!("output: {:?}", output);
            let len_len = U256::from(&output[0..32]).as_u64() as usize;
            if len_len <= 32 {
                let len = U256::from(&output[32..32 + len_len]).as_u64() as usize;
                let num = len / 20;
                for i in 0..num {
                    nodes.push(H160::from(&output[32 + len_len + i * 20..32 + len_len + (i + 1) * 20]));
                }
                println!("nodes: {:?}", nodes);
            }
        }
        assert_eq!(
            nodes,
            vec![
                H160::from_str("dd19c4f035c847d49aef6fc35f7fc3b3c801f3f7").unwrap(),
                H160::from_str("553d3c7059f118dbf6379971941050fb2757db34").unwrap(),
                H160::from_str("94b3a300a1e294d1bb11b4ad76080417a1377ff7").unwrap(),
                H160::from_str("5f26faf1e9cab2c647f76ac6e9eb2d8743e065c8").unwrap(),
            ]
        )
    }
}
