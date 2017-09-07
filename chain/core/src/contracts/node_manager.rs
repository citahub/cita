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

#![allow(dead_code)]
use libchain::call_request::CallRequest;
use libchain::chain::Chain;
use sha3::sha3_256;
use std::str::FromStr;
use types::ids::BlockId;
use util::*;

const METHOD_NAME: &'static [u8] = &*b"listNode()";

lazy_static! {
	static ref METHOD_NAME_HASH :Vec<u8> = {
        let out :&mut[u8;32] = &mut [0;32];
        let outptr = out.as_mut_ptr();
        unsafe {
            sha3_256(outptr, 32, METHOD_NAME.as_ptr(), METHOD_NAME.len());
        }
        let func = out[0..8].to_vec();
        func
	};
    static ref CONTRACT_ADDRESS: H160 = H160::from_str("0x00000000000000000000000000000000013241a2").unwrap();
}

struct NodeManager {
    list: Vec<H160>,
}

impl NodeManager {
    pub fn read(&self, chain: Chain) -> Vec<Address> {
        let call_request = CallRequest {
            from: None,
            to: *CONTRACT_ADDRESS,
            data: Some(METHOD_NAME_HASH.to_vec()),
        };

        let output = chain.eth_call(call_request, BlockId::Latest);
        let mut nodes = Vec::new();
        if let Ok(output) = output {
            let len_len = U256::from(&output[0..32]).as_u64() as usize;
            if len_len <= 32 {
                let len = U256::from(&output[32..32 + len_len]).as_u64() as usize;
                let num = len / 20;
                for i in 0..num {
                    nodes.push(H160::from(&output[32 + len_len + i * 20..32 + len_len + (i + 1) * 20]));
                }
            }
        }
        nodes
    }
}

#[cfg(test)]
mod tests {
    //#![allow(unused_must_use, unused_extern_crates)]
    extern crate cita_crypto;
    extern crate env_logger;
    extern crate mktemp;
    use self::Chain;
    use super::*;
    use cita_crypto::PrivKey;
    use db;
    use libchain::block::{Block, BlockBody};
    use libchain::genesis::{Spec, Genesis};
    use libproto::blockchain;
    use rustc_serialize::hex::FromHex;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::sync::mpsc::channel;
    use std::time::UNIX_EPOCH;
    use types::transaction::SignedTransaction;
    use util::{U256, H256, Address};
    use util::kvdb::{Database, DatabaseConfig};

