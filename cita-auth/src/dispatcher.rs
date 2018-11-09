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

use cita_types::traits::LowerHex;
use cita_types::{Address, H256};
use handler::SysConfigInfo;
use libproto::blockchain::{AccountGasLimit, BlockBody, BlockTxs, SignedTransaction};
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::Message;
use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::{Into, TryInto};
use std::sync::mpsc::Sender;
use std::thread;
use tx_pool;
use txwal::TxWal;

pub struct Dispatcher {
    txs_pool: RefCell<tx_pool::Pool>,
    wal: TxWal,
    wal_enable: bool,
}

impl Dispatcher {
    pub fn new(wal_enable: bool) -> Self {
        let mut dispatch = Dispatcher {
            txs_pool: RefCell::new(tx_pool::Pool::new(0)),
            wal: TxWal::new("/txwal"),
            wal_enable,
        };

        // restore tx data from wal to txs_pool
        if wal_enable {
            let num = dispatch.read_tx_from_wal();
            info!("recovery [{}] transactions into pool.", num);
        } else {
            /* prev: enable or disable  now: disable -> delete prev saved
               prev: enable or disable  now: enable  -> read prev saved
            */
            dispatch.wal.regenerate("/txwal");
        }
        dispatch
    }

    /// Clean transaction pool and regenerate an pool cache db
    pub fn clear_txs_pool(&mut self, package_limit: usize) {
        self.txs_pool = RefCell::new(tx_pool::Pool::new(package_limit));
        self.wal.regenerate("/txwal");
    }

    pub fn tx_pool_len(&self) -> usize {
        self.txs_pool.borrow().len()
    }

    /// package a block with new transactions,
    /// send to cita-bft
    pub fn proposal_tx_list(
        &mut self,
        height: usize,
        mq_pub: &Sender<(String, Vec<u8>)>,
        config_info: &SysConfigInfo,
    ) {
        let mut block_txs = BlockTxs::new();
        let mut body = BlockBody::new();

        let out_txs = self.get_txs_from_pool(
            height as u64,
            config_info.block_quota_limit,
            config_info.account_quota_limit.clone(),
            config_info.check_quota,
            &config_info.admin_address,
            config_info.version.unwrap(),
        );
        info!(
            "public block txs height {} with {:?} transactions",
            height,
            out_txs.len()
        );

        if !out_txs.is_empty() {
            body.set_transactions(out_txs.into());
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

    pub fn proposal_empty(&mut self, height: usize, mq_pub: &Sender<(String, Vec<u8>)>) {
        let mut block_txs = BlockTxs::new();
        let body = BlockBody::new();
        block_txs.set_height(height as u64);
        block_txs.set_body(body);
        trace!("proposal empty block height {}", height);
        let msg: Message = block_txs.into();
        mq_pub
            .send((
                routing_key!(Auth >> BlockTxs).into(),
                msg.try_into().unwrap(),
            ))
            .unwrap();
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
        block_quota_limit: u64,
        account_quota_limit: AccountGasLimit,
        check_quota: bool,
        admin_address: &Option<Address>,
        version: u32,
    ) -> Vec<SignedTransaction> {
        let txs_pool = &mut self.txs_pool.borrow_mut();
        txs_pool.package(
            height,
            block_quota_limit,
            account_quota_limit,
            check_quota,
            *admin_address,
            version,
        )
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

    // Read tx information from wal, and restore to txs_pool.
    // This function will be called in Dispatcher::new().
    pub fn read_tx_from_wal(&mut self) -> u64 {
        self.wal.read(&mut self.txs_pool.borrow_mut())
    }
}
