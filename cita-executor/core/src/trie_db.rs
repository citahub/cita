use std::sync::Arc;

use cita_database::error::DatabaseError;
use cita_database::{DataCategory, Database};
use cita_types::H256;
use hashable::HASH_NULL_RLP;
use parking_lot::RwLock;
use std::collections::HashMap;

static NULL_RLP_STATIC: [u8; 1] = [0x80; 1];

#[derive(Debug)]
pub struct TrieDB<DB>
where
    DB: Database,
{
    db: Arc<DB>,
    cache: Arc<RwLock<HashMap<Vec<u8>, Vec<u8>>>>,
}

impl<DB> TrieDB<DB>
where
    DB: Database,
{
    pub fn new(db: Arc<DB>) -> Self {
        TrieDB {
            db,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn database(&self) -> Arc<DB> {
        self.db.clone()
    }
}

/// "TrieDB" provides state read/write capabilities for executor.
impl<DB> cita_trie::DB for TrieDB<DB>
where
    DB: Database,
{
    type Error = DatabaseError;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Self::Error> {
        if H256::from(key) == HASH_NULL_RLP {
            return Ok(Some(NULL_RLP_STATIC.to_vec()));
        }
        match self.cache.read().get(key) {
            Some(v) => Ok(Some(v.to_vec())),
            None => self.db.get(Some(DataCategory::State), key),
        }
    }

    fn insert(&self, key: Vec<u8>, value: Vec<u8>) -> Result<(), Self::Error> {
        if H256::from(key.as_slice()) == HASH_NULL_RLP {
            return Ok(());
        }
        self.cache.write().insert(key, value);
        Ok(())
    }

    fn contains(&self, key: &[u8]) -> Result<bool, Self::Error> {
        if H256::from(key) == HASH_NULL_RLP {
            return Ok(true);
        }
        if self.cache.read().contains_key(key) {
            Ok(true)
        } else {
            self.db.contains(Some(DataCategory::State), key)
        }
    }

    fn remove(&self, key: &[u8]) -> Result<(), Self::Error> {
        if H256::from(key) == HASH_NULL_RLP {
            return Ok(());
        }
        self.cache.write().remove(key);
        self.db.remove(Some(DataCategory::State), key)
    }

    fn insert_batch(&self, keys: Vec<Vec<u8>>, values: Vec<Vec<u8>>) -> Result<(), Self::Error> {
        let mut cache = self.cache.write();
        for i in 0..keys.len() {
            let key = keys[i].clone();
            if H256::from(key.as_slice()) == HASH_NULL_RLP {
                continue;
            }
            let value = values[i].clone();
            cache.insert(key, value);
        }
        Ok(())
    }

    fn remove_batch(&self, keys: &[Vec<u8>]) -> Result<(), Self::Error> {
        let keys: Vec<Vec<u8>> = keys
            .iter()
            .cloned()
            .filter(|key| H256::from(key.as_slice()) != HASH_NULL_RLP)
            .collect();
        {
            let mut cache = self.cache.write();
            for key in &keys {
                cache.remove(key);
            }
        }
        self.db.remove_batch(Some(DataCategory::State), &keys)
    }

    fn flush(&self) -> Result<(), Self::Error> {
        let len = self.cache.read().len();
        let mut keys = Vec::with_capacity(len);
        let mut values = Vec::with_capacity(len);

        for (key, value) in self.cache.write().drain() {
            keys.push(key);
            values.push(value);
        }

        self.db
            .insert_batch(Some(DataCategory::State), keys.to_vec(), values.to_vec())
    }
}

impl<DB> Clone for TrieDB<DB>
where
    DB: Database,
{
    fn clone(&self) -> Self {
        TrieDB {
            db: Arc::clone(&self.db),
            cache: Arc::clone(&self.cache),
        }
    }
}
