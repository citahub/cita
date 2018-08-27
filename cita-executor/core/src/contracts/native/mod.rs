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

use byteorder::BigEndian;
use byteorder::ByteOrder;
use evm::{action_params::ActionParams, Error};
use util::sha3;

// Calculate function signature hash.
pub fn calc_func_sig(function_name: &[u8]) -> u32 {
    let data = sha3::keccak256(function_name);
    BigEndian::read_u32(&data)
}

// Extract function signature hash.
pub fn extract_func_sig(params: &ActionParams) -> Result<u32, Error> {
    if let Some(ref data) = params.data {
        if let Some(ref bytes4) = data.get(0..4) {
            Ok(BigEndian::read_u32(bytes4))
        } else {
            Err(Error::OutOfGas)
        }
    } else {
        Err(Error::OutOfGas)
    }
}

mod crosschain_verify;
pub mod factory;
#[cfg(test)]
mod storage;
#[cfg(feature = "privatetx")]
mod zk_privacy;

#[cfg(test)]
mod tests {
    #[test]
    fn calc_func_sig() {
        let testdata = vec![
            ("thisIsAMethodName(uint256)", 0xa86712e7),
            ("aMethodNameAgain(bool)", 0xa1bea0ac),
            ("thisIsAlsoAMethodName(bytes32)", 0xb77bc401),
            ("thisIsAMethodNameToo(bytes)", 0x874679ca),
        ];
        for (data, expected) in testdata.into_iter() {
            assert_eq!(super::calc_func_sig(data.as_ref()), expected);
        }
    }
}
