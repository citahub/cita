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

/// JSON-RPC Request.
use serde_json;
use std::convert::TryInto;

use jsonrpc_types_internals::construct_params;
use libproto::request::Request as ProtoRequest;

use error::Error;
use rpctypes::{Block, FilterChanges, Log, MetaData, Receipt, RpcTransaction, TxResponse};
use rpctypes::{
    BlockNumber, Boolean, CallRequest, Data, Data20, Data32, Filter, OneItemTupleTrick, Quantity,
};
use rpctypes::{Id, Params as PartialParams, Version};

pub type Logs = Vec<Log>;

#[derive(Debug, Clone, PartialEq)]
pub struct RequestInfo {
    pub jsonrpc: Option<Version>,
    pub id: Id,
}

impl RequestInfo {
    pub fn new(jsonrpc: Option<Version>, id: Id) -> Self {
        RequestInfo {
            jsonrpc: jsonrpc,
            id: id,
        }
    }
    pub fn null() -> Self {
        RequestInfo {
            jsonrpc: None,
            id: Id::Null,
        }
    }
}

impl Default for RequestInfo {
    fn default() -> Self {
        RequestInfo::new(Some(Version::default()), Id::default())
    }
}

/// JSON-RPC 2.0 Request object (http://www.jsonrpc.org/specification#request_object)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jsonrpc: Option<Version>,
    #[serde(default, skip_serializing_if = "Id::is_null")]
    pub id: Id,
    /// Contain method and params.
    #[serde(flatten)]
    pub call: Call,
}

impl Request {
    pub fn new(jsonrpc: Option<Version>, id: Id, call: Call) -> Self {
        Request { jsonrpc, id, call }
    }
    pub fn get_method(&self) -> &str {
        self.call.get_method()
    }
    pub fn get_info(&self) -> RequestInfo {
        RequestInfo::new(self.jsonrpc.clone(), self.id.clone())
    }
    pub fn into_proto(&self) -> Result<ProtoRequest, Error> {
        self.call.into_proto()
    }
}

impl Into<String> for Request {
    fn into(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PartialRequest {
    pub jsonrpc: Option<Version>,
    pub id: Id,
    /// Contain method and params.
    #[serde(flatten)]
    pub call: Option<PartialCall>,
}

impl PartialRequest {
    pub fn get_info(&self) -> RequestInfo {
        RequestInfo::new(self.jsonrpc.clone(), self.id.clone())
    }
    pub fn complete(self) -> Result<Request, Error> {
        let PartialRequest { jsonrpc, id, call } = self;
        if let Some(part_call) = call {
            part_call
                .complete()
                .map(|full_call| Request::new(jsonrpc, id, full_call))
        } else {
            Err(Error::method_not_found())
        }
    }

    pub fn complete_and_into_proto(self) -> Result<(Request, ProtoRequest), Error> {
        self.complete()
            .and_then(|full_req| full_req.into_proto().map(|proto_req| (full_req, proto_req)))
    }
}

macro_rules! define_call {
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ,) => {
        define_call!($( ($enum_name, $params_name: $params_list, $result_type) ),+);
    };
    ($( ($enum_name:ident, $params_name:ident: $params_list:expr, $result_type:ident) ),+ ) => {

        $(
            construct_params!($params_name: $params_list, $result_type);
        )+


        #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum ResponseResult {
            #[serde(rename = "null")]
            Null,
            $(
                $enum_name($result_type),
            )+
        }

        impl Default for ResponseResult {
            fn default() -> Self {
                ResponseResult::Null
            }
        }

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method", rename_all = "camelCase")]
        pub enum Call {
            $(
                $enum_name { params: $params_name},
            )+
        }

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method", rename_all = "camelCase")]
        pub enum PartialCall {
            $(
                $enum_name {
                    params: Option<serde_json::Value>
                },
            )+
        }

        impl Call {
            pub fn get_method(&self) -> &str {
                match self {
                    $(
                        &Call::$enum_name { ref params } => params.method_name(),
                    )+
                }
            }
            pub fn into_proto(&self) -> Result<ProtoRequest, Error> {
                match self {
                    $(
                        &Call::$enum_name { ref params } => params.clone().try_into(),
                    )+
                }
            }
            pub fn into_request(self, id: u64) -> Request {
                Request::new(
                    Some(Version::default()),
                    Id::Num(id),
                    self,
                )
            }
        }

        impl PartialCall {
            pub fn complete(self) -> Result<Call, Error> {
                match self {
                    $(
                        PartialCall::$enum_name { params } => {
                            if let Some(params) = params {
                                let pparams: PartialParams = serde_json::from_value(params.clone())?;
                                if pparams.len() != $params_name::required_len() {
                                    Err(Error::invalid_params_len())
                                } else {
                                    Ok(Call::$enum_name{ params: serde_json::from_value(params)? })
                                }
                            } else {
                                if $params_name::required_len() == 0 {
                                    Ok(Call::$enum_name{
                                        params: serde_json::from_value(
                                                    serde_json::Value::Array(Vec::new()))?})
                                } else {
                                    Err(Error::invalid_params("params is requeired"))
                                }
                            }
                        },
                    )+
                }
            }
        }

        $(
            impl Into<$params_name> for Call {
                fn into(self) -> $params_name{
                    if let Call::$enum_name{ params } = self {
                        params
                    } else {
                        // IMHO, in Rust, no static check can do this.
                        // If https://github.com/rust-lang/rfcs/pull/1450 merged,
                        // I think I can remove this panic.
                        panic!("The method and params are one to one correspondence.")
                    }
                }
            }

            impl From<$params_name> for Call {
                fn from(params: $params_name) -> Call {
                    Call::$enum_name{ params }
                }
            }

            impl $params_name {
                pub fn into_request(self, id: u64) -> Request {
                    Request::new(
                        Some(Version::default()),
                        Id::Num(id),
                        self.into(),
                    )
                }
            }
        )+
    };
}

