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

/// Calculate contract method signature hash and return different types.
use byteorder::{BigEndian, ByteOrder};
use evm::Error as EvmError;
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
pub fn extract_to_u32(data: &[u8]) -> Result<u32, EvmError> {
    if let Some(ref bytes4) = data.get(0..4) {
        Ok(BigEndian::read_u32(bytes4))
    } else {
        Err(EvmError::OutOfGas)
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
