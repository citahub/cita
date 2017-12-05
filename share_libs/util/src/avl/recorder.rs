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

//! AVL query recorder.

use Bytes;
use H256;
use hashable::Hashable;

/// A record of a visited node.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Record {
    /// The depth of this node.
    pub depth: u32,

    /// The raw data of the node.
    pub data: Bytes,

    /// The hash of the data.
    pub hash: H256,
}

/// Records avl nodes as they pass it.
#[derive(Debug)]
pub struct Recorder {
    nodes: Vec<Record>,
    min_depth: u32,
}

impl Default for Recorder {
    fn default() -> Self {
        Recorder::new()
    }
}

impl Recorder {
    /// Create a new `Recorder` which records all given nodes.
    #[inline]
    pub fn new() -> Self {
        Recorder::with_depth(0)
    }

    /// Create a `Recorder` which only records nodes beyond a given depth.
    pub fn with_depth(depth: u32) -> Self {
        Recorder {
            nodes: Vec::new(),
            min_depth: depth,
        }
    }

    /// Record a visited node, given its hash, data, and depth.
    pub fn record(&mut self, hash: &H256, data: &[u8], depth: u32) {
        debug_assert_eq!(data.crypt_hash(), *hash);

        if depth >= self.min_depth {
            self.nodes.push(Record {
                depth: depth,
                data: data.into(),
                hash: *hash,
            })
        }
    }

