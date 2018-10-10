// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use cita_types::{Address, H256, U256};
use contracts::solc::permission_management::Resource;
use ethabi::{decode, ParamType};

/// Parse solidity return data `address[]` to rust `Vec<Address>`
pub fn to_address_vec(output: &[u8]) -> Option<Vec<Address>> {
    if output.is_empty() {
        Some(Vec::new())
    } else {
        decode(&[ParamType::Array(Box::new(ParamType::Address))], output)
            .ok()
            .and_then(|decoded| decoded.first().cloned())
            .and_then(|decoded| decoded.to_array())
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
            .and_then(|decoded| decoded.to_array())
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
    to_u256_vec(output).map(|x| x.iter().map(|i| i.low_u64()).collect())
}

/// Parse solidity return data `uint256[]` to rust `Vec<u32>`
pub fn to_u32_vec(output: &[u8]) -> Option<Vec<u32>> {
    to_u256_vec(output).map(|x| x.iter().map(|i| i.low_u32()).collect())
}

/// Parse solidity return data `uint256` to rust `U256`
pub fn to_u256(output: &[u8]) -> Option<U256> {
    decode(&[ParamType::Uint(256)], output)
        .ok()
        .and_then(|decoded| decoded.first().cloned())
        .and_then(|decoded| decoded.to_uint())
        .map(H256::from)
        .map(U256::from)
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
        .and_then(|(cont_bytes, func_bytes)| {
            cont_bytes
                .to_array()
                .map(|cont_array| (cont_array, func_bytes))
        })
        .and_then(|(cont_array, func_bytes)| {
            func_bytes
                .to_array()
                .map(|func_array| (cont_array, func_array))
        })
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
