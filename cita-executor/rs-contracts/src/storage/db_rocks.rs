use crate::storage::db_trait::{DBError, DataCategory, DataBase};
use rocksdb::{ColumnFamily, Options, DB};
use std::sync::Arc;

pub struct RocksDB<'a> {
    path: &'a str,
    db: Arc<DB>,
}

impl<'a> RocksDB<'a> {
    pub fn new(path: &str) -> Result<RocksDB, DBError> {
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        db_opts.set_max_write_buffer_number(16);

        let catagory = vec![
            get_key_str(DataCategory::Contracts),
        ];
        let db = DB::open_cf(&db_opts, path, catagory.iter())
            .map_err(|e| DBError::Internal(e.to_string()))?;
        Ok(RocksDB {
            path,
            db: Arc::new(db),
        })
    }

    pub fn get_column(&self, column: DataCategory) -> Result<ColumnFamily, DBError> {
        self.db
            .cf_handle(get_key_str(column))
            .ok_or(DBError::NotFound)
    }

     #[cfg(test)]
    pub fn clean(&self) {
        let catagory = vec![
            get_key_str(DataCategory::Contracts),
        ];
        for i in catagory.iter() {
            self.db.drop_cf(i).unwrap();
        }
        // let _ = DB::destroy(&Options::default(), self.path);
    }
}

impl<'a> DataBase for RocksDB<'a> {
    type error = DBError;

    fn insert(
        &self,
        column: DataCategory,
        key: Vec<u8>,
        value: Vec<u8>,
    ) -> Result<(), Self::error> {
        let db = Arc::clone(&self.db);
        let c = self.get_column(column)?;
        db.put_cf(c, key, value).map_err(|e| map_rocks_error(e))?;
        Ok(())
    }

    fn insert_batch(
        &self,
        column: DataCategory,
        keys: Vec<Vec<u8>>,
        values: Vec<Vec<u8>>,
    ) -> Result<(), Self::error> {
        let db = Arc::clone(&self.db);
        let c = self.get_column(column)?;
        for i in 0..keys.len() {
            db.put_cf(c, keys[i].to_vec(), values[i].to_vec())
                .map_err(|e| map_rocks_error(e))?;
        }
        Ok(())
    }

    fn remove(&self, column: DataCategory, key: Vec<u8>) -> Result<(), Self::error> {
        let db = Arc::clone(&self.db);
        let c = self.get_column(column)?;
        db.delete_cf(c, key).map_err(|e| map_rocks_error(e))?;
        Ok(())
    }

    fn remove_batch(&self, column: DataCategory, keys: Vec<Vec<u8>>) -> Result<(), Self::error> {
        let db = Arc::clone(&self.db);
        let c = self.get_column(column)?;
        for i in 0..keys.len() {
            db.delete_cf(c, keys[i].to_vec())
                .map_err(|e| map_rocks_error(e))?;
        }
        Ok(())
    }

    fn contain(&self, column: DataCategory, key: Vec<u8>) -> Result<bool, Self::error> {
        let v = self.get(column, key)?;
        Ok(v.is_some())
    }

    fn get(&self, column: DataCategory, key: Vec<u8>) -> Result<Option<Vec<u8>>, Self::error> {
        let db = Arc::clone(&self.db);
        let c = self.get_column(column)?;
        let value = db.get_cf(c, key).map_err(|e| map_rocks_error(e))?;
        Ok(value.map(|v| v.to_vec()))
    }
}

pub fn get_key_str(column: DataCategory) -> &'static str {
    match column {
        DataCategory::Contracts => "contracts",
    }
}

pub fn map_rocks_error(err: rocksdb::Error) -> DBError {
    DBError::Internal(err.to_string())
}

#[cfg(test)]
mod tests {
    use super::RocksDB;
    use crate::storage::db_trait::{DataCategory, DataBase};

    #[test]
    fn test_get_should_return_ok() {
        let db = RocksDB::new("rocksdb/test_get_should_return_ok").unwrap();
        assert_eq!(db.get(DataCategory::Contracts, b"test".to_vec()).unwrap(), None);
        let _ = db.insert(DataCategory::Contracts, b"test".to_vec(), b"value".to_vec());
        assert_eq!(
            db.get(DataCategory::Contracts, b"test".to_vec()).unwrap(),
            Some(b"value".to_vec())
        );
        db.clean();
    }

    #[test]
    fn test_insert_batch_should_return_ok() {
        let db = RocksDB::new("rocksdb/test_insert_batch_should_return_ok").unwrap();
        db.insert_batch(
            DataCategory::Contracts,
            vec![b"test1".to_vec(), b"test2".to_vec(), b"test3".to_vec()],
            vec![b"value1".to_vec(), b"value2".to_vec(), b"value3".to_vec()],
        )
        .unwrap();

        assert_eq!(
            db.get(DataCategory::Contracts, b"test1".to_vec()).unwrap(),
            Some(b"value1".to_vec())
        );
        assert_eq!(
            db.get(DataCategory::Contracts, b"test2".to_vec()).unwrap(),
            Some(b"value2".to_vec())
        );
        assert_eq!(
            db.get(DataCategory::Contracts, b"test3".to_vec()).unwrap(),
            Some(b"value3".to_vec())
        );
        db.clean();
    }

    #[test]
    fn test_contain_should_return_true() {
        let db = RocksDB::new("rocksdb/test_contain_should_return_true").unwrap();
        let _ = db.insert(DataCategory::Contracts, b"test".to_vec(), b"value".to_vec());
        assert!(db.contain(DataCategory::Contracts, b"test".to_vec()).unwrap());
        db.clean();
    }

    #[test]
    fn test_remove_should_return_ok() {
        let db = RocksDB::new("rocksdb/test_remove_should_return_ok").unwrap();
        let _ = db.insert(DataCategory::Contracts, b"test".to_vec(), b"value".to_vec());
        assert!(db.contain(DataCategory::Contracts, b"test".to_vec()).unwrap());
        db.remove(DataCategory::Contracts, b"test".to_vec()).unwrap();
        assert!(!db.contain(DataCategory::Contracts, b"test".to_vec()).unwrap());
        db.clean();
    }

    #[test]
    fn test_remove_batch_should_return_ok() {
        let db = RocksDB::new("rocksdb/test_remove_batch_should_return_ok").unwrap();
        db.insert_batch(
            DataCategory::Contracts,
            vec![b"test1".to_vec(), b"test2".to_vec(), b"test3".to_vec()],
            vec![b"value1".to_vec(), b"value2".to_vec(), b"value3".to_vec()],
        )
        .unwrap();

        assert!(db.contain(DataCategory::Contracts, b"test1".to_vec()).unwrap());
        assert!(db.contain(DataCategory::Contracts, b"test2".to_vec()).unwrap());
        assert!(db.contain(DataCategory::Contracts, b"test3".to_vec()).unwrap());

        db.remove_batch(
            DataCategory::Contracts,
            vec![b"test1".to_vec(), b"test2".to_vec()],
        )
        .unwrap();
        assert!(!db.contain(DataCategory::Contracts, b"test1".to_vec()).unwrap());
        assert!(!db.contain(DataCategory::Contracts, b"test2".to_vec()).unwrap());
        assert!(db.contain(DataCategory::Contracts, b"test3".to_vec()).unwrap());
        db.clean();
    }
}
