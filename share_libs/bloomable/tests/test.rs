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

extern crate bigint;
extern crate bloomable;
extern crate tiny_keccak;

use bigint::{H160, H2048, H256};
use bloomable::Bloomable;
use tiny_keccak::keccak256;

fn sha3(input: &[u8]) -> H256 {
    keccak256(input).into()
}

#[test]
fn shift_bloomed() {
    let bloom: H2048 = "000000000000000000000000000000000000000\
                        010000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        000020200000000000000000000000000000000\
                        000000000000080000000010000000000000000\
                        000000000000000000000000000000000000010\
                        000000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        000000000000000000000000000000000000000\
                        00000"
        .into();
    let address: H160 = "ef2d6d194084c2de36e0dabfce45d046b37d1106".into();
    let topic: H256 = "02c69be41d0b7e40352fc85be1cd65eb03d40ef8427a0ca4596b1ead9a00e9fc".into();

    let mut my_bloom = H2048::default();
    assert!(!my_bloom.contains_bloomed(&sha3(&address)));
    assert!(!my_bloom.contains_bloomed(&sha3(&topic)));

    my_bloom.shift_bloomed(&sha3(&address));
    assert!(my_bloom.contains_bloomed(&sha3(&address)));
    assert!(!my_bloom.contains_bloomed(&sha3(&topic)));

    my_bloom.shift_bloomed(&sha3(&topic));
    assert_eq!(my_bloom, bloom);
    assert!(my_bloom.contains_bloomed(&sha3(&address)));
    assert!(my_bloom.contains_bloomed(&sha3(&topic)));
}
