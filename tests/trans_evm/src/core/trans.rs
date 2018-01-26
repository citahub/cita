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

use crypto::*;
use libproto::blockchain::{Transaction, UnverifiedTransaction};
use rustc_hex::FromHex;
use std::convert::TryInto;
use util::*;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub enum Methods {
    Sendtx(UnverifiedTransaction),
    Height,
    Blockbyheiht(u64),
    Trans(String),
    Receipt(String),
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, Clone)]
pub struct Trans {
    tx: Transaction,
}

#[allow(dead_code, unused_variables)]
impl Trans {
    pub fn new() -> Self {
        Trans {
            tx: Transaction::new(),
        }
    }

    pub fn generate_tx(code: &str, address: String, pv: &PrivKey, curh: u64) -> UnverifiedTransaction {
        let data = code.from_hex().unwrap();

        let mut tx = Transaction::new();
        tx.set_data(data);
        //设置空，则创建合约
        tx.set_to(address);
        tx.set_nonce("0".to_string());
        tx.set_valid_until_block(curh + 88);
        tx.set_quota(1000000);

        tx.sign(*pv).take_transaction_with_sig()
    }

    pub fn generate_tx_data(method: Methods) -> String {
        let txdata = match method {
            Methods::Sendtx(tx) => {
                let tx_bytes: Vec<u8> = tx.try_into().unwrap();
                format!(
                    "{{\"jsonrpc\":\"2.0\",\
                     \"method\":\"cita_sendTransaction\",\
                     \"params\":[\"{}\"],\"id\":2}}",
                    tx_bytes.to_hex()
                )
            }
            Methods::Height => format!(r#"{{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":2}}"#),
            Methods::Blockbyheiht(h) => format!(
                r#"{{"jsonrpc":"2.0","method":"cita_getBlockByNumber","params":["{}",false],"id":2}}"#,
                format!("{:#X}", h)
            ),
            Methods::Trans(hash) => format!(
                r#"{{"jsonrpc":"2.0","method":"cita_getTransaction","params":["{}"],"id":2}}"#,
                hash
            ),
            Methods::Receipt(hash) => format!(
                r#"{{"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":["{}"],"id":2}}"#,
                hash
            ),
        };
        txdata
        //Self::new(txdata)
    }
}
