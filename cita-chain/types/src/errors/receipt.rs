use libproto::executor::ReceiptError as ProtoReceiptError;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Eq)]
pub enum ReceiptError {
    // ExecutionError
    NotEnoughBaseQuota,
    BlockQuotaLimitReached,
    AccountQuotaLimitReached,
    InvalidNonce,
    NotEnoughCash,
    NoTransactionPermission,
    NoContractPermission,
    NoCallPermission,
    ExecutionInternal,
    TransactionMalformed,
    // EVM error
    OutOfQuota,
    BadJumpDestination,
    BadInstruction,
    StackUnderflow,
    OutOfStack,
    Internal,
    MutableCallInStaticContext,
    OutOfBounds,
    Reverted,
}

impl ReceiptError {
    /// Returns human-readable description
    pub fn description(self) -> String {
        let desc = match self {
            ReceiptError::NotEnoughBaseQuota => "Not enough base quota.",
            ReceiptError::BlockQuotaLimitReached => "Block quota limit reached.",
            ReceiptError::AccountQuotaLimitReached => "Account quota limit reached.",
            ReceiptError::InvalidNonce => "Invalid transaction nonce.",
            ReceiptError::NotEnoughCash => "Cost of transaction exceeds sender balance.",
            ReceiptError::NoTransactionPermission => "No transaction permission.",
            ReceiptError::NoContractPermission => "No contract permission.",
            ReceiptError::NoCallPermission => "No Call contract permission.",
            ReceiptError::ExecutionInternal => "Execution internal error.",
            ReceiptError::TransactionMalformed => "Malformed transaction.",
            ReceiptError::OutOfQuota => "Out of quota.",
            ReceiptError::BadJumpDestination => {
                "Jump position wasn't marked with JUMPDEST instruction."
            }
            ReceiptError::BadInstruction => "Instruction is not supported.",
            ReceiptError::StackUnderflow => "Not enough stack elements to execute instruction.",
            ReceiptError::OutOfStack => "Execution would exceed defined Stack Limit.",
            ReceiptError::Internal => "EVM internal error.",
            ReceiptError::MutableCallInStaticContext => "Mutable call in static context.",
            ReceiptError::OutOfBounds => "Out of bounds.",
            ReceiptError::Reverted => "Reverted.",
        };
        desc.to_string()
    }

    pub fn protobuf(self) -> ProtoReceiptError {
        match self {
            ReceiptError::NotEnoughBaseQuota => ProtoReceiptError::NotEnoughBaseQuota,
            ReceiptError::BlockQuotaLimitReached => ProtoReceiptError::BlockQuotaLimitReached,
            ReceiptError::AccountQuotaLimitReached => ProtoReceiptError::AccountQuotaLimitReached,
            ReceiptError::InvalidNonce => ProtoReceiptError::InvalidTransactionNonce,
            ReceiptError::NotEnoughCash => ProtoReceiptError::NotEnoughCash,
            ReceiptError::NoTransactionPermission => ProtoReceiptError::NoTransactionPermission,
            ReceiptError::NoContractPermission => ProtoReceiptError::NoContractPermission,
            ReceiptError::NoCallPermission => ProtoReceiptError::NoCallPermission,
            ReceiptError::ExecutionInternal => ProtoReceiptError::ExecutionInternal,
            ReceiptError::TransactionMalformed => ProtoReceiptError::TransactionMalformed,
            ReceiptError::OutOfQuota => ProtoReceiptError::OutOfQuota,
            ReceiptError::BadJumpDestination => ProtoReceiptError::BadJumpDestination,
            ReceiptError::BadInstruction => ProtoReceiptError::BadInstruction,
            ReceiptError::StackUnderflow => ProtoReceiptError::StackUnderflow,
            ReceiptError::OutOfStack => ProtoReceiptError::OutOfStack,
            ReceiptError::Internal => ProtoReceiptError::Internal,
            ReceiptError::MutableCallInStaticContext => {
                ProtoReceiptError::MutableCallInStaticContext
            }
            ReceiptError::OutOfBounds => ProtoReceiptError::OutOfBounds,
            ReceiptError::Reverted => ProtoReceiptError::Reverted,
        }
    }

    pub fn from_proto(receipt_error: ProtoReceiptError) -> Self {
        match receipt_error {
            ProtoReceiptError::NotEnoughBaseQuota => ReceiptError::NotEnoughBaseQuota,
            ProtoReceiptError::BlockQuotaLimitReached => ReceiptError::BlockQuotaLimitReached,
            ProtoReceiptError::AccountQuotaLimitReached => ReceiptError::AccountQuotaLimitReached,
            ProtoReceiptError::InvalidTransactionNonce => ReceiptError::InvalidNonce,
            ProtoReceiptError::NotEnoughCash => ReceiptError::NotEnoughCash,
            ProtoReceiptError::NoTransactionPermission => ReceiptError::NoTransactionPermission,
            ProtoReceiptError::NoContractPermission => ReceiptError::NoContractPermission,
            ProtoReceiptError::NoCallPermission => ReceiptError::NoCallPermission,
            ProtoReceiptError::ExecutionInternal => ReceiptError::ExecutionInternal,
            ProtoReceiptError::TransactionMalformed => ReceiptError::TransactionMalformed,
            ProtoReceiptError::OutOfQuota => ReceiptError::OutOfQuota,
            ProtoReceiptError::BadJumpDestination => ReceiptError::BadJumpDestination,
            ProtoReceiptError::BadInstruction => ReceiptError::BadInstruction,
            ProtoReceiptError::StackUnderflow => ReceiptError::StackUnderflow,
            ProtoReceiptError::OutOfStack => ReceiptError::OutOfStack,
            ProtoReceiptError::Internal => ReceiptError::Internal,
            ProtoReceiptError::MutableCallInStaticContext => {
                ReceiptError::MutableCallInStaticContext
            }
            ProtoReceiptError::OutOfBounds => ReceiptError::OutOfBounds,
            ProtoReceiptError::Reverted => ReceiptError::Reverted,
        }
    }
}

impl Decodable for ReceiptError {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        match rlp.as_val::<u8>()? {
            0 => Ok(ReceiptError::NotEnoughBaseQuota),
            1 => Ok(ReceiptError::BlockQuotaLimitReached),
            2 => Ok(ReceiptError::AccountQuotaLimitReached),
            3 => Ok(ReceiptError::InvalidNonce),
            4 => Ok(ReceiptError::NotEnoughCash),
            5 => Ok(ReceiptError::NoTransactionPermission),
            6 => Ok(ReceiptError::NoContractPermission),
            7 => Ok(ReceiptError::NoCallPermission),
            8 => Ok(ReceiptError::ExecutionInternal),
            9 => Ok(ReceiptError::TransactionMalformed),
            10 => Ok(ReceiptError::OutOfQuota),
            11 => Ok(ReceiptError::BadJumpDestination),
            12 => Ok(ReceiptError::BadInstruction),
            13 => Ok(ReceiptError::StackUnderflow),
            14 => Ok(ReceiptError::OutOfStack),
            15 => Ok(ReceiptError::Internal),
            16 => Ok(ReceiptError::MutableCallInStaticContext),
            17 => Ok(ReceiptError::OutOfBounds),
            18 => Ok(ReceiptError::Reverted),
            _ => Err(DecoderError::Custom("Unknown Receipt error.")),
        }
    }
}

impl Encodable for ReceiptError {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&(*self as u8));
    }
}
