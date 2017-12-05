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

use H256;
#[cfg(feature = "blake2bhash")]
use blake2b::blake2b;
use sha3::sha3_256;
#[cfg(feature = "sm3hash")]
use sm3::sm3;

/// The hash of the empty bytes string.
#[cfg(feature = "sha3hash")]
pub const HASH_EMPTY: H256 = H256([
    0xc5,
    0xd2,
    0x46,
    0x01,
    0x86,
    0xf7,
    0x23,
    0x3c,
    0x92,
    0x7e,
    0x7d,
    0xb2,
    0xdc,
    0xc7,
    0x03,
    0xc0,
    0xe5,
    0x00,
    0xb6,
    0x53,
    0xca,
    0x82,
    0x27,
    0x3b,
    0x7b,
    0xfa,
    0xd8,
    0x04,
    0x5d,
    0x85,
    0xa4,
    0x70,
]);
#[cfg(feature = "blake2bhash")]
pub const HASH_EMPTY: H256 = H256([
    0xd6,
    0x7f,
    0x72,
    0x9f,
    0x8d,
    0x19,
    0xed,
    0x2e,
    0x92,
    0xf8,
    0x17,
    0xcf,
    0x5c,
    0x31,
    0xc7,
    0x81,
    0x2d,
    0xd3,
    0x9e,
    0xd3,
    0x5b,
    0x0b,
    0x1a,
    0xae,
    0x41,
    0xc7,
    0x66,
    0x5f,
    0x46,
    0xc3,
    0x6b,
    0x9f,
]);
#[cfg(feature = "sm3hash")]
pub const HASH_EMPTY: H256 = H256([
    0x1a,
    0xb2,
    0x1d,
    0x83,
    0x55,
    0xcf,
    0xa1,
    0x7f,
    0x8e,
    0x61,
    0x19,
    0x48,
    0x31,
    0xe8,
    0x1a,
    0x8f,
    0x22,
    0xbe,
    0xc8,
    0xc7,
    0x28,
    0xfe,
    0xfb,
    0x74,
    0x7e,
    0xd0,
    0x35,
    0xeb,
    0x50,
    0x82,
    0xaa,
    0x2b,
]);

/// The hash of the RLP encoding of empty data.
#[cfg(feature = "sha3hash")]
pub const HASH_NULL_RLP: H256 = H256([
    0x56,
    0xe8,
    0x1f,
    0x17,
    0x1b,
    0xcc,
    0x55,
    0xa6,
    0xff,
    0x83,
    0x45,
    0xe6,
    0x92,
    0xc0,
    0xf8,
    0x6e,
    0x5b,
    0x48,
    0xe0,
    0x1b,
    0x99,
    0x6c,
    0xad,
    0xc0,
    0x01,
    0x62,
    0x2f,
    0xb5,
    0xe3,
    0x63,
    0xb4,
    0x21,
]);
#[cfg(feature = "blake2bhash")]
pub const HASH_NULL_RLP: H256 = H256([
    0xc1,
    0x4a,
    0xf5,
    0x91,
    0x07,
    0xef,
    0x14,
    0x00,
    0x3e,
    0x46,
    0x97,
    0xa4,
    0x0e,
    0xa9,
    0x12,
    0xd8,
    0x65,
    0xeb,
    0x14,
    0x63,
    0x08,
    0x6a,
    0x46,
    0x49,
    0x97,
    0x7c,
    0x13,
    0xea,
    0x69,
    0xb0,
    0xd9,
    0xaf,
]);
#[cfg(feature = "sm3hash")]
pub const HASH_NULL_RLP: H256 = H256([
    0x99,
    0x5b,
    0x94,
    0x98,
    0x69,
    0xf8,
    0x0f,
    0xa1,
    0x46,
    0x5a,
    0x9d,
    0x8b,
    0x6f,
    0xa7,
    0x59,
    0xec,
    0x65,
    0xc3,
    0x02,
    0x0d,
    0x59,
    0xc2,
    0x62,
    0x46,
    0x62,
    0xbd,
    0xff,
    0x05,
    0x9b,
    0xdf,
    0x19,
    0xb3,
]);

