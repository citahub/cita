use hash::H256;
use sha3::Hashable;
use hashdb::{HashDB, DBValue};
use super::avldbmut::AVLDBMut;
use super::AVLMut;

/// A mutable `AVL` implementation which hashes keys and uses a generic `HashDB` backing database.
///
/// Use it as a `AVL` or `AVLMut` trait object. You can use `raw()` to get the backing `AVLDBMut` object.
pub struct SecAVLDBMut<'db> {
    raw: AVLDBMut<'db>,
}

impl<'db> SecAVLDBMut<'db> {
    /// Create a new avl with the backing database `db` and empty `root`
    /// Initialise to the state entailed by the genesis block.
    /// This guarantees the avl is built correctly.
    pub fn new(db: &'db mut HashDB, root: &'db mut H256) -> Self {
        SecAVLDBMut { raw: AVLDBMut::new(db, root) }
    }

    /// Create a new avl with the backing database `db` and `root`.
    ///
    /// Returns an error if root does not exist.
    pub fn from_existing(db: &'db mut HashDB, root: &'db mut H256) -> super::Result<Self> {
        Ok(SecAVLDBMut { raw: AVLDBMut::from_existing(db, root)? })
    }

    /// Get the backing database.
    pub fn db(&self) -> &HashDB {
        self.raw.db()
    }

    /// Get the backing database.
    pub fn db_mut(&mut self) -> &mut HashDB {
        self.raw.db_mut()
    }
}

impl<'db> AVLMut for SecAVLDBMut<'db> {
    fn root(&mut self) -> &H256 {
        self.raw.root()
    }

    fn is_empty(&self) -> bool {
        self.raw.is_empty()
    }

    fn contains(&self, key: &[u8]) -> super::Result<bool> {
        self.raw.contains(&key.sha3())
    }

    fn get<'a, 'key>(&'a self, key: &'key [u8]) -> super::Result<Option<DBValue>>
        where 'a: 'key
    {
        self.raw.get(&key.sha3())
    }

    fn insert(&mut self, key: &[u8], value: &[u8]) -> super::Result<Option<DBValue>> {
        self.raw.insert(&key.sha3(), value)
    }

    fn remove(&mut self, key: &[u8]) -> super::Result<Option<DBValue>> {
        self.raw.remove(&key.sha3())
    }
}

#[test]
fn secavl_to_avl() {
    use memorydb::*;
    use super::avldb::*;
    use super::AVL;

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = SecAVLDBMut::new(&mut memdb, &mut root);
        t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    }
    let t = AVLDB::new(&memdb, &root).unwrap();
    assert_eq!(t.get(&(&[0x01u8, 0x23]).sha3()).unwrap().unwrap(),
               DBValue::from_slice(&[0x01u8, 0x23]));
}
