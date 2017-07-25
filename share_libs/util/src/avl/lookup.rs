use hashdb::HashDB;
use H256;
use rlp::*;

use super::{AVLError, Query};
use super::node::*;

/// AVL lookup helper object.
pub struct Lookup<'a, Q: Query> {
    /// database to query from.
    pub db: &'a HashDB,
    /// Query object to record nodes and transform data.
    pub query: Q,
    /// Hash to start at
    pub hash: H256,
}

impl<'a, Q: Query> Lookup<'a, Q> {
    /// Look up the given key. If the value is found, it will be passed to the given
    /// function to decode or copy.
    pub fn look_up(mut self, key: NodeKey) -> super::Result<Option<Q::Item>> {
        let mut hash = self.hash;

        // this loop iterates through non-inline nodes.
        for depth in 0.. {
            let node_data = match self.db.get(&hash) {
                Some(value) => value,
                None => {
                    return Err(Box::new(match depth {
                                            0 => AVLError::InvalidStateRoot(hash),
                                            _ => AVLError::IncompleteDatabase(hash),
                                        }))
                }
            };

            self.query.record(&hash, &node_data, depth);

            // this loop iterates through all inline children (usually max 1)
            // without incrementing the depth.
            let mut node_data = &node_data[..];
            loop {
                match Node::decoded(node_data) {
                    Node::Leaf(k, value) => {
                        return Ok(match k == key {
                                      true => Some(self.query.decode(value)),
                                      false => None,
                                  })
                    }
                    Node::Branch(_, k, children) => {
                        let idx = if key < k { 0 } else { 1 };
                        node_data = children[idx as usize];
                    }
                    _ => return Ok(None),
                }

                // check if new node data is inline or hash.
                let r = Rlp::new(node_data);
                if r.is_data() && r.size() == 32 {
					hash = r.as_val();
					break
				}
            }
        }
        Ok(None)
    }
}
