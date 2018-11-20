// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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
use cita_types::H256;
use libproto::blockchain::SignedTransaction;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use util::datapath::DataPath;
use util::kvdb::{Database, DatabaseConfig, KeyValueDB};

/// Wal means write ahead log
/// used to persist transaction pools message
#[derive(Clone)]
pub struct TxWal {
    db: Arc<KeyValueDB>,
}

impl TxWal {
    pub fn new(path: &str) -> Self {
        let nosql_path = DataPath::root_node_path() + path;
        // TODO: Can remove db::NUM_COLUMNS
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
        batch.put_vec(None, tx.get_tx_hash(), block_binary);
        self.db.write(batch).expect("insert tx");
    }

    pub fn write_batch(&self, txs: &[SignedTransaction]) {
        let mut batch = self.db.transaction();
        for tx in txs {
            let block_binary: Vec<u8> = tx.try_into().unwrap();
            batch.put_vec(None, tx.get_tx_hash(), block_binary);
        }
        self.db.write(batch).expect("insert batch txs");
    }

    pub fn delete_with_hash(&mut self, tx_hash: &H256) {
        let mut batch = self.db.transaction();
        batch.delete(None, tx_hash);
        self.db.write(batch).expect("delete with hash");
    }

    pub fn delete_with_hashes(&mut self, tx_hashes: &[H256]) {
        let mut batch = self.db.transaction();
        for tx_hash in tx_hashes {
            batch.delete(None, tx_hash);
        }
        self.db.write(batch).expect("delete with hashes");
    }

    pub fn read_all(&self) -> Vec<SignedTransaction> {
        let items = self.db.iter(None);
        items
            .map(|item| SignedTransaction::try_from(item.1.as_ref()).unwrap())
            .collect()
    }

    pub fn get(&self, tx_hash: &[u8]) -> Option<SignedTransaction> {
        let result = self.db.get(None, tx_hash).unwrap();
        result.map(|item| SignedTransaction::try_from(item.as_ref()).unwrap())
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
        let config = DatabaseConfig::with_columns(None);
        let db = Database::open(&config, &tempdir.to_str().unwrap()).unwrap();
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
