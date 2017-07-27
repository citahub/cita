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

use rpc_request::Version;
use libproto::request as reqlib;
use self::reqlib::Response_oneof_result as ResponseResult;
use libproto::blockchain::{TxResponse as ProtoTxResponse};
use serde_types::hash::H256;
use serde_types::U256;
use util::hash::H256 as EthH256;
use bytes::Bytes;
use Id;
use std::string::String;
use std::vec::Vec;
use rpctypes::{Receipt, Log};
use rpctypes::{Block};
use rpctypes::{RpcTransaction};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TxResponse {
    pub hash: H256,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResponseBody {
    BlockNumber(U256),
    FullBlock(Block),
    #[serde(rename="null")]
    Null,
    Receipt(Receipt),
    Transaction(RpcTransaction),
    TxResponse(TxResponse),
    PeerCount(U256),
    CallResult(Bytes),
    Logs(Vec<Log>),
    TranactionCount(U256),
    Code(Bytes),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResErrBody {
    pub code: isize,
    pub message: String,
}

pub struct RpcResult {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: ResErrBody,
    pub result: ResponseBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResErr {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: ResErrBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub result: ResponseBody,
}

impl From<RpcResult> for RpcResponse {
    fn from(result: RpcResult) -> Self {
        RpcResponse {
            jsonrpc: result.jsonrpc,
            id: result.id,
            result: result.result,
        }
    }
}

impl From<RpcResult> for ResErr {
    fn from(result: RpcResult) -> Self {
        ResErr {
            jsonrpc: result.jsonrpc,
            id: result.id,
            error: result.error,
        }
    }
}

// TODO: FIX?
impl From<ProtoTxResponse> for ResponseBody {
    fn from(transaction: ProtoTxResponse) -> Self {
        ResponseBody::TxResponse(TxResponse {
                                     hash: EthH256::from(transaction.hash.as_slice()).into(),
                                     status: String::from_utf8(transaction.result).unwrap(),
                                 })
    }
}

impl From<ResponseResult> for ResponseBody {
    fn from(res: ResponseResult) -> Self {
        match res {
            ResponseResult::block_number(bn) => ResponseBody::BlockNumber(U256::from(bn)),
            ResponseResult::none(_) => ResponseBody::Null,
            ResponseResult::block(rpc_block) => {
                ResponseBody::FullBlock(rpc_block.into())
            },
            ResponseResult::ts(x) => ResponseBody::Transaction(RpcTransaction::from(x)),
            ResponseResult::peercount(x) => ResponseBody::PeerCount(U256::from(x)),
            ResponseResult::call_result(x) => ResponseBody::CallResult(Bytes::from(x)),
            ResponseResult::logs(serialized) => serde_json::from_str::<Vec<Log>>(&serialized).ok().map_or(ResponseBody::Null, |logs| ResponseBody::Logs(logs)),
            ResponseResult::receipt(serialized) => serde_json::from_str::<Receipt>(&serialized).ok().map_or(ResponseBody::Null, |receipt| ResponseBody::Receipt(receipt)),
            ResponseResult::transaction_count(x) => {
                ResponseBody::TranactionCount(U256::from(x))
            }
            ResponseResult::code(x) => ResponseBody::Code(Bytes::from(x)),
    
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use rpc_request::Version;
    use Id;

    #[test]
    fn test_rpc_deserialize() {
        let rpc = RpcResponse {
            jsonrpc: Some(Version::V2),
            id: Id::Num(2),
            result: ResponseBody::Null,
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":2,"result":null}"#);
    }

    #[test]
    fn test_rpc_deserialize2() {
        let rpc = RpcResponse {
            jsonrpc: Some(Version::V2),
            id: Id::Str("2".to_string()),
            result: ResponseBody::BlockNumber(U256::from(3)),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":"2","result":"0x3"}"#);
    }
}
