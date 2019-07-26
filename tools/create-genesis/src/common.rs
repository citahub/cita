// use ethereum_types::Public;
pub fn clean_0x(s: &str) -> &str {
    if s.starts_with("0x") {
        &s[2..]
    } else {
        s
    }
}

pub fn string_2_bytes(value: String) -> Vec<u8> {
    let v = Box::leak(value.into_boxed_str());
    let v = clean_0x(v);
    hex::decode(v).unwrap()
}

// pub fn string_2_u256(value: String) -> U256 {
//     let v = Box::leak(value.into_boxed_str());
//     let v = clean_0x(v);
//     U256::from(v)
// }

// pub fn public_2_address(public: &Public) -> Address {
//     let hash = tiny_keccak::keccak256(&public.0);
//     let mut result = Address::default();
//     result.copy_from_slice(&hash[12..]);
//     result
// }
//
// pub fn secret_2_address(secret: &str) -> Address {
//     let a = hex::decode(clean_0x(secret)).unwrap();
//     let secret_key = secp256k1::SecretKey::parse_slice(a.as_slice()).unwrap();
//     let public_key = secp256k1::PublicKey::from_secret_key(&secret_key);
//     let serialized = public_key.serialize();
//     let public = Public::from_slice(&serialized[1..65]);
//     public_2_address(&public)
// }

// pub fn get_temp_state() -> State<StateDB> {
//     let state_db = get_temp_state_db();
//     State::new(state_db, Default::default())
// }

// pub fn new_db() -> Arc<KeyValueDB> {
//     Arc::new(kvdb::in_memory(8))
// }
//
// pub fn get_temp_state_db() -> StateDB {
//     let db = new_db();
//     let journal_db = journaldb::new(db, journaldb::Algorithm::Archive, db::COL_STATE);
//     StateDB::new(journal_db, 5 * 1024 * 1024)
// }
