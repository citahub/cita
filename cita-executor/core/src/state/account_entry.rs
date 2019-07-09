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

use cita_types::U256;

use crate::state::account::Account;
pub use crate::substate::Substate;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
/// Account modification state. Used to check if the account was
/// Modified in between commits and overall.
pub enum AccountState {
    /// Account was loaded from disk and never modified in this state object.
    CleanFresh,
    /// Account was loaded from the global cache and never modified.
    CleanCached,
    /// Account has been modified and is not committed to the trie yet.
    /// This is set if any of the account data is changed, including
    /// storage, code and ABI.
    Dirty,
    /// Account was modified and committed to the trie.
    Committed,
}

#[derive(Debug)]
/// In-memory copy of the account data. Holds the optional account
/// and the modification status.
/// Account entry can contain existing (`Some`) or non-existing
/// account (`None`)
pub struct AccountEntry {
    /// Account entry. `None` if account known to be non-existant.
    pub account: Option<Account>,
    /// Unmodified account balance.
    pub old_balance: Option<U256>,
    /// Entry state.
    pub state: AccountState,
}

// Account cache item. Contains account data and
// modification state
impl AccountEntry {
    pub fn is_dirty(&self) -> bool {
        self.state == AccountState::Dirty
    }

    pub fn is_commited(&self) -> bool {
        self.state == AccountState::Committed
    }

    pub fn exists_and_is_null(&self) -> bool {
        self.account.as_ref().map_or(false, Account::is_null)
    }

    /// Clone dirty data into new `AccountEntry`. This includes
    /// basic account data and modified storage keys.
    pub fn clone_dirty(&self) -> AccountEntry {
        AccountEntry {
            old_balance: self.old_balance,
            account: self.account.as_ref().map(Account::clone_dirty),
            state: self.state,
        }
    }

    // Create a new account entry and mark it as dirty.
    pub fn new_dirty(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            old_balance: account.as_ref().map(|a| *a.balance()),
            account,
            state: AccountState::Dirty,
        }
    }

    // Create a new account entry and mark it as clean.
    pub fn new_clean(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            old_balance: account.as_ref().map(|a| *a.balance()),
            account,
            state: AccountState::CleanFresh,
        }
    }

    // Create a new account entry and mark it as clean and cached.
    pub fn new_clean_cached(account: Option<Account>) -> AccountEntry {
        AccountEntry {
            old_balance: account.as_ref().map(|a| *a.balance()),
            account,
            state: AccountState::CleanCached,
        }
    }

    // Replace data with another entry but preserve storage cache.
    pub fn overwrite_with(&mut self, other: AccountEntry) {
        self.state = other.state;
        match other.account {
            Some(acc) => {
                if let Some(ref mut ours) = self.account {
                    ours.overwrite_with(acc);
                }
            }
            None => self.account = None,
        }
    }

    pub fn account(&self) -> Option<&Account> {
        self.account.as_ref()
    }
}
