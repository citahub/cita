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

use super::AVLMut;
use super::avldbmut::AVLDBMut;
use H256;
use hashable::Hashable;
use hashdb::{DBValue, HashDB};

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
        SecAVLDBMut {
            raw: AVLDBMut::new(db, root),
        }
    }

    /// Create a new avl with the backing database `db` and `root`.
    ///
    /// Returns an error if root does not exist.
    pub fn from_existing(db: &'db mut HashDB, root: &'db mut H256) -> super::Result<Self> {
        Ok(SecAVLDBMut {
            raw: AVLDBMut::from_existing(db, root)?,
        })
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
        self.raw.contains(&key.crypt_hash())
    }

    fn get<'a, 'key>(&'a self, key: &'key [u8]) -> super::Result<Option<DBValue>>
    where
        'a: 'key,
    {
        self.raw.get(&key.crypt_hash())
    }

    fn insert(&mut self, key: &[u8], value: &[u8]) -> super::Result<Option<DBValue>> {
        self.raw.insert(&key.crypt_hash(), value)
    }

    fn remove(&mut self, key: &[u8]) -> super::Result<Option<DBValue>> {
        self.raw.remove(&key.crypt_hash())
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
    assert_eq!(
        t.get(&(&[0x01u8, 0x23]).crypt_hash()).unwrap().unwrap(),
        DBValue::from_slice(&[0x01u8, 0x23])
    );
}
