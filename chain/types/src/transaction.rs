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

use BlockNumber;
use ed25519::{Signature, Public, pubkey_to_address, SIGNATURE_BYTES_LEN, HASH_BYTES_LEN, PUBKEY_BYTES_LEN};
use libproto::blockchain::{Transaction as ProtoTransaction, UnverifiedTransaction as ProtoUnverifiedTransaction, SignedTransaction as ProtoSignedTransaction, Crypto as ProtoCrypto};
use rlp::*;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use util::{H256, Address, U256, Bytes, HeapSizeOf, H768};

// pub const STORE_ADDRESS: H160 =  H160( [0xff; 20] );
pub const STORE_ADDRESS: &str = "ffffffffffffffffffffffffffffffffffffffff";

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    ParseError,
    InvalidHash,
    InvalidSignature,
    InvalidPubKey,
}

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
        Action::Store
    }
}

impl Decodable for Action {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            Ok(Action::Create)
        } else {
            let store_addr: Address = STORE_ADDRESS.into();
            let addr: Address = rlp.as_val()?;
            if addr == store_addr { Ok(Action::Store) } else { Ok(Action::Call(addr)) }
        }
    }
}

impl Encodable for Action {
    fn rlp_append(&self, s: &mut RlpStream) {
        let store_addr: Address = STORE_ADDRESS.into();
        match *self {
            Action::Create => s.append_internal(&""),
            Action::Call(ref addr) => s.append_internal(addr),
            Action::Store => s.append_internal(&store_addr),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// crypto type.
pub enum CryptoType {
    SECP,
    SM2,
}

impl Default for CryptoType {
    fn default() -> CryptoType {
        CryptoType::SECP
    }
}

impl Decodable for CryptoType {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        match rlp.as_val::<u8>()? {
            0 => Ok(CryptoType::SECP),
            1 => Ok(CryptoType::SM2),
            _ => Err(DecoderError::Custom("Unknown Type.")),
        }
    }
}

impl Encodable for CryptoType {
    fn rlp_append(&self, s: &mut RlpStream) {
        match *self {
            CryptoType::SECP => s.append_internal(&(0 as u8)),
            CryptoType::SM2 => s.append_internal(&(1 as u8)),
        };
    }
}

impl From<ProtoCrypto> for CryptoType {
    fn from(c: ProtoCrypto) -> CryptoType {
        match c {
            ProtoCrypto::SECP => CryptoType::SECP,
            ProtoCrypto::SM2 => CryptoType::SM2,
        }
    }
}

/// A set of information describing an externally-originating message call
/// or contract creation operation.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
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
    /// valid before this block number
    pub block_limit: BlockNumber,
}

impl HeapSizeOf for Transaction {
    fn heap_size_of_children(&self) -> usize {
        self.data.heap_size_of_children()
    }
}

impl Decodable for Transaction {
    fn decode(d: &UntrustedRlp) -> Result<Self, DecoderError> {
        if d.item_count()? != 7 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(Transaction {
               nonce: d.val_at(0)?,
               gas_price: d.val_at(1)?,
               gas: d.val_at(2)?,
               action: d.val_at(3)?,
               value: d.val_at(4)?,
               data: d.val_at(5)?,
               block_limit: d.val_at(6)?,
           })
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.rlp_append_unsigned_transaction(s)
    }
}

// TODO: refactor transaction in protobuf,
// now using the same type `ProtoTransaction`,
// it's not a good design.
impl Transaction {
    pub fn new(plain_transaction: &ProtoTransaction) -> Result<Self, Error> {
        // let nonce = plain_transaction.nonce.parse::<u32>().map_err(|_| Error::ParseError)?;
        Ok(Transaction {
               nonce: U256::from_str(plain_transaction.get_nonce()).map_err(|_| Error::ParseError)?,
               gas_price: U256::default(),
               gas: U256::from_str(plain_transaction.get_quota()).map_err(|_| Error::ParseError)?,
               action: {
                   let to = plain_transaction.get_to();
                   match to.is_empty() {
                       true => Action::Create,
                       false => match to {
                           STORE_ADDRESS => Action::Store,
                           _ => Action::Call(Address::from_str(to).map_err(|_| Error::ParseError)?),
                       },
                   }
               },
               value: U256::default(),
               data: plain_transaction.get_data().into(),
               block_limit: plain_transaction.get_valid_until_block(),
           })

    }

