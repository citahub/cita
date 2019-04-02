extern crate core_executor;

use self::core_executor::cita_db::{journaldb, kvdb, KeyValueDB};
use self::core_executor::db;
use self::core_executor::state::State;
use self::core_executor::state_db::StateDB;
use ethereum_types::Public;
use evm::cita_types::{Address, H256, U256};
use std::sync::Arc;

pub fn clean_0x(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

pub fn string_2_u256(value: String) -> U256 {
    let v = Box::leak(value.into_boxed_str());
    let v = clean_0x(v);
    U256::from(v)
}

pub fn string_2_h256(value: String) -> H256 {
    let v = Box::leak(value.into_boxed_str());
    let v = clean_0x(v);
    if v.len() < 64 {
        let mut s = String::from("0").repeat(64 - v.len());
        s.push_str(v);
        let s: &'static str = Box::leak(s.into_boxed_str());
        return H256::from(s);
    }
    H256::from(v)
}

pub fn string_2_bytes(value: String) -> Vec<u8> {
    let v = Box::leak(value.into_boxed_str());
    let v = clean_0x(v);
    hex::decode(v).unwrap()
}

pub fn public_2_address(public: &Public) -> Address {
    let hash = tiny_keccak::keccak256(&public.0);
    let mut result = Address::default();
    result.copy_from_slice(&hash[12..]);
    result
}

pub fn secret_2_address(secret: &str) -> Address {
    let a = hex::decode(clean_0x(secret)).unwrap();
    let secret_key = secp256k1::SecretKey::parse_slice(a.as_slice()).unwrap();
    let public_key = secp256k1::PublicKey::from_secret_key(&secret_key);
    let serialized = public_key.serialize();
    let mut public = Public::default();
    public.copy_from_slice(&serialized[1..65]);
    public_2_address(&public)
}

#[allow(dead_code)]
pub fn string_2_address(value: String) -> Address {
    if value.is_empty() {
        return Address::zero();
    }
    let v = Box::leak(value.into_boxed_str());
    let v = clean_0x(v);
    Address::from(v)
}

pub fn get_temp_state() -> State<StateDB> {
    let state_db = get_temp_state_db();
    State::new(state_db, 0.into(), Default::default())
}

pub fn new_db() -> Arc<KeyValueDB> {
    Arc::new(kvdb::in_memory(8))
}

pub fn get_temp_state_db() -> StateDB {
    let db = new_db();
    let journal_db = journaldb::new(db, journaldb::Algorithm::Archive, db::COL_STATE);
    StateDB::new(journal_db, 5 * 1024 * 1024)
}
