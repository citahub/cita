use hash::H256;
use sha3::Hashable;
use hashdb::HashDB;
use super::avldb::AVLDB;
use super::{AVL, AVLItem, AVLIterator, Query};

/// A `AVL` implementation which hashes keys and uses a generic `HashDB` backing database.
///
/// Use it as a `AVL` trait object. You can use `raw()` to get the backing `AVLDB` object.
pub struct SecAVLDB<'db> {
    raw: AVLDB<'db>,
}

impl<'db> SecAVLDB<'db> {
    /// Create a new avl with the backing database `db` and empty `root`
    ///
    /// Initialise to the state entailed by the genesis block.
    /// This guarantees the avl is built correctly.
    /// Returns an error if root does not exist.
    pub fn new(db: &'db HashDB, root: &'db H256) -> super::Result<Self> {
        Ok(SecAVLDB { raw: AVLDB::new(db, root)? })
    }

    /// Get a reference to the underlying raw `AVLDB` struct.
    pub fn raw(&self) -> &AVLDB {
        &self.raw
    }

    /// Get a mutable reference to the underlying raw `AVLDB` struct.
    pub fn raw_mut(&mut self) -> &mut AVLDB<'db> {
        &mut self.raw
    }
}

impl<'db> AVL for SecAVLDB<'db> {
    fn iter<'a>(&'a self) -> super::Result<Box<AVLIterator<Item = AVLItem> + 'a>> {
        AVLDB::iter(&self.raw)
    }

    fn root(&self) -> &H256 {
        self.raw.root()
    }

    fn contains(&self, key: &[u8]) -> super::Result<bool> {
        self.raw.contains(&key.sha3())
    }

    fn get_with<'a, 'key, Q: Query>(&'a self, key: &'key [u8], query: Q)
       -> super::Result<Option<Q::Item>> where 'a: 'key
    {
        self.raw.get_with(&key.sha3(), query)
    }
}

#[test]
fn avl_to_secavl() {
    use memorydb::MemoryDB;
    use hashdb::DBValue;
    use super::avldbmut::AVLDBMut;
    use avl::AVLMut;

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        t.insert(&(&[0x01u8, 0x23]).sha3(), &[0x01u8, 0x23])
            .unwrap();
    }
    let t = SecAVLDB::new(&memdb, &root).unwrap();
    assert_eq!(t.get(&[0x01u8, 0x23]).unwrap().unwrap(),
               DBValue::from_slice(&[0x01u8, 0x23]));
}
