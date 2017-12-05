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

//! In-memory avl representation.

use super::{AVLError, AVLMut};
use super::lookup::Lookup;
use super::node::Node as RlpNode;
use super::node::NodeKey;

use H256;
use bytes::ToPretty;

use elastic_array::ElasticArray1024;
use hashable::HASH_NULL_RLP;
use hashdb::{DBValue, HashDB};
use rlp::*;
use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::mem;
use std::ops::Index;

// For lookups into the Node storage buffer.
// This is deliberately non-copyable.
#[derive(Debug)]
struct StorageHandle(usize);

// Handles to nodes in the avl.
#[derive(Debug)]
enum NodeHandle {
    /// Loaded into memory.
    InMemory(StorageHandle),
    /// Either a hash or an inline node
    Hash(H256),
}

impl From<StorageHandle> for NodeHandle {
    fn from(handle: StorageHandle) -> Self {
        NodeHandle::InMemory(handle)
    }
}

impl From<H256> for NodeHandle {
    fn from(hash: H256) -> Self {
        NodeHandle::Hash(hash)
    }
}

fn empty_children() -> Box<[Option<NodeHandle>; 2]> {
    Box::new([None, None])
}

/// Node types in the AVL.
#[derive(Debug)]
enum Node {
    /// Empty node.
    Empty,
    /// A leaf node contains a key and a value.
    Leaf(NodeKey, DBValue),
    /// A branch has up to 2 children and a key.
    Branch(u32, NodeKey, Box<[Option<NodeHandle>; 2]>),
}


impl Node {
    // load an inline node into memory or get the hash to do the lookup later.
    fn inline_or_hash(node: &[u8], db: &HashDB, storage: &mut NodeStorage) -> NodeHandle {
        let r = Rlp::new(node);
        if r.is_data() && r.size() == 32 {
            NodeHandle::Hash(r.as_val::<H256>())
        } else {
            let child = Node::from_rlp(node, db, storage);
            NodeHandle::InMemory(storage.alloc(Stored::New(child)))
        }
    }

    // decode a node from rlp without getting its children.
    fn from_rlp(rlp: &[u8], db: &HashDB, storage: &mut NodeStorage) -> Self {
        match RlpNode::decoded(rlp) {
            RlpNode::Empty => Node::Empty,
            RlpNode::Leaf(k, v) => Node::Leaf(k, DBValue::from_slice(&v)),
            RlpNode::Branch(h, k, children_rlp) => {
                let mut children = empty_children();

                for i in 0..2 {
                    let raw = children_rlp[i];
                    let child_rlp = Rlp::new(raw);
                    if !child_rlp.is_empty() {
                        children[i] = Some(Self::inline_or_hash(raw, db, storage));
                    }
                }

                Node::Branch(h, k, children)
            }
        }
    }

    // encode a node to RLP
    fn into_rlp<F>(self, mut child_cb: F) -> ElasticArray1024<u8>
    where
        F: FnMut(NodeHandle, &mut RlpStream),
    {
        match self {
            Node::Empty => {
                let mut stream = RlpStream::new();
                stream.append_empty_data();
                stream.drain()
            }
            Node::Leaf(key, value) => {
                let mut stream = RlpStream::new_list(2);
                stream.append(&&*key);
                stream.append(&&*value);
                stream.drain()
            }
            Node::Branch(height, key, mut children) => {
                let mut stream = RlpStream::new_list(4);
                for child in children.iter_mut().map(Option::take) {
                    if let Some(handle) = child {
                        child_cb(handle, &mut stream);
                    } else {
                        stream.append_empty_data();
                    }
                }
                stream.append(&height);
                stream.append(&&*key);
                stream.drain()
            }
        }
    }
}

// post-inspect action.
enum Action {
    // Replace a node with a new one.
    Replace(Node),
    // Restore the original node. This trusts that the node is actually the original.
    Restore(Node),
    // if it is a new node, just clears the storage.
    Delete,
}

// post-insert action. Same as action without delete
enum InsertAction {
    // Replace a node with a new one.
    Replace(Node),
    // Restore the original node.
    Restore(Node),
}

impl InsertAction {
    fn into_action(self) -> Action {
        match self {
            InsertAction::Replace(n) => Action::Replace(n),
            InsertAction::Restore(n) => Action::Restore(n),
        }
    }

    // // unwrap the node, disregarding replace or restore state.
    // fn unwrap_node(self) -> Node {
    //     match self {
    //         InsertAction::Replace(n) |
    //         InsertAction::Restore(n) => n,
    //     }
    // }
}

// What kind of node is stored here.
enum Stored {
    // A new node.
    New(Node),
    // A cached node, loaded from the DB.
    Cached(Node, H256),
}

/// Compact and cache-friendly storage for AVL nodes.
struct NodeStorage {
    nodes: Vec<Stored>,
    free_indices: VecDeque<usize>,
}

impl NodeStorage {
    /// Create a new storage.
    fn empty() -> Self {
        NodeStorage {
            nodes: Vec::new(),
            free_indices: VecDeque::new(),
        }
    }

