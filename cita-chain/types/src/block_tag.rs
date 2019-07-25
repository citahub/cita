use cita_types::{H256, U256};
use jsonrpc_types::rpc_types::{BlockNumber as RpcBlockNumber, BlockTag as RpcBlockTag};

pub type TransactionHash = H256;
pub type BlockNumber = u64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BlockTag {
    Tag(Tag),
    Height(u64),
    Hash(H256),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tag {
    Latest,
    Earliest,
    Pending,
}

impl From<RpcBlockNumber> for BlockTag {
    fn from(n: RpcBlockNumber) -> BlockTag {
        match n {
            RpcBlockNumber::Height(height) => BlockTag::Height(height.into()),
            RpcBlockNumber::Tag(RpcBlockTag::Latest) => BlockTag::Tag(Tag::Latest),
            RpcBlockNumber::Tag(RpcBlockTag::Earliest) => BlockTag::Tag(Tag::Earliest),
            RpcBlockNumber::Tag(RpcBlockTag::Pending) => BlockTag::Tag(Tag::Pending),
        }
    }
}
