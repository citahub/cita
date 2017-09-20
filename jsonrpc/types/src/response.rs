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
use error::Error;
use libproto::TxResponse;
use libproto::response::{Response_oneof_data, Response};
use request::Version;
use rpctypes::{Receipt, Log, RpcTransaction, Block, RpcBlock};
use serde::{Serializer, Deserializer, Deserialize, Serialize};
use serde::de::Error as SError;
use serde_json;
use serde_json::{Value, from_value};
use std::vec::Vec;
use util::U256;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum RusultBody {
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
    ContractCode(Bytes),
    FilterId(U256),
    UninstallFliter(bool),
    FilterChanges(Vec<Log>),
    FilterLog(Vec<Log>),
}


impl Default for RusultBody {
    fn default() -> Self {
        RusultBody::Null
    }
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
    pub result: RusultBody,
}


impl RpcSuccess {
    pub fn new(id: Id, jsonrpc: Option<Version>) -> RpcSuccess {
        RpcSuccess {
            id: id,
            jsonrpc: jsonrpc,
            result: RusultBody::default(),
        }
    }

    pub fn set_result(mut self, reuslt: RusultBody) -> RpcSuccess {
        self.result = reuslt;
        self
    }

    pub fn to_out(self) -> Output {
        Output::Success(self)
    }
}


#[derive(Debug)]
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
                        success.set_result(RusultBody::TxResponse(tx_response)).to_out()
                    }
                    Response_oneof_data::block_number(bn) => success.set_result(RusultBody::BlockNumber(U256::from(bn))).to_out(),
                    Response_oneof_data::none(_) => success.to_out(),
                    Response_oneof_data::block(rpc_block) => {
                        let rpc_block: RpcBlock = serde_json::from_str(&rpc_block).unwrap();
                        success.set_result(RusultBody::FullBlock(rpc_block.into())).to_out()
                    }
                    Response_oneof_data::ts(x) => success.set_result(RusultBody::Transaction(RpcTransaction::from(x))).to_out(),
                    Response_oneof_data::peercount(x) => success.set_result(RusultBody::PeerCount(U256::from(x))).to_out(),
                    Response_oneof_data::call_result(x) => success.set_result(RusultBody::CallResult(Bytes::from(x))).to_out(),
                    Response_oneof_data::logs(serialized) => success.set_result(RusultBody::Logs(serde_json::from_str::<Vec<Log>>(&serialized).unwrap()))
                                                                    .to_out(),
                    Response_oneof_data::receipt(serialized) => {
                        success.set_result(serde_json::from_str::<Receipt>(&serialized)
                                               .ok()
                                               .map_or(RusultBody::Null, |receipt| RusultBody::Receipt(receipt)))
                               .to_out()
                    }
                    Response_oneof_data::transaction_count(x) => success.set_result(RusultBody::TranactionCount(U256::from(x))).to_out(),
                    Response_oneof_data::contract_code(x) => success.set_result(RusultBody::ContractCode(Bytes::from(x))).to_out(),
                    Response_oneof_data::filter_id(id) => success.set_result(RusultBody::FilterId(U256::from(id))).to_out(),
                    Response_oneof_data::uninstall_filter(is_uninstall) => success.set_result(RusultBody::UninstallFliter(is_uninstall)).to_out(),
                    Response_oneof_data::filter_changes(log) => success.set_result(RusultBody::FilterChanges(serde_json::from_str::<Vec<Log>>(&log).unwrap()))
                                                                       .to_out(),
                    Response_oneof_data::filter_logs(log) => success.set_result(RusultBody::FilterLog(serde_json::from_str::<Vec<Log>>(&log).unwrap())).to_out(),
                    Response_oneof_data::error_msg(err_msg) => Output::Failure(RpcFailure::from_options(id.clone(), jsonrpc.clone(), Error::server_error(code, err_msg.as_ref()))),
                }
            }
            _ => {
                match data.data.unwrap() {
                    Response_oneof_data::error_msg(err_msg) => Output::Failure(RpcFailure::from_options(id.clone(), jsonrpc.clone(), Error::server_error(code, err_msg.as_ref()))),
                    _ => {
                        error!("return error message!!!");
                        Output::Failure(RpcFailure::from(Error::server_error(code, "system error!")))
                    }
                }
            }
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
            result: RusultBody::Null,
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":2,"result":null}"#);
    }

    #[test]
    fn test_rpc_deserialize2() {
        let rpc = RpcSuccess {
            jsonrpc: Some(Version::V2),
            id: Id::Str("2".to_string()),
            result: RusultBody::BlockNumber(U256::from(3)),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","id":"2","result":"0x3"}"#);
    }
}
