// Copyright Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_database::{Config, Database, RocksDB, NUM_COLUMNS};
use cita_directories::DataPath;
use cita_types::H256;
use libproto::blockchain::SignedTransaction;
use libproto::{TryFrom, TryInto};
use std::sync::Arc;

/// Wal means write ahead log
/// used to persist transaction pools message
#[derive(Clone)]
pub struct TxWal {
    db: Arc<dyn Database>,
}

impl TxWal {
    pub fn new(path: &str) -> Self {
        let nosql_path = DataPath::root_node_path() + path;
        // TODO: Can remove NUM_COLUMNS(useless)
        let config = Config::with_category_num(NUM_COLUMNS);
        let db = RocksDB::open(&nosql_path, &config).unwrap();
        TxWal { db: Arc::new(db) }
    }

    pub fn regenerate(&mut self, path: &str) {
        let nosql_path = DataPath::root_node_path() + path;
        let _ = Arc::get_mut(&mut self.db).unwrap().restore(&nosql_path);
    }

    pub fn write(&self, tx: &SignedTransaction) {
        // TODO Fix the block_binary. tx_binary?
        let block_binary: Vec<u8> = tx.try_into().unwrap();
        self.db
            .insert(None, tx.get_tx_hash().to_vec(), block_binary)
            .expect("insert tx");
    }

    pub fn write_batch(&self, txs: &[SignedTransaction]) {
        let mut values: Vec<Vec<u8>> = Vec::new();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for tx in txs {
            let block_binary: Vec<u8> = tx.try_into().unwrap();
            values.push(block_binary);
            keys.push(tx.get_tx_hash().to_vec());
        }
        self.db
            .insert_batch(None, keys, values)
            .expect("insert batch txs");
    }

    pub fn delete_with_hash(&mut self, tx_hash: &H256) {
        self.db.remove(None, tx_hash).expect("delete with hash");
    }

    pub fn delete_with_hashes(&mut self, tx_hashes: &[H256]) {
        let mut keys: Vec<Vec<u8>> = Vec::new();
        for tx_hash in tx_hashes {
            keys.push(tx_hash.to_vec());
        }
        self.db
            .remove_batch(None, &keys)
            .expect("delete with hashes");
    }

    pub fn read_all(&self) -> Vec<SignedTransaction> {
        // TODO fix the unwrap
        let items = self.db.iterator(None).unwrap();

        items
            .map(|item| SignedTransaction::try_from(item.1.as_ref()).unwrap())
            .collect()
    }

    pub fn get(&self, tx_hash: &[u8]) -> Option<SignedTransaction> {
        // TODO fix the unwrap
        let result = self.db.get(None, tx_hash).unwrap();
        result.map(|item| SignedTransaction::try_from(&item).unwrap())
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use self::tempdir::TempDir;
    use super::*;
    use crypto::{CreateKey, KeyPair};
    use libproto::blockchain::Transaction;

    fn tx_wal() -> TxWal {
        let tempdir = TempDir::new("").unwrap().into_path();
        let config = Config::with_category_num(None);
        let db = RocksDB::open(&tempdir.to_str().unwrap(), &config).unwrap();
        TxWal { db: Arc::new(db) }
    }

    #[test]
    fn test_write_delete() {
        let mut wal = tx_wal();
        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let mut raw_tx = Transaction::new();
        raw_tx.quota = 1000;
        let tx = raw_tx.sign(*privkey);
        wal.write(&tx);

        let tx1 = wal.get(tx.get_tx_hash());

        assert_eq!(Some(tx.clone()), tx1);

        wal.delete_with_hash(&H256::from(tx.get_tx_hash()));
        let tx2 = wal.get(tx.get_tx_hash());

        assert_eq!(None, tx2);
    }

    #[test]
    fn test_batch() {
        let mut wal = tx_wal();

        let keypair = KeyPair::gen_keypair();
        let privkey = keypair.privkey();
        let mut raw_tx = Transaction::new();
        raw_tx.quota = 1000;
        let tx1 = raw_tx.sign(*privkey);
        let mut raw_tx2 = Transaction::new();
        raw_tx2.quota = 1000;
        let tx2 = raw_tx2.sign(*privkey);
        wal.write_batch(&vec![tx1.clone(), tx2.clone()]);

        let tx11 = wal.get(tx1.get_tx_hash());
        assert_eq!(Some(tx1.clone()), tx11);

        let tx11 = wal.get(tx2.get_tx_hash());
        assert_eq!(Some(tx2.clone()), tx11);

        wal.delete_with_hashes(&vec![
            H256::from(tx1.get_tx_hash()),
            H256::from(tx2.get_tx_hash()),
        ]);
        let tx12 = wal.get(tx1.get_tx_hash());
        let tx22 = wal.get(tx2.get_tx_hash());

        assert_eq!(None, tx12);
        assert_eq!(None, tx22);
    }
}
