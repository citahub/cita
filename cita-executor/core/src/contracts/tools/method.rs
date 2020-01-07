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

use crate::types::errors::NativeError;
use byteorder::{BigEndian, ByteOrder};
/// Calculate contract method signature hash and return different types.
use util::sha3;

pub fn encode_to_array(name: &[u8]) -> [u8; 4] {
    let mut ret = [0u8; 4];
    ret.copy_from_slice(&sha3::keccak256(name)[0..4]);
    ret
}

pub fn encode_to_vec(name: &[u8]) -> Vec<u8> {
    sha3::keccak256(name)[0..4].to_vec()
}

pub fn encode_to_u32(name: &[u8]) -> u32 {
    BigEndian::read_u32(&sha3::keccak256(name)[..])
}

// Extract first four bytes (function signature hash) as u32.
pub fn extract_to_u32(data: &[u8]) -> Result<u32, NativeError> {
    if let Some(ref bytes4) = data.get(0..4) {
        Ok(BigEndian::read_u32(bytes4))
    } else {
        Err(NativeError::Internal("out of gas".to_string()))
    }
}

#[cfg(test)]
mod tests {

    use byteorder::{BigEndian, ByteOrder};

    #[test]
    fn all() {
        let testdata = vec![
            ("thisIsAMethodName(uint256)", vec![0xa8, 0x67, 0x12, 0xe7]),
            ("aMethodNameAgain(bool)", vec![0xa1, 0xbe, 0xa0, 0xac]),
            (
                "thisIsAlsoAMethodName(bytes32)",
                vec![0xb7, 0x7b, 0xc4, 0x01],
            ),
            ("thisIsAMethodNameToo(bytes)", vec![0x87, 0x46, 0x79, 0xca]),
        ];
        for (data, expected) in testdata.into_iter() {
            let mut expected_data = [0u8; 4];
            expected_data.copy_from_slice(&expected[0..4]);
            assert_eq!(super::encode_to_array(data.as_ref()), expected_data);
            assert_eq!(super::encode_to_vec(data.as_ref()), expected);
            assert_eq!(
                super::encode_to_u32(data.as_ref()),
                BigEndian::read_u32(&expected[..])
            );
        }
    }
}