    /// Allocate a new node in the storage.
    fn alloc(&mut self, stored: Stored) -> StorageHandle {
        if let Some(idx) = self.free_indices.pop_front() {
            self.nodes[idx] = stored;
            StorageHandle(idx)
        } else {
            self.nodes.push(stored);
            StorageHandle(self.nodes.len() - 1)
        }
    }

    /// Remove a node from the storage, consuming the handle and returning the node.
    fn destroy(&mut self, handle: StorageHandle) -> Stored {
        let idx = handle.0;

        self.free_indices.push_back(idx);
        mem::replace(&mut self.nodes[idx], Stored::New(Node::Empty))
    }
}

impl<'a> Index<&'a StorageHandle> for NodeStorage {
    type Output = Node;

    fn index(&self, handle: &'a StorageHandle) -> &Node {
        match self.nodes[handle.0] {
            Stored::New(ref node) => node,
            Stored::Cached(ref node, _) => node,
        }
    }
}

/// An `AVL` implementation using a generic `HashDB` backing database.
///
/// Use it as a `AVLMut` trait object. You can use `db()` to get the backing database object.
/// Note that changes are not committed to the database until `commit` is called.
/// Querying the root or dropping the avl will commit automatically.
///
/// # Example
/// ```
/// extern crate util;
///
/// use util::avl::*;
/// use util::hashdb::*;
/// use util::memorydb::*;
/// use util::*;
/// use util::hashable::HASH_NULL_RLP;
///
/// fn main() {
///   let mut memdb = MemoryDB::new();
///   let mut root = H256::new();
///   let mut t = AVLDBMut::new(&mut memdb, &mut root);
///   assert!(t.is_empty());
///   assert_eq!(*t.root(), HASH_NULL_RLP);
///   t.insert(b"foo", b"bar").unwrap();
///   assert!(t.contains(b"foo").unwrap());
///   assert_eq!(t.get(b"foo").unwrap().unwrap(), DBValue::from_slice(b"bar"));
///   t.remove(b"foo").unwrap();
///   assert!(!t.contains(b"foo").unwrap());
/// }
/// ```
pub struct AVLDBMut<'a> {
    storage: NodeStorage,
    db: &'a mut HashDB,
    root: &'a mut H256,
    root_handle: NodeHandle,
    death_row: HashSet<H256>,
    /// The number of hash operations this avl has performed.
    /// Note that none are performed until changes are committed.
    pub hash_count: usize,
}

impl<'a> AVLDBMut<'a> {
    /// Create a new avl with backing database `db` and empty `root`.
    pub fn new(db: &'a mut HashDB, root: &'a mut H256) -> Self {
        *root = HASH_NULL_RLP;
        let root_handle = NodeHandle::Hash(HASH_NULL_RLP);
        AVLDBMut {
            storage: NodeStorage::empty(),
            db: db,
            root: root,
            root_handle: root_handle,
            death_row: HashSet::new(),
            hash_count: 0,
        }
    }

    /// Create a new avl with the backing database `db` and `root.
    /// Returns an error if `root` does not exist.
    pub fn from_existing(db: &'a mut HashDB, root: &'a mut H256) -> super::Result<Self> {
        if !db.contains(root) {
            return Err(Box::new(AVLError::InvalidStateRoot(*root)));
        }

        let root_handle = NodeHandle::Hash(*root);
        Ok(AVLDBMut {
            storage: NodeStorage::empty(),
            db: db,
            root: root,
            root_handle: root_handle,
            death_row: HashSet::new(),
            hash_count: 0,
        })
    }
    /// Get the backing database.
    pub fn db(&self) -> &HashDB {
        self.db
    }

    /// Get the backing database mutably.
    pub fn db_mut(&mut self) -> &mut HashDB {
        self.db
    }

    // cache a node by hash
    fn cache(&mut self, hash: H256) -> super::Result<StorageHandle> {
        let node_rlp = self.db
            .get(&hash)
            .ok_or_else(|| Box::new(AVLError::IncompleteDatabase(hash)))?;
        let node = Node::from_rlp(&node_rlp, &*self.db, &mut self.storage);
        Ok(self.storage.alloc(Stored::Cached(node, hash)))
    }

    // inspect a node, choosing either to replace, restore, or delete it.
    // if restored or replaced, returns the new node along with a flag of whether it was changed.
    fn inspect<F>(&mut self, stored: Stored, inspector: F) -> super::Result<Option<(Stored, bool)>>
    where
        F: FnOnce(&mut Self, Node) -> super::Result<Action>,
    {
        Ok(match stored {
            Stored::New(node) => match inspector(self, node)? {
                Action::Restore(node) => Some((Stored::New(node), false)),
                Action::Replace(node) => Some((Stored::New(node), true)),
                Action::Delete => None,
            },
            Stored::Cached(node, hash) => match inspector(self, node)? {
                Action::Restore(node) => Some((Stored::Cached(node, hash), false)),
                Action::Replace(node) => {
                    self.death_row.insert(hash);
                    Some((Stored::New(node), true))
                }
                Action::Delete => {
                    self.death_row.insert(hash);
                    None
                }
            },
        })
    }

