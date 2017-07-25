use libproto::request as reqlib;
use libproto::blockchain;
use uuid::Uuid;
use std::convert::TryInto;
use protobuf::core::parse_from_bytes;
use rustc_serialize::hex::FromHex;
use rpctypes::{BlockNumber, CallRequest, BlockTag, Filter};
use serde_json;
use super::{Id, Params, RpcError};

pub mod rpc_method {
    pub const CITA_BLOCK_HEIGHT: &'static str = "cita_blockNumber";
    pub const CITA_GET_BLOCK_BY_HASH: &'static str = "cita_getBlockByHash";
    pub const CITA_GET_BLOCK_BY_HEIGHT: &'static str = "cita_getBlockByNumber";
    pub const CITA_GET_TRANSACTION: &'static str = "cita_getTransaction";
    pub const CITA_SEND_TRANSACTION: &'static str = "cita_sendTransaction";
    pub const NET_PEER_COUNT: &'static str = "net_peerCount";
    /// Executes a new message call immediately without creating a transaction on the block chain.
    /// Parameters
    /// 1. Object - The transaction call object
    /// from: DATA, 20 Bytes - (optional) The address the transaction is sent from.
    /// to: DATA, 20 Bytes - The address the transaction is directed to.
    /// data: DATA - (optional) Hash of the method signature and encoded parameters.
    /// 2. QUANTITY|TAG - integer block height, or the string "latest" or "earliest".
    pub const CITA_GET_TRANSACTION_COUNT: &'static str = "eth_getTransactionCount";
    pub const CITA_GET_CODE: &'static str = "eth_getCode";
    pub const CITA_CALL: &'static str = "eth_call";
    pub const CITA_GET_LOGS: &'static str = "eth_getLogs";
    pub const CITA_GET_TRANSACTION_RECEIPT: &'static str = "eth_getTransactionReceipt";
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Version {
    #[serde(rename = "1.0")]
    V1,
    #[serde(rename = "2.0")]
    V2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpcRequest {
    pub jsonrpc: Option<Version>,
    pub method: String,
    pub id: Id,
    pub params: Params,
}

fn params_len(params: &Params) -> Result<usize, RpcError> {
    match *params {
        Params::Array(ref v) => Ok(v.len()),
        Params::None => Ok(0),
        _ => Err(RpcError::InvalidParams),
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum ReqType {
    TX,
    OTHER,
}

impl RpcRequest {
    pub fn req_type(&self) -> ReqType {
        if self.method == rpc_method::CITA_SEND_TRANSACTION {
            ReqType::TX
        } else {
            ReqType::OTHER
        }
    }
}

impl TryInto<blockchain::Transaction> for RpcRequest {
    type Error = RpcError;

    fn try_into(self) -> Result<blockchain::Transaction, RpcError> {
        trace!("--------request tx-------- {:?}", self);
        match self.method.as_str() {
            rpc_method::CITA_SEND_TRANSACTION => {
                let params: Result<(String,), RpcError> = self.params.parse();
                params.and_then(|content| {
                    content
                        .0
                        .from_hex()
                        .map_err(|_| RpcError::InvalidParams)
                        .and_then(|content| {
                            parse_from_bytes::<blockchain::Transaction>(&content[..])
                                .map_err(|_| RpcError::InvalidParams)
                        })
                })
            }
            _ => Err(RpcError::NotFound),
        }
    }
}

// TODO: 是否需要重构RPC API，尽量减少Protobuf的结构？
impl TryInto<reqlib::Request> for RpcRequest {
    type Error = RpcError;

    fn try_into(self) -> Result<reqlib::Request, RpcError> {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = reqlib::Request::new();
        request.set_request_id(request_id);
        // println!("params!!!!! {:?}", self.params);
        // trace!("rpc_method!! {:?} {:?}", self.method, self.params);
        match self.method.as_str() {
            rpc_method::CITA_BLOCK_HEIGHT => {
                request.set_block_number(true);
                Ok(request)
            }
            rpc_method::NET_PEER_COUNT => {
                request.set_peercount(true);
                Ok(request)
            }
            rpc_method::CITA_GET_BLOCK_BY_HASH => {
                let params: Result<(String, bool), RpcError> = self.params.parse();
                params.and_then(|params| {
                    params
                        .0
                        // TODO: 格式不够严格
                        .from_hex()
                        .map_err(|_| RpcError::InvalidParams)
                        .and_then(|hash| {
                                      let mut h = reqlib::BlockParamsByHash::new();
                                      h.set_hash(hash);
                                      h.set_include_txs(params.1);
                                      request.set_block_by_hash(h);
                                      Ok(request)
                                  })

                })
            }
            rpc_method::CITA_GET_BLOCK_BY_HEIGHT => {
                let params: Result<(BlockNumber, bool), RpcError> = self.params.parse();
                params.and_then(|params| match params.0 {
                    BlockNumber::Height(number) => {
                        let mut h = reqlib::BlockParamsByNumber::new();
                        h.set_height(number);
                        h.set_include_txs(params.1);
                        request.set_block_by_height(h);
                        Ok(request)
                    }
                    _ => Err(RpcError::InvalidParams),
                })
            }
            rpc_method::CITA_GET_TRANSACTION => {
                let params: Result<(String,), RpcError> = self.params.parse();
                params.and_then(|params| {
                    params
                        .0
                        .from_hex()
                        .map_err(|_| RpcError::InvalidParams)
                        .and_then(|hash| {
                            request.set_transaction(hash);
                            Ok(request)
                        })

                })
            }
            rpc_method::CITA_CALL => {
                let len = params_len(&self.params).map_err(
                    |_| RpcError::InvalidParams,
                )?;
                let params = match len {
                    0 => Err(RpcError::InvalidParams),
                    1 => {
                        self.params
                            .parse::<(CallRequest,)>()
                            .map(|(base,)| (base, BlockNumber::default()))
                            .map_err(|_| RpcError::InvalidParams)
                    }
                    2 => {
                        self.params
                            .parse::<(CallRequest, BlockNumber)>()
                            .map(|(base, id)| (base, id))
                            .map_err(|_| RpcError::InvalidParams)
                    }
                    _ => Err(RpcError::InvalidParams),
                };

                let (base, id) = params?;
                let mut call = reqlib::Call::new();
                call.set_from(base.from.unwrap_or_default().to_vec());
                call.set_to(base.to.to_vec());
                call.set_data(base.data.unwrap_or_default().to_vec());
                match id {
                    BlockNumber::Tag(BlockTag::Latest) => call.set_tag(reqlib::BlockTag::Latest),
                    BlockNumber::Tag(BlockTag::Earliest) => {
                        call.set_tag(reqlib::BlockTag::Earliest)
                    }
                    BlockNumber::Height(height) => call.set_height(height),
                }
                request.set_call(call);
                Ok(request)
            }
            rpc_method::CITA_GET_LOGS => {
                let params: Result<(Filter,), RpcError> = self.params.clone().parse();
                params.and_then(|filter| {
                    request.set_filter(serde_json::to_string(&filter.0).unwrap());
                    Ok(request)
                })
            }
            rpc_method::CITA_GET_TRANSACTION_RECEIPT => {
                let params: Result<(String,), RpcError> = self.params.parse();
                params.and_then(|params| {
                    params
                        .0
                        .from_hex()
                        .map_err(|_| RpcError::InvalidParams)
                        .and_then(|tx_hash| {
                            request.set_transaction_receipt(tx_hash);
                            Ok(request)
                        })

                })
            }

            rpc_method::CITA_GET_TRANSACTION_COUNT => {
                let params: Result<(String, BlockNumber), RpcError> =
                    self.params.parse().map_err(|_| RpcError::InvalidParams);
                trace!("---debug {:?}----", params);
                let (address, id) = params?;
                address
                    .from_hex()
                    .map_err(|_| RpcError::InvalidParams)
                    .and_then(|address| {
                        let mut tx_count = reqlib::TransactionCount::new();
                        tx_count.set_address(address);
                        match id {
                            BlockNumber::Tag(BlockTag::Latest) => {
                                tx_count.set_tag(reqlib::BlockTag::Latest)
                            }
                            BlockNumber::Tag(BlockTag::Earliest) => {
                                tx_count.set_tag(reqlib::BlockTag::Earliest)
                            }
                            BlockNumber::Height(height) => tx_count.set_height(height),
                        };
                        request.set_transaction_count(tx_count);
                        Ok(request)
                    })
            }

            rpc_method::CITA_GET_CODE => {
                let params: Result<(String, BlockNumber), RpcError> =
                    self.params.parse().map_err(|_| RpcError::InvalidParams);
                trace!("---debug {:?}----", params);
                let (address, id) = params?;
                address
                    .from_hex()
                    .map_err(|_| RpcError::InvalidParams)
                    .and_then(|address| {
                        let mut code = reqlib::Code::new();
                        code.set_address(address);
                        match id {
                            BlockNumber::Tag(BlockTag::Latest) => {
                                code.set_tag(reqlib::BlockTag::Latest)
                            }
                            BlockNumber::Tag(BlockTag::Earliest) => {
                                code.set_tag(reqlib::BlockTag::Earliest)
                            }
                            BlockNumber::Height(height) => code.set_height(height),
                        };
                        request.set_code(code);
                        Ok(request)
                    })

            }
            _ => Err(RpcError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use libproto::request;
    use serde_json::Value;
    use util::H160 as Hash160;
    use bytes::Bytes;

    #[test]
    fn test_rpc_serialize() {
        let rpc_body = r#"{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":"1"}"#;
        let rpc: RpcRequest = serde_json::from_str(rpc_body).unwrap();

        assert_eq!(rpc.id, Id::Str("1".to_string()));
        assert_eq!(rpc.jsonrpc, Some(Version::V2));
        assert_eq!(rpc.method, "cita_blockNumber".to_string());
        assert_eq!(rpc.params, Params::None);
    }

    #[test]
    fn test_rpc_deserialize() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: rpc_method::CITA_BLOCK_HEIGHT.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![]),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(
            rpc_body,
            r#"{"jsonrpc":"2.0","method":"cita_blockNumber","id":"2","params":[]}"#
        );
    }

    #[test]
    fn test_rpc_deserialize1() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: rpc_method::CITA_BLOCK_HEIGHT.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::None,
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(
            rpc_body,
            r#"{"jsonrpc":"2.0","method":"cita_blockNumber","id":"2","params":[]}"#
        );
    }

    #[test]
    fn test_rpc_into() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: rpc_method::CITA_BLOCK_HEIGHT.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![]),
        };

        let result: Result<request::Request, RpcError> = rpc.try_into();
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_rpc_into_err() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: rpc_method::CITA_GET_TRANSACTION_RECEIPT.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![Value::from(2)]),
        };

