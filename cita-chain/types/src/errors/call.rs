use super::execution::ExecutionError;
use std::fmt;

#[derive(Debug)]
pub enum CallError {
    /// Couldn't find the transaction in the chain.
    TransactionNotFound,
    /// Couldn't find requested block's state in the chain.
    StatePruned,
    /// Couldn't find an amount of gas that didn't result in an exception.
    Exceptional,
    /// Corrupt state.
    StateCorrupt,
    /// Error executing.
    Execution(ExecutionError),
}

impl fmt::Display for CallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CallError::*;

        let msg = match *self {
            TransactionNotFound => "Transaction couldn't be found in the chain".into(),
            StatePruned => "Couldn't find the transaction block's state in the chain".into(),
            Exceptional => "An exception happened in the execution".into(),
            StateCorrupt => "Stored state found to be corrupted.".into(),
            Execution(ref e) => format!("{}", e),
        };

        f.write_fmt(format_args!("Transaction execution error ({}).", msg))
    }
}
