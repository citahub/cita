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

//! General error types for use in ethcore.

use std::fmt;

use cita_types::{U256, U512};

/// Result of executing the transaction.
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "ipc", binary)]
pub enum ExecutionError {
    /// Returned when there gas paid for transaction execution is
    /// lower than base gas required.
    NotEnoughBaseGas {
        /// Absolute minimum gas required.
        required: U256,
        /// Gas provided.
        got: U256,
    },
    /// Returned when block (quota_used + gas) > quota_limit.
    ///
    /// If gas =< quota_limit, upstream may try to execute the transaction
    /// in next block.
    BlockGasLimitReached {
        /// Gas limit of block for transaction.
        quota_limit: U256,
        /// Gas used in block prior to transaction.
        quota_used: U256,
        /// Amount of gas in block.
        gas: U256,
    },
    AccountGasLimitReached {
        /// Account Gas limit left
        quota_limit: U256,
        /// Amount of gas in transaction
        gas: U256,
    },
    /// Returned when transaction nonce does not match state nonce.
    InvalidNonce {
        /// Nonce expected.
        expected: U256,
        /// Nonce found.
        got: U256,
    },
    /// Returned when cost of transaction (value + gas_price * gas) exceeds
    /// current sender balance.
    NotEnoughCash {
        /// Minimum required balance.
        required: U512,
        /// Actual balance.
        got: U512,
    },
    NoTransactionPermission,
    NoContractPermission,
    NoCallPermission,
    /// Returned when internal evm error occurs.
    ExecutionInternal(String),
    /// Returned when generic transaction occurs
    TransactionMalformed(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ExecutionError::*;

        let msg = match *self {
            NotEnoughBaseGas { ref required, ref got } => format!("Not enough base quota. {} is required, but only {} paid", required, got),
            BlockGasLimitReached {
                ref quota_limit,
                ref quota_used,
                ref gas,
            } => format!("Block gas limit reached. The limit is {}, {} has already been used, and {} more is required", quota_limit, quota_used, gas),
            AccountGasLimitReached { ref quota_limit, ref gas } => format!("Account gas limit reached. The limit is {}, {} more is required", quota_limit, gas),
            InvalidNonce { ref expected, ref got } => format!("Invalid transaction nonce: expected {}, found {}", expected, got),
            NotEnoughCash { ref required, ref got } => format!("Cost of transaction exceeds sender balance. {} is required but the sender only has {}", required, got),
            ExecutionInternal(ref msg) => msg.clone(),
            TransactionMalformed(ref err) => format!("Malformed transaction: {}", err),
            NoTransactionPermission => "No transaction permission".to_owned(),
            NoContractPermission => "No contract permission".to_owned(),
            NoCallPermission => "No call contract permission".to_owned(),
        };

        f.write_fmt(format_args!("Transaction execution error ({}).", msg))
    }
}
