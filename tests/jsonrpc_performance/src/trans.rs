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
use protobuf::core::Message;
use rustc_hex::FromHex;
use util::*;

#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub enum Methods {
    Sendtx(UnverifiedTransaction),
    Formaterr(UnverifiedTransaction),
    Height,
    Blockbyheiht(u64),
    Trans(String),
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

    pub fn generate_tx(
        code: &str,
        address: String,
        pv: &PrivKey,
        valid_until_block: u64,
        quota: u64,
        nonce: String,
        sign_err: bool,
    ) -> UnverifiedTransaction {
        let data = code.from_hex().unwrap();

        let mut tx = Transaction::new();
        tx.set_data(data);
        //设置空，则创建合约
        tx.set_to(address);
        tx.set_nonce(nonce);
        tx.set_valid_until_block(valid_until_block);
        tx.set_quota(quota);
        let mut signed_tx = tx.sign(*pv);
        if sign_err {
            let signature = signed_tx
                .get_transaction_with_sig()
                .get_signature()
                .to_vec();
            signed_tx
                .mut_transaction_with_sig()
                .set_signature(signature[0..16].to_vec());
        }
        signed_tx.take_transaction_with_sig()
    }

    pub fn generate_tx_data(method: Methods) -> String {
        let txdata = match method {
            Methods::Sendtx(tx) => format!(
                "{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"params\":[\"{}\"],\"id\":2}}",
                tx.write_to_bytes().unwrap().to_hex()
            ),
            Methods::Formaterr(tx) => format!(
                "{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"\":[\"{}\"],\"id\":2}}",
                tx.write_to_bytes().unwrap().to_hex()
            ),
            Methods::Height => {
                format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_blockNumber\",\"params\":[],\"id\":2}}")
            }
            Methods::Blockbyheiht(h) => format!(
                "{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getBlockByNumber\",\"params\":[\"{}\",false],\"id\":2}}",
                format!("{:#X}", h)
            ),
            Methods::Trans(hash) => format!(
                "{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getTransaction\",\"params\":[\"{}\"],\"id\":2}}",
                hash
            ),
        };
        trace!("{}", txdata);
        txdata
        //Self::new(txdata)
    }
}
