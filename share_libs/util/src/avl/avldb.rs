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

use super::{AVLError, AVLItem, AVLIterator, Query, AVL};
use super::lookup::Lookup;
use super::node::{Node, NodeKey, OwnedNode};
use H256;
use bytes::*;
use hashdb::*;
use rlp::*;
use std::fmt;

/// A `AVL` implementation using a generic `HashDB` backing database.
///
/// Use it as a `AVL` trait object. You can use `db()` to get the backing database object.
/// Use `get` and `contains` to query values associated with keys in the AVL.
///
/// # Example
/// ```
/// extern crate util;
///
/// use util::avl::*;
/// use util::hashdb::*;
/// use util::memorydb::*;
/// use util::*;
///
/// fn main() {
///   let mut memdb = MemoryDB::new();
///   let mut root = H256::new();
///   AVLDBMut::new(&mut memdb, &mut root).insert(b"foo", b"bar").unwrap();
///   let t = AVLDB::new(&memdb, &root).unwrap();
///   assert!(t.contains(b"foo").unwrap());
///   assert_eq!(t.get(b"foo").unwrap().unwrap(), DBValue::from_slice(b"bar"));
/// }
/// ```

pub struct AVLDB<'db> {
    db: &'db HashDB,
    root: &'db H256,
    /// The number of hashes performed so far in operations on this avl.
    pub hash_count: usize,
}

#[cfg_attr(feature = "dev", allow(wrong_self_convention))]
impl<'db> AVLDB<'db> {
    /// Create a new AVL with the backing database `db` and `root`
    /// Returns an error if `root` does not exist
    pub fn new(db: &'db HashDB, root: &'db H256) -> super::Result<Self> {
        if !db.contains(root) {
            Err(Box::new(AVLError::InvalidStateRoot(*root)))
        } else {
            Ok(AVLDB {
                db: db,
                root: root,
                hash_count: 0,
            })
        }
    }

    /// Get the backing database.
    pub fn db(&'db self) -> &'db HashDB {
        self.db
    }

    /// Get the data of the root node.
    fn root_data(&self) -> super::Result<DBValue> {
        self.db
            .get(self.root)
            .ok_or_else(|| Box::new(AVLError::InvalidStateRoot(*self.root)))
    }

    /// Indentation helper for `format_all`.
    fn fmt_indent(&self, f: &mut fmt::Formatter, size: usize) -> fmt::Result {
        for _ in 0..size {
            write!(f, "  ")?;
        }
        Ok(())
    }

    /// Recursion helper for implementation of formatting trait.
    fn fmt_all(&self, node: Node, f: &mut fmt::Formatter, deepness: usize) -> fmt::Result {
        match node {
            Node::Leaf(key, value) => {
                writeln!(f, "'{:?}: {:?}.", key, value.pretty())?;
            }
            Node::Branch(_, _, ref nodes) => {
                writeln!(f, "")?;
                for i in 0..2 {
                    let node = self.get_raw_or_lookup(&*nodes[i]);
                    match node.as_ref().map(|n| Node::decoded(&*n)) {
                        Ok(Node::Empty) => {}
                        Ok(n) => {
                            self.fmt_indent(f, deepness + 1)?;
                            write!(f, "'{:x} ", i)?;
                            self.fmt_all(n, f, deepness + 1)?;
                        }
                        Err(e) => {
                            write!(f, "ERROR: {}", e)?;
                        }
                    }
                }
            }
            // empty
            Node::Empty => {
                writeln!(f, "<empty>")?;
            }
        };
        Ok(())
    }

    /// Given some node-describing data `node`, return the actual node RLP.
    /// This could be a simple identity operation in the case that the node is sufficiently small,
    /// but may require a database lookup.
    fn get_raw_or_lookup(&'db self, node: &'db [u8]) -> super::Result<DBValue> {
        // check if its hash + len
        let r = Rlp::new(node);
        match r.is_data() && r.size() == 32 {
            true => {
                let key = r.as_val::<H256>();
                self.db
                    .get(&key)
                    .ok_or_else(|| Box::new(AVLError::IncompleteDatabase(key)))
            }
            false => Ok(DBValue::from_slice(node)),
        }
    }
}

impl<'db> AVL for AVLDB<'db> {
    fn iter<'a>(&'a self) -> super::Result<Box<AVLIterator<Item = AVLItem> + 'a>> {
        AVLDBIterator::new(self).map(|iter| Box::new(iter) as Box<_>)
    }

    fn root(&self) -> &H256 {
        self.root
    }

    fn get_with<'a, 'key, Q: Query>(&'a self, key: &'key [u8], query: Q) -> super::Result<Option<Q::Item>>
    where
        'a: 'key,
    {
        Lookup {
            db: self.db,
            query: query,
            hash: self.root.clone(),
        }.look_up(key.to_vec())
    }
}

