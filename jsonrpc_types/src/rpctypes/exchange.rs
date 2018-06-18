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

/// Structs for combine paramters and exchange between request handler and response handler.
use rpctypes::BlockNumber;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CountOrCode {
    pub address: ::std::vec::Vec<u8>,
    pub block_id: BlockNumber,
}

impl CountOrCode {
    pub fn new(address: Vec<u8>, block_id: BlockNumber) -> CountOrCode {
        CountOrCode {
            address: address,
            block_id: block_id,
        }
    }
}

impl Default for CountOrCode {
    fn default() -> CountOrCode {
        CountOrCode {
            address: vec![],
            block_id: BlockNumber::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockParamsByHash {
    pub hash: ::std::vec::Vec<u8>,
    pub include_txs: bool,
}

impl BlockParamsByHash {
    pub fn new(hash: Vec<u8>, include_txs: bool) -> BlockParamsByHash {
        BlockParamsByHash {
            hash: hash,
            include_txs: include_txs,
        }
    }
}

impl Default for BlockParamsByHash {
    fn default() -> BlockParamsByHash {
        BlockParamsByHash {
            hash: vec![],
            include_txs: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockParamsByNumber {
    pub block_id: BlockNumber,
    pub include_txs: bool,
}

impl BlockParamsByNumber {
    pub fn new(block_id: BlockNumber, include_txs: bool) -> BlockParamsByNumber {
        BlockParamsByNumber {
            block_id: block_id,
            include_txs: include_txs,
        }
    }
}

impl Default for BlockParamsByNumber {
    fn default() -> BlockParamsByNumber {
        BlockParamsByNumber {
            block_id: BlockNumber::default(),
            include_txs: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct RpcBlock {
    pub block: Vec<u8>,
    pub include_txs: bool,
    pub hash: Vec<u8>,
}

impl RpcBlock {
    pub fn new(hash: Vec<u8>, include_txs: bool, block: Vec<u8>) -> RpcBlock {
        RpcBlock {
            block: block,
            include_txs: include_txs,
            hash: hash,
        }
    }
}
