use journaldb;
use std::sync::Arc;
use state::State;
use util::KeyValueDB;
use state_db::*;

pub fn get_temp_state() -> State<StateDB> {
	let journal_db = get_temp_state_db();
	State::new(journal_db, 0.into(), Default::default())
}

fn new_db() -> Arc<KeyValueDB> {
	Arc::new(::util::kvdb::in_memory(8))
}

pub fn get_temp_state_db() -> StateDB {
	let db = new_db();
	let journal_db = journaldb::new(db, journaldb::Algorithm::Archive, ::db::COL_STATE);
    StateDB::new(journal_db)
}