    // walk the avl, attempting to find the key's node.
    fn lookup(&self, key: NodeKey, handle: &NodeHandle) -> super::Result<Option<DBValue>> {
        match *handle {
            NodeHandle::Hash(ref hash) => Lookup {
                db: &*self.db,
                query: DBValue::from_slice,
                hash: hash.clone(),
            }.look_up(key),
            NodeHandle::InMemory(ref handle) => match self.storage[handle] {
                Node::Empty => Ok(None),
                Node::Leaf(ref k, ref value) => {
                    if *k == key {
                        Ok(Some(DBValue::from_slice(value)))
                    } else {
                        Ok(None)
                    }
                }
                Node::Branch(_, ref k, ref children) => {
                    let idx = if key < *k { 0 } else { 1 };
                    match children[idx as usize].as_ref() {
                        Some(child) => self.lookup(key, child),
                        None => Ok(None),
                    }
                }
            },
        }
    }


    fn left_rotate_successor(&mut self, mut k1: Node) -> super::Result<Stored> {
        let mut k2 = self.left_child_destroy(&mut k1)?;
        let llh = self.left_child_height(&mut k2)?;
        let lrh = self.right_child_height(&mut k2)?;
        if llh < lrh {
            let k3 = self.right_child_destroy(&mut k2)?;
            k2 = self.right_rotate(k2, k3);
        }
        Ok(Stored::New(self.left_rotate(k1, k2)))
    }

    fn right_rotate_successor(&mut self, mut k1: Node) -> super::Result<Stored> {
        let mut k2 = self.right_child_destroy(&mut k1)?;
        let rlh = self.left_child_height(&mut k2)?;
        let rrh = self.right_child_height(&mut k2)?;
        if rrh < rlh {
            let k3 = self.left_child_destroy(&mut k2)?;
            k2 = self.left_rotate(k2, k3);
        }
        Ok(Stored::New(self.right_rotate(k1, k2)))
    }

    fn left_rotate(&mut self, mut k1: Node, mut k2: Node) -> Node {
        if let Node::Branch(ref mut h2, _, ref mut children2) = k2 {
            if let Node::Branch(ref mut h1, _, ref mut children1) = k1 {
                *h2 = *h1;
                *h1 -= 1;
                children1[0] = children2[1].take();
            } else {
                unreachable!()
            }
            children2[1] = Some(self.storage.alloc(Stored::New(k1)).into());
        } else {
            unreachable!()
        }
        k2
    }

    fn right_rotate(&mut self, mut k1: Node, mut k2: Node) -> Node {
        if let Node::Branch(ref mut h2, _, ref mut children2) = k2 {
            if let Node::Branch(ref mut h1, _, ref mut children1) = k1 {
                *h2 = *h1;
                *h1 -= 1;
                children1[1] = children2[0].take();
            } else {
                unreachable!()
            }
            children2[0] = Some(self.storage.alloc(Stored::New(k1)).into());
        } else {
            unreachable!()
        }
        k2
    }

    fn rotate_if_necessary(&mut self, stored: Stored) -> super::Result<Stored> {
        let mut node = match stored {
            Stored::New(node) => node,
            Stored::Cached(node, h) => return Ok(Stored::Cached(node, h)),
        };
        let lh = self.left_child_height(&mut node)?;
        let rh = self.right_child_height(&mut node)?;
        if lh == 2 + rh {
            self.left_rotate_successor(node)
        } else if rh == 2 + lh {
            self.right_rotate_successor(node)
        } else {
            let h = cmp::max(lh, rh) + 1;
            match node {
                Node::Empty => Ok(Stored::New(Node::Empty)),
                Node::Leaf(k, v) => Ok(Stored::New(Node::Leaf(k, v))),
                Node::Branch(_, k, c) => Ok(Stored::New(Node::Branch(h, k, c))),
            }
        }
    }

    fn height(node: &Node) -> u32 {
        match node {
            &Node::Empty => 0,
            &Node::Leaf(_, _) => 1,
            &Node::Branch(h, _, _) => h,
        }
    }

    fn left_child_destroy(&mut self, node: &mut Node) -> super::Result<Node> {
        if let Node::Branch(_, _, ref mut children) = *node {
            let h = match children[0].take() {
                Some(NodeHandle::InMemory(h)) => h,
                Some(NodeHandle::Hash(h)) => self.cache(h)?,
                _ => return Ok(Node::Empty),
            };
            match self.storage.destroy(h) {
                Stored::New(child) => Ok(child),
                Stored::Cached(child, _) => Ok(child),
            }
        } else {
            Ok(Node::Empty)
        }
    }

