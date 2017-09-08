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

use libproto::{self as libproto, MsgClass};
use libproto::blockchain::SignedTransaction;
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use util::H256;

pub type TransType = (u32, Result<SignedTransaction, H256>);

pub struct TxHandler {
    pool: ThreadPool,
    tx: Sender<TransType>,
}

impl TxHandler {
    pub fn new(pool: ThreadPool, tx: Sender<TransType>) -> Self {
        TxHandler { pool: pool, tx: tx }
    }

    pub fn receive(&self, id: u32, msg: Vec<u8>) {
        let tx_sender = self.tx.clone();
        self.pool.execute(move || {
            let (_, _, msg) = libproto::parse_msg(&msg);
            match msg {
                MsgClass::TX(unverified_tx) => {
                    tx_sender.send((id, SignedTransaction::verify_transaction(unverified_tx))).unwrap();
                }
                _ => info!("recv msg type[{:?}] error", msg),
            };
        });
    }
}
