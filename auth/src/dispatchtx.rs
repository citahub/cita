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

extern crate threadpool;
extern crate serde_json;
extern crate tx_pool;

use libproto::{submodules, topics, factory, communication, Response, TxResponse, Request};
use libproto::blockchain::{BlockBody, SignedTransaction, BlockTxs};
use protobuf::{Message, RepeatedField};
use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use txhandler::TransType;
use txwal::Txwal;
use util::H256;

pub struct Dispatchtx {
    txs_pool: RefCell<tx_pool::Pool>,
    wal: Txwal,
    filter_wal: Txwal,
    pool_limit: usize,
}

#[allow(unused_assignments)]
#[allow(unused)]
impl Dispatchtx {
    pub fn new(capacity: usize, package_limit: usize, limit: usize) -> Self {

        let mut dispatch = Dispatchtx {
            txs_pool: RefCell::new(tx_pool::Pool::new(capacity, package_limit)),
            wal: Txwal::new("/txwal"),
            filter_wal: Txwal::new("/filterwal"),
            pool_limit: limit,
        };

        dispatch.read_tx_from_wal();
        dispatch
    }

    pub fn deal_tx(&mut self, modid: u32, req_id: Vec<u8>, mut tx_response: TxResponse, tx: &SignedTransaction, mq_pub: Sender<(String, Vec<u8>)>) {
        let mut error_msg: Option<String> = None;
        if self.tx_flow_control() {
            error_msg = Some(String::from("Busy"));
        } else if !self.add_tx_to_pool(&tx) {
            error_msg = Some(String::from("Dup"));
        }

        if modid == submodules::JSON_RPC {
            let mut response = Response::new();
            response.set_request_id(req_id);

            if error_msg.is_some() {
                response.set_code(submodules::AUTH as i64);
                tx_response.status = error_msg.unwrap();
                response.set_error_msg(format!("{:?}", tx_response));
            } else {
                let tx_state = serde_json::to_string(&tx_response).unwrap();
                response.set_tx_state(tx_state);
            }

            let msg = factory::create_msg(submodules::AUTH, topics::RESPONSE, communication::MsgType::RESPONSE, response.write_to_bytes().unwrap());
            trace!("response new tx {:?}", response);
            mq_pub.send(("auth.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
        }
    }

    pub fn deal_txs(&mut self, height: usize, txs: &Vec<H256>, mq_pub: Sender<(String, Vec<u8>)>) {
        let mut block_txs = BlockTxs::new();
        let mut body = BlockBody::new();

        trace!("deal_txs inner txs height {} ", txs.len());
        if !txs.is_empty() {
            self.del_txs_from_pool_with_hash(txs.clone());
        }
        let out_txs = self.get_txs_from_pool(height as u64);
        trace!("public blcok txs height {} {:?}", height, out_txs.len());
        if !out_txs.is_empty() {
            body.set_transactions(RepeatedField::from_vec(out_txs));
        }
        block_txs.set_height(height as u64);
        block_txs.set_body(body);
        trace!("deal_txs send height {} txs {:?}", height, block_txs);
        let msg = factory::create_msg(submodules::AUTH, topics::BLOCK_TXS, communication::MsgType::BLOCK_TXS, block_txs.write_to_bytes().unwrap());
        mq_pub.send(("auth.block_txs".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }

    pub fn add_tx_to_pool(&self, tx: &SignedTransaction) -> bool {
        //交易放入pool，
        //放入pool完成后，持久化
        let ref mut txs_pool = self.txs_pool.borrow_mut();
        let success = txs_pool.enqueue(tx.clone());
        if success {
            self.wal.write(&tx);
        } else {
            self.filter_wal.write(&tx);
        }
        success
    }

    pub fn get_txs_from_pool(&self, height: u64) -> Vec<SignedTransaction> {
        let txs = self.txs_pool.borrow_mut().package(height);
        txs
    }

    pub fn tx_flow_control(&self) -> bool {
        if self.pool_limit == 0 {
            return false;
        }
        let txs_pool = self.txs_pool.borrow();
        if txs_pool.len() > self.pool_limit { true } else { false }
    }

    pub fn del_txs_from_pool_with_hash(&self, txs: Vec<H256>) {
        //收到删除通知，从pool中删除vec中的交易
        {
            self.txs_pool.borrow_mut().update_with_hash(&txs);
        }
        //改成多线程删除数据
        let mut wal = self.wal.clone();
        thread::spawn(move || for tx in txs {
                          wal.delete_with_hash(&tx);
                      });
    }

    pub fn del_txs_from_pool(&self, txs: Vec<SignedTransaction>) {
        //收到删除通知，从pool中删除vec中的交易
        {
            self.txs_pool.borrow_mut().update(&txs);
        }
        //改成多线程删除数据
        let mut wal = self.wal.clone();
        thread::spawn(move || for tx in txs {
                          wal.delete(&tx);
                      });
    }

    fn receive_new_transaction(&self, request_id: Vec<u8>, signed_tx: Option<SignedTransaction>, result: Option<TxResponse>, tx_pub: Sender<(String, Vec<u8>)>) {
        match signed_tx {
            //Verify ok
            Some(tx) => {
                let mut error_msg: Option<String> = None;
                if self.tx_flow_control() {
                    error_msg = Some(String::from("Busy"));
                } else {
                    if self.add_tx_to_pool(&tx) {
                        let mut request = Request::new();
                        request.set_request_id(request_id.clone());
                        request.set_un_tx(tx.get_transaction_with_sig().clone());
                        let msg = factory::create_msg(submodules::CONSENSUS, topics::REQUEST, communication::MsgType::REQUEST, request.write_to_bytes().unwrap());
                        tx_pub.send(("consensus.tx".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                    } else {
                        error_msg = Some(String::from("Dup"));
                    }
                }

                result.map(|mut tx_response| {
                    let mut response = Response::new();
                    response.set_request_id(request_id);
                    if error_msg.is_some() {
                        response.set_code(submodules::CONSENSUS as i64);
                        tx_response.status = error_msg.unwrap();
                        response.set_error_msg(format!("{:?}", tx_response));
                    } else {
                        let tx_state = serde_json::to_string(&tx_response).unwrap();
                        response.set_tx_state(tx_state);
                    }
                    let msg = factory::create_msg(submodules::CONSENSUS, topics::RESPONSE, communication::MsgType::RESPONSE, response.write_to_bytes().unwrap());
                    trace!("response new tx {:?}", response);
                    tx_pub.send(("consensus.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                });
            }
            //Verify failed
            None => {
                result.map(|tx_response| {
                    let mut response = Response::new();
                    response.set_request_id(request_id);
                    response.set_code(submodules::AUTH as i64);
                    response.set_error_msg(format!("{:?}", tx_response));

                    let msg = factory::create_msg(submodules::CONSENSUS, topics::RESPONSE, communication::MsgType::RESPONSE, response.write_to_bytes().unwrap());
                    trace!("response new tx {:?}", response);
                    tx_pub.send(("consensus.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                });
            }
        }
    }

    pub fn read_tx_from_wal(&mut self) -> u64 {
        self.wal.read(&mut self.txs_pool.borrow_mut())
    }

    pub fn process(&self, rx: &Receiver<TransType>, tx_pub: Sender<(String, Vec<u8>)>) {
        let res = rx.recv().unwrap();
        let (request_id, signed_tx, result) = res;
        self.receive_new_transaction(request_id, signed_tx, result, tx_pub);
    }
}
