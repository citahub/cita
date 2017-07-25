use util::{JournalDB, DBTransaction, H256, UtilError, HashDB};
use state::backend::*;

pub struct StateDB {
    /// Backing database.
    db: Box<JournalDB>,
}

impl StateDB {
    pub fn new(db: Box<JournalDB>) -> StateDB {
        StateDB { db: db }
    }

    /// Clone the database.
    pub fn boxed_clone(&self) -> StateDB {
        StateDB { db: self.db.boxed_clone() }
    }

    /// Journal all recent operations under the given era and ID.
    pub fn journal_under(&mut self,
                         batch: &mut DBTransaction,
                         now: u64,
                         id: &H256)
                         -> Result<u32, UtilError> {
        self.db.journal_under(batch, now, id)
    }

    /// Returns underlying `JournalDB`.
    pub fn journal_db(&self) -> &JournalDB {
        &*self.db
    }
}

impl Backend for StateDB {
    fn as_hashdb(&self) -> &HashDB {
        self.db.as_hashdb()
    }

    fn as_hashdb_mut(&mut self) -> &mut HashDB {
        self.db.as_hashdb_mut()
    }
}