    fn right_child_destroy(&mut self, node: &mut Node) -> super::Result<Node> {
        if let Node::Branch(_, _, ref mut children) = *node {
            let h = match children[1].take() {
                Some(NodeHandle::InMemory(h)) => h,
                Some(NodeHandle::Hash(h)) => self.cache(h)?,
                _ => return Ok(Node::Empty),
            };
            match self.storage.destroy(h) {
                Stored::New(child) => Ok(child),
                Stored::Cached(child, _) => Ok(child),
            }
        } else {
            Ok(Node::Empty)
        }
    }

    fn left_child_height(&mut self, node: &mut Node) -> super::Result<u32> {
        if let Node::Branch(_, _, ref mut children) = *node {
            let h = match children[0].take() {
                Some(NodeHandle::InMemory(h)) => h,
                Some(NodeHandle::Hash(h)) => self.cache(h)?,
                _ => return Ok(0),
            };
            let height = AVLDBMut::height(&self.storage[&h]);
            children[0] = Some(NodeHandle::InMemory(h));
            Ok(height)
        } else {
            Ok(0)
        }
    }

    fn right_child_height(&mut self, node: &mut Node) -> super::Result<u32> {
        if let Node::Branch(_, _, ref mut children) = *node {
            let h = match children[1].take() {
                Some(NodeHandle::InMemory(h)) => h,
                Some(NodeHandle::Hash(h)) => self.cache(h)?,
                _ => return Ok(0),
            };
            let height = AVLDBMut::height(&self.storage[&h]);
            children[1] = { Some(NodeHandle::InMemory(h)) };
            Ok(height)
        } else {
            Ok(0)
        }
    }

    pub fn tree_height(&mut self) -> super::Result<(super::Result<u32>, u32, super::Result<u32>)> {
        let h = self.root.clone();
        let node_rlp = self.db
            .get(&h)
            .ok_or_else(|| Box::new(AVLError::IncompleteDatabase(h)))?;
        let mut node = Node::from_rlp(&node_rlp, &*self.db, &mut self.storage);
        Ok((
            self.left_child_height(&mut node),
            AVLDBMut::height(&node),
            self.right_child_height(&mut node),
        ))
    }

    /// insert a key, value pair into the AVL, creating new nodes if necessary.
    fn insert_at(
        &mut self,
        handle: NodeHandle,
        key: NodeKey,
        value: DBValue,
        old_val: &mut Option<DBValue>,
    ) -> super::Result<(StorageHandle, bool)> {
        let h = match handle {
            NodeHandle::InMemory(h) => h,
            NodeHandle::Hash(h) => self.cache(h)?,
        };
        let stored = self.storage.destroy(h);
        let (new_stored, changed) = self.inspect(stored, move |avl, stored| {
            avl.insert_inspector(stored, key, value, old_val)
                .map(|a| a.into_action())
        })?
            .expect("Insertion never deletes.");

        match changed {
            true => {
                let new_stored = self.rotate_if_necessary(new_stored)?;
                Ok((self.storage.alloc(new_stored), true))
            }
            false => Ok((self.storage.alloc(new_stored), false)),
        }
    }

    /// the insertion inspector.
    #[cfg_attr(feature = "dev", allow(cyclomatic_complexity))]
    fn insert_inspector(
        &mut self,
        node: Node,
        key: NodeKey,
        value: DBValue,
        old_val: &mut Option<DBValue>,
    ) -> super::Result<InsertAction> {
        trace!(target: "avl", "augmented (key: {:?}, value: {:?})", key, value.pretty());

        Ok(match node {
            Node::Empty => {
                trace!(target: "avl", "empty: COMPOSE");
                InsertAction::Replace(Node::Leaf(key, value))
            }
            Node::Branch(h, k, mut children) => {
                trace!(target: "avl", "branch: ROUTE,AUGMENT");
                let idx = if key < k { 0 } else { 1 };
                if let Some(child) = children[idx].take() {
                    // original had something there. recurse down into it.
                    let (new_child, changed) = self.insert_at(child, key, value, old_val)?;
                    children[idx] = Some(new_child.into());
                    if !changed {
                        // the new node we composed didn't change. that means our branch is untouched too.
                        return Ok(InsertAction::Restore(Node::Branch(h, k, children)));
                    }
                } else {
                    // original had nothing there. compose a leaf.
                    let leaf = self.storage.alloc(Stored::New(Node::Leaf(key, value)));
                    children[idx] = Some(leaf.into());
                }

                InsertAction::Replace(Node::Branch(h, k, children))
            }
            Node::Leaf(k, v) => {
                if k == key {
                    trace!(target: "avl", "equivalent-leaf: REPLACE");
                    // equivalent leaf.
                    let unchanged = v == value;
                    *old_val = Some(v);

                    match unchanged {
                        // unchanged. restore
                        true => InsertAction::Restore(Node::Leaf(key, value)),
                        false => InsertAction::Replace(Node::Leaf(key, value)),
                    }
                } else {
                    trace!(target: "avl", " (exist={:?}; new={:?}): TRANSMUTE,AUGMENT", k, key);

                    // one of us isn't empty: transmute to branch here
                    let mut children = empty_children();
                    let branch = match k < key {
                        true => {
                            let left_leaf = Node::Leaf(k.clone(), v);
                            let right_leaf = Node::Leaf(key.clone(), value);
                            children[0] = Some(self.storage.alloc(Stored::New(left_leaf)).into());
                            children[1] = Some(self.storage.alloc(Stored::New(right_leaf)).into());
                            Node::Branch(1, key, children)
                        }
                        false => {
                            let left_leaf = Node::Leaf(key.clone(), value);
                            let right_leaf = Node::Leaf(k.clone(), v);
                            children[0] = Some(self.storage.alloc(Stored::New(left_leaf)).into());
                            children[1] = Some(self.storage.alloc(Stored::New(right_leaf)).into());
                            Node::Branch(1, k, children)
                        }
                    };

                    InsertAction::Replace(branch)
                }
            }
        })
    }

