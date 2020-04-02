// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cita_types::H256;
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
