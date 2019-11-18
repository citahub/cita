#[derive(Debug)]
pub enum DBError {
    NotFound,
    B,
    Internal(String),
}

#[derive(Debug, Copy, Clone)]
pub enum DataCategory {
    Contracts,
}

pub trait DataBase {
    type error;

    fn insert(&self, column: DataCategory, key: Vec<u8>, value: Vec<u8>)
        -> Result<(), Self::error>;

    fn insert_batch(
        &self,
        column: DataCategory,
        keys: Vec<Vec<u8>>,
        values: Vec<Vec<u8>>,
    ) -> Result<(), Self::error>;

    fn remove(&self, column: DataCategory, key: Vec<u8>) -> Result<(), Self::error>;

    fn remove_batch(&self, column: DataCategory, keys: Vec<Vec<u8>>) -> Result<(), Self::error>;

    fn contain(&self, column: DataCategory, key: Vec<u8>) -> Result<bool, Self::error>;

    fn get(&self, column: DataCategory, key: Vec<u8>) -> Result<Option<Vec<u8>>, Self::error>;
}
