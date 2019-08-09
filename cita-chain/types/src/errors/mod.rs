mod execution;
mod receipt;

pub use execution::ExecutionError;
pub use receipt::ReceiptError;

#[derive(Debug)]
pub enum Error {
    Execution(ExecutionError),
    Receipt(ReceiptError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err = match self {
            Error::Execution(ref err) => format!("Execution error {:?}", err),
            Error::Receipt(ref err) => format!("Receipt error {:?}", err),
        };
        write!(f, "{}", err)
    }
}

impl From<ExecutionError> for Error {
    fn from(err: ExecutionError) -> Error {
        Error::Execution(err)
    }
}

impl From<ReceiptError> for Error {
    fn from(err: ReceiptError) -> Error {
        Error::Receipt(err)
    }
}