    fn init_chain() -> Arc<Chain> {
        let _ = env_logger::init();
        let tempdir = mktemp::Temp::new_dir().unwrap().to_path_buf();
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
        let genesis = Genesis {
            spec: Spec {
                alloc: HashMap::new(),
                prevhash: H256::from(0),
                timestamp: 0,
            },
            block: Block::default(),
        };
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
            tx.set_valid_until_block(0);

            let mut uv_tx = blockchain::UnverifiedTransaction::new();
            uv_tx.set_transaction(tx);

            let mut stx = blockchain::SignedTransaction::new();
            stx.set_transaction_with_sig(uv_tx);
            stx.sign(*privkey);
            let new_tx = SignedTransaction::new(&stx).unwrap();
            txs.push(new_tx);

        }
        body.set_transactions(txs);
        block.set_body(body);
        block
    }

    #[test]
    fn test_node_manager_contract() {
        let privkey = PrivKey::from("fc8937b92a38faf0196bdac328723c52da0e810f78d257c9ca8c0e304d6a3ad5bf700d906baec07f766b6492bea4223ed2bcbcfd978661983b8af4bc115d2d66");
        println!("privkey: {:?}", privkey);
        let chain = init_chain();
        let data = "60606040526040805190810160405280600160ff168152602001600260ff168152506001906002620000339291906200017e565b5034156200004057600080fd5b604051620010db380380620010db833981016040528080518201919050505b60008090505b81518110156200017557600260008084848151811015156200008357fe5b9060200190602002015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff02191690836002811115620000e357fe5b021790555060018054806001018281620000fe9190620001fa565b916000526020600020900160005b84848151811015156200011b57fe5b90602001906020020151909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505b808060010191505062000065565b5b505062000297565b828054828255906000526020600020908101928215620001e7579160200282015b82811115620001e65782518260006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908360ff160217905550916020019190600101906200019f565b5b509050620001f6919062000229565b5090565b81548183558181151162000224578183600052602060002091820191016200022391906200026f565b5b505050565b6200026c91905b808211156200026857600081816101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690555060010162000230565b5090565b90565b6200029491905b808211156200029057600081600090555060010162000276565b5090565b90565b610e3480620002a76000396000f30060606040523615610076576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680632d4ede931461007b57806330ccebb5146100cc578063609df32f1461011f578063645b8b1b146101ae578063dd4c97a014610209578063ddad2ffe1461025a575b600080fd5b341561008657600080fd5b6100b2600480803573ffffffffffffffffffffffffffffffffffffffff169060200190919050506102ab565b604051808215151515815260200191505060405180910390f35b34156100d757600080fd5b610103600480803573ffffffffffffffffffffffffffffffffffffffff16906020019091905050610517565b604051808260ff1660ff16815260200191505060405180910390f35b341561012a57600080fd5b610132610578565b6040518080602001828103825283818151815260200191508051906020019080838360005b838110156101735780820151818401525b602081019050610157565b50505050905090810190601f1680156101a05780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b34156101b957600080fd5b6101e5600480803573ffffffffffffffffffffffffffffffffffffffff16906020019091905050610615565b604051808260028111156101f557fe5b60ff16815260200191505060405180910390f35b341561021457600080fd5b610240600480803573ffffffffffffffffffffffffffffffffffffffff16906020019091905050610635565b604051808215151515815260200191505060405180910390f35b341561026557600080fd5b610291600480803573ffffffffffffffffffffffffffffffffffffffff1690602001909190505061083f565b604051808215151515815260200191505060405180910390f35b60006002808111156102b957fe5b6000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff16600281111561031057fe5b141515610383577f74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d582604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a160009050610512565b60008060008473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff021916908360028111156103de57fe5b02179055506001805490506103f283610a54565b1415610464577f74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d582604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a160009050610512565b600161046f83610a54565b81548110151561047b57fe5b906000526020600020900160005b6101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690557f74976f07ac4bfb6a02b2dbd3bc158d4984ee6027d938e870692126ca9e1931d582604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a1600190505b919050565b60008060008373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff16600281111561057057fe5b90505b919050565b610580610d75565b61060f600180548060200260200160405190810160405280929190818152602001828054801561060557602002820191906000526020600020905b8160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116105bb575b5050505050610af8565b90505b90565b60006020528060005260406000206000915054906101000a900460ff1681565b60006001600281111561064457fe5b6000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff16600281111561069b57fe5b14151561070e577f5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb82604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a16000905061083a565b60026000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff0219169083600281111561076957fe5b0217905550600180548060010182816107829190610d89565b916000526020600020900160005b84909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550507f5d55f24dd047ef52a5f36ddefc8c424e4b26c8415d8758be1bbb88b5c65e04eb82604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a1600190505b919050565b60006001600281111561084e57fe5b6000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff1660028111156108a557fe5b1415610917577ffd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c182604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a160009050610a4f565b60028081111561092357fe5b6000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060009054906101000a900460ff16600281111561097a57fe5b1415151561098757600080fd5b60016000808473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002060006101000a81548160ff021916908360028111156109e257fe5b02179055507ffd96b5bdd2e0412ade018159455c7af2bed1366ab61906962a1b5638f29c68c182604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a1600190505b919050565b600080600090505b600180549050811015610aee57600181815481101515610a7857fe5b906000526020600020900160005b9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff161415610ae057809150610af2565b5b8080600101915050610a5c565b8091505b50919050565b610b00610d75565b60008083511115610b2f57610b2c836000815181101515610b1d57fe5b90602001906020020151610b96565b91505b600190505b8251811015610b8f57610b7f610b68610b638584815181101515610b5457fe5b90602001906020020151610b96565b610c7d565b610b7184610c7d565b610cac90919063ffffffff16565b91505b8080600101915050610b34565b5b50919050565b610b9e610d75565b610ba6610db5565b60006014604051805910610bb75750595b908082528060200260200182016040525b509150600090505b6014811015610c72578060130360080260020a8473ffffffffffffffffffffffffffffffffffffffff16811515610c0357fe5b047f0100000000000000000000000000000000000000000000000000000000000000028282815181101515610c3457fe5b9060200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1916908160001a9053505b8080600101915050610bd0565b8192505b5050919050565b610c85610dc9565b60006020830190506040805190810160405280845181526020018281525091505b50919050565b610cb4610d75565b610cbc610d75565b60008360000151856000015101604051805910610cd65750595b908082528060200260200182016040525b509150602082019050610d038186602001518760000151610d28565b610d1c8560000151820185602001518660000151610d28565b8192505b505092915050565b60005b602082101515610d5157825184526020840193506020830192505b602082039150610d2b565b6001826020036101000a039050801983511681855116818117865250505b50505050565b602060405190810160405280600081525090565b815481835581811511610db057818360005260206000209182019101610daf9190610de3565b5b505050565b602060405190810160405280600081525090565b604080519081016040528060008152602001600081525090565b610e0591905b80821115610e01576000816000905550600101610de9565b5090565b905600a165627a7a723058209a12c84384e536ea9b64f93b9e3506744ccdf6a5c8c2ceda7efa441819104ca20029"
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
        assert_eq!(nodes, vec![H160::from(0x01), H160::from(0x02)])
    }
}
