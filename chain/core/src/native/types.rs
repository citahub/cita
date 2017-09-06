// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use evm::Ext;
use std::boxed::Box;
use std::ops::Deref;
use util::{U256, H256, Hashable};
pub trait KV {
    fn put(ext: &mut Ext, key: &H256, value: &Self) -> Result<(), String>;
    fn get(ext: &Ext, key: &H256) -> Result<Box<Self>, String>;
    fn del(ext: &mut Ext, key: &H256);
}

impl KV for U256 {
    fn put(ext: &mut Ext, key: &H256, value: &Self) -> Result<(), String> {
        if ext.set_storage(*key, H256::from(value)).is_ok() {
            Ok(())
        } else {
            Err(format!("error on set_storage"))
        }
    }
    fn get<'a>(ext: &Ext, key: &H256) -> Result<Box<Self>, String> {
        if let Ok(value) = ext.storage_at(key) {
            Ok(Box::new(U256::from(value)))
        } else {
            Err(format!("error on storage_at"))
        }
    }
    fn del(ext: &mut Ext, key: &H256) {
        let _ = Self::put(ext, key, &Self::zero());
    }
}

impl KV for Vec<u8> {
    fn put(ext: &mut Ext, key: &H256, value: &Self) -> Result<(), String> {
        let length = value.len();
        if length < 32 {
            let mut byte32 = [0u8; 32];
            let mut index = 0;
            for c in value.iter() {
                byte32[index] = *c;
                index += 1;
            }
            while index < 31 {
                byte32[index] = 0;
                index += 1;
            }
            byte32[index] = (length * 2) as u8;
            if ext.set_storage(*key, H256::from_slice(&byte32)).is_err() {
                return Err(format!("error on set_storage"));
            }
        } else {
            if ext.set_storage(*key, H256::from((length * 2 + 1) as u64)).is_err() {
                return Err(format!("error on set_storage"));
            }
            let mut key = U256::from(key.crypt_hash());
            for chunk in value.chunks(32) {
                let value = H256::from(chunk);
                if ext.set_storage(H256::from(key), value).is_err() {
                    return Err(format!("error on set_storage"));
                }
                key = key + U256::one();
            }
        }
        Ok(())
    }

    fn get(ext: &Ext, key: &H256) -> Result<Box<Self>, String> {
        let mut value = Self::new();
        if let Ok(v) = ext.storage_at(&key) {
            if v[31] % 2 == 0 {
                let len = (v[31] / 2) as usize;
                for i in 0..len {
                    value.push(v[i]);
                }
                Ok(Box::new(value))
            } else {
                let mut len = ((v.low_u64() as usize) - 1) / 2;
                let mut key = U256::from(key.crypt_hash());
                while len > 0 {
                    if let Ok(v) = ext.storage_at(&H256::from(key)) {
                        debug!(target: "native", "key: {:?}, value: {:?}", H256::from(key), v);
                        if len > 32 {
                            value.extend_from_slice(v.as_ref());
                            key = key + U256::one();
                            len -= 32;
                        } else {
                            for i in 0..len {
                                value.push(v[i]);
                            }
                            len = 0;
                        }
                    } else {
                        return Err(format!("error on storage_at"));
                    }
                }
                Ok(Box::new(value))
            }
        } else {
            Err(format!("error on storage_at"))
        }
    }

    fn del(ext: &mut Ext, key: &H256) {
        U256::del(ext, key)
    }
}

impl KV for String {
    fn put(ext: &mut Ext, key: &H256, value: &Self) -> Result<(), String> {
        let bytes = Vec::<u8>::from(value.as_bytes());
        Vec::<u8>::put(ext, key, &bytes)
    }
    fn get(ext: &Ext, key: &H256) -> Result<Box<Self>, String> {
        if let Ok(bytes) = Vec::<u8>::get(ext, key) {
            let mut value = Vec::<u8>::new();
            value.extend_from_slice(bytes.deref());
            if let Ok(value) = String::from_utf8(value) {
                Ok(Box::new(value))
            } else {
                Err(format!("error on from_utf8"))
            }

        } else {
            Err(format!("error on get"))

        }
    }
    fn del(ext: &mut Ext, key: &H256) {
        U256::del(ext, key)
    }
}

