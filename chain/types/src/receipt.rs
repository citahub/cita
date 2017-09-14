// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Receipt


use BlockNumber;
use log_entry::{LogBloom, LogEntry, LocalizedLogEntry};
use rlp::*;
use util::{H256, U256, Address};
use util::HeapSizeOf;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ReceiptError {
    OutOfGas,
    NoTransactionPermission,
    NoContractPermission,
}

impl ReceiptError {
    /// Returns human-readable description
    pub fn description(&self) -> String {
        let desc = match *self {
            ReceiptError::OutOfGas => "Out of gas.",
            ReceiptError::NoTransactionPermission => "No transaction permission.",
            ReceiptError::NoContractPermission => "No contract permission.",
        };
        desc.to_string()
    }
}

impl Decodable for ReceiptError {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        match rlp.as_val::<u8>()? {
            0 => Ok(ReceiptError::OutOfGas),
            1 => Ok(ReceiptError::NoTransactionPermission),
            2 => Ok(ReceiptError::NoContractPermission),
            _ => Err(DecoderError::Custom("Unknown Receipt error.")),
        }
    }
}

impl Encodable for ReceiptError {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&(*self as u8));
    }
}

/// Information describing execution of a transaction.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    /// The state root after executing the transaction. Optional since EIP98
    pub state_root: Option<H256>,
    /// The total gas used in the block following execution of the transaction.
    pub gas_used: U256,
    /// The OR-wide combination of all logs' blooms for this transaction.
    pub log_bloom: LogBloom,
    /// The logs stemming from this transaction.
    pub logs: Vec<LogEntry>,
    /// Transaction transact error
    pub error: Option<ReceiptError>,
}

impl Receipt {
    /// Create a new receipt.
    pub fn new(state_root: Option<H256>, gas_used: U256, logs: Vec<LogEntry>, error: Option<ReceiptError>) -> Receipt {
        Receipt {
            state_root: state_root,
            gas_used: gas_used,
            log_bloom: logs.iter().fold(LogBloom::default(), |mut b, l| {
                b = &b | &l.bloom();
                b
            }), //TODO: use |= operator
            logs: logs,
            error: error,
        }
    }
}

impl Encodable for Receipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        if let Some(ref root) = self.state_root {
            s.begin_list(4);
            s.append(root);
        } else {
            s.begin_list(3);
        }
        s.append(&self.gas_used);
        s.append(&self.log_bloom);
        s.append_list(&self.logs);
        s.append(&self.error);
    }
}

impl Decodable for Receipt {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        if rlp.item_count()? == 3 {
            Ok(Receipt {
                   state_root: None,
                   gas_used: rlp.val_at(0)?,
                   log_bloom: rlp.val_at(1)?,
                   logs: rlp.list_at(2)?,
                   error: rlp.val_at(3)?,
               })
        } else {
            Ok(Receipt {
                   state_root: Some(rlp.val_at(0)?),
                   gas_used: rlp.val_at(1)?,
                   log_bloom: rlp.val_at(2)?,
                   logs: rlp.list_at(3)?,
                   error: rlp.val_at(4)?,
               })
        }
    }
}

impl HeapSizeOf for Receipt {
    fn heap_size_of_children(&self) -> usize {
        self.logs.heap_size_of_children()
    }
}

/// Receipt with additional info.
#[derive(Debug, Clone, PartialEq)]
pub struct RichReceipt {
    /// Transaction hash.
    pub transaction_hash: H256,
    /// Transaction index.
    pub transaction_index: usize,
    /// The total gas used in the block following execution of the transaction.
    pub cumulative_gas_used: U256,
    /// The gas used in the execution of the transaction. Note the difference of meaning to `Receipt::gas_used`.
    pub gas_used: U256,
    /// Contract address.
    pub contract_address: Option<Address>,
    /// Logs
    pub logs: Vec<LogEntry>,
    /// Logs bloom
    pub log_bloom: LogBloom,
    /// State root
    pub state_root: Option<H256>,
    /// Receipt error
    pub error: Option<ReceiptError>,
}

/// Receipt with additional info.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalizedReceipt {
    /// Transaction hash.
    pub transaction_hash: H256,
    /// Transaction index.
    pub transaction_index: usize,
    /// Block hash.
    pub block_hash: H256,
    /// Block number.
    pub block_number: BlockNumber,
    /// The total gas used in the block following execution of the transaction.
    pub cumulative_gas_used: U256,
    /// The gas used in the execution of the transaction. Note the difference of meaning to `Receipt::gas_used`.
    pub gas_used: U256,
    /// Contract address.
    pub contract_address: Option<Address>,
    /// Logs
    pub logs: Vec<LocalizedLogEntry>,
    /// Logs bloom
    pub log_bloom: LogBloom,
    /// State root
    pub state_root: Option<H256>,
    /// Receipt error
    pub error: Option<ReceiptError>,
}

#[cfg(test)]
mod tests {
    extern crate rustc_hex;

    use self::rustc_hex::{ToHex, FromHex};
    use super::Receipt;
    use log_entry::LogEntry;
    use util::hashable::HASH_NAME;

    #[test]
    fn test_no_state_root() {
        let mut expected = Vec::new();
        if HASH_NAME == "sha3" {
            expected = FromHex::from_hex("f9014183040caeb9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000c0").unwrap();
        } else if HASH_NAME == "blake2b" {
            expected = FromHex::from_hex("f9014183040caeb9010000000000000000000000000040000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000c0").unwrap();
        }
        let r = Receipt::new(
            None,
            0x40cae.into(),
            vec![
                LogEntry {
                    address: "dcf421d093428b096ca501a7cd1a740855a7976f".into(),
                    topics: vec![],
                    data: vec![0u8; 32],
                },
            ],
            None,
        );
        let s = ToHex::to_hex(&::rlp::encode(&r)[..]);
        println!("test_no_state_root {:?}", s);
        assert_eq!(&::rlp::encode(&r)[..], &expected[..]);
    }

    #[test]
    fn test_basic() {
        let mut expected = Vec::new();
        if HASH_NAME == "sha3" {
            expected = FromHex::from_hex("f90162a02f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee83040caeb9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000c0").unwrap();
        } else if HASH_NAME == "blake2b" {
            expected = FromHex::from_hex("f90162a02f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee83040caeb9010000000000000000000000000040000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f838f794dcf421d093428b096ca501a7cd1a740855a7976fc0a00000000000000000000000000000000000000000000000000000000000000000c0").unwrap();
        }
        let r = Receipt::new(
            Some("2f697d671e9ae4ee24a43c4b0d7e15f1cb4ba6de1561120d43b9a4e8c4a8a6ee".into()),
            0x40cae.into(),
            vec![
                LogEntry {
                    address: "dcf421d093428b096ca501a7cd1a740855a7976f".into(),
                    topics: vec![],
                    data: vec![0u8; 32],
                },
            ],
            None,
        );
        let encoded = ::rlp::encode(&r);
        let slice: &[u8] = &encoded;
        println!("test_basic {:?}", ToHex::to_hex(&slice));
        assert_eq!(&encoded[..], &expected[..]);
        let decoded: Receipt = ::rlp::decode(&encoded);
        assert_eq!(decoded, r);
    }
}