    /// Remove a node from the avl based on key.
    fn remove_at(
        &mut self,
        handle: NodeHandle,
        key: NodeKey,
        old_val: &mut Option<DBValue>,
    ) -> super::Result<Option<(StorageHandle, bool)>> {
        let stored = match handle {
            NodeHandle::InMemory(h) => self.storage.destroy(h),
            NodeHandle::Hash(h) => {
                let handle = self.cache(h)?;
                self.storage.destroy(handle)
            }
        };

        if let Some((new_stored, changed)) = self.inspect(stored, move |avl, node| {
            avl.remove_inspector(node, key, old_val)
        })? {
            match changed {
                true => {
                    let new_stored = self.rotate_if_necessary(new_stored)?;
                    Ok(Some((self.storage.alloc(new_stored), true)))
                }
                false => Ok(Some((self.storage.alloc(new_stored), false))),
            }
        } else {
            Ok(None)
        }
    }

    /// the removal inspector
    fn remove_inspector(&mut self, node: Node, key: NodeKey, old_val: &mut Option<DBValue>) -> super::Result<Action> {
        Ok(match node {
            Node::Empty => Action::Delete,
            Node::Leaf(k, v) => match k == key {
                true => Action::Delete,
                false => Action::Restore(Node::Leaf(k, v)),
            },
            Node::Branch(h, k, mut children) => {
                let idx = if key < k { 0 } else { 1 };
                if let Some(child) = children[idx].take() {
                    trace!(target: "avl", "removing value out of branch child, key={:?}", key);
                    match self.remove_at(child, key, old_val)? {
                        Some((new, changed)) => {
                            children[idx] = Some(new.into());
                            let branch = Node::Branch(h, k, children);
                            match changed {
                                // child was changed, so we were too.
                                true => Action::Replace(branch),
                                // unchanged, so we are too.
                                false => Action::Restore(branch),
                            }
                        }
                        None => {
                            // the child we took was deleted.
                            // the node may need fixing.
                            // trace!(target: "avl", "branch child deleted, key={:?}", key);
                            let mut node = Node::Branch(h, k, children);
                            match idx {
                                1 => Action::Replace(self.left_child_destroy(&mut node)?),
                                0 => Action::Replace(self.right_child_destroy(&mut node)?),
                                _ => unreachable!(),
                            }
                        }
                    }
                } else {
                    // no change needed.
                    Action::Restore(Node::Branch(h, k, children))
                }
            }
        })
    }

    /// Commit the in-memory changes to disk, freeing their storage and
    /// updating the state root.
    pub fn commit(&mut self) {
        trace!(target: "avl", "Committing avl changes to db.");

        // always kill all the nodes on death row.
        trace!(target: "avl", "{:?} nodes to remove from db", self.death_row.len());
        for hash in self.death_row.drain() {
            self.db.remove(&hash);
        }

        let handle = match self.root_handle() {
            NodeHandle::Hash(_) => return, // no changes necessary.
            NodeHandle::InMemory(h) => h,
        };

        match self.storage.destroy(handle) {
            Stored::New(node) => {
                let root_rlp = node.into_rlp(|child, stream| self.commit_node(child, stream));
                *self.root = self.db.insert(&root_rlp[..]);
                self.hash_count += 1;

                trace!(target: "avl", "root node rlp: {:?}", (&root_rlp[..]).pretty());
                self.root_handle = NodeHandle::Hash(*self.root);
            }
            Stored::Cached(node, hash) => {
                // probably won't happen, but update the root and move on.
                *self.root = hash;
                self.root_handle = NodeHandle::InMemory(self.storage.alloc(Stored::Cached(node, hash)));
            }
        }
    }

    /// commit a node, hashing it, committing it to the db,
    /// and writing it to the rlp stream as necessary.
    fn commit_node(&mut self, handle: NodeHandle, stream: &mut RlpStream) {
        match handle {
            NodeHandle::Hash(h) => stream.append(&h),
            NodeHandle::InMemory(h) => match self.storage.destroy(h) {
                Stored::Cached(_, h) => stream.append(&h),
                Stored::New(node) => {
                    let node_rlp = node.into_rlp(|child, stream| self.commit_node(child, stream));
                    if node_rlp.len() >= 32 {
                        let hash = self.db.insert(&node_rlp[..]);
                        self.hash_count += 1;
                        stream.append(&hash)
                    } else {
                        stream.append_raw(&node_rlp, 1)
                    }
                }
            },
        };
    }

