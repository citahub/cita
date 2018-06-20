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

use libproto::request::Request as ProtoRequest;

use super::params::{
    BlockNumberParams, CallParams, GetAbiParams, GetBalanceParams, GetBlockByHashParams,
    GetBlockByNumberParams, GetCodeParams, GetFilterChangesParams, GetFilterLogsParams,
    GetLogsParams, GetMetaDataParams, GetTransactionCountParams, GetTransactionParams,
    GetTransactionProofParams, GetTransactionReceiptParams, NewBlockFilterParams, NewFilterParams,
    PeerCountParams, SendRawTransactionParams, SendTransactionParams, UninstallFilterParams,
};
use error::Error;
use rpctypes::{Id, Params as PartialParams, Version};

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
            id: Id::Num(1),
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
    pub jsonrpc: Option<Version>,
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
    ($( ($name:ident, $rpcname:expr, $params_len:expr) ),+ ,) => {
        define_call!($( ($name, $rpcname, $params_len) ),+);
    };
    ($( ($name:ident, $rpcname:expr, $params_len:expr) ),+ ) => {

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method")]
        pub enum Call {
            $(
                #[serde(rename = $rpcname)]
                $name { params: concat_idents!($name, Params) },
            )+
        }

        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        #[serde(tag = "method")]
        pub enum PartialCall {
            $(
                #[serde(rename = $rpcname)]
                $name {
                    params: Option<serde_json::Value>
                },
            )+
        }

        impl Call {
            pub fn get_method(&self) -> &str {
                match self {
                    $(
                        &Call::$name { params: _ } => $rpcname,
                    )+
                }
            }
            pub fn into_proto(&self) -> Result<ProtoRequest, Error> {
                match self {
                    $(
                        &Call::$name { ref params } => params.clone().try_into(),
                    )+
                }
            }
        }

        impl PartialCall {
            pub fn complete(self) -> Result<Call, Error> {
                match self {
                    $(
                        PartialCall::$name { params } => {
                            if let Some(params) = params {
                                let pparams: PartialParams = serde_json::from_value(params.clone())?;
                                if pparams.len() != $params_len {
                                    Err(Error::invalid_params_len())
                                } else {
                                    Ok(Call::$name{params: serde_json::from_value(params)?})
                                }
                            } else {
                                if $params_len == 0 {
                                    Ok(Call::$name{
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
            impl Into<concat_idents!($name, Params)> for Call {
                fn into(self) -> concat_idents!($name, Params) {
                    if let Call::$name{params} = self {
                        params
                    } else {
                        // IMHO, in Rust, no static check can do this.
                        // If https://github.com/rust-lang/rfcs/pull/1450 merged,
                        // I think I can remove this panic.
                        panic!("The method and params are one to one correspondence.")
                    }
                }
            }

            impl From<concat_idents!($name, Params)> for Call {
                fn from(params: concat_idents!($name, Params)) -> Call {
                    Call::$name{params: params}
                }
            }

            impl From<concat_idents!($name, Params)> for Request {
                fn from(params: concat_idents!($name, Params)) -> Request {
                    Request::new(
                        Some(Version::default()),
                        Id::default(),
                        Call::$name{params},
                    )
                }
            }

            impl From<concat_idents!($name, Params)> for String {
                fn from(params: concat_idents!($name, Params)) -> String {
                    let req: Request = params.into();
                    req.into()
                }
            }
        )+
    };
}

define_call!(
    // (Call-Item-Name-in-Enum, Method-Name-in-JSONRPC, Params-Length)
    (BlockNumber, "blockNumber", 0),
    (PeerCount, "peerCount", 0),
    (SendRawTransaction, "sendRawTransaction", 1),
    (SendTransaction, "sendTransaction", 1),
    (GetBlockByHash, "getBlockByHash", 2),
    (GetBlockByNumber, "getBlockByNumber", 2),
    (GetTransactionReceipt, "getTransactionReceipt", 1),
    (GetLogs, "getLogs", 1),
    (Call, "call", 2),
    (GetTransaction, "getTransaction", 1),
    (GetTransactionCount, "getTransactionCount", 2),
    (GetCode, "getCode", 2),
    (GetAbi, "getAbi", 2),
    (GetBalance, "getBalance", 2),
    (NewFilter, "newFilter", 1),
    (NewBlockFilter, "newBlockFilter", 0),
    (UninstallFilter, "uninstallFilter", 1),
    (GetFilterChanges, "getFilterChanges", 1),
    (GetFilterLogs, "getFilterLogs", 1),
    (GetTransactionProof, "getTransactionProof", 1),
    (GetMetaData, "getMetaData", 1),
);

#[cfg(test)]
mod tests {
    use super::{BlockNumberParams, Error, GetTransactionReceiptParams, PartialRequest, Request};
    use cita_types::H256;
    use serde_json;
    use std::convert::Into;

    macro_rules! test_ser_and_de {
        ($type:ty, $data:ident, $json_params:tt) => {
            let serialized = serde_json::to_value(&$data).unwrap();
            let jsonval = json!($json_params);
            assert_eq!(serialized, jsonval);
            let jsonstr = jsonval.to_string();
            let deserialized = serde_json::from_str::<$type>(&jsonstr);
            if let Ok(deserialized) = deserialized {
                assert_eq!(deserialized, $data);
            } else {
                assert_eq!(&jsonstr, "");
            }
        };
    }

    #[test]
    fn serialize_and_deserialize() {
        let params = GetTransactionReceiptParams::new(H256::from(10).into());
        test_ser_and_de!(
            GetTransactionReceiptParams,
            params,
            ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        );

        let full_req: Request = params.into();
        test_ser_and_de!(Request, full_req,  {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        });

        let req_str: String = full_req.clone().into();
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        });

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"],
        });
        assert_eq!(part_req.complete().unwrap(), full_req);

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt"
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": null,
        });
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::invalid_params("params is requeired")
        );

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "getTransactionReceipt",
            "params": [1, 2],
        });
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::invalid_params_len()
        );

        let params = BlockNumberParams::new();
        test_ser_and_de!(BlockNumberParams, params, []);

        let full_req: Request = params.into();
        test_ser_and_de!(Request, full_req,  {
            "jsonrpc": "2.0",
            "id": null,
            "method": "blockNumber",
            "params": [],
        });

        let req_str: String = full_req.clone().into();
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "blockNumber",
            "params": [],
        });

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "blockNumber"
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
            "method": "blockNumber",
            "params": null,
        });
        assert_eq!(part_req.complete().unwrap(), full_req);

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
        });
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::method_not_found()
        );

        let req_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "method": "not_aMethod",
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let part_req = serde_json::from_str::<PartialRequest>(&req_str).unwrap();
        test_ser_and_de!(PartialRequest, part_req, {
            "jsonrpc": "2.0",
            "id": null,
        });
        assert_eq!(
            part_req.complete().err().unwrap(),
            Error::method_not_found()
        );
    }
}
