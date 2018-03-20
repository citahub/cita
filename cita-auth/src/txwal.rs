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

use chain_core::db;
use libproto::blockchain::SignedTransaction;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use tx_pool::Pool;
use util::H256;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig, KeyValueDB};

#[derive(Clone)]
pub struct TxWal {
    db: Arc<KeyValueDB>,
}

impl TxWal {
    pub fn new(path: &str) -> Self {
        let nosql_path = DataPath::root_node_path() + path;
        let config = DatabaseConfig::with_columns(db::NUM_COLUMNS);
        let db = Database::open(&config, &nosql_path).unwrap();
        TxWal { db: Arc::new(db) }
    }

    pub fn regenerate(&mut self, path: &str) {
        let nosql_path = DataPath::root_node_path() + path;
        let _ = self.db.restore(&nosql_path);
    }

    pub fn write(&self, tx: &SignedTransaction) {
        let mut batch = self.db.transaction();
        let block_binary: Vec<u8> = tx.try_into().unwrap();
        batch.put_vec(None, tx.clone().get_tx_hash(), block_binary);
        let _ = self.db.write(batch);
    }

    pub fn delete(&mut self, tx: &SignedTransaction) {
        let tx = tx.clone();
        let mut batch = self.db.transaction();
        batch.delete(None, tx.get_tx_hash());
        let _ = self.db.write(batch);
    }

    pub fn delete_with_hash(&mut self, txhash: &H256) {
        let mut batch = self.db.transaction();
        batch.delete(None, txhash);
        let _ = self.db.write(batch);
    }

    pub fn read(&self, pool: &mut Pool) -> u64 {
        let mut num: u64 = 0;
        let ite = self.db.iter(None);
        for item in ite {
            let tx = SignedTransaction::try_from(item.1.as_ref()).unwrap();
            num += 1;
            pool.enqueue(tx);
        }
        info!("read tx num [{}] from pool.", num);
        num
    }
}