impl<'db> fmt::Debug for AVLDB<'db> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "c={:?} [", self.hash_count)?;
        let root_encoded = self.db.get(self.root).expect("AVL root not found!");
        self.fmt_all(Node::decoded(&root_encoded), f, 0)?;
        writeln!(f, "]")
    }
}

#[derive(Clone, Eq, PartialEq)]
enum Status {
    Entering,
    At,
    AtChild(usize),
    Exiting,
}

#[derive(Clone, Eq, PartialEq)]
struct Crumb {
    node: OwnedNode,
    status: Status,
}

impl Crumb {
    /// Move on to next status in the node's sequence.
    fn increment(&mut self) {
        self.status = match (&self.status, &self.node) {
            (_, &OwnedNode::Empty) => Status::Exiting,
            (&Status::Entering, _) => Status::At,
            (&Status::At, &OwnedNode::Branch(_, _, _)) => Status::AtChild(0),
            (&Status::AtChild(0), &OwnedNode::Branch(_, _, _)) => Status::AtChild(1),
            _ => Status::Exiting,
        }
    }
}

/// Iterator for going through all values in the avl.
#[derive(Clone)]
pub struct AVLDBIterator<'a> {
    db: &'a AVLDB<'a>,
    trail: Vec<Crumb>,
}

impl<'a> AVLDBIterator<'a> {
    /// Create a new iterator.
    pub fn new(db: &'a AVLDB) -> super::Result<AVLDBIterator<'a>> {
        let mut r = AVLDBIterator {
            db: db,
            trail: vec![],
        };

        db.root_data().and_then(|root| r.descend(&root))?;
        Ok(r)
    }

    fn seek_descend(&mut self, node_data: DBValue, key: &NodeKey) -> super::Result<()> {
        let node = Node::decoded(&node_data);
        match node {
            Node::Leaf(ref k, _) => {
                if k <= key {
                    println!("{:?}, {:?}", key, k);
                    self.trail.push(Crumb {
                        status: Status::At,
                        node: node.clone().into(),
                    });
                } else {
                    self.trail.push(Crumb {
                        status: Status::Entering,
                        node: node.clone().into(),
                    });
                }

                Ok(())
            }
            Node::Branch(_, ref k, ref nodes) => {
                let idx = if key < k { 0 } else { 1 };
                self.trail.push(Crumb {
                    status: Status::AtChild(idx as usize),
                    node: node.clone().into(),
                });
                let child = self.db.get_raw_or_lookup(&*nodes[idx as usize])?;
                self.seek_descend(child, &key)
            }
            _ => Ok(()),
        }
    }

    /// Descend into a payload.
    fn descend(&mut self, d: &[u8]) -> super::Result<()> {
        self.trail.push(Crumb {
            status: Status::Entering,
            node: Node::decoded(&self.db.get_raw_or_lookup(d)?).into(),
        });
        Ok(())
    }
}

impl<'a> AVLIterator for AVLDBIterator<'a> {
    /// Position the iterator on the first element with key >= `key`
    fn seek(&mut self, key: &[u8]) -> super::Result<()> {
        self.trail.clear();
        let root = self.db.root_data()?;
        self.seek_descend(root, &key.to_vec())
    }
}

impl<'a> Iterator for AVLDBIterator<'a> {
    type Item = AVLItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let b = match self.trail.last_mut() {
                Some(b) => {
                    b.increment();
                    b.clone()
                }
                None => return None,
            };
            match (b.status, b.node) {
                (Status::Exiting, _) => {
                    self.trail.pop();
                    // continue
                }
                (Status::At, OwnedNode::Leaf(k, v)) => {
                    return Some(Ok((k, v)));
                }
                (Status::At, OwnedNode::Branch(_, _, _)) => {}
                (Status::AtChild(i), OwnedNode::Branch(_, _, ref children)) if children[i].len() > 0 => {
                    if let Err(e) = self.descend(&*children[i]) {
                        return Some(Err(e));
                    }
                    // continue
                }
                (Status::AtChild(_), OwnedNode::Branch(_, _, _)) => {
                    // continue
                }
                _ => panic!(), // Should never see Entering or AtChild without a Branch here.
            }
        }
    }
}

