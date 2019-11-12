#[derive(Debug, Copy, Clone)]
pub enum DataCategory {
    Contracts,
}

#[derive(Debug)]
pub enum DBError {
    A,
    B,
    Internal(String),
}

pub trait DB {
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

pub fn get_key_str(column: DataCategory) -> &'static str {
    match column {
        DataCategory::Block => "block",
        DataCategory::Transaction => "transaction",
    }
}