pub trait KVVec<T> {
    // fn len() -> u64;
    fn put_item(ext: &mut Ext, pos: &H256, index: u64, value: &T) -> Result<(), String>;
    fn get_item(ext: &Ext, pos: &H256, index: u64) -> Result<Box<T>, String>;
    fn set_len(ext: &mut Ext, pos: &H256, len: u64) -> Result<(), String>;
    fn get_len(ext: &mut Ext, pos: &H256) -> Result<u64, String>;
}
pub struct Array<T> {
    _data: T,
}

impl<T> KVVec<T> for Array<T>
where
    T: KV,
{
    fn put_item(ext: &mut Ext, pos: &H256, index: u64, value: &T) -> Result<(), String> {
        let mut key = U256::from(pos.crypt_hash());
        key = key + U256::from(index);
        if T::put(ext, &H256::from(key), value).is_err() {
            return Err(format!("error on T::put"));
        }
        Ok(())
    }
    fn get_item(ext: &Ext, pos: &H256, index: u64) -> Result<Box<T>, String> {
        let mut key = U256::from(pos.crypt_hash());
        key = key + U256::from(index);
        T::get(ext, &H256::from(key))
    }
    fn set_len(ext: &mut Ext, pos: &H256, len: u64) -> Result<(), String> {
        U256::put(ext, pos, &U256::from(len))
    }
    fn get_len(ext: &mut Ext, pos: &H256) -> Result<u64, String> {
        if let Ok(len) = U256::get(ext, pos) {
            Ok(len.low_u64())
        } else {
            Err(format!("error on U256::get"))
        }
    }
}

pub trait KVMap<Key, Value>
where
    Value: KV,
{
    // fn len() -> u64;
    fn put_item(ext: &mut Ext, pos: &H256, key: Key, value: &Value) -> Result<(), String>;
    fn get_item(ext: &Ext, pos: &H256, key: Key) -> Result<Box<Value>, String>;
}

pub struct Map<Key, Value> {
    _key: Key,
    _value: Value,
}

pub trait Serialize {
    fn serialize(self: Self) -> Vec<u8>;
}

impl Serialize for U256 {
    fn serialize(self: Self) -> Vec<u8> {
        let mut bytes = vec![0u8; 32];
        self.to_big_endian(&mut bytes);
        bytes
    }
}

impl Serialize for String {
    fn serialize(self: Self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl<Key, Value> KVMap<Key, Value> for Map<Key, Value>
where
    Value: KV,
    Key: Serialize,
{
    fn put_item(ext: &mut Ext, pos: &H256, key: Key, value: &Value) -> Result<(), String> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(Key::serialize(key).as_ref());
        bytes.extend_from_slice(pos.as_ref());
        let key = bytes.crypt_hash();
        Value::put(ext, &key, value)
    }
    fn get_item(ext: &Ext, pos: &H256, key: Key) -> Result<Box<Value>, String> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(Key::serialize(key).as_ref());
        bytes.extend_from_slice(pos.as_ref());
        let key = bytes.crypt_hash();
        Value::get(ext, &key)
    }
}


#[cfg(test)]
mod tests {
    extern crate env_logger;
    use super::*;
    use evm::tests::FakeExt;