#[test]
fn iterator() {
    use memorydb::*;
    use super::AVLMut;
    use super::avldbmut::*;

    let d = vec![
        DBValue::from_slice(b"A"),
        DBValue::from_slice(b"AA"),
        DBValue::from_slice(b"AB"),
        DBValue::from_slice(b"B"),
    ];

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        for x in &d {
            t.insert(x, x).unwrap();
        }
    }

    let t = AVLDB::new(&memdb, &root).unwrap();
    assert_eq!(
        d.iter().map(|i| i.clone().to_vec()).collect::<Vec<_>>(),
        t.iter().unwrap().map(|x| x.unwrap().0).collect::<Vec<_>>()
    );
    assert_eq!(
        d,
        t.iter().unwrap().map(|x| x.unwrap().1).collect::<Vec<_>>()
    );
}

#[test]
fn iterator_seek() {
    use memorydb::*;
    use super::AVLMut;
    use super::avldbmut::*;

    let d = vec![
        DBValue::from_slice(b"A"),
        DBValue::from_slice(b"AA"),
        DBValue::from_slice(b"AB"),
        DBValue::from_slice(b"B"),
    ];

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        for x in &d {
            t.insert(x, x).unwrap();
        }
    }

    let t = AVLDB::new(&memdb, &root).unwrap();
    let mut iter = t.iter().unwrap();
    assert_eq!(
        iter.next(),
        Some(Ok((b"A".to_vec(), DBValue::from_slice(b"A"))))
    );
    iter.seek(b"!").unwrap();
    assert_eq!(d, iter.map(|x| x.unwrap().1).collect::<Vec<_>>());
    let mut iter = t.iter().unwrap();
    iter.seek(b"A").unwrap();
    assert_eq!(&d[1..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
    let mut iter = t.iter().unwrap();
    iter.seek(b"AA").unwrap();
    assert_eq!(&d[2..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
    let mut iter = t.iter().unwrap();
    iter.seek(b"A!").unwrap();
    assert_eq!(&d[1..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
    let mut iter = t.iter().unwrap();
    iter.seek(b"AB").unwrap();
    assert_eq!(&d[3..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
    let mut iter = t.iter().unwrap();
    iter.seek(b"AB!").unwrap();
    assert_eq!(&d[3..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
    let mut iter = t.iter().unwrap();
    iter.seek(b"B").unwrap();
    assert_eq!(&d[4..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
    let mut iter = t.iter().unwrap();
    iter.seek(b"C").unwrap();
    assert_eq!(&d[4..], &iter.map(|x| x.unwrap().1).collect::<Vec<_>>()[..]);
}

#[test]
fn get_len() {
    use memorydb::*;
    use super::AVLMut;
    use super::avldbmut::*;

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    {
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        t.insert(b"A", b"ABC").unwrap();
        t.insert(b"B", b"ABCBA").unwrap();
    }

    let t = AVLDB::new(&memdb, &root).unwrap();
    assert_eq!(t.get_with(b"A", |x: &[u8]| x.len()), Ok(Some(3)));
    assert_eq!(t.get_with(b"B", |x: &[u8]| x.len()), Ok(Some(5)));
    assert_eq!(t.get_with(b"C", |x: &[u8]| x.len()), Ok(None));
}
