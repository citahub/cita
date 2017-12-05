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

//! AVL interface and implementation.

use H256;
use hashable::HASH_NULL_RLP;
use hashdb::{DBValue, HashDB};
use std::fmt;

/// Export the standardmap module.
pub mod standardmap;
/// Export the node module.
pub mod node;
/// Export the avldb module.
pub mod avldb;
/// Export the avldbmut module.
pub mod avldbmut;
/// Export the secavldb module.
pub mod secavldb;
/// Export the secavldbmut module.
pub mod secavldbmut;
/// AVL query recording.
pub mod recorder;


mod fatdb;
mod fatdbmut;
mod lookup;

// pub use self::standardmap::{Alphabet, StandardMap, ValueMode};

pub use self::avldb::{AVLDBIterator, AVLDB};
pub use self::avldbmut::AVLDBMut;
pub use self::fatdb::{FatDB, FatDBIterator};
pub use self::fatdbmut::FatDBMut;
pub use self::recorder::Recorder;
pub use self::secavldb::SecAVLDB;
pub use self::secavldbmut::SecAVLDBMut;

/// AVL Errors.
///
/// These borrow the data within them to avoid excessive copying on every
/// AVL operation.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AVLError {
    /// Attempted to create an AVL with a state root not in the DB.
    InvalidStateRoot(H256),
    /// AVL item not found in the database,
    IncompleteDatabase(H256),
}

impl fmt::Display for AVLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AVLError::InvalidStateRoot(ref root) => write!(f, "Invalid state root: {}", root),
            AVLError::IncompleteDatabase(ref missing) => write!(f, "Database missing expected key: {}", missing),
        }
    }
}

/// AVL result type. Boxed to avoid copying around extra space for `H256`s on successful queries.
pub type Result<T> = ::std::result::Result<T, Box<AVLError>>;

/// AVL-Item type.
pub type AVLItem<'a> = Result<(Vec<u8>, DBValue)>;

/// Description of what kind of query will be made to the AVL.
///
/// This is implemented for any &mut recorder (where the query will return
/// a DBValue), any function taking raw bytes (where no recording will be made),
/// or any tuple of (&mut Recorder, FnOnce(&[u8]))
pub trait Query {
    /// Output item.
    type Item;

    /// Decode a byte-slice into the desired item.
    fn decode(self, &[u8]) -> Self::Item;

    /// Record that a node has been passed through.
    fn record(&mut self, &H256, &[u8], u32) {}
}

impl<'a> Query for &'a mut Recorder {
    type Item = DBValue;

    fn decode(self, value: &[u8]) -> DBValue {
        DBValue::from_slice(value)
    }
    fn record(&mut self, hash: &H256, data: &[u8], depth: u32) {
        (&mut **self).record(hash, data, depth);
    }
}

impl<F, T> Query for F
where
    F: for<'a> FnOnce(&'a [u8]) -> T,
{
    type Item = T;

    fn decode(self, value: &[u8]) -> T {
        (self)(value)
    }
}

impl<'a, F, T> Query for (&'a mut Recorder, F)
where
    F: FnOnce(&[u8]) -> T,
{
    type Item = T;

    fn decode(self, value: &[u8]) -> T {
        (self.1)(value)
    }
    fn record(&mut self, hash: &H256, data: &[u8], depth: u32) {
        self.0.record(hash, data, depth)
    }
}

/// A key-value datastore implemented as a database-backed modified Merkle tree.
pub trait AVL {
    /// Return the root of the avl.
    fn root(&self) -> &H256;

    /// Is the avl empty?
    fn is_empty(&self) -> bool {
        *self.root() == HASH_NULL_RLP
    }

    /// Does the avl contain a given key?
    fn contains(&self, key: &[u8]) -> Result<bool> {
        self.get(key).map(|x| x.is_some())
    }

    /// What is the value of the given key in this AVL?
    fn get<'a, 'key>(&'a self, key: &'key [u8]) -> Result<Option<DBValue>>
    where
        'a: 'key,
    {
        self.get_with(key, DBValue::from_slice)
    }

    /// Search for the key with the given query parameter. See the docs of the `Query`
    /// trait for more details.
    fn get_with<'a, 'key, Q: Query>(&'a self, key: &'key [u8], query: Q) -> Result<Option<Q::Item>>
    where
        'a: 'key;

    /// Returns a depth-first iterator over the elements of avl.
    fn iter<'a>(&'a self) -> Result<Box<AVLIterator<Item = AVLItem> + 'a>>;
}

/// A key-value datastore implemented as a database-backed modified Merkle tree.
pub trait AVLMut {
    /// Return the root of the AVL.
    fn root(&mut self) -> &H256;

    /// Is the AVL empty?
    fn is_empty(&self) -> bool;

    /// Does the AVL contain a given key?
    fn contains(&self, key: &[u8]) -> Result<bool> {
        self.get(key).map(|x| x.is_some())
    }

