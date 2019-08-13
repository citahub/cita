use std::fmt;

#[derive(Debug)]
pub enum NativeError {
    Internal(String),
}

impl fmt::Display for NativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            NativeError::Internal(str) => format!("Internal error {:?}", str),
        };
        write!(f, "{}", printable)
    }
}
