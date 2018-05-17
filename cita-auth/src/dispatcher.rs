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
use libproto::{BatchRequest, Message, Request, Response};
use libproto::blockchain::{AccountGasLimit, BlockBody, BlockTxs, SignedTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use protobuf::RepeatedField;
use serde_json;

use cita_types::H256;
use cita_types::traits::LowerHex;
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
use util::RwLock;
use uuid::Uuid;
use verifier::Verifier;

pub struct Dispatcher {
    txs_pool: RefCell<tx_pool::Pool>,
    tx_pool_cap: Arc<AtomicUsize>,
    wal: TxWal,
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
            wal_enable: wal_enable,
            pool_limit: limit,
            data_from_pool: AtomicBool::new(false),
            batch_forward_info: batch_forward_info,
            response_jsonrpc_cnt: 0,
            start_verify_time: SystemTime::now(),
            add_to_pool_cnt: 0,
        };

        // restore tx data from wal to txs_pool
        if wal_enable {
            let num = dispatch.read_tx_from_wal();
            info!("recovery [{}] transactions into pool.", num);
            if num > 0 {
                dispatch.data_from_pool.store(true, Ordering::SeqCst);
            }
        }
        dispatch
    }

    /// Clean transaction pool and regenerate an pool cache db
    pub fn clear_txs_pool(&mut self, package_limit: usize) {
        self.txs_pool = RefCell::new(tx_pool::Pool::new(package_limit));
        self.wal.regenerate("/txwal");
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

    /// Add new transaction to pool, response jsonrpc request
    /// package buffer request forward to peer
    pub fn deal_tx(
        &mut self,
        key: String,
        req_id: Vec<u8>,
        tx_response: TxResponse,
        tx: &SignedTransaction,
        mq_pub: &Sender<(String, Vec<u8>)>,
        verifier: Arc<RwLock<Verifier>>,
    ) {
        let mut error_msg: Option<String> = None;

        let tx_hash = tx.crypt_hash();
        let ret = verifier.read().check_hash_exist(&tx_hash);
        if ret {
            if verifier.read().is_inited() {
                error_msg = Some(String::from("Dup"));
            } else {
                error_msg = Some(String::from("NotReady"));
            }
        } else if !verifier.read().verify_tx_quota(tx) {
            error_msg = Some(String::from("QuotaNotEnough"));
        } else {
            // add tx to txs_pool and wal.
            if self.add_tx_to_pool(tx) {
                self.update_capacity();
            } else {
                error_msg = Some(String::from("Dup"));
            }
        }

        if error_msg.is_none() {
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

                // Forward tx to peer, regardless the tx is come from Jsonrpc or
                // Other nodes in the network.
                // Wow, it's so much duplicate packages in the network, maybe we can come up with
                // a better way to deal with this kind of network propagation later.
                self.batch_forward_tx_to_peer(mq_pub);
            }
        }

        // RPC response
        if RoutingKey::from(&key).is_sub_module(SubModules::Jsonrpc) {
            let mut response = Response::new();
            response.set_request_id(req_id);
            if error_msg.is_some() {
                response.set_code(ErrorCode::tx_auth_error());
                response.set_error_msg(error_msg.unwrap());
            } else {
                let tx_state = serde_json::to_string(&tx_response).unwrap();
                response.set_tx_state(tx_state);
            }

            self.response_jsonrpc_cnt += 1;
            trace!(
                "response new tx {:?}, with response_jsonrpc_cnt = {}",
                response,
                self.response_jsonrpc_cnt
            );

            let msg: Message = response.into();
            mq_pub
                .send((
                    routing_key!(Auth >> Response).into(),
                    msg.try_into().unwrap(),
                ))
                .unwrap();
        }

        if 0 == self.add_to_pool_cnt {
            self.start_verify_time = SystemTime::now();
        }
        self.add_to_pool_cnt += 1;
    }

    /// Delete transactions from pool, and then package a block with new transactions,
    /// send to cita-bft
    pub fn deal_txs(
        &mut self,
        height: usize,
        txs: &HashSet<H256>,
        mq_pub: &Sender<(String, Vec<u8>)>,
        block_gas_limit: u64,
        account_gas_limit: AccountGasLimit,
        check_quota: bool,
    ) {
        let mut block_txs = BlockTxs::new();
        let mut body = BlockBody::new();

        trace!("deal_txs inner txs height {} ", txs.len());
        if !txs.is_empty() {
            self.del_txs_from_pool_with_hash(txs);
        }

        let out_txs = self.get_txs_from_pool(
            height as u64,
            block_gas_limit,
            account_gas_limit,
            check_quota,
        );
        info!(
            "public block txs height {} with {:?} transactions",
            height,
            out_txs.len()
        );
        {
            let duration = self.start_verify_time.elapsed().unwrap();
            let time_duration_in_usec = duration.as_secs() * 1_000_000 + (duration.subsec_nanos() / 1_000) as u64;
            if 0 != time_duration_in_usec {
                debug!(
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
            .send((
                routing_key!(Auth >> BlockTxs).into(),
                msg.try_into().unwrap(),
            ))
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
        // 交易放入pool，
        // 放入pool完成后，持久化
        trace!("add tx {} to pool", tx.get_tx_hash().lower_hex());
        let txs_pool = &mut self.txs_pool.borrow_mut();
        let success = txs_pool.enqueue(tx.clone());
        if self.wal_enable {
            if success {
                self.wal.write(tx);
            } else {
                warn!(
                    "the transaction {} is already exist",
                    tx.get_tx_hash().lower_hex()
                );
            }
        }
        success
    }

    pub fn get_txs_from_pool(
        &self,
        height: u64,
        block_gas_limit: u64,
        account_gas_limit: AccountGasLimit,
        check_quota: bool,
    ) -> Vec<SignedTransaction> {
        if self.data_from_pool.load(Ordering::SeqCst) {
            self.data_from_pool.store(false, Ordering::SeqCst);
            Vec::new()
        } else {
            let txs_pool = &mut self.txs_pool.borrow_mut();
            txs_pool.package(height, block_gas_limit, account_gas_limit, check_quota)
        }
    }

    pub fn del_txs_from_pool_with_hash(&self, txs: &HashSet<H256>) {
        //收到删除通知，从pool中删除vec中的交易
        {
            self.txs_pool.borrow_mut().update_with_hash(txs);
        }
        //改成多线程删除数据
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
        //收到删除通知，从pool中删除vec中的交易
        {
            self.txs_pool.borrow_mut().update(&txs);
        }
        //改成多线程删除数据
        if self.wal_enable {
            let mut wal = self.wal.clone();
            thread::spawn(move || {
                for tx in txs {
                    wal.delete(&tx);
                }
            });
        }
    }

    // Read tx information from wal, and restore to txs_pool.
    // This function will be called in Dispatcher::new().
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
            .send((
                routing_key!(Auth >> Request).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();

        self.batch_forward_info.forward_stamp = SystemTime::now();
        self.batch_forward_info.new_tx_request_buffer.clear();
    }
}