    // a hack to get the root node's handle
    fn root_handle(&self) -> NodeHandle {
        match self.root_handle {
            NodeHandle::Hash(h) => NodeHandle::Hash(h),
            NodeHandle::InMemory(StorageHandle(x)) => NodeHandle::InMemory(StorageHandle(x)),
        }
    }
}

impl<'a> AVLMut for AVLDBMut<'a> {
    fn root(&mut self) -> &H256 {
        self.commit();
        self.root
    }

    fn is_empty(&self) -> bool {
        match self.root_handle {
            NodeHandle::Hash(h) => h == HASH_NULL_RLP,
            NodeHandle::InMemory(ref h) => match self.storage[h] {
                Node::Empty => true,
                _ => false,
            },
        }
    }

    fn get<'x, 'key>(&'x self, key: &'key [u8]) -> super::Result<Option<DBValue>>
    where
        'x: 'key,
    {
        self.lookup(key.to_vec(), &self.root_handle)
    }


    fn insert(&mut self, key: &[u8], value: &[u8]) -> super::Result<Option<DBValue>> {
        if value.is_empty() {
            return self.remove(key);
        }

        let mut old_val = None;

        trace!(target: "avl", "insert: key={:?}, value={:?}", key.pretty(), value.pretty());

        let root_handle = self.root_handle();
        let (new_handle, changed) = self.insert_at(
            root_handle,
            key.to_vec(),
            DBValue::from_slice(value),
            &mut old_val,
        )?;

        trace!(target: "avl", "insert: altered avl={}", changed);
        self.root_handle = NodeHandle::InMemory(new_handle);

        Ok(old_val)
    }

    fn remove(&mut self, key: &[u8]) -> super::Result<Option<DBValue>> {
        trace!(target: "avl", "remove: key={:?}", key.pretty());

        let root_handle = self.root_handle();
        let key = key.to_vec();
        let mut old_val = None;

        match self.remove_at(root_handle, key, &mut old_val)? {
            Some((handle, changed)) => {
                trace!(target: "avl", "remove: altered avl={}", changed);
                self.root_handle = NodeHandle::InMemory(handle);
            }
            None => {
                trace!(target: "avl", "remove: obliterated avl");
                self.root_handle = NodeHandle::Hash(HASH_NULL_RLP);
                *self.root = HASH_NULL_RLP;
            }
        }

        Ok(old_val)
    }
}

