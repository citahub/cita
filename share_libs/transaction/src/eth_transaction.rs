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

use crypto::{Signature, Public, pubkey_to_address, SIGNATURE_BYTES_LEN, HASH_BYTES_LEN, PUBKEY_BYTES_LEN};
use error::Error;
use libproto::blockchain::{Transaction as ProtoTransaction, SignedTransaction as ProtoSignedTransaction};
use std::ops::Deref;
use std::str::FromStr;
use util::{H256, Address, U256, Bytes, HeapSizeOf, H520, H512};

// pub const STORE_ADDRESS: Address =  H160( [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff] );
pub const STORE_ADDRESS: &str = "ffffffffffffffffffff";

#[derive(Debug, Clone, PartialEq, Eq)]
/// Transaction action type.
pub enum Action {
    /// Just store the data.
    Store,
    /// Create creates new contract.
    Create,
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(Address),
}

impl Default for Action {
    fn default() -> Action {
        Action::Create
    }
}

/// A set of information describing an externally-originating message call
/// or contract creation operation.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct VMTransaction {
    /// Nonce.
    pub nonce: U256,
    /// Gas price.
    pub gas_price: U256,
    /// Gas paid up front for transaction execution.
    pub gas: U256,
    /// Action, can be either call or contract create.
    pub action: Action,
    /// Transfered value.
    pub value: U256,
    /// Transaction data.
    pub data: Bytes,
    /// Protobufed Plain Transaction
    pub hash: H256,
}

impl HeapSizeOf for VMTransaction {
    fn heap_size_of_children(&self) -> usize {
        self.data.heap_size_of_children()
    }
}

// TODO: refactor transaction in protobuf,
// now using the same type `ProtoTransaction`,
// it's not a good design.
impl VMTransaction {
    pub fn new(plain_transaction: ProtoTransaction, tx_hash: H256) -> Result<Self, Error> {
        let nonce = plain_transaction.nonce.parse::<u32>().map_err(|_| Error::ParseError)?;
        Ok(VMTransaction {
               nonce: nonce.into(),
               gas_price: U256::default(),
               gas: U256::from(1_000_000u64),
               action: {
                   let to = plain_transaction.get_to();
                   match to.is_empty() {
                       true => Action::Create,
                       false => match to {
                           STORE_ADDRESS => Action::Store,
                           _ => Action::Call(Address::from_str(plain_transaction.get_to()).map_err(|_| Error::ParseError)?),
                       },
                   }
               },
               value: U256::default(),
               data: plain_transaction.data.into(),
               hash: tx_hash,
           })

    }

    // Specify the sender; this won't survive the serialize/deserialize process, but can be cloned.
    pub fn fake_sign(self, from: Address) -> SignedTransaction {
        let signature = Signature::from_rsv(&H256::default(), &H256::default(), 0);
        SignedTransaction {
            transaction: self,
            tx_hash: 0.into(),
            signature: signature,
            sender: from,
            public: Public::default(),
        }
    }

    pub fn crypt_hash(&self) -> H256 {
        self.hash
    }
}

/// A `UnverifiedTransaction` with successfully recovered `sender`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignedTransaction {
    transaction: VMTransaction,
    tx_hash: H256,
    signature: Signature,
    sender: Address,
    public: Public,
}

impl Deref for SignedTransaction {
    type Target = VMTransaction;
    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl SignedTransaction {
    /// Try to verify transaction and recover sender.
    pub fn new(mut stx: ProtoSignedTransaction) -> Result<Self, Error> {
        if stx.tx_hash.len() != HASH_BYTES_LEN {
            return Err(Error::InvaliHash);
        }

        if stx.get_transaction_with_sig().get_signature().len() != SIGNATURE_BYTES_LEN {
            return Err(Error::InvalidSignature);
        }

        if stx.signer.len() != PUBKEY_BYTES_LEN {
            return Err(Error::InvalidPubKey);
        }

        let tx_hash = H256::from_slice(&stx.tx_hash);
        let signature: Signature = H520::from_slice(stx.get_transaction_with_sig().get_signature()).into();
        let public = H512::from_slice(&stx.signer);
        let sender = pubkey_to_address(&public);
        Ok(SignedTransaction {
               transaction: VMTransaction::new(stx.mut_transaction_with_sig().take_transaction(), tx_hash.clone())?,
               tx_hash: tx_hash,
               signature: signature,
               sender: sender,
               public: public,
           })
    }

    /// Returns transaction sender.
    pub fn sender(&self) -> Address {
        self.sender
    }

    /// Returns a public key of the sender.
    pub fn public_key(&self) -> Public {
        self.public
    }
}