    pub fn nonce(&self) -> &U256 {
        &self.nonce
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    // Specify the sender; this won't survive the serialize/deserialize process, but can be cloned.
    pub fn fake_sign(self, from: Address) -> SignedTransaction {
        let signature = Signature::default();
        SignedTransaction {
            transaction: UnverifiedTransaction {
                unsigned: self,
                signature: signature,
                hash: 0.into(),
                crypto_type: CryptoType::default(),
            },
            sender: from,
            public: Public::default(),
        }
    }

    /// Append object with a without signature into RLP stream
    pub fn rlp_append_unsigned_transaction(&self, s: &mut RlpStream) {
        s.begin_list(7);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.action);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.block_limit);
    }

    /// get the protobuf transaction
    pub fn proto_transaction(&self) -> ProtoTransaction {
        let mut pt = ProtoTransaction::new();
        pt.set_nonce(self.nonce.to_hex());
        pt.set_valid_until_block(self.block_limit);
        pt.set_data(self.data.clone());
        pt.set_quota(self.gas.to_hex());
        match self.action {
            Action::Create => pt.clear_to(),
            Action::Call(ref to) => pt.set_to(to.hex()),
            Action::Store => pt.set_to(STORE_ADDRESS.into()),
        }
        pt
    }
}

/// Signed transaction information without verified signature.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UnverifiedTransaction {
    /// Plain Transaction.
    unsigned: Transaction,
    /// The signature
    signature: Signature,
    /// The Crypto Type
    crypto_type: CryptoType,
    /// Hash of the transaction
    hash: H256,
}

impl Deref for UnverifiedTransaction {
    type Target = Transaction;

    fn deref(&self) -> &Self::Target {
        &self.unsigned
    }
}

impl DerefMut for UnverifiedTransaction {
    fn deref_mut(&mut self) -> &mut Transaction {
        &mut self.unsigned
    }
}

impl Decodable for UnverifiedTransaction {
    fn decode(d: &UntrustedRlp) -> Result<Self, DecoderError> {
        if d.item_count()? != 4 {
            return Err(DecoderError::RlpIncorrectListLen);
        }
        Ok(UnverifiedTransaction {
               unsigned: d.val_at(0)?,
               signature: d.val_at(1)?,
               crypto_type: d.val_at(2)?,
               hash: d.val_at(3)?,
           })
    }
}

impl Encodable for UnverifiedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.rlp_append_sealed_transaction(s)
    }
}

impl UnverifiedTransaction {
    fn new(utx: &ProtoUnverifiedTransaction, hash: H256) -> Result<Self, Error> {

        if utx.get_signature().len() != SIGNATURE_BYTES_LEN {
            return Err(Error::InvalidSignature);
        }

        Ok(UnverifiedTransaction {
               unsigned: Transaction::new(utx.get_transaction())?,
               signature: Signature::from(H768::from(utx.get_signature())),
               crypto_type: CryptoType::from(utx.get_crypto()),
               hash: hash,
           })
    }

    /// Append object with a signature into RLP stream
    fn rlp_append_sealed_transaction(&self, s: &mut RlpStream) {
        s.begin_list(4);
        s.append(&self.unsigned);
        s.append(&self.signature);
        s.append(&self.crypto_type);
        s.append(&self.hash);
    }

    ///	Reference to unsigned part of this transaction.
    pub fn as_unsigned(&self) -> &Transaction {
        &self.unsigned
    }

    pub fn hash(&self) -> H256 {
        self.hash
    }

    /// get protobuf unverified transaction
    pub fn proto_unverified(&self) -> ProtoUnverifiedTransaction {
        let mut untx = ProtoUnverifiedTransaction::new();
        let tx = self.unsigned.proto_transaction();

        untx.set_transaction(tx);
        untx.set_signature(self.signature.to_vec());

        match self.crypto_type {
            CryptoType::SECP => untx.set_crypto(ProtoCrypto::SECP),
            CryptoType::SM2 => untx.set_crypto(ProtoCrypto::SM2),
        }
        untx
    }
}