    /// What is the value of the given key in this AVL?
    fn get<'a, 'key>(&'a self, key: &'key [u8]) -> Result<Option<DBValue>>
    where
        'a: 'key;

    /// Insert a `key`/`value` pair into the AVL. An empty value is equivalent to removing
    /// `key` from the AVL. Returns the old value associated with this key, if it existed.
    fn insert(&mut self, key: &[u8], value: &[u8]) -> Result<Option<DBValue>>;

    /// Remove a `key` from the AVL. Equivalent to making it equal to the empty
    /// value. Returns the old value associated with this key, if it existed.
    fn remove(&mut self, key: &[u8]) -> Result<Option<DBValue>>;
}

/// A AVL iterator that also supports random access.
pub trait AVLIterator: Iterator {
    /// Position the iterator on the first element with key > `key`
    fn seek(&mut self, key: &[u8]) -> Result<()>;
}

/// AVL types
#[derive(Debug, PartialEq, Clone)]
pub enum AVLSpec {
    /// Generic AVL.
    Generic,
    /// Secure AVL.
    Secure,
    ///    Secure AVL with fat database.
    Fat,
}

impl Default for AVLSpec {
    fn default() -> AVLSpec {
        AVLSpec::Secure
    }
}

/// AVL factory.
#[derive(Default, Clone)]
pub struct AVLFactory {
    spec: AVLSpec,
}

/// All different kinds of AVLs.
/// This is used to prevent a heap allocation for every created AVL.
pub enum AVLKinds<'db> {
    /// A generic avl db.
    Generic(AVLDB<'db>),
    /// A secure avl db.
    Secure(SecAVLDB<'db>),
    /// A fat avl db.
    Fat(FatDB<'db>),
}

// wrapper macro for making the match easier to deal with.
macro_rules! wrapper {
    ($me: ident, $f_name: ident, $($param: ident),*) => {
        match *$me {
            AVLKinds::Generic(ref t) => t.$f_name($($param),*),
            AVLKinds::Secure(ref t) => t.$f_name($($param),*),
            AVLKinds::Fat(ref t) => t.$f_name($($param),*),
        }
    }
}

impl<'db> AVL for AVLKinds<'db> {
    fn root(&self) -> &H256 {
        wrapper!(self, root,)
    }

    fn is_empty(&self) -> bool {
        wrapper!(self, is_empty,)
    }

    fn contains(&self, key: &[u8]) -> Result<bool> {
        wrapper!(self, contains, key)
    }

    fn get_with<'a, 'key, Q: Query>(&'a self, key: &'key [u8], query: Q) -> Result<Option<Q::Item>>
    where
        'a: 'key,
    {
        wrapper!(self, get_with, key, query)
    }

    fn iter<'a>(&'a self) -> Result<Box<AVLIterator<Item = AVLItem> + 'a>> {
        wrapper!(self, iter,)
    }
}

#[cfg_attr(feature = "dev", allow(wrong_self_convention))]
impl AVLFactory {
    /// Creates new factory.
    pub fn new(spec: AVLSpec) -> Self {
        AVLFactory { spec: spec }
    }

    /// Create new immutable instance of AVL.
    pub fn readonly<'db>(&self, db: &'db HashDB, root: &'db H256) -> Result<AVLKinds<'db>> {
        match self.spec {
            AVLSpec::Generic => Ok(AVLKinds::Generic(AVLDB::new(db, root)?)),
            AVLSpec::Secure => Ok(AVLKinds::Secure(SecAVLDB::new(db, root)?)),
            AVLSpec::Fat => Ok(AVLKinds::Fat(FatDB::new(db, root)?)),
        }
    }

    /// Create new mutable instance of AVL.
    pub fn create<'db>(&self, db: &'db mut HashDB, root: &'db mut H256) -> Box<AVLMut + 'db> {
        match self.spec {
            AVLSpec::Generic => Box::new(AVLDBMut::new(db, root)),
            AVLSpec::Secure => Box::new(SecAVLDBMut::new(db, root)),
            AVLSpec::Fat => Box::new(FatDBMut::new(db, root)),
        }
    }

    /// Create new mutable instance of AVL and check for errors.
    pub fn from_existing<'db>(&self, db: &'db mut HashDB, root: &'db mut H256) -> Result<Box<AVLMut + 'db>> {
        match self.spec {
            AVLSpec::Generic => Ok(Box::new(AVLDBMut::from_existing(db, root)?)),
            AVLSpec::Secure => Ok(Box::new(SecAVLDBMut::from_existing(db, root)?)),
            AVLSpec::Fat => Ok(Box::new(FatDBMut::from_existing(db, root)?)),
        }
    }

    /// Returns true iff the avl DB is a fat DB (allows enumeration of keys).
    pub fn is_fat(&self) -> bool {
        self.spec == AVLSpec::Fat
    }
}
