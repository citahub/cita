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

//! Account state encoding and decoding

use account_db::{AccountDB, AccountDBMut};
use cita_types::{Address, H256, U256};
use rlp::{RlpStream, UntrustedRlp};
use snapshot::Error;
use std::collections::HashSet;
use types::basic_account::BasicAccount;
use util::hashdb::HashDB;
use util::{Bytes, Trie, TrieDB, TrieDBMut, TrieMut};
use util::{HASH_EMPTY, HASH_NULL_RLP};

// An empty account -- these were replaced with RLP null data for a space optimization in v1.
const ACC_EMPTY: BasicAccount = BasicAccount {
    nonce: U256([0, 0, 0, 0]),
    balance: U256([0, 0, 0, 0]),
    storage_root: HASH_NULL_RLP,
    code_hash: HASH_EMPTY,
    abi_hash: HASH_EMPTY,
};

// whether an encoded account has code and how it is referred to.
#[repr(u8)]
enum CodeState {
    // the account has no code.
    Empty = 0,
    // raw code is encoded.
    Inline = 1,
    // the code is referred to by hash.
    Hash = 2,
}

impl CodeState {
    fn from(x: u8) -> Result<Self, Error> {
        match x {
            0 => Ok(CodeState::Empty),
            1 => Ok(CodeState::Inline),
            2 => Ok(CodeState::Hash),
            _ => Err(Error::UnrecognizedCodeState(x)),
        }
    }

    fn raw(self) -> u8 {
        self as u8
    }
}

// walk the account's storage trie, returning a vector of RLP items containing the
// account address hash, account properties and the storage. Each item contains at most `max_storage_items`
// storage records split according to snapshot format definition.
#[allow(unknown_lints, clippy::implicit_hasher)] // TODO clippy
pub fn to_fat_rlps(
    address: &Address,
    acc: &BasicAccount,
    acct_db: &AccountDB,
    used_code: &mut HashSet<H256>,
    used_abi: &mut HashSet<H256>,
    first_chunk_size: usize,
    max_chunk_size: usize,
) -> Result<Vec<Bytes>, Error> {
    let db = TrieDB::new(acct_db, &acc.storage_root).unwrap();
    let mut chunks = Vec::new();
    let mut db_iter = db.iter().map_err(|err| *err)?;
    let mut target_chunk_size = first_chunk_size;
    let mut account_stream = RlpStream::new_list(2);
    let mut leftover: Option<Vec<u8>> = None;

    loop {
        account_stream.append(address);
        account_stream.begin_list(7);

        account_stream.append(&acc.nonce).append(&acc.balance);

        // [has_code, code_hash].
        if acc.code_hash == HASH_EMPTY {
            account_stream
                .append(&CodeState::Empty.raw())
                .append_empty_data();
        } else if used_code.contains(&acc.code_hash) {
            account_stream
                .append(&CodeState::Hash.raw())
                .append(&acc.code_hash);
        } else {
            match acct_db.get(&acc.code_hash) {
                Some(c) => {
                    used_code.insert(acc.code_hash);
                    account_stream.append(&CodeState::Inline.raw()).append(&&*c);
                }
                None => {
                    info!("code lookup failed during snapshot");
                    account_stream.append(&false).append_empty_data();
                }
            }
        }

        // [has_abi, abi_hash].
        if acc.abi_hash == HASH_EMPTY {
            account_stream
                .append(&CodeState::Empty.raw())
                .append_empty_data();
        } else if used_abi.contains(&acc.abi_hash) {
            account_stream
                .append(&CodeState::Hash.raw())
                .append(&acc.abi_hash);
        } else {
            match acct_db.get(&acc.abi_hash) {
                Some(c) => {
                    used_abi.insert(acc.abi_hash);
                    account_stream.append(&CodeState::Inline.raw()).append(&&*c);
                }
                None => {
                    info!("abi lookup failed during snapshot");
                    account_stream.append(&false).append_empty_data();
                }
            }
        }

        account_stream.begin_unbounded_list();
        if account_stream.len() > target_chunk_size {
            // account does not fit, push an empty record to mark a new chunk
            target_chunk_size = max_chunk_size;
            chunks.push(Vec::new());
        }

        if let Some(pair) = leftover.take() {
            if !account_stream.append_raw_checked(&pair, 1, target_chunk_size) {
                return Err(Error::ChunkTooSmall);
            }
        }

        loop {
            match db_iter.next() {
                Some(Ok((k, v))) => {
                    let pair = {
                        let mut stream = RlpStream::new_list(2);
                        stream.append(&k).append(&&*v);
                        stream.drain()
                    };
                    if !account_stream.append_raw_checked(&pair, 1, target_chunk_size) {
                        account_stream.complete_unbounded_list();
                        let stream =
                            ::std::mem::replace(&mut account_stream, RlpStream::new_list(2));
                        chunks.push(stream.out());
                        target_chunk_size = max_chunk_size;
                        leftover = Some(pair.into_vec());

                        break;
                    }
                }
                Some(Err(e)) => {
                    return Err((*e).into());
                }
                None => {
                    account_stream.complete_unbounded_list();
                    let stream = ::std::mem::replace(&mut account_stream, RlpStream::new_list(2));
                    chunks.push(stream.out());

                    return Ok(chunks);
                }
            }
        }
    }
}

