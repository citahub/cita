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

use libproto::{key_to_id, parse_msg, MsgClass, factory, submodules, topics, communication, tx_verify_req_msg};
use libproto::auth::Ret;
use libproto::blockchain::{SignedTransaction, UnverifiedTransaction};
use protobuf::Message;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use util::H256;

pub type TransType = (Option<SignedTransaction>, Option<(H256, Ret)>);

pub struct TxHandler {
    pool: ThreadPool,
    tx: Sender<TransType>,
    tx_pub: Sender<(String, Vec<u8>)>,
    unverified: Arc<Mutex<HashMap<H256, (u32, UnverifiedTransaction)>>>,
}



impl TxHandler {
    pub fn new(pool: ThreadPool, tx: Sender<TransType>, tx_pub: Sender<(String, Vec<u8>)>) -> Self {
        TxHandler {
            pool: pool,
            tx: tx,
            tx_pub: tx_pub,
            unverified: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn handle(&self, key: String, msg: Vec<u8>) {
        let tx = self.tx.clone();
        let tx_pub = self.tx_pub.clone();
        let unverified = self.unverified.clone();
        self.pool.execute(move || {
            let (_, _, msg) = parse_msg(&msg);
            match msg {
                MsgClass::TX(unverified_tx) => {
                    let id = key_to_id(&key);
                    let verify_tx_req = tx_verify_req_msg(&unverified_tx);
                    let hash: H256 = verify_tx_req.get_tx_hash().into();
                    {
                        let mut txs = unverified.lock().unwrap();
                        txs.insert(hash, (id, unverified_tx.clone()));
                    }
                    let msg = factory::create_msg(submodules::CONSENSUS, topics::VERIFY_TX_REQ, communication::MsgType::VERIFY_TX_REQ, verify_tx_req.write_to_bytes().unwrap());
                    trace!("send verify req, hash: {:?}", hash);
                    tx_pub.send(("consensus.verify_req".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                }
                MsgClass::VERIFYTXRESP(resp) => {
                    //remove tx from unverified
                    let tx_hash: H256 = resp.get_tx_hash().into();
                    let unverified_tx = {
                        let mut txs = unverified.lock().unwrap();
                        txs.remove(&tx_hash)
                    };
                    trace!("receive verify resp, hash: {:?}, ret: {:?}", tx_hash, resp.get_ret());

                    unverified_tx.map(|(id, unverified_tx)| {
                        let mut signed_tx_op: Option<SignedTransaction> = None;
                        let mut result = (tx_hash.clone(), Ret::Ok);
                        match resp.get_ret() {
                            Ret::Ok => {
                                let mut signed_tx = SignedTransaction::new();
                                signed_tx.set_transaction_with_sig(unverified_tx);
                                signed_tx.set_signer(resp.get_signer().to_vec());
                                signed_tx.set_tx_hash(tx_hash.to_vec());
                                signed_tx_op = Some(signed_tx);

                            }
                            ret @ _ => result.1 = ret,
                        }

                        if id == submodules::NET {
                            tx.send((signed_tx_op, None)).unwrap();
                        } else {
                            tx.send((signed_tx_op, Some(result))).unwrap();
                        }
                    });
                }
                _ => info!("receive error message: {:?}", msg),
            };
        });
    }
}
