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

use core::txhandler::TxHandler;
use core::txwal::Txwal;
use libproto::{submodules, topics, factory, communication};
use libproto::auth::Ret;
use libproto::blockchain::{TxResponse, SignedTransaction};
use protobuf::Message;
use pubsub::start_pubsub;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use tx_pool::Pool;
use util::H256;

pub struct Dispatchtx {
    tx_pool: Arc<RwLock<Pool>>,
    wal: Txwal,
    filter_wal: Txwal,
    data_from_pool: AtomicBool,
    pool_limit: usize,
}

#[allow(unused_assignments)]
#[allow(unused)]
impl Dispatchtx {
    pub fn new(capacity: usize, package_limit: usize, limit: usize) -> Self {

        let mut dispatch = Dispatchtx {
            tx_pool: Arc::new(RwLock::new(Pool::new(capacity, package_limit))),
            wal: Txwal::new("/txwal"),
            filter_wal: Txwal::new("/filterwal"),
            data_from_pool: AtomicBool::new(false),
            pool_limit: limit,
        };

        let num = dispatch.read_tx_from_wal();
        if num > 0 {
            dispatch.data_from_pool.store(true, Ordering::SeqCst);
        }
        dispatch
    }


    pub fn add_tx_to_pool(&self, tx: &SignedTransaction) -> bool {
        //交易放入pool，
        //放入pool完成后，持久化
        let mut tx_pool = self.tx_pool.write().unwrap();
        let trans = tx.clone();
        let success = tx_pool.enqueue(trans);
        if success {
            self.wal.write(&tx);
        } else {
            self.filter_wal.write(&tx);
        }
        success
    }

    pub fn get_txs_from_pool(&self, height: u64) -> Vec<SignedTransaction> {
        if self.data_from_pool.load(Ordering::SeqCst) {
            self.data_from_pool.store(false, Ordering::SeqCst);
            Vec::new()
        } else {
            let mut tx_pool = self.tx_pool.write().unwrap();
            let txs = tx_pool.package(height);
            txs
        }
    }

    pub fn tx_flow_control(&self) -> bool {

        if self.pool_limit == 0 {
            return false;
        }

        let tx_pool = self.tx_pool.read().unwrap();
        if tx_pool.len() > self.pool_limit { true } else { false }
    }

    pub fn del_txs_from_pool(&self, txs: Vec<SignedTransaction>) {
        //收到删除通知，从pool中删除vec中的交易
        {
            self.tx_pool.write().unwrap().update(&txs);
        }
        //改成多线程删除数据
        let mut wal = self.wal.clone();
        thread::spawn(move || for tx in txs {
                          wal.delete(&tx);
                      });
    }

    //TODO error return JsonRpc
    fn receive_new_transaction(&self, signed_tx: Option<SignedTransaction>, result: Option<(H256, Ret)>, tx_pub: Sender<(String, Vec<u8>)>) {
        let mut error_msg: Option<String> = None;
        signed_tx.map(|signed_tx| if self.tx_flow_control() {
                          error_msg = Some(String::from("BUSY"));
                      } else {
                          let is_success = self.add_tx_to_pool(&signed_tx);
                          if is_success {
                              let msg = factory::create_msg(submodules::CONSENSUS, topics::NEW_TX, communication::MsgType::TX, signed_tx.get_transaction_with_sig().write_to_bytes().unwrap());
                              tx_pub.send(("consensus.tx".to_string(), msg.write_to_bytes().unwrap())).unwrap();
                          } else {
                              error_msg = Some(String::from("4:DUP"));
                          }
                      });

        // tx from net, we don't need to reply
        if result.is_none() {
            return;
        }

        let (hash, ret) = result.unwrap();
        let mut tx_response = TxResponse::new();
        tx_response.set_hash(hash.to_vec());
        let ret = match ret {
            Ret::Ok => "4:OK".to_string(),
            Ret::Dup => "4:DUP".to_string(),
            Ret::NotReady => "Not Ready".to_string(),
            Ret::OutOfTime => "Out ot Time".to_string(),
            Ret::BadSig => "BAD SIG".to_string(),
            Ret::Err => "Err".to_string(),
        };
        tx_response.set_result(ret.into_bytes());

        if error_msg.is_some() {
            tx_response.set_result(error_msg.unwrap().into_bytes());
        }
        let msg = factory::create_msg(submodules::CONSENSUS, topics::TX_RESPONSE, communication::MsgType::TX_RESPONSE, tx_response.write_to_bytes().unwrap());
        trace!("response new tx {:?}", tx_response.get_hash());
        tx_pub.send(("consensus.rpc".to_string(), msg.write_to_bytes().unwrap())).unwrap();
    }

    pub fn read_tx_from_wal(&mut self) -> u64 {
        let mut tx_pool = self.tx_pool.write().unwrap();
        self.wal.read(&mut tx_pool)
    }

    pub fn process(&self, rx: &Receiver<(Option<SignedTransaction>, Option<(H256, Ret)>)>, tx_pub: Sender<(String, Vec<u8>)>) {
        let res = rx.recv().unwrap();
        let (signed_tx, result) = res;
        self.receive_new_transaction(signed_tx, result, tx_pub);
    }
}

pub fn sub_new_tx(dispatch: Arc<Dispatchtx>, num_thds: usize) {
    let _ = thread::Builder::new().name("consensus_new_tx".to_string()).spawn(move || {
        let (tx, rx) = channel();
        let threadpool = threadpool::ThreadPool::with_name("consensus_recv_tx_pool".to_string(), num_thds);
        let (tx_sub, rx_sub) = channel();
        let (tx_pub, rx_pub) = channel();
        let handler = TxHandler::new(threadpool, tx, tx_pub.clone());
        start_pubsub("consensus_tx", vec!["net.tx", "jsonrpc.new_tx", "verify_tx_consensus"], tx_sub, rx_pub);
        thread::spawn(move || loop {
                          let (key, body) = rx_sub.recv().unwrap();
                          handler.handle(key, body);
                      });
        loop {
            dispatch.process(&rx, tx_pub.clone());
        }
    });
}