impl<'a> Drop for AVLDBMut<'a> {
    fn drop(&mut self) {
        self.commit();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::AVLMut;
    use memorydb::*;
    // use super::super::standardmap::*;

    // fn populate_avl<'db>(
    //     db: &'db mut HashDB,
    //     root: &'db mut H256,
    //     v: &[(Vec<u8>, Vec<u8>)]
    // ) -> AVLDBMut<'db> {
    //     let mut t = AVLDBMut::new(db, root);
    //     for i in 0..v.len() {
    //         let key: &[u8] = &v[i].0;
    //         let val: &[u8] = &v[i].1;
    //         t.insert(key, val).unwrap();
    //     }
    //     t
    // }

    // fn unpopulate_avl<'db>(t: &mut AVLDBMut<'db>, v: &[(Vec<u8>, Vec<u8>)]) {
    //     for i in v {
    //         let key: &[u8] = &i.0;
    //         t.remove(key).unwrap();
    //     }
    // }

    // #[test]
    // fn playpen() {
    //     let mut seed = H256::default();
    //     for test_i in 0..10 {
    //         if test_i % 50 == 0 {
    //             debug!("{:?} of 10000 stress tests done", test_i);
    //         }
    //         let x = StandardMap {
    //                 alphabet: Alphabet::Custom(b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_".to_vec()),
    //                 min_key: 5,
    //                 journal_key: 0,
    //                 value_mode: ValueMode::Index,
    //                 count: 100,
    //             }
    //             .make_with(&mut seed);

    //         let real = avl_root(x.clone());
    //         let mut memdb = MemoryDB::new();
    //         let mut root = H256::default();
    //         let mut memavl = populate_avl(&mut memdb, &mut root, &x);

    //         memavl.commit();
    //         if *memavl.root() != real {
    //             println!("TRIE MISMATCH");
    //             println!("");
    //             println!("{:?} vs {:?}", memavl.root(), real);
    //             for i in &x {
    //                 println!("{:?} -> {:?}", i.0.pretty(), i.1.pretty());
    //             }
    //         }
    //         assert_eq!(*memavl.root(), real);
    //         unpopulate_avl(&mut memavl, &x);
    //         memavl.commit();
    //         if *memavl.root() != HASH_NULL_RLP {
    //             println!("- TRIE MISMATCH");
    //             println!("");
    //             println!("{:?} vs {:?}", memavl.root(), real);
    //             for i in &x {
    //                 println!("{:?} -> {:?}", i.0.pretty(), i.1.pretty());
    //             }
    //         }
    //         assert_eq!(*memavl.root(), HASH_NULL_RLP);
    //     }
    // }

    #[test]
    fn init() {
        let mut memdb = MemoryDB::new();
        let mut root = H256::default();
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        assert_eq!(*t.root(), HASH_NULL_RLP);
    }

    // #[test]
    // fn insert_on_empty() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], vec![0x01u8, 0x23])]));
    // }

    #[test]
    fn remove_to_empty() {
        let big_value = b"00000000000000000000000000000000";

        let mut memdb = MemoryDB::new();
        let mut root = H256::default();
        let mut t1 = AVLDBMut::new(&mut memdb, &mut root);
        t1.insert(&[0x01, 0x23], &big_value.to_vec()).unwrap();
        t1.insert(&[0x01, 0x34], &big_value.to_vec()).unwrap();
        let mut memdb2 = MemoryDB::new();
        let mut root2 = H256::default();
        let mut t2 = AVLDBMut::new(&mut memdb2, &mut root2);
        t2.insert(&[0x01], &big_value.to_vec()).unwrap();
        t2.insert(&[0x01, 0x23], &big_value.to_vec()).unwrap();
        t2.insert(&[0x01, 0x34], &big_value.to_vec()).unwrap();
        t2.remove(&[0x01]).unwrap();
    }

    // #[test]
    // fn insert_replace_root() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    //     t.insert(&[0x01u8, 0x23], &[0x23u8, 0x45]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], vec![0x23u8, 0x45])]));
    // }

    // #[test]
    // fn insert_make_branch_root() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    //     t.insert(&[0x11u8, 0x23], &[0x11u8, 0x23]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], vec![0x01u8, 0x23]),
    //                               (vec![0x11u8, 0x23], vec![0x11u8, 0x23])]));
    // }

    // #[test]
    // fn insert_into_branch_root() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    //     t.insert(&[0xf1u8, 0x23], &[0xf1u8, 0x23]).unwrap();
    //     t.insert(&[0x81u8, 0x23], &[0x81u8, 0x23]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], vec![0x01u8, 0x23]),
    //                               (vec![0x81u8, 0x23], vec![0x81u8, 0x23]),
    //                               (vec![0xf1u8, 0x23], vec![0xf1u8, 0x23])]));
    // }

    // #[test]
    // fn insert_value_into_branch_root() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    //     t.insert(&[], &[0x0]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![], vec![0x0]),
    //                               (vec![0x01u8, 0x23], vec![0x01u8, 0x23])]));
    // }

    // #[test]
    // fn insert_split_leaf() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
    //     t.insert(&[0x01u8, 0x34], &[0x01u8, 0x34]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], vec![0x01u8, 0x23]),
    //                               (vec![0x01u8, 0x34], vec![0x01u8, 0x34])]));
    // }

    // #[test]
    // fn insert_split_extenstion() {
    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01, 0x23, 0x45], &[0x01]).unwrap();
    //     t.insert(&[0x01, 0xf3, 0x45], &[0x02]).unwrap();
    //     t.insert(&[0x01, 0xf3, 0xf5], &[0x03]).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01, 0x23, 0x45], vec![0x01]),
    //                               (vec![0x01, 0xf3, 0x45], vec![0x02]),
    //                               (vec![0x01, 0xf3, 0xf5], vec![0x03])]));
    // }

    // #[test]
    // fn insert_big_value() {
    //     let big_value0 = b"00000000000000000000000000000000";
    //     let big_value1 = b"11111111111111111111111111111111";

    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], big_value0).unwrap();
    //     t.insert(&[0x11u8, 0x23], big_value1).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], big_value0.to_vec()),
    //                               (vec![0x11u8, 0x23], big_value1.to_vec())]));
    // }

    // #[test]
    // fn insert_duplicate_value() {
    //     let big_value = b"00000000000000000000000000000000";

    //     let mut memdb = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut memdb, &mut root);
    //     t.insert(&[0x01u8, 0x23], big_value).unwrap();
    //     t.insert(&[0x11u8, 0x23], big_value).unwrap();
    //     assert_eq!(*t.root(),
    //                avl_root(vec![(vec![0x01u8, 0x23], big_value.to_vec()),
    //                               (vec![0x11u8, 0x23], big_value.to_vec())]));
    // }

    #[test]
    fn test_at_empty() {
        let mut memdb = MemoryDB::new();
        let mut root = H256::default();
        let t = AVLDBMut::new(&mut memdb, &mut root);
        assert_eq!(t.get(&[0x5]), Ok(None));
    }

    #[test]
    fn test_at_one() {
        let mut memdb = MemoryDB::new();
        let mut root = H256::default();
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
        assert_eq!(
            t.get(&[0x1, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0x1u8, 0x23])
        );
        t.commit();
        assert_eq!(
            t.get(&[0x1, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0x1u8, 0x23])
        );
    }

    #[test]
    fn test_at_three() {
        let mut memdb = MemoryDB::new();
        let mut root = H256::default();
        let mut t = AVLDBMut::new(&mut memdb, &mut root);
        t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
        t.insert(&[0xf1u8, 0x23], &[0xf1u8, 0x23]).unwrap();
        t.insert(&[0x81u8, 0x23], &[0x81u8, 0x23]).unwrap();
        assert_eq!(
            t.get(&[0x01, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0x01u8, 0x23])
        );
        assert_eq!(
            t.get(&[0xf1, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0xf1u8, 0x23])
        );
        assert_eq!(
            t.get(&[0x81, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0x81u8, 0x23])
        );
        assert_eq!(t.get(&[0x82, 0x23]), Ok(None));
        t.commit();
        assert_eq!(
            t.get(&[0x01, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0x01u8, 0x23])
        );
        assert_eq!(
            t.get(&[0xf1, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0xf1u8, 0x23])
        );
        assert_eq!(
            t.get(&[0x81, 0x23]).unwrap().unwrap(),
            DBValue::from_slice(&[0x81u8, 0x23])
        );
        assert_eq!(t.get(&[0x82, 0x23]), Ok(None));
    }

    // #[test]
    // fn stress() {
    //     let mut seed = H256::default();
    //     for _ in 0..50 {
    //         let x = StandardMap {
    //                 alphabet: Alphabet::Custom(b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_".to_vec()),
    //                 min_key: 5,
    //                 journal_key: 0,
    //                 value_mode: ValueMode::Index,
    //                 count: 4,
    //             }
    //             .make_with(&mut seed);

    //         let real = avl_root(x.clone());
    //         let mut memdb = MemoryDB::new();
    //         let mut root = H256::new();
    //         let mut memavl = populate_avl(&mut memdb, &mut root, &x);
    //         let mut y = x.clone();
    //         y.sort_by(|ref a, ref b| a.0.cmp(&b.0));
    //         let mut memdb2 = MemoryDB::new();
    //         let mut root2 = H256::new();
    //         let mut memavl_sorted = populate_avl(&mut memdb2, &mut root2, &y);
    //         if *memavl.root() != real || *memavl_sorted.root() != real {
    //             println!("TRIE MISMATCH");
    //             println!("");
    //             println!("ORIGINAL... {:?}", memavl.root());
    //             for i in &x {
    //                 println!("{:?} -> {:?}", i.0.pretty(), i.1.pretty());
    //             }
    //             println!("SORTED... {:?}", memavl_sorted.root());
    //             for i in &y {
    //                 println!("{:?} -> {:?}", i.0.pretty(), i.1.pretty());
    //             }
    //         }
    //         assert_eq!(*memavl.root(), real);
    //         assert_eq!(*memavl_sorted.root(), real);
    //     }
    // }

    #[test]
    fn test_avl_existing() {
        let mut root = H256::default();
        let mut db = MemoryDB::new();
        {
            let mut t = AVLDBMut::new(&mut db, &mut root);
            t.insert(&[0x01u8, 0x23], &[0x01u8, 0x23]).unwrap();
        }

        {
            let _ = AVLDBMut::from_existing(&mut db, &mut root);
        }
    }

    // #[test]
    // fn insert_empty() {
    //     let mut seed = H256::default();
    //     let x = StandardMap {
    //             alphabet: Alphabet::Custom(b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_".to_vec()),
    //             min_key: 5,
    //             journal_key: 0,
    //             value_mode: ValueMode::Index,
    //             count: 4,
    //         }
    //         .make_with(&mut seed);

    //     let mut db = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut db, &mut root);
    //     for &(ref key, ref value) in &x {
    //         t.insert(key, value).unwrap();
    //     }

    //     assert_eq!(*t.root(), avl_root(x.clone()));

    //     for &(ref key, _) in &x {
    //         t.insert(key, &[]).unwrap();
    //     }

    //     assert!(t.is_empty());
    //     assert_eq!(*t.root(), HASH_NULL_RLP);
    // }

    // #[test]
    // fn return_old_values() {
    //     let mut seed = H256::default();
    //     let x = StandardMap {
    //             alphabet: Alphabet::Custom(b"@QWERTYUIOPASDFGHJKLZXCVBNM[/]^_".to_vec()),
    //             min_key: 5,
    //             journal_key: 0,
    //             value_mode: ValueMode::Index,
    //             count: 4,
    //         }
    //         .make_with(&mut seed);

    //     let mut db = MemoryDB::new();
    //     let mut root = H256::default();
    //     let mut t = AVLDBMut::new(&mut db, &mut root);
    //     for &(ref key, ref value) in &x {
    //         assert!(t.insert(key, value).unwrap().is_none());
    //         assert_eq!(t.insert(key, value).unwrap(),
    //                    Some(DBValue::from_slice(value)));
    //     }

    //     for (key, value) in x {
    //         assert_eq!(t.remove(&key).unwrap(), Some(DBValue::from_slice(&value)));
    //         assert!(t.remove(&key).unwrap().is_none());
    //     }
    // }
}
