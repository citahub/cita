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


use crypto::{KeyPair, PrivKey, PubKey, pubkey_to_address};
use libproto::blockchain::{UnverifiedTransaction, Transaction};
use protobuf::Message;
use rustc_hex::FromHex;
use util::*;
use util::crypto::CreateKey;

#[derive(Clone, Debug)]
pub enum RpcMethod {
    SendTransaction(UnverifiedTransaction),
    Height,
    GetBlockbyheiht(u64),
    GetTransaction(String),
    GetReceipt(String),
}


pub struct Client {
    key_pair: KeyPair,
    eoa: Address,
    contract_address: Vec<Address>,
}


impl Client {
    pub fn new() -> Self {

        let key_pair = KeyPair::gen_keypair();
        let address = pubkey_to_address(key_pair.pubkey());
        Client {
            key_pair: key_pair,
            eoa: address,
            contract_address: vec![],
        }
    }

    fn generate_tx(&self, code: String, address: String, pv: &PrivKey, curh: u64) -> UnverifiedTransaction {
        let data = code.from_hex().unwrap();
        let mut tx = Transaction::new();
        tx.set_data(data);
        //设置空，则创建合约
        tx.set_to(address);
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(curh + 88);
        tx.set_quota(99999999999);
        tx.sign(*pv).take_transaction_with_sig()
    }

    pub fn create_contract_data(&self, code: String, to: String, height: u64) -> String {
        self.get_data_by_method(RpcMethod::SendTransaction(self.generate_tx(code, to, self.key_pair.privkey(), height)))
    }

    pub fn get_data_by_method(&self, method: RpcMethod) -> String {

        let tx_data = match method {
            RpcMethod::SendTransaction(tx) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"params\":[\"{}\"],\"id\":2}}", tx.write_to_bytes().unwrap().to_hex())
            }
            RpcMethod::Height => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_blockNumber\",\"params\":[],\"id\":2}}")
            }
            RpcMethod::GetBlockbyheiht(h) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getBlockByNumber\",\"params\":[{},false],\"id\":2}}", h)
            }
            RpcMethod::GetTransaction(hash) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getTransaction\",\"params\":[\"{}\"],\"id\":2}}", hash)
            }
            RpcMethod::GetReceipt(hash) => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionReceipt\",\"params\":[\"{}\"],\"id\":2}}", hash)
            }
        };
        tx_data
    }
}
