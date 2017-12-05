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

use super::{AVLDBIterator, AVLItem, AVLIterator, Query, AVL, AVLDB};
use H256;
use hashable::Hashable;
use hashdb::HashDB;

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
        let fatdb = FatDB {
            raw: AVLDB::new(db, root)?,
        };

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
        self.raw.contains(&key.crypt_hash())
    }

    fn get_with<'a, 'key, Q: Query>(&'a self, key: &'key [u8], query: Q) -> super::Result<Option<Q::Item>>
    where
        'a: 'key,
    {
        self.raw.get_with(&key.crypt_hash(), query)
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
        self.avl_iterator.seek(&key.crypt_hash())
    }
}

impl<'db> Iterator for FatDBIterator<'db> {
    type Item = AVLItem<'db>;

    fn next(&mut self) -> Option<Self::Item> {
        self.avl_iterator.next().map(|res| {
            res.map(|(hash, value)| {
                let aux_hash = hash.crypt_hash();
                (
                    self.avl
                        .db()
                        .get(&aux_hash)
                        .expect("Missing fatdb hash")
                        .to_vec(),
                    value,
                )
            })
        })
    }
}

#[test]
fn fatdb_to_avl() {
    use memorydb::MemoryDB;
    use hashdb::DBValue;
    use avl::{AVLMut, FatDBMut};

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = FatDBMut::new(&mut memdb, &mut root);
        t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    }
    let t = FatDB::new(&memdb, &root).unwrap();
    assert_eq!(
        t.get(&[0x01u8, 0x23]).unwrap().unwrap(),
        DBValue::from_slice(&[0x01u8, 0x23])
    );
    assert_eq!(
        t.iter().unwrap().map(Result::unwrap).collect::<Vec<_>>(),
        vec![
            (
                vec![0x01u8, 0x23],
                DBValue::from_slice(&[0x01u8, 0x23] as &[u8]),
            ),
        ]
    );
}
