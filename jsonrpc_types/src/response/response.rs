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

use error::Error;
use libproto::response::{Response, Response_oneof_data};
use request::{RequestInfo, ResponseResult};
use rpctypes::{FilterChanges, Id, Log, MetaData, Receipt, RpcBlock, RpcTransaction, Version};
use serde::de::Error as SError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use serde_json::{from_value, Value};
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcFailure {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: Error,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcSuccess {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    pub result: ResponseResult,
}

impl RpcSuccess {
    pub fn new(info: RequestInfo) -> Self {
        RpcSuccess {
            jsonrpc: info.jsonrpc,
            id: info.id,
            result: ResponseResult::default(),
        }
    }

    pub fn set_result(mut self, reuslt: ResponseResult) -> Self {
        self.result = reuslt;
        self
    }

    pub fn output(self) -> Output {
        Output::Success(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Output {
    /// Success
    Success(RpcSuccess),
    /// Failure
    Failure(RpcFailure),
}

impl Output {
    /// Creates new output given `Result`, `Id` and `Version`.
    pub fn from(data: Response, info: RequestInfo) -> Self {
        let code = data.get_code();
        match code {
            0 => {
                let success = RpcSuccess::new(info.clone());
                //success
                match data.data.unwrap() {
                    Response_oneof_data::tx_state(tx_state) => {
                        let tx_response = serde_json::from_str(&tx_state).unwrap();
                        // SendTransaction, SendRawTransaction
                        success
                            .set_result(ResponseResult::SendTransaction(tx_response))
                            .output()
                    }
                    Response_oneof_data::block_number(bn) => success
                        .set_result(ResponseResult::BlockNumber(bn.into()))
                        .output(),
                    Response_oneof_data::none(_) => success.output(),
                    Response_oneof_data::block(rpc_block) => {
                        let rpc_block: RpcBlock = serde_json::from_str(&rpc_block).unwrap();
                        // GetBlockByHash, GetBlockByNumber
                        success
                            .set_result(ResponseResult::GetBlockByHash(rpc_block.into()))
                            .output()
                    }
                    Response_oneof_data::ts(x) => success
                        .set_result(ResponseResult::GetTransaction(RpcTransaction::from(x)))
                        .output(),
                    Response_oneof_data::peercount(x) => success
                        .set_result(ResponseResult::PeerCount(x.into()))
                        .output(),
                    Response_oneof_data::call_result(x) => {
                        success.set_result(ResponseResult::Call(x.into())).output()
                    }
                    Response_oneof_data::logs(serialized) => success
                        .set_result(ResponseResult::GetLogs(
                            serde_json::from_str::<Vec<Log>>(&serialized).unwrap(),
                        ))
                        .output(),
                    Response_oneof_data::receipt(serialized) => {
                        success
                            .set_result(serde_json::from_str::<Receipt>(&serialized).ok().map_or(
                                ResponseResult::Null,
                                ResponseResult::GetTransactionReceipt,
                            ))
                            .output()
                    }
                    Response_oneof_data::transaction_count(x) => success
                        .set_result(ResponseResult::GetTransactionCount(x.into()))
                        .output(),
                    Response_oneof_data::contract_code(x) => success
                        .set_result(ResponseResult::GetCode(x.into()))
                        .output(),
                    Response_oneof_data::contract_abi(x) => success
                        .set_result(ResponseResult::GetAbi(x.into()))
                        .output(),
                    Response_oneof_data::balance(x) => success
                        .set_result(ResponseResult::GetBalance(x.as_slice().into()))
                        .output(),
                    Response_oneof_data::filter_id(id) => {
                        // NewFilter, NewBlockFilter
                        success
                            .set_result(ResponseResult::NewFilter(id.into()))
                            .output()
                    }
                    Response_oneof_data::uninstall_filter(is_uninstall) => success
                        .set_result(ResponseResult::UninstallFilter(is_uninstall.into()))
                        .output(),
                    Response_oneof_data::filter_changes(data) => {
                        let changes = serde_json::from_str::<FilterChanges>(&data)
                            .expect("failed to parse into FilterChanges");
                        success
                            .set_result(ResponseResult::GetFilterChanges(changes))
                            .output()
                    }
                    Response_oneof_data::filter_logs(log) => success
                        .set_result(ResponseResult::GetFilterLogs(
                            serde_json::from_str::<Vec<Log>>(&log).unwrap(),
                        ))
                        .output(),
                    Response_oneof_data::transaction_proof(proof) => success
                        .set_result(ResponseResult::GetTransactionProof(proof.into()))
                        .output(),
                    Response_oneof_data::error_msg(err_msg) => Output::Failure(
                        RpcFailure::from_options(info, Error::server_error(code, err_msg)),
                    ),
                    Response_oneof_data::meta_data(data) => success
                        .set_result(ResponseResult::GetMetaData(
                            serde_json::from_str::<MetaData>(&data).unwrap(),
                        ))
                        .output(),
                }
            }
            _ => match data.data.unwrap() {
                Response_oneof_data::error_msg(err_msg) => Output::Failure(
                    RpcFailure::from_options(info, Error::server_error(code, err_msg)),
                ),
                _ => {
                    error!("return system error!!!");
                    Output::Failure(RpcFailure::from(Error::server_error(code, "system error!")))
                }
            },
        }
    }

    /// Creates new failure output indicating malformed request.
    pub fn invalid_request(info: RequestInfo) -> Self {
        Output::Failure(RpcFailure::from_options(info, Error::invalid_request()))
    }
}

impl<'a> Deserialize<'a> for Output {
    fn deserialize<D>(deserializer: D) -> Result<Output, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(Output::Failure)
            .or_else(|_| from_value(v).map(Output::Success))
            .map_err(|_| D::Error::custom("")) // types must match
    }
}

impl Serialize for Output {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Output::Success(ref s) => s.serialize(serializer),
            Output::Failure(ref f) => f.serialize(serializer),
        }
    }
}

impl From<Error> for RpcFailure {
    fn from(err: Error) -> Self {
        RpcFailure::from_options(RequestInfo::null(), err)
    }
}

impl RpcFailure {
    pub fn from_options(info: RequestInfo, err: Error) -> Self {
        RpcFailure {
            jsonrpc: info.jsonrpc,
            id: info.id,
            error: err,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RpcResponse {
    /// Single response
    Single(Output),
    /// Response to batch request (batch of responses)
    Batch(Vec<Output>),
}

impl<'a> Deserialize<'a> for RpcResponse {
    fn deserialize<D>(deserializer: D) -> Result<RpcResponse, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v: Value = Deserialize::deserialize(deserializer)?;
        from_value(v.clone())
            .map(RpcResponse::Batch)
            .or_else(|_| from_value(v).map(RpcResponse::Single))
            .map_err(|_| D::Error::custom("")) // types must match
    }
}

impl Serialize for RpcResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            RpcResponse::Single(ref o) => o.serialize(serializer),
            RpcResponse::Batch(ref b) => b.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RpcSuccess;
    use request::{RequestInfo, ResponseResult};
    use rpctypes::{Id, Version};
    use serde_json;

    #[test]
    fn test_rpc_deserialize() {
        let rpc = RpcSuccess::new(RequestInfo::new(Some(Version::V2), Id::Num(2)))
            .set_result(ResponseResult::Null);

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":2,"result":null}"#);
    }

    #[test]
    fn test_rpc_deserialize2() {
        let rpc = RpcSuccess::new(RequestInfo::new(
            Some(Version::V2),
            Id::Str("2".to_string()),
        )).set_result(ResponseResult::BlockNumber(3u64.into()));

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":"2","result":"0x3"}"#);
    }
}
