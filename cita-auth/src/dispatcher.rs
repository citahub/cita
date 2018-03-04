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

use error::ErrorCode;
use jsonrpc_types::rpctypes::TxResponse;
use libproto::{BatchRequest, Message, Request, Response, SubModules};
use libproto::blockchain::{AccountGasLimit, BlockBody, BlockTxs, SignedTransaction};
use protobuf::RepeatedField;
use serde_json;

use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::{Into, TryInto};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::SystemTime;
use tx_pool;
use txwal::TxWal;
use util::{H256, ToPretty};
use uuid::Uuid;

pub struct Dispatcher {
    txs_pool: RefCell<tx_pool::Pool>,
    tx_pool_cap: Arc<AtomicUsize>,
    wal: TxWal,
    filter_wal: TxWal,
    wal_enable: bool,
    pool_limit: usize,
    data_from_pool: AtomicBool,
    batch_forward_info: BatchForwardInfo,
    response_jsonrpc_cnt: u64,
    start_verify_time: SystemTime,
    add_to_pool_cnt: u64,
}

pub struct BatchForwardInfo {
    count_per_batch: usize,
    buffer_duration: u32, //in unit of ns
    forward_stamp: SystemTime,
    new_tx_request_buffer: Vec<Request>,
}

impl Dispatcher {
    pub fn new(
        package_limit: usize,
        limit: usize,
        count_per_batch: usize,
        buffer_duration: u32,
        wal_enable: bool,
    ) -> Self {
        let batch_forward_info = BatchForwardInfo {
            count_per_batch: count_per_batch,
            buffer_duration: buffer_duration,
            forward_stamp: SystemTime::now(),
            new_tx_request_buffer: Vec::new(),
        };

        let mut dispatch = Dispatcher {
            txs_pool: RefCell::new(tx_pool::Pool::new(package_limit)),
            tx_pool_cap: Arc::new(AtomicUsize::new(limit)),
            wal: TxWal::new("/txwal"),
            filter_wal: TxWal::new("/filterwal"),
            wal_enable: wal_enable,
            pool_limit: limit,
            data_from_pool: AtomicBool::new(false),
            batch_forward_info: batch_forward_info,
            response_jsonrpc_cnt: 0,
            start_verify_time: SystemTime::now(),
            add_to_pool_cnt: 0,
        };
        if wal_enable {
            let num = dispatch.read_tx_from_wal();
            if num > 0 {
                dispatch.data_from_pool.store(true, Ordering::SeqCst);
            }
        }
        dispatch
    }

    pub fn tx_pool_capacity(&self) -> Arc<AtomicUsize> {
        self.tx_pool_cap.clone()
    }

    fn update_capacity(&mut self) {
        let tx_pool_len = self.txs_pool.borrow().len();
        if self.pool_limit >= tx_pool_len {
            let capacity = self.pool_limit - tx_pool_len;
            self.tx_pool_cap.store(capacity, Ordering::SeqCst);
        } else {
            self.tx_pool_cap.store(0, Ordering::SeqCst);
        }
    }

    pub fn deal_tx(
        &mut self,
        submodule: SubModules,
        req_id: Vec<u8>,
        tx_response: TxResponse,
        tx: &SignedTransaction,
        mq_pub: &Sender<(String, Vec<u8>)>,
    ) {
        let mut error_msg: Option<String> = None;
        if self.add_tx_to_pool(tx) {
            self.update_capacity();
        } else {
            error_msg = Some(String::from("Dup"));
        }

        if submodule == SubModules::Jsonrpc {
            let mut response = Response::new();
            response.set_request_id(req_id);

            if error_msg.is_some() {
                response.set_code(ErrorCode::tx_auth_error());
                response.set_error_msg(error_msg.unwrap());
            } else {
                let tx_state = serde_json::to_string(&tx_response).unwrap();
                response.set_tx_state(tx_state);

                let request_id = Uuid::new_v4().as_bytes().to_vec();
                trace!("send auth.tx with request_id {:?}", request_id);
                let mut request = Request::new();
                request.set_un_tx(tx.get_transaction_with_sig().clone());
                request.set_request_id(request_id);
                self.batch_forward_info.new_tx_request_buffer.push(request);

                let count_buffered = self.batch_forward_info.new_tx_request_buffer.len();
                let time_elapsed = self.batch_forward_info
                    .forward_stamp
                    .elapsed()
                    .unwrap()
                    .subsec_nanos();

                if count_buffered > self.batch_forward_info.count_per_batch
                    || time_elapsed > self.batch_forward_info.buffer_duration
                {
                    trace!(
                        "Going to send new tx batch to peer auth with {} new tx and buffer {} ns",
                        count_buffered,
                        time_elapsed
                    );

                    self.batch_forward_tx_to_peer(mq_pub);
                }
            }
            self.response_jsonrpc_cnt += 1;
            trace!(
                "response new tx {:?}, with response_jsonrpc_cnt = {}",
                response,
                self.response_jsonrpc_cnt
            );

            let msg: Message = response.into();
            mq_pub
                .send(("auth.rpc".to_string(), msg.try_into().unwrap()))
                .unwrap();
        }
        if 0 == self.add_to_pool_cnt {
            self.start_verify_time = SystemTime::now();
        }
        self.add_to_pool_cnt += 1;
    }

