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
#![rustfmt_skip]

use BlockNumber;
use log_entry::{LocalizedLogEntry, LogBloom, LogEntry};
use rlp::*;
use util::{Address, H256, U256, Bytes};
use util::HeapSizeOf;
use std::str::FromStr;
use libproto::executor::{Receipt as ProtoReceipt, ReceiptError as ProtoReceiptError, ReceiptErrorWithOption, StateRoot};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
pub enum ReceiptError {
    //ExecutionError
    NoTransactionPermission,
    NoContractPermission,
    NotEnoughBaseGas,
    BlockGasLimitReached,
    AccountGasLimitReached,
    //EVM error(chain/core/src/evm/evm.rs)
    OutOfGas,
    BadJumpDestination,
    BadInstruction,
    StackUnderflow,
    OutOfStack,
    Internal,
}

impl ReceiptError {
    /// Returns human-readable description
    pub fn description(&self) -> String {
        let desc = match *self {
            ReceiptError::NoTransactionPermission => "No transaction permission.",
            ReceiptError::NoContractPermission => "No contract permission.",
            ReceiptError::NotEnoughBaseGas => "Not enough base gas.",
            ReceiptError::BlockGasLimitReached => "Block gas limit reached.",
            ReceiptError::AccountGasLimitReached => "Account gas limit reached.",
            ReceiptError::OutOfGas => "Out of gas.",
            ReceiptError::BadJumpDestination => "Jump position wasn't marked with JUMPDEST instruction.",
            ReceiptError::BadInstruction => "Instruction is not supported.",
            ReceiptError::StackUnderflow => "Not enough stack elements to execute instruction.",
            ReceiptError::OutOfStack => "Execution would exceed defined Stack Limit.",
            ReceiptError::Internal => "EVM internal error.",
        };
        desc.to_string()
    }

    pub fn protobuf(&self) -> ProtoReceiptError {
        match *self {
            ReceiptError::NoTransactionPermission => ProtoReceiptError::NoTransactionPermission,
            ReceiptError::NoContractPermission => ProtoReceiptError::NoContractPermission,
            ReceiptError::NotEnoughBaseGas => ProtoReceiptError::NotEnoughBaseGas,
            ReceiptError::BlockGasLimitReached => ProtoReceiptError::BlockGasLimitReached,
            ReceiptError::AccountGasLimitReached => ProtoReceiptError::AccountGasLimitReached,
            ReceiptError::OutOfGas => ProtoReceiptError::OutOfGas,
            ReceiptError::BadJumpDestination => ProtoReceiptError::BadJumpDestination,
            ReceiptError::BadInstruction => ProtoReceiptError::BadInstruction,
            ReceiptError::StackUnderflow => ProtoReceiptError::StackUnderflow,
            ReceiptError::OutOfStack => ProtoReceiptError::OutOfStack,
            ReceiptError::Internal => ProtoReceiptError::Internal,
        }
    }
    
    fn from_proto(receipt_error: ProtoReceiptError) -> Self {
        match receipt_error {
            ProtoReceiptError::NoTransactionPermission => ReceiptError::NoTransactionPermission,
            ProtoReceiptError::NoContractPermission => ReceiptError::NoContractPermission,
            ProtoReceiptError::NotEnoughBaseGas => ReceiptError::NotEnoughBaseGas,
            ProtoReceiptError::BlockGasLimitReached => ReceiptError::BlockGasLimitReached,
            ProtoReceiptError::AccountGasLimitReached => ReceiptError::AccountGasLimitReached,
            ProtoReceiptError::OutOfGas => ReceiptError::OutOfGas,
            ProtoReceiptError::BadJumpDestination => ReceiptError::BadJumpDestination,
            ProtoReceiptError::BadInstruction => ReceiptError::BadInstruction,
            ProtoReceiptError::StackUnderflow => ReceiptError::StackUnderflow,
            ProtoReceiptError::OutOfStack => ReceiptError::OutOfStack,
            ProtoReceiptError::Internal => ReceiptError::Internal,
        }
    }

}

impl Decodable for ReceiptError {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        match rlp.as_val::<u8>()? {
            0 => Ok(ReceiptError::NoTransactionPermission),
            1 => Ok(ReceiptError::NoContractPermission),
            2 => Ok(ReceiptError::NotEnoughBaseGas),
            3 => Ok(ReceiptError::BlockGasLimitReached),
            4 => Ok(ReceiptError::AccountGasLimitReached),
            5 => Ok(ReceiptError::BadJumpDestination),
            6 => Ok(ReceiptError::BadInstruction),
            7 => Ok(ReceiptError::StackUnderflow),
            8 => Ok(ReceiptError::OutOfStack),
            9 => Ok(ReceiptError::Internal),
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
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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
    pub account_nonce: U256,
}

impl Receipt {
    /// Create a new receipt.
    pub fn new(state_root: Option<H256>, gas_used: U256, logs: Vec<LogEntry>, error: Option<ReceiptError>, account_nonce: U256) -> Receipt {
        Receipt {
            state_root: state_root,
            gas_used: gas_used,
            log_bloom: logs.iter().fold(LogBloom::default(), |mut b, l| {
                b = &b | &l.bloom();
                b
            }), //TODO: use |= operator
            logs: logs,
            error: error,
            account_nonce: account_nonce
        }
    }

