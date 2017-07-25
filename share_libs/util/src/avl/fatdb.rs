use hash::H256;
use sha3::Hashable;
use hashdb::HashDB;
use super::{AVLDB, AVL, AVLDBIterator, AVLItem, AVLIterator, Query};

/// A `AVL` implementation which hashes keys and uses a generic `HashDB` backing database.
/// Additionaly it stores inserted hash-key mappings for later retrieval.
///
/// Use it as a `AVL` or `AVLMut` trait object.
pub struct FatDB<'db> {
    raw: AVLDB<'db>,
}

impl<'db> FatDB<'db> {
    /// Create a new avl with the backing database `db` and empty `root`
    /// Initialise to the state entailed by the genesis block.
    /// This guarantees the avl is built correctly.
    pub fn new(db: &'db HashDB, root: &'db H256) -> super::Result<Self> {
        let fatdb = FatDB { raw: AVLDB::new(db, root)? };

        Ok(fatdb)
    }

    /// Get the backing database.
    pub fn db(&self) -> &HashDB {
        self.raw.db()
    }
}

impl<'db> AVL for FatDB<'db> {
    fn iter<'a>(&'a self) -> super::Result<Box<AVLIterator<Item = AVLItem> + 'a>> {
        FatDBIterator::new(&self.raw).map(|iter| Box::new(iter) as Box<_>)
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

/// Itarator over inserted pairs of key values.
pub struct FatDBIterator<'db> {
    avl_iterator: AVLDBIterator<'db>,
    avl: &'db AVLDB<'db>,
}

impl<'db> FatDBIterator<'db> {
    /// Creates new iterator.
    pub fn new(avl: &'db AVLDB) -> super::Result<Self> {
        Ok(FatDBIterator {
               avl_iterator: AVLDBIterator::new(avl)?,
               avl: avl,
           })
    }
}

impl<'db> AVLIterator for FatDBIterator<'db> {
    fn seek(&mut self, key: &[u8]) -> super::Result<()> {
        self.avl_iterator.seek(&key.sha3())
    }
}

impl<'db> Iterator for FatDBIterator<'db> {
    type Item = AVLItem<'db>;

    fn next(&mut self) -> Option<Self::Item> {
		self.avl_iterator.next()
			.map(|res|
				res.map(|(hash, value)| {
					let aux_hash = hash.sha3();
					(self.avl.db().get(&aux_hash).expect("Missing fatdb hash").to_vec(), value)
				})
			)
	}
}

#[test]
fn fatdb_to_avl() {
    use memorydb::MemoryDB;
    use hashdb::DBValue;
    use avl::{FatDBMut, AVLMut};

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = FatDBMut::new(&mut memdb, &mut root);
        t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    }
    let t = FatDB::new(&memdb, &root).unwrap();
    assert_eq!(t.get(&[0x01u8, 0x23]).unwrap().unwrap(),
               DBValue::from_slice(&[0x01u8, 0x23]));
    assert_eq!(t.iter().unwrap().map(Result::unwrap).collect::<Vec<_>>(),
               vec![(vec![0x01u8, 0x23], DBValue::from_slice(&[0x01u8, 0x23] as &[u8]))]);
}
