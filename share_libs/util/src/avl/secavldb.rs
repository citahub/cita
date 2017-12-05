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

use super::{AVLItem, AVLIterator, Query, AVL};
use super::avldb::AVLDB;
use H256;
use hashable::Hashable;
use hashdb::HashDB;

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
        Ok(SecAVLDB {
            raw: AVLDB::new(db, root)?,
        })
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
        self.raw.contains(&key.crypt_hash())
    }

    fn get_with<'a, 'key, Q: Query>(&'a self, key: &'key [u8], query: Q) -> super::Result<Option<Q::Item>>
    where
        'a: 'key,
    {
        self.raw.get_with(&key.crypt_hash(), query)
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
        t.insert(&(&[0x01u8, 0x23]).crypt_hash(), &[0x01u8, 0x23])
            .unwrap();
    }
    let t = SecAVLDB::new(&memdb, &root).unwrap();
    assert_eq!(
        t.get(&[0x01u8, 0x23]).unwrap().unwrap(),
        DBValue::from_slice(&[0x01u8, 0x23])
    );
}