pub trait JsonRpcRequest {
    type Response;
    fn required_len() -> usize;
    fn method_name(&self) -> &'static str;
    fn value_vec(self) -> Vec<serde_json::Value>;
}

// Q. How to add a JSON-RPC method?
//
// A.
//  First, add a tuple into the follow macro.
//
//    - The 1st item in tuple is a enum name used in `Call` / `PartialCall`.
//      The enum name will used to generate the JSON-RPC method name.
//      The enum name is PascalCase and JSON-RPC method name is camelCase.
//
//    - The 2st item is params type name and it's structure.
//      The params type has some methods, such as `new()` and `method_name()`.
//      More details can found in the definition of `construct_params`.
//
//    - The 3rd item is the type of result of Response object on success.
//
//  Second, implement `TryInto<ProtoRequest>` for the new params type.
//
//  DONE!
define_call!(
    (BlockNumber, BlockNumberParams: [], Quantity),
    (PeerCount, PeerCountParams: [], Quantity),
    (SendRawTransaction, SendRawTransactionParams: [Data], TxResponse),
    (SendTransaction, SendTransactionParams: [Data], TxResponse),
    (GetBlockByHash, GetBlockByHashParams: [Data32, Boolean], Block),
    (GetBlockByNumber, GetBlockByNumberParams: [BlockNumber, Boolean], Block),
    (GetTransactionReceipt, GetTransactionReceiptParams: [Data32], Receipt),
    (GetLogs, GetLogsParams: [Filter], Logs),
    (Call, CallParams: [CallRequest, BlockNumber], Data),
    (GetTransaction, GetTransactionParams: [Data32], RpcTransaction),
    (GetTransactionCount, GetTransactionCountParams: [Data20, BlockNumber], Quantity),
    (GetCode, GetCodeParams: [Data20, BlockNumber], Data),
    (GetAbi, GetAbiParams: [Data20, BlockNumber], Data),
    (GetBalance, GetBalanceParams: [Data20, BlockNumber], Quantity),
    (NewFilter, NewFilterParams: [Filter], Quantity),
    (NewBlockFilter, NewBlockFilterParams: [], Quantity),
    (UninstallFilter, UninstallFilterParams: [Quantity], Boolean),
    (GetFilterChanges, GetFilterChangesParams: [Quantity], FilterChanges),
    (GetFilterLogs, GetFilterLogsParams: [Quantity], Logs),
    (GetTransactionProof, GetTransactionProofParams: [Data32], Data),
    (GetMetaData, GetMetaDataParams: [BlockNumber], MetaData),
);
