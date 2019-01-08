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

use account_db::AccountDBMut;
use cita_db::{sec_trie_root, HashDB, TrieFactory};
use cita_types::{H256, U256};
use hashable::Hashable;
use rlp::{self, RlpStream};
use state::Account;
use std::collections::BTreeMap;
use std::fmt;
use util::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An account, expressed as Plain-Old-Data (hence the name).
/// Does not have a DB overlay cache, code hash or anything like that.
pub struct PodAccount {
    /// The balance of the account.
    pub balance: U256,
    /// The nonce of the account.
    pub nonce: U256,
    /// The code of the account or `None` in the special case that it is unknown.
    pub code: Option<Bytes>,
    /// The abi of the account or `None` in the special case that it is unknown.
    pub abi: Option<Bytes>,
    /// The storage of the account.
    pub storage: BTreeMap<H256, H256>,
}

impl PodAccount {
    /// Convert Account to a PodAccount.
    /// NOTE: This will silently fail unless the account is fully cached.
    pub fn from_account(acc: &Account) -> PodAccount {
        PodAccount {
            balance: *acc.balance(),
            nonce: *acc.nonce(),
            storage: acc
                .storage_changes()
                .iter()
                .fold(BTreeMap::new(), |mut m, (k, v)| {
                    m.insert(*k, *v);
                    m
                }),
            code: acc.code().map(|x| x.to_vec()),
            abi: acc.abi().map(|x| x.to_vec()),
        }
    }

    /// Returns the RLP for this account.
    pub fn rlp(&self) -> Bytes {
        let mut stream = RlpStream::new_list(5);
        stream.append(&self.nonce);
        stream.append(&self.balance);
        stream.append(&sec_trie_root(
            self.storage
                .iter()
                .map(|(k, v)| (k.to_vec(), rlp::encode(&U256::from(&**v)).to_vec()))
                .collect(),
        ));
        stream.append(&self.code.as_ref().unwrap_or(&vec![]).crypt_hash());
        stream.append(&self.abi.as_ref().unwrap_or(&vec![]).crypt_hash());
        stream.out()
    }

    /// Place additional data into given hash DB.
    pub fn insert_additional(&self, db: &mut AccountDBMut, factory: &TrieFactory) {
        match self.code {
            Some(ref c) if !c.is_empty() => {
                db.insert(c);
            }
            _ => {}
        }
        match self.abi {
            Some(ref c) if !c.is_empty() => {
                db.insert(c);
            }
            _ => {}
        }
        let mut r = H256::new();
        let mut t = factory.create(db, &mut r);
        for (k, v) in &self.storage {
            if let Err(e) = t.insert(k, &rlp::encode(&U256::from(&**v))) {
                warn!("Encountered potential DB corruption: {}", e);
            }
        }
    }
}

impl fmt::Display for PodAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(bal={}; nonce={}; code={} bytes, #{}; abi={} bytes, #{}; storage={} items)",
            self.balance,
            self.nonce,
            self.code.as_ref().map_or(0, |c| c.len()),
            self.code
                .as_ref()
                .map_or_else(H256::new, |c| c.crypt_hash()),
            self.abi.as_ref().map_or(0, |c| c.len()),
            self.abi.as_ref().map_or_else(H256::new, |c| c.crypt_hash()),
            self.storage.len(),
        )
    }
}
