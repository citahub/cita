// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;

use cita_types::{Address, H256};
use state::Account;
use util::*;

/// State backend. See module docs for more details.
pub trait Backend: Send {
    /// Treat the backend as a read-only hashdb.
    fn as_hashdb(&self) -> &HashDB;

    /// Treat the backend as a writeable hashdb.
    fn as_hashdb_mut(&mut self) -> &mut HashDB;

    /// Add an account entry to the cache.
    fn add_to_account_cache(&mut self, addr: Address, data: Option<Account>, modified: bool);

    /// Sync all account entries from backend's local cache to global cache.
    fn sync_account_cache(&mut self);

    /// Add a global code cache entry. This doesn't need to worry about canonicality because
    /// it simply maps hashes to raw code and will always be correct in the absence of
    /// hash collisions.
    fn cache_code(&self, hash: H256, code: Arc<Vec<u8>>);

    /// Get basic copy of the cached account. Not required to include storage.
    /// Returns 'None' if cache is disabled or if the account is not cached.
    fn get_cached_account(&self, addr: &Address) -> Option<Account>;

    /// Get value from a cached account.
    /// `None` is passed to the closure if the account entry cached
    /// is known not to exist.
    /// `None` is returned if the entry is not cached.
    fn get_cached<F, U>(&self, a: &Address, f: F) -> Option<U>
    where
        F: FnOnce(Option<&mut Account>) -> U;

    /// Get cached code based on hash.
    fn get_cached_code(&self, hash: &H256) -> Option<Arc<Vec<u8>>>;

    /// Note that an account with the given address is non-null.
    fn note_non_null_account(&self, address: &Address);

    /// Check whether an account is known to be empty. Returns true if known to be
    /// empty, false otherwise.
    fn is_known_null(&self, address: &Address) -> bool;
}
