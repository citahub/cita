use util::*;

/// State backend. See module docs for more details.
pub trait Backend: Send {
    /// Treat the backend as a read-only hashdb.
    fn as_hashdb(&self) -> &HashDB;

    /// Treat the backend as a writeable hashdb.
    fn as_hashdb_mut(&mut self) -> &mut HashDB;
}
