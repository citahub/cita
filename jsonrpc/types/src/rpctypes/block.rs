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

use libproto::blockchain::{Commit as ProtoCommit, BlockHeader as ProtoBlockHeader};
use serde_types::hash::H256;
use proof::CitaProof;
use super::{BlockTransaction , FullTransaction, TransactionHash};
use serde_types::U256;
use libproto::request::RpcBlock;
use libproto::blockchain::Block as ProtoBlock;
use protobuf::core::parse_from_bytes;


#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Commit {
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockBody {
    pub transactions: Vec<BlockTransaction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub timestamp: u64,
    #[serde(rename = "prevHash")]
    pub prev_hash: H256,
    pub proof: CitaProof,
    pub commit: Commit,
    pub height: U256,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Block {
    pub version: u32,
    pub hash: H256,
    pub header: BlockHeader,
    pub body: BlockBody,
}

impl From<ProtoCommit> for Commit {
    fn from(commit: ProtoCommit) -> Self {
        let mut state_root: H256 = H256::default();
        state_root.0.clone_from_slice(commit.get_state_root());

        let mut transactions_root: H256 = H256::default();
        transactions_root.0.clone_from_slice(
            commit.get_transactions_root(),
        );

        let mut receipts_root: H256 = H256::default();
        receipts_root.0.clone_from_slice(commit.get_receipts_root());

        Commit {
            state_root: state_root,
            transactions_root: transactions_root,
            receipts_root: receipts_root,
            gas_used: U256::from(commit.get_gas_used()),
        }
    }
}

impl From<ProtoBlockHeader> for BlockHeader {
    fn from(proto_header: ProtoBlockHeader) -> Self {
        let mut prev_hash: H256 = H256::default();
        prev_hash.0.clone_from_slice(proto_header.get_prevhash());

        BlockHeader {
            timestamp: proto_header.timestamp,
            prev_hash: prev_hash,
            proof: proto_header.clone().take_proof().into(),
            commit: Commit::from(proto_header.clone().take_commit()),
            height: U256::from(proto_header.get_height()),
        }
    }
}

impl From<RpcBlock> for Block {
    fn from(block: RpcBlock) -> Self {
        let mut blk = parse_from_bytes::<ProtoBlock>(block.block.as_slice()).unwrap();
        let proto_header = blk.take_header();
        let mut proto_body = blk.take_body();
        let block_transactions = proto_body.take_transactions();
        let transactions;
        if block.include_txs {
            transactions = block_transactions
                .into_iter()
                .map(|x| BlockTransaction::Full(FullTransaction::from(x)))
                .collect()
        } else {
            transactions = block_transactions
                .into_iter()
                .map(|x| BlockTransaction::Hash(TransactionHash::from(x)))
                .collect()
        }

        let mut bhash: H256 = H256::default();
        bhash.0.clone_from_slice(block.hash.as_slice());
        Block {
            version: blk.version,
            header: BlockHeader::from(proto_header),
            body: BlockBody { transactions: transactions },
            hash: bhash,
        }
    }
}
