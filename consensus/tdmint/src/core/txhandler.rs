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

use crypto::pubkey_to_address;
use libproto::{self as libproto, Response_oneof_result, submodules, topics, factory, communication, MsgClass};
use libproto::blockchain::{SignedTransaction, UnverifiedTransaction};
use libproto::communication::{Message, MsgType};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use util::{snappy, H256};
use util::Mutex;
use uuid::Uuid;

pub type TransType = (u32, Result<SignedTransaction, H256>);

pub struct TxHandler {
    pool: ThreadPool,
    tx: Sender<TransType>,
    tx_pub: Sender<(String, Vec<u8>)>,
    tx_pool: Arc<Mutex<HashMap<Vec<u8>, SignedTransaction>>>,
}

impl TxHandler {
    pub fn new(pool: ThreadPool, tx: Sender<TransType>, tx_pub: Sender<(String, Vec<u8>)>) -> Self {
        TxHandler {
            pool: pool,
            tx: tx,
            tx_pub: tx_pub,
            tx_pool: Arc::new(Mutex::new(HashMap::with_capacity(1000))),
        }
    }

    pub fn receive(&self, id: u32, msg: Vec<u8>) {
        let tx_sender = self.tx.clone();
        let tx_pool = self.tx_pool.clone();
        let tx_pub = self.tx_pub.clone();
        self.pool.execute(move || {
            let (_, _, msg) = libproto::parse_msg(&msg);
            match msg {
                MsgClass::TX(unverified_tx) => {
                    match SignedTransaction::verify_transaction(unverified_tx) {
                        Ok(sign_tx) => {
                            //send to chain for authrity
                            let address = pubkey_to_address(H256::from_slice(sign_tx.get_signer()));
                            //content
                            let request_id = Uuid::new_v4().as_bytes().to_vec();
                            let mut request = libproto::Request::new();
                            request.set_request_id(request_id.clone());

                            let mut call = libproto::Call::new();
                            call.set_to(Vec::from(address.0));
                            //call.set_data(base.data.unwrap_or_default().to_vec());
                            request.set_call(call);

                            let msg = factory::create_msg(submodules::CONSENSUS, topics::REQUEST, communication::MsgType::ROLE, request.write_to_bytes().unwrap());
                            tx_pub.send(("consensus.role".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                            tx_pool.lock().insert(request_id, sign_tx);
                        }
                        Err(_) => {
                            //TODO error deal
                        }
                    }
                }

                MsgClass::RESPONSE(response) => {
                    let tx = {
                        tx_pool.lock().remove(response.get_request_id_for_reflect())
                    };
                    match response.result {
                        Response_oneof_result::call_result(result) => {
                            match tx {
                                Some(tx) => {
                                    tx_sender.send((id, OK(tx))).unwrap();
                                }
                                None => {
                                    error!("tx not exsit!");
                                }
                            }
                        }
                        _ => {
                            error!("error!!!");
                        }
                    }
                }
                _ => info!("recv msg type[{:?}] error", msg.get_field_type()),
            };
        });
    }
}
