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
use sha3::sha3_256;

// Calculate function signature hash.
pub fn calc_func_sig(function_name: &[u8]) -> u32 {
    let out: &mut [u8; 32] = &mut [0; 32];
    let outptr = out.as_mut_ptr();
    unsafe {
        sha3_256(outptr, 32, function_name.as_ptr(), function_name.len());
    }
    let signature = BigEndian::read_u32(out.get(0..4).unwrap());
    signature
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
mod tests;
#[cfg(feature = "privatetx")]
mod zk_privacy;
