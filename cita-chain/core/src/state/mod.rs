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

// CITA, Copyright 2016-2017 Cryptape Technologies LLC.
// Remove cache.

//! A mutable state representation suitable to execute transactions.
//! Generic over a `Backend`. Deals with `Account`s.
//! Unconfirmed sub-states are managed with `checkpoint`s which may be canonicalized
//! or rolled back.

use cita_types::{Address, H256};
use std::collections::hash_set::HashSet;
use util::*;
pub mod backend;
use self::backend::*;
use state_db::*;

pub struct State<B: Backend> {
    db: B,
    root: H256,
    /* cache: RefCell<HashMap<Address, AccountEntry>>,
    // The original account is preserved in
    checkpoints: RefCell<Vec<HashMap<Address, Option<AccountEntry>>>>,
    
    account_start_nonce: U256,
    factories: Factories,
    */
}

/// Mode of dealing with null accounts.
#[derive(PartialEq)]
pub enum CleanupMode<'a> {
    /// Create accounts which would be null.
    ForceCreate,
    /// Don't delete null accounts upon touching, but also don't create them.
    NoEmpty,
    /// Add encountered null accounts to the provided kill-set, to be deleted later.
    KillEmpty(&'a mut HashSet<Address>),
}

impl<B: Backend> State<B> {
    /// Creates new state with existing state root
    pub fn from_existing(db: B, root: H256) -> Result<State<B>, TrieError> {
        if !db.as_hashdb().contains(&root) {
            return Err(TrieError::InvalidStateRoot(root));
        }

        let state = State { db, root };

        Ok(state)
    }

    /// Destroy the current object and return root and database.
    pub fn drop(self) -> (H256, B) {
        (self.root, self.db)
    }

    pub fn db(self) -> B {
        self.db
    }

    /// Return reference to root
    pub fn root(&self) -> &H256 {
        &self.root
    }
}

//Need
/* impl< B: Backend > fmt::Debug for State< B > {
fn fmt( & self, f: & mut fmt::Formatter) -> fmt::Result {
    write ! (f, "{:?}", self.cache.borrow())
}
}*/

// TODO: cloning for `State` shouldn't be possible in general; Remove this and use
// checkpoints where possible.
impl Clone for State<StateDB> {
    fn clone(&self) -> State<StateDB> {
        State {
            db: self.db.boxed_clone(),
            root: self.root,
        }
    }
}
