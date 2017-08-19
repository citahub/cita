// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Database utilities and definitions.


use rlp::{Decodable, Encodable, RlpStream, UntrustedRlp, DecoderError, encode, decode};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, Index};
use util::{DBTransaction, KeyValueDB, RwLock, HeapSizeOf};

// database columns
/// Column for State
pub const COL_STATE: Option<u32> = Some(0);
/// Column for Block headers
pub const COL_HEADERS: Option<u32> = Some(1);
/// Column for Block bodies
pub const COL_BODIES: Option<u32> = Some(2);
/// Column for Extras
pub const COL_EXTRA: Option<u32> = Some(3);
/// Column for Traces
pub const COL_TRACE: Option<u32> = Some(4);
/// Column for the empty accounts bloom filter.
pub const COL_ACCOUNT_BLOOM: Option<u32> = Some(5);
/// Column for general information from the local node which can persist.
pub const COL_NODE_INFO: Option<u32> = Some(6);
/// Number of columns in DB
pub const NUM_COLUMNS: Option<u32> = Some(7);

/// Contains all block receipts.
#[derive(Clone)]
pub struct DBList<T> {
    pub data: Vec<T>,
}

impl<T> Decodable for DBList<T>
where
    T: Decodable,
{
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(DBList { data: rlp.as_list()? })
    }
}

impl<T> Encodable for DBList<T>
where
    T: Encodable,
{
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.data);
    }
}

impl<T> HeapSizeOf for DBList<T>
where
    T: HeapSizeOf,
{
    fn heap_size_of_children(&self) -> usize {
        self.data.heap_size_of_children()
    }
}

impl<T> Index<usize> for DBList<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        &self.data[i]
    }
}

impl<T> DBList<T> {
    fn new() -> Self {
        DBList { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn remove_item(&mut self, value: &T)
    where
        T: PartialEq,
    {
        self.data.remove_item(value);
    }
}

//TODO: Use a better way ?
#[derive(Clone, Copy)]
pub enum ConstKey {
    /// Current block hash.
    CurrentHash,
    /// Current block height.
    CurrentHeight,
}

/// Modes for updating caches.
#[derive(Clone, Copy)]
pub enum CacheUpdatePolicy {
    /// Overwrite entries.
    Overwrite,
    /// Remove entries.
    Remove,
}

/// Modes for updating caches.
#[derive(Clone, Copy)]
pub enum AppendPolicy {
    /// Overwrite entries list.
    Overwrite,
    /// update entries list.
    Update,
    /// remove entrie in list.
    Remove,
}

/// A cache for arbitrary key-value pairs.
pub trait Cache<K, V> {
    /// Insert an entry into the cache and get the old value.
    fn insert(&mut self, k: K, v: V) -> Option<V>;

    /// Remove an entry from the cache, getting the old value if it existed.
    fn remove(&mut self, k: &K) -> Option<V>;

    /// Query the cache for a key's associated value.
    fn get(&self, k: &K) -> Option<&V>;
}

impl<K, V> Cache<K, V> for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        HashMap::insert(self, k, v)
    }

    fn remove(&mut self, k: &K) -> Option<V> {
        HashMap::remove(self, k)
    }

    fn get(&self, k: &K) -> Option<&V> {
        HashMap::get(self, k)
    }
}

/// Should be used to get database key associated with given value.
pub trait Key<T> {
    /// The db key associated with this value.
    type Target: Deref<Target = [u8]>;

    /// Returns db key.
    fn key(&self) -> Self::Target;
}

/// Should be used to write value into database.
pub trait Writable {
    /// Writes the value into the database.
    fn write<T, R>(&mut self, col: Option<u32>, key: &Key<T, Target = R>, value: &T)
    where
        T: Encodable,
        R: Deref<Target = [u8]>;

    /// append the value into the database.
    fn append<T, R>(&mut self, col: Option<u32>, key: &Key<T, Target = R>, value: &DBList<T>)
    where
        T: Encodable,
        R: Deref<Target = [u8]>;

    /// Deletes key from the databse.
    fn delete<T, R>(&mut self, col: Option<u32>, key: &Key<T, Target = R>)
    where
        T: Encodable,
        R: Deref<Target = [u8]>;

    /// Writes the value into the database and updates the cache.
    fn write_with_cache<K, T, R>(&mut self, col: Option<u32>, cache: &mut Cache<K, T>, key: K, value: T, policy: CacheUpdatePolicy)
    where
        K: Key<T, Target = R> + Hash + Eq,
        T: Encodable,
        R: Deref<Target = [u8]>,
    {
        self.write(col, &key, &value);
        match policy {
            CacheUpdatePolicy::Overwrite => {
                cache.insert(key, value);
            }
            CacheUpdatePolicy::Remove => {
                cache.remove(&key);
            }
        }
    }

