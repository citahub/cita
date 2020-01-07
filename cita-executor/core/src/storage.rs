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
use std::boxed::Box;
use std::convert::From;
use util::sha3;

use crate::types::errors::NativeError;
use cita_vm::evm::DataProvider;

pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, NativeError>;
}
pub trait Deserialize: Sized {
    fn deserialize(bytes: &[u8]) -> Result<Self, NativeError>;
}

impl Serialize for U256 {
    fn serialize(&self) -> Result<Vec<u8>, NativeError> {
        let mut vec = vec![0; 32];
        self.to_big_endian(&mut vec);
        Ok(vec)
    }
}
impl Deserialize for U256 {
    fn deserialize(bytes: &[u8]) -> Result<Self, NativeError> {
        Ok(U256::from(bytes))
    }
}

impl Serialize for String {
    fn serialize(&self) -> Result<Vec<u8>, NativeError> {
        Ok(self.to_owned().into_bytes())
    }
}
impl Deserialize for String {
    fn deserialize(bytes: &[u8]) -> Result<Self, NativeError> {
        Self::from_utf8(bytes.to_owned()).map_err(|_| NativeError::Internal("dup coin".to_string()))
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self) -> Result<Vec<u8>, NativeError> {
        Ok(self.clone())
    }
}
impl Deserialize for Vec<u8> {
    fn deserialize(bytes: &[u8]) -> Result<Self, NativeError> {
        Ok(Vec::from(bytes))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scalar {
    position: H256,
}

impl Scalar {
    pub fn new(position: H256) -> Self {
        Scalar { position }
    }
    // single element
    pub fn set(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        value: U256,
    ) -> Result<(), NativeError> {
        data_provider.set_storage(addr, self.position, H256::from(value));
        Ok(())
    }

    pub fn get(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
    ) -> Result<U256, NativeError> {
        let value = data_provider.get_storage(addr, &self.position);
        Ok(U256::from(value))
    }

    // bytes & string
    pub fn set_bytes<T>(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        value: &T,
    ) -> Result<(), NativeError>
    where
        T: Serialize,
    {
        let encoded = value.serialize()?;
        let length = encoded.len();
        if length < 32 {
            let mut byte32 = [0u8; 32];
            byte32[0..encoded.len()].copy_from_slice(&encoded);
            byte32[31] = (length * 2) as u8;
            data_provider.set_storage(addr, self.position, H256::from_slice(&byte32));
        } else {
            data_provider.set_storage(addr, self.position, H256::from((length * 2 + 1) as u64));
            let mut key = U256::from(H256::from_slice(&sha3::keccak256(&self.position)));
            for chunk in encoded.chunks(32) {
                let value = H256::from(chunk);
                data_provider.set_storage(addr, H256::from(key), value);
                key += U256::one();
            }
        }
        Ok(())
    }

    pub fn get_bytes<T>(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
    ) -> Result<Box<T>, NativeError>
    where
        T: Deserialize,
    {
        let mut bytes = Vec::<u8>::new();
        let first = data_provider.get_storage(addr, &self.position);
        if first[31] % 2 == 0 {
            let len = (first[31] / 2) as usize;
            bytes.extend_from_slice(&first[0..len]);
            let decoded = T::deserialize(&bytes)?;
            Ok(Box::new(decoded))
        } else {
            let mut len = ((first.low_u64() as usize) - 1) / 2;
            let mut key = U256::from(H256::from_slice(&sha3::keccak256(&self.position)));
            let mut bytes = Vec::new();
            while len > 0 {
                let v = data_provider.get_storage(addr, &H256::from(key));
                if len > 32 {
                    bytes.extend_from_slice(v.as_ref());
                    key += U256::one();
                    len -= 32;
                } else {
                    bytes.extend_from_slice(&v[0..len]);
                    len = 0;
                }
            }
            Ok(Box::new(T::deserialize(&bytes)?))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array {
    position: H256,
}
impl Array {
    pub fn new(position: H256) -> Self {
        Array { position }
    }

    #[inline]
    fn key(&self, index: u64) -> H256 {
        let mut key = U256::from(H256::from_slice(&sha3::keccak256(&self.position)));
        key += U256::from(index);
        H256::from(key)
    }

    pub fn set(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        index: u64,
        value: &U256,
    ) -> Result<(), NativeError> {
        let scalar = Scalar::new(self.key(index));
        scalar.set(data_provider, addr, *value)
    }

    pub fn get(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
        index: u64,
    ) -> Result<U256, NativeError> {
        let scalar = Scalar::new(self.key(index));
        scalar.get(data_provider, addr)
    }

    pub fn set_bytes<T>(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        index: u64,
        value: &T,
    ) -> Result<(), NativeError>
    where
        T: Serialize,
    {
        let scalar = Scalar::new(self.key(index));
        scalar.set_bytes(data_provider, addr, value)
    }

    pub fn get_bytes<T>(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
        index: u64,
    ) -> Result<Box<T>, NativeError>
    where
        T: Deserialize,
    {
        let scalar = Scalar::new(self.key(index));
        scalar.get_bytes(data_provider, addr)
    }

    pub fn set_len(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        len: u64,
    ) -> Result<(), NativeError> {
        data_provider.set_storage(addr, self.position, H256::from(len));
        Ok(())
    }

    pub fn get_len(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
    ) -> Result<u64, NativeError> {
        let len = data_provider.get_storage(addr, &self.position);
        Ok(len.low_u64())
    }

    pub fn get_array(self: &mut Self, index: u64) -> Array {
        Array::new(self.key(index))
    }
    pub fn get_map(self: &mut Self, index: u64) -> Map {
        Map::new(self.key(index))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map {
    position: H256,
}

impl Map {
    pub fn new(position: H256) -> Self {
        Map { position }
    }

    #[inline]
    fn key<Key>(&self, key: &Key) -> Result<H256, NativeError>
    where
        Key: Serialize,
    {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&key.serialize()?);
        bytes.extend_from_slice(self.position.as_ref());
        Ok(H256::from_slice(&sha3::keccak256(&bytes)))
    }

    pub fn set<Key>(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        key: &Key,
        value: U256,
    ) -> Result<(), NativeError>
    where
        Key: Serialize,
    {
        Scalar::new(self.key(key)?).set(data_provider, addr, value)
    }

    pub fn get<Key>(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
        key: &Key,
    ) -> Result<U256, NativeError>
    where
        Key: Serialize,
    {
        Scalar::new(self.key(key)?).get(data_provider, addr)
    }

    pub fn set_bytes<Key, Value>(
        self: &Self,
        data_provider: &mut dyn DataProvider,
        addr: &Address,
        key: &Key,
        value: &Value,
    ) -> Result<(), NativeError>
    where
        Key: Serialize,
        Value: Serialize,
    {
        Scalar::new(self.key(key)?).set_bytes(data_provider, addr, value)
    }

    pub fn get_bytes<Key, Value>(
        self: &Self,
        data_provider: &dyn DataProvider,
        addr: &Address,
        key: &Key,
    ) -> Result<Value, NativeError>
    where
        Key: Serialize,
        Value: Deserialize,
    {
        Ok(*Scalar::new(self.key(key)?).get_bytes(data_provider, addr)?)
    }

    pub fn get_array<Key>(self: &mut Self, key: &Key) -> Result<Array, NativeError>
    where
        Key: Serialize,
    {
        Ok(Array::new(self.key(key)?))
    }

    pub fn get_map<Key>(self: &mut Self, key: &Key) -> Result<Map, NativeError>
    where
        Key: Serialize,
    {
        Ok(Map::new(self.key(key)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cita_vm::evm::extmock::DataProviderMock;
    use std::str::FromStr;

    #[test]

    fn test_scalar_bytes() {
        // let mut data_provider = DataProviderMock::default();
        let mut data_provider = DataProviderMock::default();

        let scalar = Scalar::new(H256::from(0));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) length=30
        let expected = format!("012345678901234567890123456789");
        assert!(scalar
            .set_bytes(&mut data_provider, &code_address, &expected)
            .is_ok());
        let value = scalar.get_bytes::<String>(&data_provider, &code_address);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());

        // 2) length=31
        let expected = format!("0123456789012345678901234567890");
        assert!(scalar
            .set_bytes(&mut data_provider, &code_address, &expected)
            .is_ok());
        let value = scalar.get_bytes::<String>(&data_provider, &code_address);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());

        // 3) length=32
        let expected = format!("01234567890123456789012345678901");
        assert!(scalar
            .set_bytes(&mut data_provider, &code_address, &expected)
            .is_ok());
        let value = scalar.get_bytes::<String>(&data_provider, &code_address);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());

        // 4) length=43
        let expected = format!("012345678901234567890123456789012");
        assert!(scalar
            .set_bytes(&mut data_provider, &code_address, &expected)
            .is_ok());
        let value = scalar.get_bytes::<String>(&data_provider, &code_address);
        assert!(value.is_ok());
        assert_eq!(*value.unwrap().as_ref(), expected.clone());
    }

    #[test]
    fn test_scalar_u256() {
        let mut data_provider = DataProviderMock::default();
        let scalar = Scalar::new(H256::from(0));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        let expected = U256::from(0x123456);
        assert!(scalar
            .set(&mut data_provider, &code_address, expected.clone())
            .is_ok());
        let value = scalar.get(&data_provider, &code_address);
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), expected.clone());
    }

    #[test]
    fn test_array_simple() {
        let mut data_provider = DataProviderMock::default();
        let length = 7u64;
        let array = Array {
            position: H256::from(0),
        };
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) length
        assert!(array
            .set_len(&mut data_provider, &code_address, length)
            .is_ok());
        assert_eq!(
            array.get_len(&data_provider, &code_address).unwrap(),
            length
        );

        // 2) array[1] = 0x1234
        let index = 1;
        let expected = U256::from(0x1234);
        assert!(array
            .set(&mut data_provider, &code_address, index, &expected)
            .is_ok());
        let value = array.get(&data_provider, &code_address, index);
        assert_eq!(value.unwrap(), expected.clone());

        // 3) array[3] = 0x2234
        let index = 3;
        let expected = U256::from(0x2234);
        assert!(array
            .set(&mut data_provider, &code_address, index, &expected)
            .is_ok());
        let value = array.get(&data_provider, &code_address, index);
        assert_eq!(value.unwrap(), expected.clone());
    }

    #[test]
    fn test_array_with_sub_array() {
        let mut data_provider = DataProviderMock::default();
        let mut array = Array::new(H256::from(0));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) length = 7
        let length = 7;
        assert!(array
            .set_len(&mut data_provider, &code_address, length)
            .is_ok());
        assert_eq!(
            array.get_len(&data_provider, &code_address).unwrap(),
            length
        );

        // 2) array[1].len = 8
        let index = 1;
        let subarray_length = 8;
        let subarray = array.get_array(index);
        assert!(subarray
            .set_len(&mut data_provider, &code_address, subarray_length)
            .is_ok());
        assert_eq!(
            subarray.get_len(&mut data_provider, &code_address).unwrap(),
            subarray_length
        );

        // 3) array[1][2] = 0x1234
        let index = 2;
        let expected = U256::from(0x1234);
        assert!(subarray
            .set(&mut data_provider, &code_address, index, &expected)
            .is_ok());
        assert_eq!(
            subarray.get(&data_provider, &code_address, index).unwrap(),
            expected
        );

        // 4) array[1][4] = 0x2234
        let index = 4;
        let expected = U256::from(0x2234);
        assert!(subarray
            .set(&mut data_provider, &code_address, index, &expected)
            .is_ok());
        assert_eq!(
            subarray.get(&data_provider, &code_address, index).unwrap(),
            expected
        );
    }

    #[test]
    fn test_array_with_sub_map() {
        let mut data_provider = DataProviderMock::default();
        let mut array = Array::new(H256::from(0));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) length = 7
        let length = 7;
        assert!(array
            .set_len(&mut data_provider, &code_address, length)
            .is_ok());
        assert_eq!(
            array.get_len(&data_provider, &code_address).unwrap(),
            length
        );

        // 2) array[1][2] = 0x1234
        let index = 1;
        let key = U256::from(2);
        let submap = array.get_map(index);
        let expected = U256::from(0x1234);
        assert!(submap
            .set(&mut data_provider, &code_address, &key, expected)
            .is_ok());
        assert_eq!(
            submap
                .get::<U256>(&data_provider, &code_address, &key)
                .unwrap(),
            expected
        );

        // 4) array[1]["key"] = "1234"
        let key = String::from("key");
        let expected = String::from("1234");
        assert!(submap
            .set_bytes::<String, String>(&mut data_provider, &code_address, &key, &expected)
            .is_ok());
        assert_eq!(
            submap
                .get_bytes::<String, String>(&data_provider, &code_address, &key)
                .unwrap(),
            expected.clone()
        );
    }