    pub fn protobuf(&self) -> ProtoReceipt {
        let mut receipt_proto = ProtoReceipt::new();
        let mut state_root_option = StateRoot::new();
        let mut receipt_error_with_option = ReceiptErrorWithOption::new();


        if let Some(state_root) = self.state_root {
            state_root_option.set_state_root(state_root.to_vec());
            receipt_proto.set_state_root(state_root_option);
        }

        if let Some(error) = self.error {
            receipt_error_with_option.set_error(error.protobuf());
            receipt_proto.set_error(receipt_error_with_option);
        }

        receipt_proto.set_gas_used(self.gas_used.to_hex());
        receipt_proto.set_log_bloom(self.log_bloom.to_vec());
        receipt_proto.logs = self.logs
            .clone()
            .into_iter()
            .map(|log_entry| log_entry.protobuf())
            .collect();
        receipt_proto.set_account_nonce(self.account_nonce.as_u64());
        receipt_proto
    }
}

impl From<ProtoReceipt> for Receipt {
    fn from(receipt: ProtoReceipt) -> Self {
        let mut state_root = None;
        if receipt.state_root.is_some() {
            state_root = Some(H256::from_slice(
                receipt.clone().take_state_root().get_state_root(),
            ));
        }

        let gas_used: U256 = U256::from_str(receipt.get_gas_used()).unwrap();
        let account_nonce: U256 = U256::from(receipt.get_account_nonce());
        let mut error = None;

        let logs = receipt.get_logs().into_iter().map(|log_entry|{
            let address: Address = Address::from_slice(log_entry.get_address());
            let topics: Vec<H256> = log_entry.get_topics().into_iter().map(|topic|{H256::from_slice(topic)}).collect();
            let data: Bytes = Bytes::from(log_entry.get_data());
            LogEntry {
                address: address,
                topics: topics,
                data: data,
            }
        }).collect();

        if receipt.error.is_some() {
            error = Some(ReceiptError::from_proto(receipt.clone().take_error().get_error()));
        }

        Receipt::new(state_root, gas_used, logs, error, account_nonce)
    }
}

impl Encodable for Receipt {
    fn rlp_append(&self, s: &mut RlpStream) {
        if let Some(ref root) = self.state_root {
            s.begin_list(6);
            s.append(root);
        } else {
            s.begin_list(5);
        }
        s.append(&self.gas_used);
        s.append(&self.log_bloom);
        s.append_list(&self.logs);
        s.append(&self.error);
        s.append(&self.account_nonce);
    }
}

impl Decodable for Receipt {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        if rlp.item_count()? == 5 {
            Ok(Receipt {
                   state_root: None,
                   gas_used: rlp.val_at(0)?,
                   log_bloom: rlp.val_at(1)?,
                   logs: rlp.list_at(2)?,
                   error: rlp.val_at(3)?,
                   account_nonce: rlp.val_at(4)?,
               })
        } else {
            Ok(Receipt {
                   state_root: Some(rlp.val_at(0)?),
                   gas_used: rlp.val_at(1)?,
                   log_bloom: rlp.val_at(2)?,
                   logs: rlp.list_at(3)?,
                   error: rlp.val_at(4)?,
                   account_nonce: rlp.val_at(5)?,
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
    use super::*;
    use log_entry::LogEntry;

    #[test]
    fn test_no_state_root() {
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
            1.into(),
        );
        let encoded = ::rlp::encode(&r);
        println!("encode ok");
        let decoded: Receipt = ::rlp::decode(&encoded);
        println!("decoded: {:?}", decoded);
        assert_eq!(decoded, r);

    }

    #[test]
    fn test_basic() {
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
            1.into(),
        );
        let encoded = ::rlp::encode(&r);
        let decoded: Receipt = ::rlp::decode(&encoded);
        println!("decoded: {:?}", decoded);
        assert_eq!(decoded, r);
    }

    #[test]
    fn test_with_error() {
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
            Some(ReceiptError::NoTransactionPermission),
            1.into(),
        );
        let encoded = ::rlp::encode(&r);
        let decoded: Receipt = ::rlp::decode(&encoded);
        println!("decoded: {:?}", decoded);
        assert_eq!(decoded, r);
    }
}
