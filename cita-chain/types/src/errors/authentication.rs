use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum AuthenticationError {
    NoTransactionPermission,
    NoContractPermission,
    NoCallPermission,
    InvalidTransaction,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            AuthenticationError::NoTransactionPermission => "No transaction permission.".to_owned(),
            AuthenticationError::NoContractPermission => "No create contract permision.".to_owned(),
            AuthenticationError::NoCallPermission => "No contract call permission.".to_owned(),
            AuthenticationError::InvalidTransaction => "Invalid transaction.".to_owned(),
        };
        write!(f, "{}", printable)
    }
}