        let result: Result<request::Request, RpcError> = rpc.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_rpc_into_err2() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: "cita_xxx".to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![]),
        };

        let result: Result<request::Request, RpcError> = rpc.try_into();
        match result {
            Ok(_) => assert!(false),
            Err(RpcError::NotFound) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_rpc_request_parse() {
        let rpc = "{\"id\":\"-8799978260242268161\",\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[\"1\",\"0x0a2833616538386665333730633339333834666331366461326339653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String, String), RpcError> = request.params.parse();
        assert!(params.is_ok());
    }

    #[test]
    fn test_rpc_request_parse1() {
        let rpc = "{\"id\":\"-8799978260242268161\",\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[\"0x0a2833616538386665333730633339333834666331366461326339653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String, String), RpcError> = request.params.parse();
        assert!(params.is_err());
    }

    #[test]
    fn test_rpc_request_parse2() {
        let rpc = "{\"id\":\"-8799978260242268161\",\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[\"0x0a2833616538386665333730633339333834666331366461326339653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String,), RpcError> = request.params.parse();
        assert!(params.is_ok());
    }

    #[test]
    // 交易添加valid_until_block后，兼容测试以前的交易。
    fn test_blocklimit_backword_compatibility() {
        let rpc = r#"{"jsonrpc":"2.0","method":"cita_sendTransaction","params":["0x1201311a85010a401201311a3b2239080a12350a2430356162636538642d316431662d343536352d396636342d62623164303236393365333910641a03303037220443495441280312417922853b51d097df76791aa10836942c66bc522c24c8804c93e9230fc67dde897bbed399fa0f9e9ac0abc598cd92215fb362b9e31251bf784511be61d045703e00"],"id":2}"#;
        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String,), RpcError> = request.params.parse();
        assert!(params.is_ok());
    }

    #[test]
    fn eth_call_with_blockid_deserialization() {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_call","params":[{"from":"d46e8dd67c5d32be8058bb8eb970870f07244567","to":"b60e8dd61c5d32be8058bb8eb970870f07233155","data":"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"}, 22],"id":2}"#;
        let rpc_request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let request: Result<request::Request, RpcError> = rpc_request.try_into();

        assert!(request.is_ok());
        let request = request.unwrap();
        let call = request.get_call();
        assert_eq!(
            call.get_from(),
            Hash160::from("0xd46e8dd67c5d32be8058bb8eb970870f07244567")
                .to_vec()
                .as_slice()
        );
        assert_eq!(
            call.get_to(),
            Hash160::from("0xb60e8dd61c5d32be8058bb8eb970870f07233155")
                .to_vec()
                .as_slice()
        );
        assert_eq!(
            call.get_data(),
            Bytes(
                "d46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"
                    .from_hex()
                    .unwrap(),
            ).to_vec()
                .as_slice()
        );
        assert_eq!(call.get_height(), 22);
    }

    #[test]
    fn eth_call_deserialization() {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_call","params":[{"from":"d46e8dd67c5d32be8058bb8eb970870f07244567","to":"b60e8dd61c5d32be8058bb8eb970870f07233155","data":"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"}],"id":2}"#;
        let rpc_request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let request: Result<request::Request, RpcError> = rpc_request.try_into();

        assert!(request.is_ok());
        let request = request.unwrap();
        let call = request.get_call();
        assert_eq!(
            call.get_from(),
            Hash160::from("0xd46e8dd67c5d32be8058bb8eb970870f07244567")
                .to_vec()
                .as_slice()
        );
        assert_eq!(
            call.get_to(),
            Hash160::from("0xb60e8dd61c5d32be8058bb8eb970870f07233155")
                .to_vec()
                .as_slice()
        );
        assert_eq!(
            call.get_data(),
            Bytes(
                "d46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"
                    .from_hex()
                    .unwrap(),
            ).to_vec()
                .as_slice()
        );
        assert_eq!(call.get_tag(), request::BlockTag::Latest);
    }

    #[test]
    fn cita_get_log_deserialization() {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_getLogs","params":[{"fromBlock":1,"toBlock":2,"address":"8888f1f195afa192cfee860698584c030f4c9db1","topics": ["0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b", null, ["0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b", "0x0000000000000000000000000aff3454fce5edbc8cca8697c15331677e6ebccc"]]}],"id":2}"#;
        let rpc_request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let request: Result<request::Request, RpcError> = rpc_request.clone().try_into();

        assert!(request.is_ok());
        let request = request.unwrap();
        let filter = request.get_filter();
        let params: Result<(Filter,), RpcError> = rpc_request.params.clone().parse();
        assert_eq!(serde_json::to_string(&params.unwrap().0).unwrap(), filter);
    }
}
