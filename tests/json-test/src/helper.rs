// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_types::{Address, H256, U256};
use ethereum_types::Public;

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