    /// Writes the values into the database and updates the cache.
    fn extend_with_cache<K, T, R>(&mut self, col: Option<u32>, cache: &mut Cache<K, T>, values: HashMap<K, T>, policy: CacheUpdatePolicy)
    where
        K: Key<T, Target = R> + Hash + Eq,
        T: Encodable,
        R: Deref<Target = [u8]>,
    {
        match policy {
            CacheUpdatePolicy::Overwrite => {
                for (key, value) in values {
                    self.write(col, &key, &value);
                    cache.insert(key, value);
                }
            }
            CacheUpdatePolicy::Remove => {
                for (key, value) in &values {
                    self.write(col, key, value);
                    cache.remove(key);
                }
            }
        }
    }

    /// Writes and removes the values into the database and updates the cache.
    fn extend_with_option_cache<K, T, R>(&mut self, col: Option<u32>, cache: &mut Cache<K, Option<T>>, values: HashMap<K, Option<T>>, policy: CacheUpdatePolicy)
    where
        K: Key<T, Target = R> + Hash + Eq,
        T: Encodable,
        R: Deref<Target = [u8]>,
    {
        match policy {
            CacheUpdatePolicy::Overwrite => {
                for (key, value) in values {
                    match value {
                        Some(ref v) => self.write(col, &key, v),
                        None => self.delete(col, &key),
                    }
                    cache.insert(key, value);
                }
            }
            CacheUpdatePolicy::Remove => {
                for (key, value) in values {
                    match value {
                        Some(v) => self.write(col, &key, &v),
                        None => self.delete(col, &key),
                    }
                    cache.remove(&key);
                }
            }
        }
    }

    /// Writes the value into the database and updates the cache.
    fn write_with_cache_append<K, T, R, D>(&mut self, col: Option<u32>, db: &D, cache: &mut Cache<K, DBList<T>>, key: K, value: T, policy: AppendPolicy)
    where
        K: Key<T, Target = R> + Hash + Eq + Clone,
        T: Encodable + Decodable + Clone + PartialEq,
        D: Readable + ?Sized,
        R: Deref<Target = [u8]>,
    {
        match policy {
            AppendPolicy::Overwrite => {
                let mut list = DBList::new();
                list.push(value);
                self.append(col, &key, &list);
                cache.insert(key, list);
            }
            AppendPolicy::Update => {
                let mut list = match db.get_list_with_cache(col, cache, &key) {
                    Some(v) => v,
                    None => DBList::new(),
                };
                list.push(value);
                self.append(col, &key, &list);
                cache.insert(key, list);
            }
            AppendPolicy::Remove => {
                let mut list = match db.get_list_with_cache(col, cache, &key) {
                    Some(v) => v,
                    None => return,
                };
                list.remove_item(&value);
                self.append(col, &key, &list);
                cache.insert(key, list);
            }
        }
    }

    /// Writes the values into the database and updates the cache.
    fn extend_with_cache_append<K, T, R, D>(&mut self, col: Option<u32>, db: &D, cache: &mut Cache<K, DBList<T>>, values: HashMap<K, T>, policy: AppendPolicy)
    where
        K: Key<T, Target = R> + Hash + Eq + Clone,
        T: Encodable + Decodable + Clone + PartialEq,
        D: Readable + ?Sized,
        R: Deref<Target = [u8]>,
    {
        match policy {
            AppendPolicy::Overwrite => {
                for (key, value) in values {
                    let mut list = DBList::new();
                    list.push(value);
                    self.append(col, &key, &list);
                    cache.insert(key, list);
                }
            }
            AppendPolicy::Update => {
                for (key, value) in values {
                    let mut list = match db.get_list_with_cache(col, cache, &key) {
                        Some(v) => v,
                        None => DBList::new(),
                    };
                    list.push(value);
                    self.append(col, &key, &list);
                    cache.insert(key, list);
                }
            }
            AppendPolicy::Remove => {
                for (key, value) in values {
                    let mut list = match db.get_list_with_cache(col, cache, &key) {
                        Some(v) => v,
                        None => return,
                    };
                    list.remove_item(&value);
                    self.append(col, &key, &list);
                    cache.insert(key, list);

                }
            }
        }
    }
}

/// Should be used to read values from database.
pub trait Readable {
    /// Returns value for given key.
    fn read<T, R>(&self, col: Option<u32>, key: &Key<T, Target = R>) -> Option<T>
    where
        T: Decodable,
        R: Deref<Target = [u8]>;

