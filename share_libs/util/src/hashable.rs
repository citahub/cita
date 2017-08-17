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
#[cfg(feature = "sha3hash")]
use sha3::sha3_256;

/// The hash of the empty bytes string.
#[cfg(feature = "sha3hash")]
pub const HASH_EMPTY: H256 = H256(
    [
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
    ],
);
#[cfg(feature = "blake2bhash")]
pub const HASH_EMPTY: H256 = H256(
    [
        0x3f,
        0x7b,
        0x21,
        0x67,
        0x31,
        0xe3,
        0x61,
        0x92,
        0x29,
        0x04,
        0x5a,
        0x65,
        0x31,
        0x72,
        0xef,
        0xa8,
        0xfb,
        0x0b,
        0x74,
        0x60,
        0xf7,
        0x70,
        0x13,
        0xd9,
        0x70,
        0x41,
        0x32,
        0x7d,
        0x4f,
        0x29,
        0x79,
        0xeb,
    ],
);

/// The hash of the RLP encoding of empty data.
#[cfg(feature = "sha3hash")]
pub const HASH_NULL_RLP: H256 = H256(
    [
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
    ],
);
#[cfg(feature = "blake2bhash")]
pub const HASH_NULL_RLP: H256 = H256(
    [
        0x18,
        0xee,
        0x80,
        0xf6,
        0xc8,
        0xda,
        0xd1,
        0x15,
        0xda,
        0xf2,
        0x15,
        0xfd,
        0x4e,
        0xc3,
        0xee,
        0x6b,
        0x77,
        0x41,
        0x34,
        0x7b,
        0xd8,
        0x82,
        0xfa,
        0x18,
        0xc2,
        0x86,
        0x53,
        0xd5,
        0x2c,
        0x1f,
        0xa9,
        0x5e,
    ],
);

/// The hash of the RLP encoding of empty list.
#[cfg(feature = "sha3hash")]
pub const HASH_EMPTY_LIST_RLP: H256 = H256(
    [
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
    ],
);
#[cfg(feature = "blake2bhash")]
pub const HASH_EMPTY_LIST_RLP: H256 = H256(
    [
        0x67,
        0x51,
        0xe4,
        0x89,
        0xd1,
        0x97,
        0x6d,
        0x49,
        0x22,
        0xe9,
        0x21,
        0xda,
        0x09,
        0xa1,
        0x83,
        0xa3,
        0x6c,
        0x11,
        0xd2,
        0x5a,
        0x6f,
        0x31,
        0x86,
        0xb7,
        0xef,
        0x7d,
        0x02,
        0x07,
        0x5f,
        0xb7,
        0x17,
        0x2f,
    ],
);

#[cfg(feature = "blake2bhash")]
pub const BLAKE2BKEY: &str = "Cryptape";

pub trait Hashable {
    /// Calculate crypt HASH of this object.
    fn crypt_hash(&self) -> H256;

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
    fn crypt_hash(&self) -> H256 {
        let mut ret: H256 = H256::zero();
        self.crypt_hash_into(&mut *ret);
        ret
    }
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
    fn crypt_hash(&self) -> H256 {
        let mut ret: H256 = H256::zero();
        self.crypt_hash_into(&mut *ret);
        ret
    }
    fn crypt_hash_into(&self, dest: &mut [u8]) {
        let input: &[u8] = self.as_ref();

        unsafe {
            blake2b(dest.as_mut_ptr(), dest.len(), input.as_ptr(), input.len(), BLAKE2BKEY.as_bytes().as_ptr(), BLAKE2BKEY.len());
        }
    }
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
        assert_eq!([0x41u8; 32].crypt_hash(), From::from("59cad5948673622c1d64e2322488bf01619f7ff45789741b15a9f782ce9290a8"));
    }

    #[test]
    #[cfg(feature = "blake2bhash")]
    fn blake2b_empty() {
        assert_eq!([0u8; 0].crypt_hash(), HASH_EMPTY);
    }
    #[test]
    #[cfg(feature = "blake2bhash")]
    fn blake2b_as() {
        assert_eq!([0x41u8; 32].crypt_hash(), From::from("d2d8853149e5dfb62715f76c8fe27decf0638ee941658c131e1408e09fbd38a1"));
    }

}
