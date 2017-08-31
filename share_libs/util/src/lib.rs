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
#![allow(unused_extern_crates)]
extern crate rustc_hex;
extern crate rocksdb;
extern crate elastic_array;
extern crate libc;
extern crate target_info;
extern crate bigint;
extern crate parking_lot;
extern crate ansi_term;
extern crate rlp;
extern crate regex;
extern crate lru_cache;
extern crate heapsize;
extern crate itertools;
extern crate devtools;
extern crate ethcore_logger;
#[cfg(feature = "sha3hash")]
extern crate sha3;
#[cfg(feature = "blake2bhash")]
extern crate blake2b;

#[macro_use]
extern crate log as rlog;

pub mod avl;
pub mod merklehash;
pub mod hashable;
pub mod common;
pub mod error;
pub mod bytes;
pub mod misc;
pub mod vector;
pub mod hashdb;
pub mod memorydb;
pub mod overlaydb;
pub mod journaldb;
pub mod kvdb;
pub mod triehash;
pub mod trie;
pub mod nibbleslice;
pub mod nibblevec;
pub mod semantic_version;
pub mod snappy;
pub mod cache;
pub mod crypto;


pub use ansi_term::{Colour, Style};
pub use bigint::*;
pub use bytes::*;
// pub use timer::*;
pub use error::*;
pub use hashable::*;
pub use hashdb::*;
pub use heapsize::HeapSizeOf;
pub use itertools::Itertools;
pub use journaldb::JournalDB;
pub use kvdb::*;
pub use memorydb::MemoryDB;
pub use misc::*;
pub use overlaydb::*;
pub use parking_lot::{Condvar, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use semantic_version::*;
pub use trie::{Trie, TrieMut, TrieDB, TrieDBMut, TrieFactory, TrieError, SecTrieDB, SecTrieDBMut};
pub use triehash::*;
pub use vector::*;

/// 160-bit integer representing account address
pub type Address = H160;
pub type Bloom = H2048;
