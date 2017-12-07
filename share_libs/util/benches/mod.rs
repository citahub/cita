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

//! benchmarking for AVL and Trie
// TODO: bench merklehash.
#![cfg_attr(test, feature(test))]
extern crate test;
extern crate util;

use test::Bencher;
use util::{H256, HashDB};
use util::avl::AVLMut;
use util::avl::avldbmut::*;
use util::avl::standardmap::*;
use util::memorydb::*;
use util::trie::TrieMut;
use util::trie::triedbmut::*;

const SIZE: usize = 1000000;

fn populate_avl<'db>(db: &'db mut HashDB, root: &'db mut H256, v: &[(Vec<u8>, Vec<u8>)]) -> AVLDBMut<'db> {
    let mut t = AVLDBMut::new(db, root);
    for i in 0..v.len() {
        let key: &[u8] = &v[i].0;
        let val: &[u8] = &v[i].1;
        t.insert(key, val).unwrap();
    }
    t
}

// fn unpopulate_avl<'db>(t: &mut AVLDBMut<'db>, v: &[(Vec<u8>, Vec<u8>)]) {
//     for i in v {
//         let key: &[u8] = &i.0;
//         t.remove(key).unwrap();
//     }
// }


#[bench]
fn bench_avl_update(b: &mut Bencher) {
    let mut seed = H256::default();
    let x = StandardMap {
        alphabet: Alphabet::All,
        min_key: 32,
        journal_key: 0,
        value_mode: ValueMode::Mirror,
        count: SIZE,
    }.make_with(&mut seed);

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    let mut memavl = populate_avl(&mut memdb, &mut root, &x);

    memavl.commit();
    let mut i: usize = 1000;
    b.iter(|| {
        memavl
            .insert(&x[i % SIZE].0, &(i.to_string().as_bytes().to_vec()))
            .unwrap();
        // memavl.commit();
        i += 1;
    });
}

#[bench]
fn bench_avl_commit(b: &mut Bencher) {
    let mut seed = H256::default();
    let x = StandardMap {
        alphabet: Alphabet::All,
        min_key: 4,
        journal_key: 0,
        value_mode: ValueMode::Mirror,
        count: SIZE,
    }.make_with(&mut seed);

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    let mut memavl = populate_avl(&mut memdb, &mut root, &x);

    memavl.commit();
    let mut i: usize = 1000;
    b.iter(|| {
        memavl
            .insert(&x[i % SIZE].0, &(i.to_string().as_bytes().to_vec()))
            .unwrap();
        memavl.commit();
        i += 1;
    });
}

#[bench]
fn bench_avl_query(b: &mut Bencher) {
    let mut seed = H256::default();
    let x = StandardMap {
        alphabet: Alphabet::All,
        min_key: 32,
        journal_key: 0,
        value_mode: ValueMode::Mirror,
        count: SIZE,
    }.make_with(&mut seed);

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    let mut memavl = populate_avl(&mut memdb, &mut root, &x);

    memavl.commit();
    let mut i: usize = 0;
    b.iter(|| {
        //memavl.insert(&x[i%SIZE].0, &(i.to_string().as_bytes().to_vec()));
        // memavl.commit();
        memavl.get(&x[i % SIZE].0).unwrap().unwrap();
        i += 1;
    });
}

fn populate_trie<'db>(db: &'db mut HashDB, root: &'db mut H256, v: &[(Vec<u8>, Vec<u8>)]) -> TrieDBMut<'db> {
    let mut t = TrieDBMut::new(db, root);
    for i in 0..v.len() {
        let key: &[u8] = &v[i].0;
        let val: &[u8] = &v[i].1;
        t.insert(key, val).unwrap();
    }
    t
}

// fn unpopulate_trie<'db>(t: &mut TrieDBMut<'db>, v: &[(Vec<u8>, Vec<u8>)]) {
//     for i in v {
//         let key: &[u8] = &i.0;
//         t.remove(key).unwrap();
//     }
// }

#[bench]
fn bench_trie_update(b: &mut Bencher) {
    let mut seed = H256::default();
    let x = StandardMap {
        alphabet: Alphabet::All,
        min_key: 32,
        journal_key: 0,
        value_mode: ValueMode::Mirror,
        count: SIZE,
    }.make_with(&mut seed);

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    let mut memtrie = populate_trie(&mut memdb, &mut root, &x);

    memtrie.commit();
    let mut i: usize = 1000;
    b.iter(|| {
        memtrie
            .insert(&x[i % SIZE].0, &(i.to_string().as_bytes().to_vec()))
            .unwrap();
        // memtrie.commit();
        i += 1;
    });
}

#[bench]
fn bench_trie_commit(b: &mut Bencher) {
    let mut seed = H256::default();
    let x = StandardMap {
        alphabet: Alphabet::All,
        min_key: 32,
        journal_key: 0,
        value_mode: ValueMode::Mirror,
        count: SIZE,
    }.make_with(&mut seed);

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    let mut memtrie = populate_trie(&mut memdb, &mut root, &x);

    memtrie.commit();
    let mut i: usize = 1000;
    b.iter(|| {
        memtrie
            .insert(&x[i % SIZE].0, &(i.to_string().as_bytes().to_vec()))
            .unwrap();
        memtrie.commit();
        i += 1;
    });
}

#[bench]
fn bench_trie_query(b: &mut Bencher) {
    let mut seed = H256::default();
    let x = StandardMap {
        alphabet: Alphabet::All,
        min_key: 32,
        journal_key: 0,
        value_mode: ValueMode::Mirror,
        count: SIZE,
    }.make_with(&mut seed);

    let mut memdb = MemoryDB::new();
    let mut root = H256::default();
    let mut memtrie = populate_trie(&mut memdb, &mut root, &x);

    memtrie.commit();
    let mut i: usize = 0;
    b.iter(|| {
        memtrie.get(&x[i % SIZE].0).unwrap().unwrap();
        i += 1;
    });
}
