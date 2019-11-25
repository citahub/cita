// Copyright Cryptape Technologies LLC.
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

use crate::contracts::solc::permission_management::Resource;
use cita_types::{Address, H256, U256};
use ethabi::{decode, ParamType, Token};

/// Parse solidity return data `address[]` to rust `Vec<Address>`
pub fn to_address_vec(output: &[u8]) -> Option<Vec<Address>> {
    if output.is_empty() {
        Some(Vec::new())
    } else {
        decode(&[ParamType::Array(Box::new(ParamType::Address))], output)
            .ok()
            .and_then(|decoded| decoded.first().cloned())
            .and_then(Token::to_array)
            .and_then(|addrs| {
                let mut v = Vec::new();
                for x in addrs {
                    if let Some(x) = x.to_address() {
                        v.push(Address::from(x))
                    } else {
                        return None;
                    }
                }
                Some(v)
            })
    }
}

/// Parse solidity return data `uint256[]` to rust `Vec<u256>`
pub fn to_u256_vec(output: &[u8]) -> Option<Vec<U256>> {
    if output.is_empty() {
        Some(Vec::new())
    } else {
        decode(&[ParamType::Array(Box::new(ParamType::Uint(256)))], output)
            .ok()
            .and_then(|decoded| decoded.first().cloned())
            .and_then(Token::to_array)
            .and_then(|uints| {
                let mut v = Vec::new();
                for x in uints {
                    if let Some(x) = x.to_uint() {
                        let h = H256::from(x);
                        v.push(U256::from(h))
                    } else {
                        return None;
                    }
                }
                Some(v)
            })
    }
}

/// Parse solidity return data `uint256[]` to rust `Vec<u64>`
pub fn to_u64_vec(output: &[u8]) -> Option<Vec<u64>> {
    to_u256_vec(output).map(|x| x.iter().map(U256::low_u64).collect())
}

/// Parse solidity return data `uint256[]` to rust `Vec<u32>`
pub fn to_u32_vec(output: &[u8]) -> Option<Vec<u32>> {
    to_u256_vec(output).map(|x| x.iter().map(U256::low_u32).collect())
}

/// Parse solidity return data `uint256` to rust `U256`
pub fn to_u256(output: &[u8]) -> Option<U256> {
    decode(&[ParamType::Uint(256)], output)
        .ok()
        .and_then(|decoded| decoded.first().cloned())
        .and_then(Token::to_uint)
        .map(H256::from)
        .map(U256::from)
}

/// Parse solidity return data `Address` to rust `Address`
pub fn to_address(output: &[u8]) -> Option<Address> {
    decode(&[ParamType::Address], output)
        .ok()
        .and_then(|decoded| decoded.first().cloned())
        .and_then(Token::to_address)
        .map(Address::from)
}
/// Parse solidity return data `uint256` to rust `u64`
pub fn to_u64(output: &[u8]) -> Option<u64> {
    to_u256(output).map(|x| x.low_u64())
}

/// Parse solidity return data `uint256` to rust `u32`
pub fn to_u32(output: &[u8]) -> Option<u32> {
    to_u256(output).map(|x| x.low_u32())
}

/// Parse solidity return data `Address[], bytes4[]` to rust `Vec<Resource>`
pub fn to_resource_vec(output: &[u8]) -> Option<Vec<Resource>> {
    // Decode the address[] and bytes4[]
    if output.is_empty() {
        Some(Vec::new())
    } else {
        decode(
            &[
                ParamType::Array(Box::new(ParamType::Address)),
                ParamType::Array(Box::new(ParamType::FixedBytes(4))),
            ],
            output,
        )
        .ok()
        .and_then(|mut decoded| {
            if decoded.len() < 2 {
                return None;
            }
            Some((decoded.remove(0), decoded.remove(0)))
        })
        .and_then(|(cont_bytes, func_bytes)| cont_bytes.to_array().map(|cont_array| (cont_array, func_bytes)))
        .and_then(|(cont_array, func_bytes)| func_bytes.to_array().map(|func_array| (cont_array, func_array)))
        .and_then(|(cont_array, func_array)| {
            let mut v = Vec::new();
            for x in cont_array {
                if let Some(x) = x.to_address() {
                    v.push(Address::from(x))
                } else {
                    return None;
                }
            }
            Some((v, func_array))
        })
        .and_then(|(cont_vec, func_array)| {
            let mut v = Vec::new();
            for x in func_array {
                if let Some(x) = x.to_fixed_bytes() {
                    v.push(x)
                } else {
                    return None;
                }
            }
            Some((cont_vec, v))
        })
        .map(|(cont_vec, func_vec)| {
            cont_vec
                .into_iter()
                .zip(func_vec.into_iter())
                .map(|(cont, func)| Resource::new(cont, func))
                .collect()
        })
    }
}
