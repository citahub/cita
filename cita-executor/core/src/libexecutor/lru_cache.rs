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

use std::collections::{btree_map::Keys, btree_map::Values, BTreeMap, HashSet};

/// This structure is used to perform lru based on block height
/// supports sequential lru and precise deletion
#[derive(Debug)]
pub struct LRUCache<K, V> {
    cache_by_key: BTreeMap<K, Vec<V>>,
    cache_by_value: BTreeMap<V, K>,
    lru_number: u64,
}

impl<K, V> LRUCache<K, V>
where
    K: Ord + Clone + ::std::hash::Hash,
    V: Ord + Clone,
{
    /// New with the max cache
    pub fn new(lru_number: u64) -> Self {
        LRUCache {
            cache_by_key: BTreeMap::new(),
            cache_by_value: BTreeMap::new(),
            lru_number,
        }
    }

    /// Determine if key exists
    pub fn contains_by_key(&self, key: &K) -> bool {
        self.cache_by_key.contains_key(key)
    }

    /// Determine if value exists
    pub fn contains_by_value(&self, value: &V) -> bool {
        self.cache_by_value.contains_key(value)
    }

    /// Extend key-value pairs
    pub fn extend(&mut self, extend: &[V], key: K) -> &mut Self {
        if !extend.is_empty() {
            extend.iter().for_each(|value| {
                let _ = self.cache_by_value.insert(value.clone(), key.clone());
            });
            self.cache_by_key.insert(key, extend.to_owned());
        }
        self
    }

    /// Precise prune value
    pub fn prune(&mut self, value_list: &[V]) -> &mut Self {
        let keys: HashSet<K> = value_list
            .iter()
            .map(|value| self.cache_by_value.remove(&value).unwrap())
            .collect();

        keys.iter().for_each(|key| {
            self.cache_by_key.entry(key.clone()).and_modify(|values| {
                *values = values
                    .iter()
                    .filter(|ref value| !value_list.contains(&value))
                    .map(|value| value.to_owned())
                    .collect::<Vec<V>>();
            });
            if self
                .cache_by_key
                .get(key)
                .map(|x| x.is_empty())
                .unwrap_or(false)
            {
                self.cache_by_key.remove(key);
            }
        });
        self
    }

    /// Execute lru
    pub fn lru(&mut self) -> Vec<V> {
        if self.lru_number <= self.cache_by_value.len() as u64 {
            let temp = self.cache_by_key.clone();
            let (k, v) = temp.iter().next().unwrap();
            self.cache_by_key.remove(k);

            let v: Vec<V> = v
                .into_iter()
                .filter(|value| match self.cache_by_value.get(value) {
                    Some(ref key) if key == &k => true,
                    None | Some(_) => false,
                })
                .map(|value| value.to_owned())
                .collect();

            v.iter().for_each(|value| {
                let _ = self.cache_by_value.remove(value);
            });

            v
        } else {
            Vec::new()
        }
    }

    /// Gets an iterator over the values of the map, in order by key.
    pub fn values(&self) -> Keys<V, K> {
        self.cache_by_value.keys()
    }

    /// Gets an iterator over the keys of the map, in sorted order.
    pub fn keys(&self) -> Values<V, K> {
        self.cache_by_value.values()
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;
    use cita_types::Address;

    #[test]
    fn test_lru() {
        let mut cache = LRUCache::new(2);
        cache
            .extend(&vec![Address::from([0; 20]), Address::from([1; 20])], 1)
            .extend(&vec![Address::from([2; 20]), Address::from([3; 20])], 2);
        assert!(cache.contains_by_value(&Address::from([0; 20])));
        assert!(cache.contains_by_value(&Address::from([3; 20])));

        cache.prune(&vec![Address::from([0; 20]), Address::from([1; 20])]);
        assert_eq!(cache.contains_by_value(&Address::from([0; 20])), false);
        assert_eq!(cache.contains_by_value(&Address::from([1; 20])), false);
        assert_eq!(cache.contains_by_value(&Address::from([2; 20])), true);

        cache.extend(&vec![Address::from([2; 20]), Address::from([3; 20])], 3);
        assert_eq!(cache.lru(), Vec::new());

        cache.extend(&vec![Address::from([2; 20]), Address::from([3; 20])], 4);
        assert_eq!(cache.lru(), Vec::new());

        cache.extend(&vec![Address::from([4; 20]), Address::from([5; 20])], 5);
        assert_eq!(
            cache.lru(),
            vec![Address::from([2; 20]), Address::from([3; 20])]
        );

        cache.extend(&vec![Address::from([4; 20]), Address::from([5; 20])], 5);
        assert_eq!(
            cache.lru(),
            vec![Address::from([4; 20]), Address::from([5; 20])]
        );
    }
}
