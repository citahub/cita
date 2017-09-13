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

use super::{BlockTransaction, FullTransaction, TransactionHash};
use super::RpcBlock;
use libproto::blockchain::Block as ProtoBlock;
use libproto::blockchain::BlockHeader as ProtoBlockHeader;
use proof::CitaProof;
use protobuf::core::parse_from_bytes;
use util::{H256, U256};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockBody {
    pub transactions: Vec<BlockTransaction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub timestamp: u64,
    #[serde(rename = "prevHash")]
    pub prev_hash: H256,
    pub number: U256,
    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    pub gas_limit: U256,
    pub proof: Option<CitaProof>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Block {
    pub version: u32,
    pub hash: H256,
    pub header: BlockHeader,
    pub body: BlockBody,
}

impl From<ProtoBlockHeader> for BlockHeader {
    fn from(proto_header: ProtoBlockHeader) -> Self {
        let proof = match proto_header.get_height() {
            0 => None,
            _ => Some(proto_header.clone().take_proof().into()),
        };
        trace!("number = {}, proof = {:?}", U256::from(proto_header.get_height()), proof);

        BlockHeader {
            timestamp: proto_header.timestamp,
            prev_hash: H256::from(proto_header.get_prevhash()),
            number: U256::from(proto_header.get_height()),
            state_root: H256::from(proto_header.get_state_root()),
            transactions_root: H256::from(proto_header.get_transactions_root()),
            receipts_root: H256::from(proto_header.get_receipts_root()),
            gas_used: U256::from(proto_header.get_gas_used()),
            gas_limit: U256::from(proto_header.get_gas_limit()),
            proof: proof,
        }
    }
}

impl From<RpcBlock> for Block {
    fn from(block: RpcBlock) -> Self {
        let mut blk = parse_from_bytes::<ProtoBlock>(block.block.as_slice()).unwrap();
        let proto_header = blk.take_header();
        let mut proto_body = blk.take_body();
        let block_transactions = proto_body.take_transactions();
        let transactions = match block.include_txs {
            true => block_transactions.into_iter().map(|x| BlockTransaction::Full(FullTransaction::from(x))).collect(),
            false => block_transactions.into_iter().map(|x| BlockTransaction::Hash(TransactionHash::from(x))).collect(),
        };

        Block {
            version: blk.version,
            header: BlockHeader::from(proto_header),
            body: BlockBody { transactions: transactions },
            hash: H256::from_slice(&block.hash),
        }
    }
}
