use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPrivKey,
    InvalidPubKey,
    InvalidMessage,
    InvalidSignature,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            Error::InvalidPrivKey => "Invalid Private Key",
            Error::InvalidPubKey => "Invalid Public Key",
            Error::InvalidMessage => "Invalid Message",
            Error::InvalidSignature => "Invalid Signature",
        };
        f.write_fmt(format_args!("Crypto error: {}", message))
    }
}