    fn read_list<T, R>(&self, col: Option<u32>, key: &Key<T, Target = R>) -> Option<DBList<T>>
    where
        T: Decodable,
        R: Deref<Target = [u8]>;

    /// Returns value for given key either in cache or in database.
    fn read_with_cache<K, T, C>(&self, col: Option<u32>, cache: &RwLock<C>, key: &K) -> Option<T>
    where
        K: Key<T> + Eq + Hash + Clone,
        T: Clone + Decodable,
        C: Cache<K, T>,
    {
        {
            let read = cache.read();
            if let Some(v) = read.get(key) {
                return Some(v.clone());
            }
        }

        self.read(col, key).map(|value: T| {
                                    let mut write = cache.write();
                                    write.insert(key.clone(), value.clone());
                                    value
                                })
    }

    fn read_list_with_cache<K, T, C>(&self, col: Option<u32>, cache: &RwLock<C>, key: &K) -> Option<DBList<T>>
    where
        K: Key<T> + Eq + Hash + Clone,
        T: Clone + Decodable,
        C: Cache<K, DBList<T>>,
    {
        {
            let read = cache.read();
            if let Some(v) = read.get(key) {
                return Some(v.clone());
            }
        }

        self.read_list(col, key).map(|value: DBList<T>| {
                                         let mut write = cache.write();
                                         write.insert(key.clone(), value.clone());
                                         value
                                     })
    }

    fn get_list_with_cache<K, T>(&self, col: Option<u32>, cache: &Cache<K, DBList<T>>, key: &K) -> Option<DBList<T>>
    where
        K: Key<T> + Eq + Hash + Clone,
        T: Clone + Decodable,
    {
        {
            if let Some(v) = cache.get(key) {
                return Some(v.clone());
            }
        }

        self.read_list(col, key)
    }

    /// Returns true if given value exists.
    fn exists<T, R>(&self, col: Option<u32>, key: &Key<T, Target = R>) -> bool
    where
        R: Deref<Target = [u8]>;

    /// Returns true if given value exists either in cache or in database.
    fn exists_with_cache<K, T, R, C>(&self, col: Option<u32>, cache: &RwLock<C>, key: &K) -> bool
    where
        K: Eq + Hash + Key<T, Target = R>,
        R: Deref<Target = [u8]>,
        C: Cache<K, T>,
    {
        {
            let read = cache.read();
            if read.get(key).is_some() {
                return true;
            }
        }

        self.exists::<T, R>(col, key)
    }
}

impl Writable for DBTransaction {
    fn write<T, R>(&mut self, col: Option<u32>, key: &Key<T, Target = R>, value: &T)
    where
        T: Encodable,
        R: Deref<Target = [u8]>,
    {
        self.put(col, &key.key(), &encode(value));
    }

    fn append<T, R>(&mut self, col: Option<u32>, key: &Key<T, Target = R>, value: &DBList<T>)
    where
        T: Encodable,
        R: Deref<Target = [u8]>,
    {
        self.put(col, &key.key(), &encode(value));
    }

    fn delete<T, R>(&mut self, col: Option<u32>, key: &Key<T, Target = R>)
    where
        T: Encodable,
        R: Deref<Target = [u8]>,
    {
        self.delete(col, &key.key());
    }
}

impl<KVDB: KeyValueDB + ?Sized> Readable for KVDB {
    fn read<T, R>(&self, col: Option<u32>, key: &Key<T, Target = R>) -> Option<T>
    where
        T: Decodable,
        R: Deref<Target = [u8]>,
    {
        let result = self.get(col, &key.key());

        match result {
            Ok(option) => option.map(|v| decode(&v)),
            Err(err) => {
                panic!("db get failed, key: {:?}, err: {:?}", &key.key() as &[u8], err);
            }
        }
    }

    fn read_list<T, R>(&self, col: Option<u32>, key: &Key<T, Target = R>) -> Option<DBList<T>>
    where
        T: Decodable,
        R: Deref<Target = [u8]>,
    {
        let result = self.get(col, &key.key());

        match result {
            Ok(option) => option.map(|v| decode(&v)),
            Err(err) => {
                panic!("db get failed, key: {:?}, err: {:?}", &key.key() as &[u8], err);
            }
        }
    }

    fn exists<T, R>(&self, col: Option<u32>, key: &Key<T, Target = R>) -> bool
    where
        R: Deref<Target = [u8]>,
    {
        let result = self.get(col, &key.key());

        match result {
            Ok(v) => v.is_some(),
            Err(err) => {
                panic!("db get failed, key: {:?}, err: {:?}", &key.key() as &[u8], err);
            }
        }
    }
}
