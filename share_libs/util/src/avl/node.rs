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

use bytes::*;
use hashdb::DBValue;
use rlp::*;

/// Partial node key type.
pub type HashKey = Vec<u8>;
pub type NodeKey = Vec<u8>;

/// Type of node in the avl and essential information thereof.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Node<'a> {
    /// Null avl node; could be an empty root or an empty branch entry.
    Empty,
    /// Leaf node; has key slice and value. Value may not be empty.
    Leaf(NodeKey, &'a [u8]),
    /// Branch node; has a key, 2 child nodes (each possibly null) and a height.
    Branch(u32, NodeKey, [&'a [u8]; 2]),
}

impl<'a> Node<'a> {
    /// Decode the `node_rlp` and return the Node.
    pub fn decoded(node_rlp: &'a [u8]) -> Self {
        let r = Rlp::new(node_rlp);
        match r.prototype() {
            // either leaf or extension - decode first item with NibbleSlice::???
            // and use is_leaf return to figure out which.
            // if leaf, second item is a value (is_data())
            // if extension, second item is a node (either SHA3 to be looked up and
            // fed back into this function or inline RLP which can be fed back into this function).
            Prototype::List(2) => Node::Leaf(r.at(0).data().to_vec(), r.at(1).data()),
            // branch - first 16 are nodes, 17th is a value (or empty).
            Prototype::List(4) => {
                let mut nodes = [&[] as &[u8]; 2];
                for i in 0..2 {
                    nodes[i] = r.at(i).as_raw();
                }
                let height: u32 = decode(r.at(2).data());
                Node::Branch(height, r.at(3).data().to_vec(), nodes)
            }
            // an empty branch index.
            Prototype::Data(0) => Node::Empty,
            // something went wrong.
            _ => panic!("Rlp is not valid."),
        }
    }

    /// Encode the node into RLP.
    ///
    /// Will always return the direct node RLP even if it's 32 or more bytes. To get the
    /// RLP which would be valid for using in another node, use `encoded_and_added()`.
    pub fn encoded(&self) -> Bytes {
        match *self {
            Node::Leaf(ref key, ref value) => {
                let mut stream = RlpStream::new_list(2);
                stream.append(key);
                stream.append(value);
                stream.out()
            }
            Node::Branch(ref height, ref key, ref nodes) => {
                let mut stream = RlpStream::new_list(4);
                for i in 0..2 {
                    stream.append_raw(nodes[i], 1);
                }
                stream.append(height);
                stream.append(key);
                stream.out()
            }
            Node::Empty => {
                let mut stream = RlpStream::new();
                stream.append_empty_data();
                stream.out()
            }
        }
    }
}

/// An owning node type. Useful for avl iterators.
#[derive(Debug, PartialEq, Eq)]
pub enum OwnedNode {
    /// Empty avl node.
    Empty,
    /// Leaf node: partial key and value.
    Leaf(NodeKey, DBValue),
    /// Branch node: a key, 2 children and a height.
    Branch(u32, NodeKey, [HashKey; 2]),
}

impl Clone for OwnedNode {
    fn clone(&self) -> Self {
        match *self {
            OwnedNode::Empty => OwnedNode::Empty,
            OwnedNode::Leaf(ref k, ref v) => OwnedNode::Leaf(k.clone(), v.clone()),
            OwnedNode::Branch(h, ref k, ref c) => {
                let mut children = [HashKey::new(), HashKey::new()];

                for (owned, borrowed) in children.iter_mut().zip(c.iter()) {
                    *owned = borrowed.clone()
                }

                OwnedNode::Branch(h, k.clone(), children)
            }
        }
    }
}

impl<'a> From<Node<'a>> for OwnedNode {
    fn from(node: Node<'a>) -> Self {
        match node {
            Node::Empty => OwnedNode::Empty,
            Node::Leaf(k, v) => OwnedNode::Leaf(k.into(), DBValue::from_slice(v)),
            Node::Branch(h, k, c) => {
                let mut children = [HashKey::new(), HashKey::new()];

                for (owned, borrowed) in children.iter_mut().zip(c.iter()) {
                    *owned = borrowed.to_vec()
                }

                OwnedNode::Branch(h, k.into(), children)
            }
        }
    }
}