    pub fn deal_txs(
        &mut self,
        height: usize,
        txs: &HashSet<H256>,
        mq_pub: &Sender<(String, Vec<u8>)>,
        block_gas_limit: u64,
        account_gas_limit: AccountGasLimit,
    ) {
        let mut block_txs = BlockTxs::new();
        let mut body = BlockBody::new();

        trace!("deal_txs inner txs height {} ", txs.len());
        if !txs.is_empty() {
            self.del_txs_from_pool_with_hash(txs);
        }

        let out_txs = self.get_txs_from_pool(height as u64, block_gas_limit, account_gas_limit);
        info!(
            "public block txs height {} with {:?} txs on timestamp: {:?}",
            height,
            out_txs.len(),
            SystemTime::now()
        );
        {
            let duration = self.start_verify_time.elapsed().unwrap();
            let time_duration_in_usec = duration.as_secs() * 1_000_000 + (duration.subsec_nanos() / 1_000) as u64;
            if 0 != time_duration_in_usec {
                info!(
                    "{} txs have been added into tx_pool, and time cost is {:?}, tps: {:?}",
                    self.add_to_pool_cnt,
                    duration,
                    self.add_to_pool_cnt * 1_000_000 / time_duration_in_usec
                );
            }
            self.add_to_pool_cnt = 0;
        }

        self.update_capacity();
        if !out_txs.is_empty() {
            body.set_transactions(RepeatedField::from_vec(out_txs));
        }
        block_txs.set_height(height as u64);
        block_txs.set_body(body);
        trace!("deal_txs send height {}", height);
        let msg: Message = block_txs.into();
        mq_pub
            .send(("auth.block_txs".to_string(), msg.try_into().unwrap()))
            .unwrap();
    }

    pub fn wait_timeout_process(&mut self, mq_pub: &Sender<(String, Vec<u8>)>) {
        let time_elapsed = self.batch_forward_info
            .forward_stamp
            .elapsed()
            .unwrap()
            .subsec_nanos();
        let count_buffered = self.batch_forward_info.new_tx_request_buffer.len();
        if !self.batch_forward_info.new_tx_request_buffer.is_empty() {
            trace!(
                "wait_timeout_process is going to send new tx batch to peer auth with {} new tx and buffer {} ns",
                count_buffered,
                time_elapsed
            );
            self.batch_forward_tx_to_peer(mq_pub);
        }
    }

    pub fn add_tx_to_pool(&self, tx: &SignedTransaction) -> bool {
        trace!("add tx {} to pool", tx.get_tx_hash().pretty());
        let txs_pool = &mut self.txs_pool.borrow_mut();
        let success = txs_pool.enqueue(tx.clone());
        if self.wal_enable {
            if success {
                self.wal.write(tx);
            } else {
                self.filter_wal.write(tx);
            }
        }
        success
    }

    pub fn get_txs_from_pool(
        &self,
        height: u64,
        block_gas_limit: u64,
        account_gas_limit: AccountGasLimit,
    ) -> Vec<SignedTransaction> {
        if self.data_from_pool.load(Ordering::SeqCst) {
            self.data_from_pool.store(false, Ordering::SeqCst);
            Vec::new()
        } else {
            let txs_pool = &mut self.txs_pool.borrow_mut();
            txs_pool.package(height, block_gas_limit, account_gas_limit)
        }
    }

    pub fn del_txs_from_pool_with_hash(&self, txs: &HashSet<H256>) {
        {
            self.txs_pool.borrow_mut().update_with_hash(txs);
        }
        //TODO Refactor: use thread pool!
        if self.wal_enable {
            let mut wal = self.wal.clone();
            let txs = txs.clone();
            thread::spawn(move || {
                for tx in txs {
                    wal.delete_with_hash(&tx);
                }
            });
        }
    }

    pub fn del_txs_from_pool(&self, txs: Vec<SignedTransaction>) {
        {
            self.txs_pool.borrow_mut().update(&txs);
        }
        //TODO Refactor: use thread pool!
        if self.wal_enable {
            let mut wal = self.wal.clone();
            thread::spawn(move || {
                for tx in txs {
                    wal.delete(&tx);
                }
            });
        }
    }

    pub fn read_tx_from_wal(&mut self) -> u64 {
        let size = self.wal.read(&mut self.txs_pool.borrow_mut());
        self.update_capacity();
        size
    }

    fn batch_forward_tx_to_peer(&mut self, mq_pub: &Sender<(String, Vec<u8>)>) {
        trace!(
            "batch_forward_tx_to_peer is going to send {} new tx to peer",
            self.batch_forward_info.new_tx_request_buffer.len()
        );
        let mut batch_request = BatchRequest::new();
        batch_request.set_new_tx_requests(RepeatedField::from_slice(
            &self.batch_forward_info.new_tx_request_buffer[..],
        ));

        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = Request::new();
        request.set_batch_req(batch_request);
        request.set_request_id(request_id);

        let msg: Message = request.into();
        mq_pub
            .send(("auth.tx".to_string(), msg.try_into().unwrap()))
            .unwrap();

        self.batch_forward_info.forward_stamp = SystemTime::now();
        self.batch_forward_info.new_tx_request_buffer.clear();
    }
}
