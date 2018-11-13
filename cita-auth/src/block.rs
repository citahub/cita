// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use cita_types::H256;
use handler::verify_tx_sig;
use libproto::{BlockTxn, GetBlockTxn, Origin};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::ops::{Deref, DerefMut};

// Origin, GetBlockTxn, SendFlag
pub type BlockTxnReq = (Origin, GetBlockTxn, bool);

pub struct BlockTxnMessage {
    pub origin: Origin,
    pub block_txn: BlockTxn,
}

impl Deref for BlockTxnMessage {
    type Target = BlockTxn;

    fn deref(&self) -> &BlockTxn {
        &self.block_txn
    }
}

impl DerefMut for BlockTxnMessage {
    fn deref_mut(&mut self) -> &mut BlockTxn {
        &mut self.block_txn
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BadOrigin { expected: Origin, got: Origin },
    BadBlockHash { expected: H256, got: H256 },
    BadShortID,
    BadTxSignature,
}

type Pubkey = Vec<u8>;
type PubkeyAndHash = (Pubkey, H256);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match *self {
            BadOrigin { expected, got } => {
                write!(f, "Bad origin: expect {}, got {}", expected, got)
            }
            BadBlockHash { expected, got } => {
                write!(f, "Bad block hash: expected {}, got {}", expected, got)
            }
            BadShortID => write!(f, "Including transaction with bad short id"),
            BadTxSignature => write!(f, "Including transaction with bad signature"),
        }
    }
}

impl BlockTxnMessage {
    pub fn validate(&mut self, req: &BlockTxnReq) -> Result<Vec<PubkeyAndHash>, Error> {
        let (expected_origin, expect_block_txn, _) = req;

        // Validate origin
        let origin = self.origin;
        if *expected_origin != origin {
            return Err(Error::BadOrigin {
                expected: *expected_origin,
                got: origin,
            });
        }

        let expected_block_hash = expect_block_txn.get_block_hash();
        let expected_short_ids = expect_block_txn.get_short_ids();
        let block_hash = self.take_block_hash();
        let transactions = self.take_transactions();

        // Validate block hash
        if block_hash != expected_block_hash {
            return Err(Error::BadBlockHash {
                expected: H256::from(expected_block_hash),
                got: H256::from(block_hash.as_slice()),
            });
        }

        // Validate short_ids
        if expected_short_ids.len() != transactions.len() {
            return Err(Error::BadShortID);
        }

        if expected_short_ids
            .iter()
            .zip(transactions.iter())
            .any(|(short_id, transaction)| short_id != &transaction.crypt_hash().to_vec())
        {
            return Err(Error::BadShortID);
        }

        // Validate transaction signature
        let results: Vec<Option<PubkeyAndHash>> = expected_short_ids
            .into_par_iter()
            .zip(transactions.into_par_iter())
            .map(|(short_id, transaction)| {
                let tx_hash = H256::from_slice(short_id);
                let result = verify_tx_sig(
                    transaction.get_crypto(),
                    &tx_hash,
                    transaction.get_signature(),
                );
                match result {
                    Ok(pubkey) => Some((pubkey, tx_hash)),
                    Err(_) => None,
                }
            })
            .collect();

        if results.iter().any(|result| result.is_none()) {
            return Err(Error::BadTxSignature);
        };
        Ok(results.into_iter().map(|x| x.unwrap()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libproto::UnverifiedTransaction;

    #[test]
    fn validate_origin() {
        let expected: BlockTxnReq = (1, GetBlockTxn::default(), true);
        let mut block_txn_message = BlockTxnMessage {
            origin: 2,
            block_txn: BlockTxn::default(),
        };

        let result = block_txn_message.validate(&expected);
        assert_eq!(
            result,
            Err(Error::BadOrigin {
                expected: 1,
                got: 2
            })
        );
    }

    #[test]
    fn validate_block_hash() {
        let mut b1 = GetBlockTxn::new();
        let h1 = H256::from(1);
        b1.set_block_hash(h1.to_vec());
        let expected: BlockTxnReq = (1, b1, true);

        let mut b2 = BlockTxn::new();
        let h2 = H256::from(2);
        b2.set_block_hash(h2.to_vec());
        let mut block_txn_message = BlockTxnMessage {
            origin: 1,
            block_txn: b2,
        };

        let result = block_txn_message.validate(&expected);
        assert_eq!(
            result,
            Err(Error::BadBlockHash {
                expected: h1,
                got: h2
            })
        );
    }

    #[test]
    fn validate_short_ids_len() {
        let b1 = GetBlockTxn::new();
        let expected: BlockTxnReq = (1, b1, true);

        let mut b2 = BlockTxn::new();
        b2.set_transactions(vec![UnverifiedTransaction::new()].into());
        let mut block_txn_message = BlockTxnMessage {
            origin: 1,
            block_txn: b2,
        };

        let result = block_txn_message.validate(&expected);
        assert_eq!(result, Err(Error::BadShortID));
    }

    #[test]
    fn validate_short_ids() {
        let mut b1 = GetBlockTxn::new();
        let h1 = H256::from(1);
        b1.set_short_ids(vec![h1.to_vec()].into());
        let expected: BlockTxnReq = (1, b1, true);

        let mut b2 = BlockTxn::new();
        b2.set_transactions(vec![UnverifiedTransaction::new()].into());
        let mut block_txn_message = BlockTxnMessage {
            origin: 1,
            block_txn: b2,
        };

        let result = block_txn_message.validate(&expected);
        assert_eq!(result, Err(Error::BadShortID));
    }

    #[test]
    fn validate_tx_signature() {
        let mut b1 = GetBlockTxn::new();
        let t1 = UnverifiedTransaction::new();
        b1.set_short_ids(vec![t1.crypt_hash().to_vec()].into());
        let expected: BlockTxnReq = (1, b1, true);

        let mut b2 = BlockTxn::new();
        b2.set_transactions(vec![t1].into());
        let mut block_txn_message = BlockTxnMessage {
            origin: 1,
            block_txn: b2,
        };

        let result = block_txn_message.validate(&expected);
        assert_eq!(result, Err(Error::BadTxSignature));
    }
}
