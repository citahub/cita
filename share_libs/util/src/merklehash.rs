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

//! Generetes complete merkle tree root.
//!
//! This module should be used to generate complete merkle tree root hash.

use H256;
use hashable::{Hashable, HASH_NULL_RLP};
use rlp::*;

pub fn complete_merkle_root<I>(input: I) -> H256
where
    I: IntoIterator<Item = Vec<u8>>,
{
    let gen_input = input.into_iter().map(|v| v.crypt_hash()).collect();
    gen_merkle_root(gen_input)
}

pub fn complete_merkle_root_raw(input: Vec<H256>) -> H256 {
    gen_merkle_root(input)
}

fn lowest_children_len(amount: usize) -> usize {
    let mut n: usize = 1;
    let mut r: usize = 0;

    while n <= amount {
        r = amount - n;
        n <<= 1;
    }

    r << 1
}

fn gen_merkle_root(input: Vec<H256>) -> H256 {
    let inlen = input.len();
    // in case of empty slice, just return HASH_NULL_RLP
    if inlen == 0 {
        return HASH_NULL_RLP;
    }

    let lwlen = lowest_children_len(inlen);
    let mut i: usize = 0;
    let mut nodes = Vec::new();

    while i < lwlen {
        nodes.push(merge(&input[i], &input[i + 1]));
        i += 2;
    }

    for j in i..inlen {
        nodes.push(input[j]);
    }

    let nlen = nodes.len();
    let mut d = 1;
    while d < nlen {
        let mut j = 0;
        while j < nlen {
            nodes[j] = merge(&nodes[j], &nodes[j + d]);
            j += d + d;
        }
        d <<= 1;
    }

    nodes[0]
}

fn merge(left: &H256, right: &H256) -> H256 {
    let mut stream = RlpStream::new();
    stream.append(left);
    stream.append(right);
    stream.out().crypt_hash()
}

#[cfg(test)]
#[cfg(feature = "sha3hash")]
mod tests {
    use super::complete_merkle_root;
    use super::complete_merkle_root_raw;
    use H256;
    use std::str::FromStr;

    #[test]
    fn complete_test() {
        assert_eq!(
            complete_merkle_root(vec![
                b"A".to_vec(),
                b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_vec(),
            ]),
            H256::from_str("9bd41e0d43f4ec7a703edc2eb9fbb4106e1bc2a845e9ee1d4f3f4cf99b8549e6").unwrap()
        );

        assert_eq!(
            complete_merkle_root(vec![
                b"A".to_vec(),
                b"aaaa".to_vec(),
                b"abaa".to_vec(),
                b"aaba".to_vec(),
                b"aaab".to_vec(),
            ]),
            H256::from_str("8e827ab731f2416f6057b9c7f241b1841e345ffeabb4274e35995a45f4d42a1a").unwrap()
        );

        assert_eq!(
            complete_merkle_root(vec![]),
            H256::from_str("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421").unwrap()
        );

        assert_eq!(
            complete_merkle_root(vec![
                b"a".to_vec(),
                b"b".to_vec(),
                b"c".to_vec(),
                b"d".to_vec(),
                b"e".to_vec(),
                b"f".to_vec(),
                b"g".to_vec(),
            ]),
            H256::from_str("768dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1").unwrap()
        );
    }

    #[test]
    fn complete_test_raw() {
        assert_eq!(
            complete_merkle_root_raw(vec![
                H256::from_str("8e827ab731f2416f6057b9c7f241b1841e345ffeabb4274e35995a45f4d42a1a").unwrap(),
                H256::from_str("768dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1").unwrap(),
                H256::from_str("e68dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1").unwrap(),
                H256::from_str("f68dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1").unwrap(),
                H256::from_str("968dfb4ca3311fa3bf4d696dde334e30edf3542e8ea114a4f9d18fb34365f1d1").unwrap(),
            ]),
            H256::from_str("e30a149e738cfaf89fb3a2267d7109a1bda978320426c2ff8b3a2d77aa103a6a").unwrap()
        );
    }

}