/// A `UnverifiedTransaction` with successfully recovered `sender`.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SignedTransaction {
    transaction: UnverifiedTransaction,
    sender: Address,
    public: Public,
}

/// RLP dose not support struct nesting well
impl Decodable for SignedTransaction {
    fn decode(d: &UntrustedRlp) -> Result<Self, DecoderError> {
        if d.item_count()? != 11 {
            return Err(DecoderError::RlpIncorrectListLen);
        }

        let public: H256 = d.val_at(10)?;

        Ok(SignedTransaction {
               transaction: UnverifiedTransaction {
                   unsigned: Transaction {
                       nonce: d.val_at(0)?,
                       gas_price: d.val_at(1)?,
                       gas: d.val_at(2)?,
                       action: d.val_at(3)?,
                       value: d.val_at(4)?,
                       data: d.val_at(5)?,
                       block_limit: d.val_at(6)?,
                   },
                   signature: d.val_at(7)?,
                   crypto_type: d.val_at(8)?,
                   hash: d.val_at(9)?,
               },
               sender: pubkey_to_address(&public),
               public: public,
           })
    }
}

/// RLP dose not support struct nesting well
impl Encodable for SignedTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(11);

        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.action);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.block_limit);

        s.append(&self.signature);
        s.append(&self.crypto_type);
        s.append(&self.hash);
        //TODO: remove it
        s.append(&self.public);
    }
}

impl HeapSizeOf for SignedTransaction {
    fn heap_size_of_children(&self) -> usize {
        self.transaction.heap_size_of_children()
    }
}

impl Deref for SignedTransaction {
    type Target = UnverifiedTransaction;
    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl DerefMut for SignedTransaction {
    fn deref_mut(&mut self) -> &mut UnverifiedTransaction {
        &mut self.transaction
    }
}

impl SignedTransaction {
    /// Try to verify transaction and recover sender.
    pub fn new(stx: &ProtoSignedTransaction) -> Result<Self, Error> {
        if stx.get_tx_hash().len() != HASH_BYTES_LEN {
            return Err(Error::InvalidHash);
        }

        if stx.get_signer().len() != PUBKEY_BYTES_LEN {
            return Err(Error::InvalidPubKey);
        }

        let tx_hash = H256::from(stx.get_tx_hash());
        let public = H256::from_slice(stx.get_signer());
        let sender = pubkey_to_address(&public);
        Ok(SignedTransaction {
               transaction: UnverifiedTransaction::new(stx.get_transaction_with_sig(), tx_hash)?,
               sender: sender,
               public: public,
           })
    }

    /// Returns transaction sender.
    pub fn sender(&self) -> &Address {
        &self.sender
    }

    /// Returns a public key of the sender.
    pub fn public_key(&self) -> &Public {
        &self.public
    }

    ///get protobuf of signed transaction
    pub fn protobuf(&self) -> ProtoSignedTransaction {
        let mut stx = ProtoSignedTransaction::new();
        let utx = self.transaction.proto_unverified();
        stx.set_transaction_with_sig(utx);
        stx.set_tx_hash(self.hash().to_vec());
        stx.set_signer(self.public.to_vec());
        stx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rlp;

    #[test]
    fn test_encode_and_decode() {
        let mut stx = SignedTransaction::default();
        stx.data = vec![1; 200];
        let stx_rlp = rlp::encode(&stx);
        let stx: SignedTransaction = rlp::decode(&stx_rlp);
        let stx_encoded = rlp::encode(&stx).into_vec();

        assert_eq!(stx_rlp, stx_encoded);
    }

    #[test]
    fn test_protobuf() {
        let mut stx = SignedTransaction::default();
        stx.gas = U256::from(u64::max_value() / 100000);
        let stx_rlp = rlp::encode(&stx);
        let stx_proto = stx.protobuf();
        let stx = SignedTransaction::new(&stx_proto).unwrap();
        let stx_encoded = rlp::encode(&stx).into_vec();
        let stx: SignedTransaction = rlp::decode(&stx_encoded);
        let stx_encoded = rlp::encode(&stx).into_vec();

        assert_eq!(stx_rlp, stx_encoded);
    }
}