    /// Drain all visited records.
    pub fn drain(&mut self) -> Vec<Record> {
        ::std::mem::replace(&mut self.nodes, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sha3hash")]
    use H256;
    use hashable::Hashable;

    #[test]
    fn basic_recorder() {
        let mut basic = Recorder::new();

        let node1 = vec![1, 2, 3, 4];
        let node2 = vec![4, 5, 6, 7, 8, 9, 10];

        let (hash1, hash2) = (node1.crypt_hash(), node2.crypt_hash());
        basic.record(&hash1, &node1, 0);
        basic.record(&hash2, &node2, 456);

        let record1 = Record {
            data: node1,
            hash: hash1,
            depth: 0,
        };

        let record2 = Record {
            data: node2,
            hash: hash2,
            depth: 456,
        };

        assert_eq!(basic.drain(), vec![record1, record2]);
    }

    #[test]
    fn basic_recorder_min_depth() {
        let mut basic = Recorder::with_depth(400);

        let node1 = vec![1, 2, 3, 4];
        let node2 = vec![4, 5, 6, 7, 8, 9, 10];

        let hash1 = node1.crypt_hash();
        let hash2 = node2.crypt_hash();
        basic.record(&hash1, &node1, 0);
        basic.record(&hash2, &node2, 456);

        let records = basic.drain();

        assert_eq!(records.len(), 1);

        assert_eq!(
            records[0].clone(),
            Record {
                data: node2,
                hash: hash2,
                depth: 456,
            }
        );
    }

    #[test]
    #[cfg(feature = "sha3hash")]
    fn avl_record() {
        use avl::{AVLDBMut, AVLMut, AVL, AVLDB};
        use memorydb::MemoryDB;

        let mut db = MemoryDB::new();

        let mut root = H256::default();

        {
            let mut x = AVLDBMut::new(&mut db, &mut root);

            x.insert(b"dog", b"cat").unwrap();
            x.insert(b"hotdog", b"hotcat").unwrap();
            x.insert(b"insert", b"remove").unwrap();
            x.insert(b"letter", b"confusion").unwrap();
            x.insert(b"lunch", b"time").unwrap();
            x.insert(b"notdog", b"notcat").unwrap();
            x.insert(b"pirate", b"aargh!").unwrap();
            x.insert(b"yo ho ho", b"and a bottle of rum").unwrap();
        }

        let avl = AVLDB::new(&db, &root).unwrap();
        let mut recorder = Recorder::new();

        avl.get_with(b"pirate", &mut recorder).unwrap().unwrap();

        let nodes: Vec<_> = recorder.drain().into_iter().map(|r| r.data).collect();

        println!("avl: {:?}", avl);

        assert_eq!(
            nodes,
            vec![
                vec![
                    248,
                    73,
                    160,
                    181,
                    169,
                    217,
                    164,
                    151,
                    176,
                    64,
                    247,
                    69,
                    93,
                    121,
                    209,
                    96,
                    5,
                    119,
                    9,
                    203,
                    238,
                    2,
                    33,
                    133,
                    8,
                    239,
                    68,
                    237,
                    136,
                    150,
                    2,
                    147,
                    184,
                    126,
                    229,
                    160,
                    66,
                    145,
                    68,
                    88,
                    146,
                    191,
                    53,
                    149,
                    18,
                    202,
                    189,
                    109,
                    109,
                    70,
                    156,
                    28,
                    180,
                    55,
                    93,
                    52,
                    129,
                    146,
                    32,
                    157,
                    193,
                    60,
                    132,
                    214,
                    72,
                    187,
                    85,
                    14,
                    4,
                    133,
                    108,
                    117,
                    110,
                    99,
                    104,
                ],
                vec![
                    248,
                    74,
                    160,
                    235,
                    187,
                    181,
                    135,
                    42,
                    119,
                    116,
                    133,
                    135,
                    78,
                    83,
                    128,
                    229,
                    7,
                    81,
                    178,
                    124,
                    101,
                    109,
                    132,
                    214,
                    14,
                    168,
                    150,
                    176,
                    131,
                    209,
                    134,
                    118,
                    224,
                    53,
                    6,
                    160,
                    229,
                    219,
                    120,
                    166,
                    142,
                    233,
                    76,
                    223,
                    212,
                    158,
                    153,
                    93,
                    188,
                    126,
                    55,
                    74,
                    152,
                    164,
                    205,
                    208,
                    22,
                    134,
                    117,
                    63,
                    172,
                    19,
                    16,
                    157,
                    21,
                    37,
                    218,
                    201,
                    3,
                    134,
                    112,
                    105,
                    114,
                    97,
                    116,
                    101,
                ],
                vec![
                    247,
                    206,
                    134,
                    112,
                    105,
                    114,
                    97,
                    116,
                    101,
                    134,
                    97,
                    97,
                    114,
                    103,
                    104,
                    33,
                    221,
                    136,
                    121,
                    111,
                    32,
                    104,
                    111,
                    32,
                    104,
                    111,
                    147,
                    97,
                    110,
                    100,
                    32,
                    97,
                    32,
                    98,
                    111,
                    116,
                    116,
                    108,
                    101,
                    32,
                    111,
                    102,
                    32,
                    114,
                    117,
                    109,
                    2,
                    136,
                    121,
                    111,
                    32,
                    104,
                    111,
                    32,
                    104,
                    111,
                ],
            ]
        );


        avl.get_with(b"letter", &mut recorder).unwrap().unwrap();

        let nodes: Vec<_> = recorder.drain().into_iter().map(|r| r.data).collect();

        assert_eq!(
            nodes,
            vec![
                vec![
                    248,
                    73,
                    160,
                    181,
                    169,
                    217,
                    164,
                    151,
                    176,
                    64,
                    247,
                    69,
                    93,
                    121,
                    209,
                    96,
                    5,
                    119,
                    9,
                    203,
                    238,
                    2,
                    33,
                    133,
                    8,
                    239,
                    68,
                    237,
                    136,
                    150,
                    2,
                    147,
                    184,
                    126,
                    229,
                    160,
                    66,
                    145,
                    68,
                    88,
                    146,
                    191,
                    53,
                    149,
                    18,
                    202,
                    189,
                    109,
                    109,
                    70,
                    156,
                    28,
                    180,
                    55,
                    93,
                    52,
                    129,
                    146,
                    32,
                    157,
                    193,
                    60,
                    132,
                    214,
                    72,
                    187,
                    85,
                    14,
                    4,
                    133,
                    108,
                    117,
                    110,
                    99,
                    104,
                ],
                vec![
                    248,
                    74,
                    160,
                    95,
                    141,
                    166,
                    103,
                    63,
                    1,
                    92,
                    14,
                    158,
                    9,
                    87,
                    5,
                    60,
                    163,
                    192,
                    62,
                    124,
                    1,
                    58,
                    165,
                    30,
                    236,
                    3,
                    217,
                    125,
                    143,
                    196,
                    141,
                    56,
                    2,
                    65,
                    146,
                    160,
                    104,
                    173,
                    196,
                    176,
                    92,
                    45,
                    58,
                    53,
                    136,
                    24,
                    159,
                    65,
                    190,
                    17,
                    99,
                    1,
                    58,
                    153,
                    90,
                    126,
                    194,
                    236,
                    56,
                    65,
                    96,
                    51,
                    176,
                    214,
                    173,
                    92,
                    90,
                    101,
                    3,
                    134,
                    105,
                    110,
                    115,
                    101,
                    114,
                    116,
                ],
                vec![
                    233,
                    206,
                    134,
                    105,
                    110,
                    115,
                    101,
                    114,
                    116,
                    134,
                    114,
                    101,
                    109,
                    111,
                    118,
                    101,
                    209,
                    134,
                    108,
                    101,
                    116,
                    116,
                    101,
                    114,
                    137,
                    99,
                    111,
                    110,
                    102,
                    117,
                    115,
                    105,
                    111,
                    110,
                    2,
                    134,
                    108,
                    101,
                    116,
                    116,
                    101,
                    114,
                ],
            ]
        );
    }
}