// decode a fat rlp, and rebuild the storage trie as we go.
// returns the account structure along with its newly recovered code,
// if it exists.
pub fn from_fat_rlp(
    acct_db: &mut AccountDBMut,
    rlp: &UntrustedRlp,
    mut storage_root: H256,
) -> Result<(BasicAccount, Option<Bytes>, Option<Bytes>), Error> {
    //use trie::{TrieDBMut, TrieMut};

    // check for special case of empty account.
    if rlp.is_empty() {
        return Ok((ACC_EMPTY, None, None));
    }

    let nonce = rlp.val_at(0)?;
    let balance = rlp.val_at(1)?;
    let code_state: CodeState = {
        let raw: u8 = rlp.val_at(2)?;
        CodeState::from(raw)?
    };

    // load the code if it exists.
    let (code_hash, new_code) = match code_state {
        CodeState::Empty => (HASH_EMPTY, None),
        CodeState::Inline => {
            let code: Bytes = rlp.val_at(3)?;
            let code_hash = acct_db.insert(&code);

            (code_hash, Some(code))
        }
        CodeState::Hash => {
            let code_hash = rlp.val_at(3)?;

            (code_hash, None)
        }
    };

    let abi_state: CodeState = {
        let raw: u8 = rlp.val_at(4)?;
        CodeState::from(raw)?
    };

    // load the abi if it exists.
    let (abi_hash, new_abi) = match abi_state {
        CodeState::Empty => (HASH_EMPTY, None),
        CodeState::Inline => {
            let abi: Bytes = rlp.val_at(5)?;
            let abi_hash = acct_db.insert(&abi);

            (abi_hash, Some(abi))
        }
        CodeState::Hash => {
            let abi_hash = rlp.val_at(5)?;

            (abi_hash, None)
        }
    };

    {
        let mut storage_trie = if storage_root.is_zero() {
            TrieDBMut::new(acct_db, &mut storage_root)
        } else {
            TrieDBMut::from_existing(acct_db, &mut storage_root).map_err(|err| *err)?
        };
        let pairs = rlp.at(6)?;
        for pair_rlp in pairs.iter() {
            let k: Bytes = pair_rlp.val_at(0)?;
            let v: Bytes = pair_rlp.val_at(1)?;

            storage_trie.insert(&k, &v).map_err(|err| *err)?;
        }
    }

    let acc = BasicAccount {
        nonce,
        storage_root,
        code_hash,
        balance,
        abi_hash,
    };

    Ok((acc, new_code, new_abi))
}
