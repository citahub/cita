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

use Id;
use bytes::Bytes;
use cita_types::{H256, U256};
use error::Error;
use libproto::response::{Response, Response_oneof_data};
use request::Version;
use rpctypes::{Block, FilterChanges, Log, MetaData, Receipt, RpcBlock, RpcTransaction, TxResponse};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error as SError;
use serde_json;
use serde_json::{from_value, Value};
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultBody {
    BlockNumber(U256),
    FullBlock(Block),
    #[serde(rename = "null")] Null,
    Receipt(Receipt),
    Transaction(RpcTransaction),
    TxResponse(TxResponse),
    PeerCount(U256),
    CallResult(Bytes),
    Logs(Vec<Log>),
    TranactionCount(U256),
    ContractCode(Bytes),
    ContractAbi(Bytes),
    FilterId(U256),
    UninstallFliter(bool),
    FilterChanges(FilterChanges),
    FilterLog(Vec<Log>),
    TxProof(Bytes),
    MetaData(MetaData),
    Balance(U256),
}

impl Default for ResultBody {
    fn default() -> Self {
        ResultBody::Null
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcFailure {
    #[serde(skip_serializing_if = "Option::is_none")] pub jsonrpc: Option<Version>,
    pub id: Id,
    pub error: Error,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RpcSuccess {
    #[serde(skip_serializing_if = "Option::is_none")] pub jsonrpc: Option<Version>,
    pub id: Id,
    pub result: ResultBody,
}

impl RpcSuccess {
    pub fn new(id: Id, jsonrpc: Option<Version>) -> RpcSuccess {
        RpcSuccess {
            id: id,
            jsonrpc: jsonrpc,
            result: ResultBody::default(),
        }
    }

    pub fn set_result(mut self, reuslt: ResultBody) -> RpcSuccess {
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
    pub fn from(data: Response, id: Id, jsonrpc: Option<Version>) -> Self {
        let success = RpcSuccess::new(id.clone(), jsonrpc.clone());
        let code = data.get_code();
        match code {
            0 => {
                //success
                match data.data.unwrap() {
                    Response_oneof_data::tx_state(tx_state) => {
                        let tx_response = serde_json::from_str(&tx_state).unwrap();
                        success
                            .set_result(ResultBody::TxResponse(tx_response))
                            .output()
                    }
                    Response_oneof_data::block_number(bn) => success
                        .set_result(ResultBody::BlockNumber(U256::from(bn)))
                        .output(),
                    Response_oneof_data::none(_) => success.output(),
                    Response_oneof_data::block(rpc_block) => {
                        let rpc_block: RpcBlock = serde_json::from_str(&rpc_block).unwrap();
                        success
                            .set_result(ResultBody::FullBlock(rpc_block.into()))
                            .output()
                    }
                    Response_oneof_data::ts(x) => success
                        .set_result(ResultBody::Transaction(RpcTransaction::from(x)))
                        .output(),
                    Response_oneof_data::peercount(x) => success
                        .set_result(ResultBody::PeerCount(U256::from(x)))
                        .output(),
                    Response_oneof_data::call_result(x) => success
                        .set_result(ResultBody::CallResult(Bytes::from(x)))
                        .output(),
                    Response_oneof_data::logs(serialized) => success
                        .set_result(ResultBody::Logs(
                            serde_json::from_str::<Vec<Log>>(&serialized).unwrap(),
                        ))
                        .output(),
                    Response_oneof_data::receipt(serialized) => success
                        .set_result(
                            serde_json::from_str::<Receipt>(&serialized)
                                .ok()
                                .map_or(ResultBody::Null, ResultBody::Receipt),
                        )
                        .output(),
                    Response_oneof_data::transaction_count(x) => success
                        .set_result(ResultBody::TranactionCount(U256::from(x)))
                        .output(),
                    Response_oneof_data::contract_code(x) => success
                        .set_result(ResultBody::ContractCode(Bytes::from(x)))
                        .output(),
                    Response_oneof_data::contract_abi(x) => success
                        .set_result(ResultBody::ContractAbi(Bytes::from(x)))
                        .output(),
                    Response_oneof_data::balance(x) => success
                        .set_result(ResultBody::Balance(U256::from(H256::from(x.as_slice()))))
                        .output(),
                    Response_oneof_data::filter_id(id) => success
                        .set_result(ResultBody::FilterId(U256::from(id)))
                        .output(),
                    Response_oneof_data::uninstall_filter(is_uninstall) => success
                        .set_result(ResultBody::UninstallFliter(is_uninstall))
                        .output(),
                    Response_oneof_data::filter_changes(data) => {
                        let changes =
                            serde_json::from_str::<FilterChanges>(&data).expect("failed to parse into FilterChanges");
                        success
                            .set_result(ResultBody::FilterChanges(changes))
                            .output()
                    }
                    Response_oneof_data::filter_logs(log) => success
                        .set_result(ResultBody::FilterLog(
                            serde_json::from_str::<Vec<Log>>(&log).unwrap(),
                        ))
                        .output(),
                    Response_oneof_data::transaction_proof(proof) => success
                        .set_result(ResultBody::TxProof(Bytes::from(proof)))
                        .output(),
                    Response_oneof_data::error_msg(err_msg) => Output::Failure(RpcFailure::from_options(
                        id.clone(),
                        jsonrpc.clone(),
                        Error::server_error(code, err_msg),
                    )),
                    Response_oneof_data::meta_data(data) => success
                        .set_result(ResultBody::MetaData(
                            serde_json::from_str::<MetaData>(&data).unwrap(),
                        ))
                        .output(),
                }
            }
            _ => match data.data.unwrap() {
                Response_oneof_data::error_msg(err_msg) => Output::Failure(RpcFailure::from_options(
                    id.clone(),
                    jsonrpc.clone(),
                    Error::server_error(code, err_msg),
                )),
                _ => {
                    error!("return system error!!!");
                    Output::Failure(RpcFailure::from(Error::server_error(code, "system error!")))
                }
            },
        }
    }

    /// Creates new failure output indicating malformed request.
    pub fn invalid_request(id: Id, jsonrpc: Option<Version>) -> Self {
        Output::Failure(RpcFailure {
            id: id,
            jsonrpc: jsonrpc,
            error: Error::invalid_request(),
        })
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
    use super::*;
    use Id;
    use request::Version;
    use serde_json;

    #[test]
    fn test_rpc_deserialize() {
        let rpc = RpcSuccess {
            jsonrpc: Some(Version::V2),
            id: Id::Num(2),
            result: ResultBody::Null,
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":2,"result":null}"#);
    }

    #[test]
    fn test_rpc_deserialize2() {
        let rpc = RpcSuccess {
            jsonrpc: Some(Version::V2),
            id: Id::Str("2".to_string()),
            result: ResultBody::BlockNumber(U256::from(3)),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":"2","result":"0x3"}"#);
    }
}
