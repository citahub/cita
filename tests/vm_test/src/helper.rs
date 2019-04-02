use evm::cita_types::{Address, H256, U256};

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

#[allow(dead_code)]
pub fn string_2_address(value: String) -> Address {
    if value.is_empty() {
        return Address::zero();
    }
    let v = Box::leak(value.into_boxed_str());
    let v = clean_0x(v);
    Address::from(v)
}