    #[test]
    fn test_map_simple() {
        let mut data_provider = DataProviderMock::default();
        let map = Map::new(H256::from(1));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) map["key"] = "value"
        let key = U256::from(1);
        let value = U256::from(0x1234);
        assert!(map
            .set(&mut data_provider, &code_address, &key, value)
            .is_ok());
        assert_eq!(map.get(&data_provider, &code_address, &key).unwrap(), value);

        // 2) map[0] = "1234567890"
        let key = U256::from(1);
        let value = String::from("1234567890");
        assert!(map
            .set_bytes(&mut data_provider, &code_address, &key, &value)
            .is_ok());
        assert_eq!(
            map.get_bytes::<U256, String>(&data_provider, &code_address, &key)
                .unwrap(),
            value.clone()
        );

        // 3) map[0] = "123456789012345678901234567890123"
        let key = U256::from(1);
        let value = String::from("123456789012345678901234567890123");
        assert!(map
            .set_bytes(&mut data_provider, &code_address, &key, &value)
            .is_ok());
        assert_eq!(
            map.get_bytes::<U256, String>(&data_provider, &code_address, &key)
                .unwrap(),
            value
        );

        // 4) map["key"] = 0x1234;
        let key = String::from("key");
        let value = U256::from(0x1234);
        assert!(map
            .set(&mut data_provider, &code_address, &key, value)
            .is_ok());
        assert_eq!(map.get(&data_provider, &code_address, &key).unwrap(), value);
    }

    #[test]
    fn test_map_with_sub_array() {
        let mut data_provider = DataProviderMock::default();
        let mut map = Map::new(H256::from(1));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) map["key1"]["key2"] = "1234567890"
        let key1 = String::from("key1");
        let index = 2u64;
        let value = String::from("1234567890");
        let sub_array = map.get_array(&key1).unwrap();
        assert!(sub_array
            .set_bytes(&mut data_provider, &code_address, index.clone(), &value)
            .is_ok());
        assert_eq!(
            *sub_array
                .get_bytes::<String>(&data_provider, &code_address, index.clone())
                .unwrap(),
            value.clone()
        );

        // 2) map["key1"][2] = "1234567890"
        let key1 = String::from("key1");
        let index = 4u64;
        let value = String::from("1234567890");
        let sub_array = map.get_array(&key1).unwrap();
        assert!(sub_array
            .set_bytes(&mut data_provider, &code_address, index.clone(), &value)
            .is_ok());
        assert_eq!(
            *sub_array
                .get_bytes::<String>(&data_provider, &code_address, index.clone())
                .unwrap(),
            value.clone()
        );
    }

    #[test]
    fn test_map_with_sub_map() {
        let mut data_provider = DataProviderMock::default();
        let mut map = Map::new(H256::from(1));
        let code_address = Address::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap();

        // 1) map["key1"]["key2"] = "1234567890"
        let key1 = String::from("key1");
        let key2 = String::from("key2");
        let value = String::from("1234567890");
        let sub_map = map.get_map(&key1).unwrap();
        assert!(sub_map
            .set_bytes(&mut data_provider, &code_address, &key2, &value)
            .is_ok());
        assert_eq!(
            sub_map
                .get_bytes::<String, String>(&data_provider, &code_address, &key2)
                .unwrap(),
            value.clone()
        );

        // 2) map["key1"][2] = "1234567890"
        let key1 = String::from("key1");
        let key2 = U256::from(2);
        let value = String::from("1234567890");
        let sub_map = map.get_map(&key1).unwrap();
        assert!(sub_map
            .set_bytes(&mut data_provider, &code_address, &key2, &value)
            .is_ok());
        assert_eq!(
            sub_map
                .get_bytes::<_, String>(&data_provider, &code_address, &key2)
                .unwrap(),
            value.clone()
        );
    }
}