    #[test]
    fn test_u256() {
        let mut ext = FakeExt::new();

        let original = U256::from(0x1234);
        assert!(U256::put(&mut ext, &H256::from(0), &original).is_ok());
        if let Ok(expected) = U256::get(&mut ext, &H256::from(0)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_string() {
        let mut ext = FakeExt::new();

        let original = String::from("abcdefghijabcdefghijabcdefghij");
        assert!(String::put(&mut ext, &H256::from(0), &original).is_ok());
        if let Ok(expected) = String::get(&mut ext, &H256::from(0)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = String::from("abcdefghijabcdefghijabcdefghija");
        assert!(String::put(&mut ext, &H256::from(1), &original).is_ok());
        if let Ok(expected) = String::get(&mut ext, &H256::from(1)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = String::from("abcdefghijabcdefghijabcdefghijab");
        assert!(String::put(&mut ext, &H256::from(2), &original).is_ok());
        if let Ok(expected) = String::get(&mut ext, &H256::from(2)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = String::from("abcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghijabcdefghij");
        assert!(String::put(&mut ext, &H256::from(3), &original).is_ok());
        if let Ok(expected) = String::get(&mut ext, &H256::from(3)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }
    }


    #[test]
    fn test_bytes() {
        let mut ext = FakeExt::new();

        let original = Vec::<u8>::from(concat!("12345678901234567890", "12345678901"));
        assert!(Vec::<u8>::put(&mut ext, &H256::from(0), &original).is_ok());
        if let Ok(expected) = Vec::<u8>::get(&mut ext, &H256::from(0)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = Vec::<u8>::from(concat!("12345678901234567890", "12345678901234567890", "12345678901234567890", "123"));
        assert!(Vec::<u8>::put(&mut ext, &H256::from(1), &original).is_ok());
        if let Ok(expected) = Vec::<u8>::get(&mut ext, &H256::from(1)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_array() {
        let mut ext = FakeExt::new();

        let original = 10u64;
        assert!(Array::<U256>::set_len(&mut ext, &H256::from(0), original).is_ok());
        if let Ok(expected) = Array::<U256>::get_len(&mut ext, &H256::from(0)) {
            assert_eq!(original, expected);
        } else {
            assert!(false)
        }

        let original = U256::from(0x1234);
        assert!(Array::<U256>::put_item(&mut ext, &H256::from(0), 1, &original).is_ok());
        if let Ok(expected) = Array::<U256>::get_item(&mut ext, &H256::from(0), 1) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = U256::from(0x2234);
        assert!(Array::<U256>::put_item(&mut ext, &H256::from(0), 2, &original).is_ok());
        if let Ok(expected) = Array::<U256>::get_item(&mut ext, &H256::from(0), 2) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        ////////////////////////////////////////////////////////////////////////////////
        let original = 3u64;
        assert!(Array::<String>::set_len(&mut ext, &H256::from(0), original).is_ok());
        if let Ok(expected) = Array::<String>::get_len(&mut ext, &H256::from(0)) {
            assert_eq!(original, expected);
        } else {
            assert!(false)
        }

        let original = String::from("1234");
        assert!(Array::<String>::put_item(&mut ext, &H256::from(1), 2, &original).is_ok());
        if let Ok(expected) = Array::<String>::get_item(&mut ext, &H256::from(1), 2) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_map() {
        let mut ext = FakeExt::new();

        let original = U256::from(0x1234);
        assert!(Map::<U256, U256>::put_item(&mut ext, &H256::from(1), U256::from(2), &original).is_ok());
        if let Ok(expected) = Map::<U256, U256>::get_item(&mut ext, &H256::from(1), U256::from(2)) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = U256::from(0x2234);
        assert!(Map::<String, U256>::put_item(&mut ext, &H256::from(2), String::from("123"), &original).is_ok());
        if let Ok(expected) = Map::<String, U256>::get_item(&mut ext, &H256::from(2), String::from("123")) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false)
        }

        let original = String::from("abcdefghijabcdefghijabcdefghijabc");
        assert!(Map::<String, String>::put_item(&mut ext, &H256::from(2), String::from("123"), &original).is_ok());
        if let Ok(expected) = Map::<String, String>::get_item(&mut ext, &H256::from(2), String::from("123")) {
            assert_eq!(original, *expected.as_ref());
        } else {
            assert!(false);
        }
    }
}
