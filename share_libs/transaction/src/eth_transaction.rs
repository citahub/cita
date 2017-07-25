use std::ops::Deref;
use util::{H256, Address, U256, Bytes, HeapSizeOf, H520};
use crypto::{Signature, Public, recover, pubkey_to_address, Error as EthkeyError};
use libproto::blockchain::{Transaction as ProtoTransaction, SignedTransaction as ProtoSignedTransaction, Crypto, Content};
use protobuf::parse_from_bytes;
use util::FixedHash;
use std::str::FromStr;
use error::Error;
use serde_types::hash::H256 as Hash256;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Transaction action type.
pub enum Action {
    /// Create creates new contract.
    Create,
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(Address),
}

impl Default for Action {
    fn default() -> Action { Action::Create }
}

/// A set of information describing an externally-originating message call
/// or contract creation operation.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct EthTransaction {
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
    pub hash: H256
}

impl HeapSizeOf for EthTransaction {
    fn heap_size_of_children(&self) -> usize {
        self.data.heap_size_of_children()
    }
}

// TODO: refactor transaction in protobuf,
// now using the same type `ProtoTransaction`,
// it's not a good design.
impl EthTransaction {
	pub fn new(plain_transaction: ProtoTransaction) -> Result<Self, Error> {
		if let Ok(content) = parse_from_bytes::<Content>(plain_transaction.get_content()) {
			let to = plain_transaction.get_to();
			let nonce = content.nonce.parse::<u32>().map_err(|_| Error::ParseError )?;
			Ok(EthTransaction {
				nonce: nonce.into(),
				gas_price: U256::default(),
				gas: U256::from(content.gas),
				action: if to.is_empty() { Action::Create } else { Action::Call(Address::from_str(plain_transaction.get_to()).map_err(|_| Error::ParseError )?) },
				value: U256::default(),
				data: content.data.into(),
				hash: plain_transaction.sha3()
			})
		} else {
			Err(Error::ParseError)
		}
	}

	// Specify the sender; this won't survive the serialize/deserialize process, but can be cloned.
	pub fn fake_sign(self, from: Address) -> SignedTransaction {
		let signature = Signature::from_rsv(&Hash256::default(), &Hash256::default(), 0);
		SignedTransaction {
			transaction: UnverifiedTransaction {
				unsigned: self,
				signature: signature,
				hash_with_signature: 0.into(),
				hash_without_signature: 0.into()
			},
			sender: from,
			public: Public::default(),
		}
	}

	pub fn sha3(&self) -> H256 {
		self.hash
	}
}

/// Signed transaction information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnverifiedTransaction {
    /// Plain Transaction.
    unsigned: EthTransaction,
    signature: Signature,
    hash_with_signature: H256,
    hash_without_signature: H256
}

impl Deref for UnverifiedTransaction {
    type Target = EthTransaction;

    fn deref(&self) -> &Self::Target {
        &self.unsigned
    }
}

impl UnverifiedTransaction {
    pub fn new(tx: ProtoTransaction) -> Option<Self> {
        let hash_with_signature = tx.sha3();
        if let Ok(proto_signed_tx) = parse_from_bytes::<ProtoSignedTransaction>(tx.get_content()) {
            let crypto = proto_signed_tx.get_crypto();
            match crypto {
                Crypto::SECP => {
                    if let Ok(plain_transaction) = parse_from_bytes::<ProtoTransaction>(&proto_signed_tx.get_transaction()) {
                        let signature: Signature = H520::from_slice(proto_signed_tx.get_signature()).0.into();
                        if let Ok(eth_transaction) = EthTransaction::new(plain_transaction.clone()) {
                            let hash_without_signature = plain_transaction.sha3();
                            return Some(UnverifiedTransaction {
                                unsigned: eth_transaction,
                                signature: signature,
                                hash_without_signature: hash_without_signature,
                                hash_with_signature: hash_with_signature
                            });
                        }
                    }
                }
                _ => { return None },
            }
        }
        return None;
    }

    /// Recovers the public key of the sender.
    pub fn recover_public(&self) -> Result<Public, EthkeyError> {
        Ok(recover(&self.signature(), &self.hash_without_signature().into())?)
    }

    ///	Reference to unsigned part of this transaction.
    pub fn as_unsigned(&self) -> &EthTransaction {
        &self.unsigned
    }

    /// Get the hash transaction.
    pub fn hash(&self) -> H256 {
        self.hash_with_signature
    }

    /// Get the hash of plain transaction
    pub fn hash_without_signature(&self) -> H256 {
        self.hash_without_signature
    }

    pub fn signature(&self) -> Signature {
        self.signature.clone()
    }
}

/// A `UnverifiedTransaction` with successfully recovered `sender`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignedTransaction {
    transaction: UnverifiedTransaction,
    sender: Address,
    public: Public,
}

impl HeapSizeOf for SignedTransaction {
    fn heap_size_of_children(&self) -> usize {
        self.transaction.unsigned.heap_size_of_children()
    }
}

impl Deref for SignedTransaction {
    type Target = UnverifiedTransaction;
    fn deref(&self) -> &Self::Target {
        &self.transaction
    }
}

impl From<SignedTransaction> for UnverifiedTransaction {
    fn from(tx: SignedTransaction) -> Self {
        tx.transaction
    }
}

// TODO: 验证 From！！！
impl SignedTransaction {
    /// Try to verify transaction and recover sender.
    // TODO: Verify sender and pubkey
    pub fn new(transaction: UnverifiedTransaction) -> Result<Self, EthkeyError> {
        let public = transaction.recover_public()?;
        let sender = pubkey_to_address(&public);
        Ok(SignedTransaction {
            transaction: transaction,
            sender: sender.into(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use protobuf::Message;
    use crypto::{sign, KeyPair, PrivKey};

    #[test]
    fn test_transaction() {
        // Content
        let nonce = U256::from(3);
        let mut content = Content::new();
        content.set_nonce(String::from("3"));
        content.set_data(vec![10, 10]);
        content.set_gas(10001);

        // Protobuf Plain Transaction
        let address = "ef2d6d194084c2de36e0dabfce45d046b37d1106";
        let mut tx = ProtoTransaction::new();
        tx.set_to(address.into());
        tx.set_valid_until_block(4294967296u64);
        tx.set_content(content.write_to_bytes().unwrap());

        let privkey = PrivKey::from(H256::from_str("a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65").unwrap());
        let keypair = KeyPair::from_privkey(privkey.into()).unwrap();
        let message = tx.sha3();
        let signature = sign(keypair.privkey(), &message.into()).unwrap();

        // Protobuf SignedTransaction
        let mut stx = ProtoSignedTransaction::new();
        stx.set_transaction(tx.write_to_bytes().unwrap());
        stx.set_crypto(Crypto::SECP);
        stx.set_signature(signature.to_vec());

        tx.clear_content();
        tx.set_content(stx.write_to_bytes().unwrap());

        // UnverifiedTransaction
        let transaction = UnverifiedTransaction::new(tx.clone()).unwrap();
        assert_eq!(transaction.signature(), signature);
        assert_eq!(transaction.hash_without_signature(), message);
        assert_eq!(transaction.nonce, nonce);
        assert_eq!(transaction.data, vec![10, 10]);

        // SignedTransaction
        let signed_transaction = SignedTransaction::new(transaction).unwrap();
        assert_eq!(signed_transaction.hash(), tx.sha3());
        assert_eq!(&signed_transaction.public_key(), keypair.pubkey());
        assert_eq!(signed_transaction.gas, U256::from(10001));
    }
}