/// The hash of the RLP encoding of empty list.
#[cfg(feature = "sha3hash")]
pub const HASH_EMPTY_LIST_RLP: H256 = H256([
    0x1d,
    0xcc,
    0x4d,
    0xe8,
    0xde,
    0xc7,
    0x5d,
    0x7a,
    0xab,
    0x85,
    0xb5,
    0x67,
    0xb6,
    0xcc,
    0xd4,
    0x1a,
    0xd3,
    0x12,
    0x45,
    0x1b,
    0x94,
    0x8a,
    0x74,
    0x13,
    0xf0,
    0xa1,
    0x42,
    0xfd,
    0x40,
    0xd4,
    0x93,
    0x47,
]);
#[cfg(feature = "blake2bhash")]
pub const HASH_EMPTY_LIST_RLP: H256 = H256([
    0x7b,
    0x7b,
    0x72,
    0xfb,
    0x1e,
    0x60,
    0xea,
    0x75,
    0x69,
    0x9e,
    0x30,
    0x3f,
    0xbc,
    0x97,
    0xc5,
    0xce,
    0xb5,
    0x78,
    0xba,
    0x92,
    0x43,
    0x2a,
    0x64,
    0xe2,
    0x18,
    0xc9,
    0xab,
    0xbc,
    0xd9,
    0x72,
    0xa5,
    0x83,
]);
#[cfg(feature = "sm3hash")]
pub const HASH_EMPTY_LIST_RLP: H256 = H256([
    0x47,
    0x44,
    0x68,
    0x32,
    0xc4,
    0x4e,
    0x75,
    0x55,
    0x27,
    0x02,
    0x2e,
    0x3e,
    0x57,
    0x21,
    0x33,
    0x92,
    0x2b,
    0x49,
    0x76,
    0x8d,
    0x46,
    0x0f,
    0xb1,
    0x74,
    0x12,
    0xc1,
    0xb6,
    0xc8,
    0xfa,
    0x64,
    0xed,
    0x48,
]);

#[cfg(feature = "blake2bhash")]
pub const BLAKE2BKEY: &str = "CryptapeCryptape";

#[cfg(feature = "sha3hash")]
pub const HASH_NAME: &str = "sha3";
#[cfg(feature = "blake2bhash")]
pub const HASH_NAME: &str = "blake2b";
#[cfg(feature = "sm3hash")]
pub const HASH_NAME: &str = "sm3";

pub trait Hashable {
    /// Calculate crypt HASH of this object.
    fn crypt_hash(&self) -> H256 {
        let mut ret: H256 = H256::zero();
        self.crypt_hash_into(&mut *ret);
        ret
    }

    /// Calculate crypt HASH of this object and place result into dest.
    fn crypt_hash_into(&self, dest: &mut [u8]) {
        self.crypt_hash().copy_to(dest);
    }
}

#[cfg(feature = "sha3hash")]
impl<T> Hashable for T
where
    T: AsRef<[u8]>,
{
    fn crypt_hash_into(&self, dest: &mut [u8]) {
        let input: &[u8] = self.as_ref();

        unsafe {
            sha3_256(dest.as_mut_ptr(), dest.len(), input.as_ptr(), input.len());
        }
    }
}

#[cfg(feature = "blake2bhash")]
impl<T> Hashable for T
where
    T: AsRef<[u8]>,
{
    fn crypt_hash_into(&self, dest: &mut [u8]) {
        let input: &[u8] = self.as_ref();

        unsafe {
            blake2b(
                dest.as_mut_ptr(),
                dest.len(),
                input.as_ptr(),
                input.len(),
                BLAKE2BKEY.as_bytes().as_ptr(),
                BLAKE2BKEY.len(),
            );
        }
    }
}

#[cfg(feature = "sm3hash")]
impl<T> Hashable for T
where
    T: AsRef<[u8]>,
{
    fn crypt_hash_into(&self, dest: &mut [u8]) {
        let input: &[u8] = self.as_ref();
        unsafe {
            sm3(input.as_ptr(), input.len(), dest.as_mut_ptr());
        }
    }
}

pub fn sha3(val: &[u8]) -> H256 {
    let out: &mut [u8; 32] = &mut [0; 32];
    let outptr = out.as_mut_ptr();
    unsafe {
        sha3_256(outptr, 32, val.as_ptr(), val.len());
    }
    H256::from_slice(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "sha3hash")]
    fn sha3_empty() {
        assert_eq!([0u8; 0].crypt_hash(), HASH_EMPTY);
    }
    #[test]
    #[cfg(feature = "sha3hash")]
    fn sha3_as() {
        assert_eq!(
            [0x41u8; 32].crypt_hash(),
            From::from("59cad5948673622c1d64e2322488bf01619f7ff45789741b15a9f782ce9290a8")
        );
    }

    #[test]
    #[cfg(feature = "blake2bhash")]
    fn blake2b_empty() {
        assert_eq!([0u8; 0].crypt_hash(), HASH_EMPTY);
    }
    #[test]
    #[cfg(feature = "blake2bhash")]
    fn blake2b_as() {
        assert_eq!(
            [0x41u8; 32].crypt_hash(),
            From::from("8a786e4840b7b5ad9b0cfa44539b886086c2e1050bb802c8e40ecf09b3a64a11")
        );
    }

    #[test]
    #[cfg(feature = "sm3hash")]
    fn test_sm3() {
        let hash = [0u8; 0].crypt_hash();
        assert_eq!(hash, HASH_EMPTY);

        let hash = [0x80; 1].crypt_hash();
        assert_eq!(hash, HASH_NULL_RLP);

        let hash = [0xC0; 1].crypt_hash();
        assert_eq!(hash, HASH_EMPTY_LIST_RLP);
    }
}
