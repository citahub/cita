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

use self::reqlib::Response_oneof_result as ResponseResult;
use Id;
use bytes::Bytes;
use error::Error;
use libproto::blockchain::TxResponse as ProtoTxResponse;
use libproto::request as reqlib;
use request::Version;
use rpctypes::{Receipt, Log, RpcTransaction, Block, RpcBlock};
use serde_json;
use std::string::String;
use std::vec::Vec;
use util::{H256, U256};


//TODO respone contain error
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
    #[serde(rename = "null")]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcFailure {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcSuccess {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub result: ResponseBody,
}


// TODO: FIX?
impl From<ProtoTxResponse> for ResponseBody {
    fn from(transaction: ProtoTxResponse) -> Self {
        ResponseBody::TxResponse(TxResponse {
                                     hash: H256::from(transaction.hash.as_slice()).into(),
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
                let rpc_block: RpcBlock = serde_json::from_str(&rpc_block).unwrap();
                ResponseBody::FullBlock(rpc_block.into())
            }
            ResponseResult::ts(x) => ResponseBody::Transaction(RpcTransaction::from(x)),
            ResponseResult::peercount(x) => ResponseBody::PeerCount(U256::from(x)),
            ResponseResult::call_result(x) => ResponseBody::CallResult(Bytes::from(x)),
            ResponseResult::logs(serialized) => {
                serde_json::from_str::<Vec<Log>>(&serialized)
                    .ok()
                    .map_or(ResponseBody::Null, |logs| ResponseBody::Logs(logs))
            }
            ResponseResult::receipt(serialized) => {
                serde_json::from_str::<Receipt>(&serialized)
                    .ok()
                    .map_or(ResponseBody::Null, |receipt| ResponseBody::Receipt(receipt))
            }
            ResponseResult::transaction_count(x) => ResponseBody::TranactionCount(U256::from(x)),
            ResponseResult::code(x) => ResponseBody::Code(Bytes::from(x)),

        }
    }
}

impl From<Error> for RpcFailure {
    fn from(err: Error) -> Self {
        RpcFailure {
            id: Id::Null,
            jsonrpc: None,
            error: err,
        }
    }
}

impl RpcFailure {
    pub fn from_options(id: Id, jsonrpc: Option<Version>, err: Error) -> RpcFailure {
        RpcFailure {
            id: id,
            jsonrpc: jsonrpc,
            error: err,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use Id;
    use request::Version;
    use serde_json;

    #[test]
    fn test_rpc_deserialize() {
        let rpc = RpcSuccess {
            jsonrpc: Some(Version::V2),
            id: Id::Num(2),
            result: ResponseBody::Null,
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":2,"result":null}"#);
    }

    #[test]
    fn test_rpc_deserialize2() {
        let rpc = RpcSuccess {
            jsonrpc: Some(Version::V2),
            id: Id::Str("2".to_string()),
            result: ResponseBody::BlockNumber(U256::from(3)),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":"2","result":"0x3"}"#);
    }
}
