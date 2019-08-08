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

use super::util::OutOfBounds;
use cita_types::U256;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Errors concerning transaction processing.
pub enum TransactionError {
    /// Transaction is already imported to the queue
    AlreadyImported,
    /// Transaction is not valid anymore (state already has higher nonce)
    Old,
    /// Transaction has too low fee
    /// (there is already a transaction with the same sender-nonce but higher gas price)
    TooCheapToReplace,
    /// Transaction was not imported to the queue because limit has been reached.
    LimitReached,
    /// Transaction's gas price is below threshold.
    InsufficientGasPrice {
        /// Minimal expected gas price
        minimal: U256,
        /// Transaction gas price
        got: U256,
    },
    /// Transaction's gas is below currently set minimal gas requirement.
    InsufficientGas {
        /// Minimal expected gas
        minimal: U256,
        /// Transaction gas
        got: U256,
    },
    /// Sender doesn't have enough funds to pay for this transaction
    InsufficientBalance {
        /// Senders balance
        balance: U256,
        /// Transaction cost
        cost: U256,
    },
    /// Transactions gas is higher then current gas limit
    GasLimitExceeded {
        /// Current gas limit
        limit: U256,
        /// Declared transaction gas
        got: U256,
    },
    /// Transaction's gas limit (aka gas) is invalid.
    InvalidGasLimit(OutOfBounds<U256>),
    /// Transaction sender is banned.
    SenderBanned,
    /// Transaction receipient is banned.
    RecipientBanned,
    /// Contract creation code is banned.
    CodeBanned,
    /// Invalid network ID given.
    InvalidNetworkId,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TransactionError::*;
        let msg = match *self {
            AlreadyImported => "Already imported".into(),
            Old => "No longer valid".into(),
            TooCheapToReplace => "Gas price too low to replace".into(),
            LimitReached => "Transaction limit reached".into(),
            InsufficientGasPrice { minimal, got } => {
                format!("Insufficient gas price. Min={}, Given={}", minimal, got)
            }
            InsufficientGas { minimal, got } => {
                format!("Insufficient gas. Min={}, Given={}", minimal, got)
            }
            InsufficientBalance { balance, cost } => format!(
                "Insufficient balance for transaction. Balance={}, Cost={}",
                balance, cost
            ),
            GasLimitExceeded { limit, got } => {
                format!("Gas limit exceeded. Limit={}, Given={}", limit, got)
            }
            InvalidGasLimit(ref err) => format!("Invalid gas limit. {}", err),
            SenderBanned => "Sender is temporarily banned.".into(),
            RecipientBanned => "Recipient is temporarily banned.".into(),
            CodeBanned => "Contract code is temporarily banned.".into(),
            InvalidNetworkId => {
                "Transaction of this network ID is not allowed on this chain.".into()
            }
        };

        f.write_fmt(format_args!("Transaction error ({})", msg))
    }
}
