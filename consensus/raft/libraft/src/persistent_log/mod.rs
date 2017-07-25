//! The persistent storage of Raft state.
//!
//! In your consuming application you will want to implement this trait on one of your structures.
//! This could adapt to a database, a file, or even just POD.
//!
//! *Note:* Your consuming application should not necessarily interface with this data. It is meant
//! for internal use by the library, we simply chose not to be opinionated about how data is stored.
mod mem;

use std::error;
use std::fmt::Debug;
use std::result;

pub use persistent_log::mem::{MemLog, Error};

use LogIndex;
use Term;
use ServerId;

/// A store of persistent Raft state.
pub trait Log: Clone + Debug + Send + 'static {
    type Error: error::Error + Debug + Sized + 'static;

    /// Returns the latest known term.
    fn current_term(&self) -> result::Result<Term, Self::Error>;

    /// Sets the current term to the provided value. The provided term must be greater than
    /// the current term. The `voted_for` value will be reset`.
    fn set_current_term(&mut self, term: Term) -> result::Result<(), Self::Error>;

    /// Increment the current term. The `voted_for` value will be reset.
    fn inc_current_term(&mut self) -> result::Result<Term, Self::Error>;

    /// Returns the candidate id of the candidate voted for in the current term (or none).
    fn voted_for(&self) -> result::Result<Option<ServerId>, Self::Error>;

    /// Sets the candidate id voted for in the current term.
    fn set_voted_for(&mut self, server: ServerId) -> result::Result<(), Self::Error>;

    /// Returns the index of the latest persisted log entry (0 if the log is empty).
    fn latest_log_index(&self) -> result::Result<LogIndex, Self::Error>;

    /// Returns the term of the latest persisted log entry (0 if the log is empty).
    fn latest_log_term(&self) -> result::Result<Term, Self::Error>;

    /// Returns the entry at the provided log index.
    fn entry(&self, index: LogIndex) -> result::Result<(Term, &[u8]), Self::Error>;

    /// Returns the given range of entries (excluding the right endpoint).
    fn entries(&self,
               lo: LogIndex,
               hi: LogIndex)
               -> result::Result<Vec<(Term, &[u8])>, Self::Error> {
        // TODO: can make LogIndex compatible for use in ranges.
        (lo.as_u64()..hi.as_u64())
            .map(|index| self.entry(LogIndex::from(index)))
            .collect::<Result<_, _>>()
    }


    /// Appends the provided entries to the log beginning at the given index.
    fn append_entries(&mut self,
                      from: LogIndex,
                      entries: &[(Term, &[u8])])
                      -> result::Result<(), Self::Error>;
}
