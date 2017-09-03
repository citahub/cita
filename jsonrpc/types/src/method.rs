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

use super::{Params, Error, RpcRequest};
#[warn(non_snake_case)]
use libproto::blockchain;
use libproto::request as reqlib;
use protobuf::core::parse_from_bytes;
use rpctypes::{BlockNumber, CallRequest, Filter, CountAndCode, BlockParamsByHash, BlockParamsByNumber};
use rustc_serialize::hex::FromHex;
use serde_json;
use std::str::FromStr;
use util::{H256, H160, U256};
use util::clean_0x;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum RpcReqType {
    TX(blockchain::UnverifiedTransaction),
    REQ(reqlib::Request),
}


pub mod method {
    pub const CITA_BLOCK_BUMBER: &'static str = "cita_blockNumber";
    pub const CITA_GET_BLOCK_BY_HASH: &'static str = "cita_getBlockByHash";
    pub const CITA_GET_BLOCK_BY_NUMBER: &'static str = "cita_getBlockByNumber";
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
    pub const ETH_GET_TRANSACTION_COUNT: &'static str = "eth_getTransactionCount";
    pub const ETH_GET_CODE: &'static str = "eth_getCode";
    pub const ETH_CALL: &'static str = "eth_call";
    pub const ETH_GET_LOGS: &'static str = "eth_getLogs";
    pub const ETH_GET_TRANSACTION_RECEIPT: &'static str = "eth_getTransactionReceipt";

    /// filter
    pub const ETH_NEW_FILTER: &'static str = "eth_newFilter";
    pub const ETH_NEW_BLOCK_FILTER: &'static str = "eth_newBlockFilter";
    pub const ETH_UNINSTALL_FILTER: &'static str = "eth_uninstallFilter";
    pub const ETH_GET_FILTER_CHANGES: &'static str = "eth_getFilterChanges";
    pub const ETH_GET_FILTER_LOGS: &'static str = "eth_getFilterLogs";
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MethodHandler;
impl MethodHandler {
    pub fn params_len(&self, params: &Params) -> Result<usize, Error> {
        match *params {
            Params::Array(ref v) => Ok(v.len()),
            Params::None => Ok(0),
            _ => Err(Error::invalid_params("param is a few")),
        }
    }

    pub fn create_request(&self) -> reqlib::Request {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = reqlib::Request::new();
        request.set_request_id(request_id);
        request
    }

    pub fn from_req(&self, rpc: RpcRequest) -> Result<RpcReqType, Error> {
        match rpc.method.as_str() {
            method::CITA_BLOCK_BUMBER => {
                let number = self.block_number(rpc)?;
                Ok(RpcReqType::REQ(number))
            }
            method::NET_PEER_COUNT => {
                let peer = self.peer_count(rpc)?;
                Ok(RpcReqType::REQ(peer))
            }
            method::CITA_GET_BLOCK_BY_HASH => {
                let block = self.get_block_by_hash(rpc)?;
                Ok(RpcReqType::REQ(block))
            }
            method::CITA_GET_BLOCK_BY_NUMBER => {
                let bn = self.get_block_by_number(rpc)?;
                Ok(RpcReqType::REQ(bn))
            }
            method::CITA_GET_TRANSACTION => {
                let gt = self.get_transaction(rpc)?;
                Ok(RpcReqType::REQ(gt))
            }
            method::ETH_CALL => {
                let call = self.call(rpc)?;
                Ok(RpcReqType::REQ(call))
            }
            method::ETH_GET_LOGS => {
                let gl = self.get_logs(rpc)?;
                Ok(RpcReqType::REQ(gl))
            }
            method::ETH_GET_TRANSACTION_RECEIPT => {
                let gt = self.get_transaction_receipt(rpc)?;
                Ok(RpcReqType::REQ(gt))
            }
            method::ETH_GET_TRANSACTION_COUNT => {
                let gt = self.get_transaction_count(rpc)?;
                Ok(RpcReqType::REQ(gt))
            }
            method::ETH_GET_CODE => {
                let gc = self.get_code(rpc)?;
                Ok(RpcReqType::REQ(gc))
            }
            method::CITA_SEND_TRANSACTION => {
                let tx = self.send_transaction(rpc)?;
                Ok(RpcReqType::TX(tx))
            }

            method::ETH_NEW_FILTER => {
                let new_filter = self.new_filter(rpc)?;
                Ok(RpcReqType::REQ(new_filter))
            }

            method::ETH_NEW_BLOCK_FILTER => {
                let new_block_filter = self.new_block_filter(rpc)?;
                Ok(RpcReqType::REQ(new_block_filter))
            }

            method::ETH_UNINSTALL_FILTER => {
                let uninstall_filter = self.uninstall_filter(rpc)?;
                Ok(RpcReqType::REQ(uninstall_filter))
            }
            method::ETH_GET_FILTER_CHANGES => {
                let changes = self.get_filter_changes(rpc)?;
                Ok(RpcReqType::REQ(changes))
            }
            method::ETH_GET_FILTER_LOGS => {
                let filter = self.get_filter_logs(rpc)?;
                Ok(RpcReqType::REQ(filter))
            }

            _ => Err(Error::method_not_found()),
        }
    }
}


impl MethodHandler {
    pub fn send_transaction(&self, req_rpc: RpcRequest) -> Result<blockchain::UnverifiedTransaction, Error> {
        let params: (String,) = req_rpc.params.parse()?;
        let data = clean_0x(&params.0);
        data.from_hex()
            .map_err(|_err| {
                         let err_msg = format!("param not hex string : {:?}", _err);
                         Error::parse_error_msg(err_msg.as_ref())
                     })
            .and_then(|content| {
                          parse_from_bytes::<blockchain::UnverifiedTransaction>(&content[..]).map_err(|_err| {
                                                                                                          let err_msg = format!("parse protobuf UnverifiedTransaction data error : {:?}", _err);
                                                                                                          Error::parse_error_msg(err_msg.as_ref())
                                                                                                      })
                      })

    }


    pub fn peer_count(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        drop(req_rpc);
        let mut request = self.create_request();
        request.set_peercount(true);
        Ok(request)
    }

    pub fn block_number(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        drop(req_rpc);
        let mut request = self.create_request();
        request.set_block_number(true);
        Ok(request)
    }


    pub fn get_block_by_hash(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let (hash_0x, is_block): (String, bool) = req_rpc.params.parse()?;
        let hash = H256::from_str(clean_0x(&hash_0x)).map_err(|_err| Error::invalid_params(format!("{:?}", _err)))?;
        serde_json::to_string(&BlockParamsByHash::new(hash.to_vec(), is_block))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_hash| {
                     request.set_block_by_hash(block_hash);
                     request
                 })

    }


    pub fn get_block_by_number(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (BlockNumber, bool) = req_rpc.params.parse()?;
        serde_json::to_string(&BlockParamsByNumber::new(params.0, params.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_height| {
                     request.set_block_by_height(block_height);
                     request
                 })
    }


    pub fn get_transaction(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (String,) = req_rpc.params.parse()?;
        let hash = H256::from_str(clean_0x(&params.0)).map_err(|_err| Error::invalid_params(format!("{:?}", _err)))?;
        request.set_transaction(hash.0.to_vec());
        Ok(request)
    }

    pub fn call(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let len = self.params_len(&req_rpc.params)?;
        let params = match len {
            0 => Err(Error::invalid_params("must have 1 or 2 param!")),
            1 => {
                req_rpc.params
                       .parse::<(CallRequest,)>()
                       .map(|(base,)| (base, BlockNumber::default()))
                       .map_err(|_| Error::parse_error())
            }
            2 => {
                req_rpc.params
                       .parse::<(CallRequest, BlockNumber)>()
                       .map(|(base, id)| (base, id))
                       .map_err(|_| Error::parse_error())
            }
            _ => Err(Error::invalid_params("have much param!")),
        };

        let (base, id) = params?;
        let mut call = reqlib::Call::new();
        call.set_from(base.from.unwrap_or_default().to_vec());
        call.set_to(base.to.to_vec());
        call.set_data(base.data.unwrap_or_default().to_vec());
        serde_json::to_string(&id).map_err(|err| Error::invalid_params(err.to_string())).map(|heigth| {
                                                                                                 call.set_height(heigth);
                                                                                                 request.set_call(call);
                                                                                                 request
                                                                                             })
    }


    pub fn get_logs(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (Filter,) = req_rpc.params.parse()?;
        let filter = params.0;
        request.set_filter(serde_json::to_string(&filter).unwrap());
        Ok(request)
    }


    pub fn get_transaction_receipt(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (String,) = req_rpc.params.parse()?;
        let hash = H256::from_str(clean_0x(&params.0)).map_err(|_err| Error::invalid_params(format!("{:?}", _err)))?;
        request.set_transaction_receipt(hash.0.to_vec());
        Ok(request)
    }


    pub fn get_transaction_count(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let tx_count = self.code_count(req_rpc)?;
        trace!("code_count {:?}", tx_count);
        request.set_transaction_count(tx_count);
        Ok(request)
    }


    fn code_count(&self, req_rpc: RpcRequest) -> Result<String, Error> {
        let (address, number): (H160, BlockNumber) = req_rpc.params.parse()?;
        let count_code = CountAndCode::new(address.to_vec(), number);
        match serde_json::to_string(&count_code) {
            Ok(data) => Ok(data),
            Err(err) => Err(Error::invalid_params(format!("{:?}", err))),// return error information
        }
    }


    pub fn get_code(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let code = self.code_count(req_rpc)?;
        request.set_code(code);
        Ok(request)
    }

    pub fn new_filter(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (Filter,) = req_rpc.params.parse()?;
        let filter = serde_json::to_string(&params.0).expect("new_filter param error");
        request.set_new_filter(serde_json::to_string(&filter).unwrap());
        Ok(request)
    }

    pub fn new_block_filter(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        drop(req_rpc);
        let mut request = self.create_request();
        request.set_new_block_filter(true);
        Ok(request)
    }

    pub fn uninstall_filter(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (String,) = req_rpc.params.parse()?;
        let filter_id = U256::from_str(clean_0x(&params.0)).map_err(|_err| Error::invalid_params(format!("{:?}", _err)))?;
        request.set_uninstall_filter(filter_id.into());
        Ok(request)
    }

    pub fn get_filter_changes(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (String,) = req_rpc.params.parse()?;
        let filter_id = U256::from_str(clean_0x(&params.0)).map_err(|_err| Error::invalid_params(format!("{:?}", _err)))?;
        request.set_filter_changes(filter_id.into());
        Ok(request)
    }

    pub fn get_filter_logs(&self, req_rpc: RpcRequest) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let params: (String,) = req_rpc.params.parse()?;
        let filter_id = U256::from_str(clean_0x(&params.0)).map_err(|_err| Error::invalid_params(format!("{:?}", _err)))?;
        request.set_filter_logs(filter_id.into());
        Ok(request)
    }
}

//以后把这种测试，放到单独的测试文件。
#[cfg(test)]
mod tests {
    use super::*;
    use Id;
    use bytes::Bytes;
    use libproto::request;
    use method::MethodHandler;
    use params::Params;
    use request::Version;
    use serde_json;
    use serde_json::Value;
    use util::H160 as Hash160;


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
            method: method::CITA_BLOCK_BUMBER.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![]),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","method":"cita_blockNumber","id":"2","params":[]}"#);
    }

    #[test]
    fn test_rpc_deserialize1() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: method::CITA_BLOCK_BUMBER.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::None,
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(rpc_body, r#"{"jsonrpc":"2.0","method":"cita_blockNumber","id":"2","params":[]}"#);
    }

    #[test]
    fn test_rpc_into() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: method::CITA_BLOCK_BUMBER.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![]),
        };

        let handler = MethodHandler;
        let result: Result<request::Request, Error> = handler.block_number(rpc);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_rpc_into_err() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: method::ETH_GET_TRANSACTION_RECEIPT.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![Value::from(2)]),
        };

        let handler = MethodHandler;
        let result: Result<request::Request, Error> = handler.get_transaction_receipt(rpc);
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

        let handler = MethodHandler;
        assert!(handler.from_req(rpc).is_err());
    }

    #[test]
    fn test_cita_send_transaction() {
        let rpc = RpcRequest {
            jsonrpc: Some(Version::V2),
            method: method::CITA_SEND_TRANSACTION.to_owned(),
            id: Id::Str("2".to_string()),
            params: Params::Array(vec![
                Value::from("0x0a81010a1d0a033132331201301a0c613763356163343731623437209f8d062a01011260b3cf414a7abe01729890d40ba2a10811af4e48e74f16ea2397dfdb609fc311bf81c35bb10b5f790879d782b17ccb31896af30958fb02670352332c46996aef09b2a0f7852f7129d72d57db882f3b6b26a5a3ccd90b1abed5fe1f8ef652ccb89b12206dc0247fe5d8d4521bf75b3895e9cab3720aee654922f3f995c250a859feca351a20b2a0f7852f7129d72d57db882f3b6b26a5a3ccd90b1abed5fe1f8ef652ccb89b".to_owned()),
            ]),
        };
        let handler = MethodHandler;
        let result1: Result<blockchain::UnverifiedTransaction, Error> = handler.send_transaction(rpc1);
        let result2: Result<blockchain::UnverifiedTransaction, Error> = handler.send_transaction(rpc2);
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_rpc_request_parse() {
        let rpc = "{\"id\":\"-8799978260242268161\",\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[\"1\",\"0x0a2833616538386665333730633339333834666331366461326339653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String, String), Error> = request.params.parse();
        assert!(params.is_ok());
    }

    #[test]
    fn test_rpc_request_parse1() {
        let rpc = "{\"id\":\"-8799978260242268161\",\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[\"0x0a2833616538386665333730633339333834666331366461326339653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String, String), Error> = request.params.parse();
        assert!(params.is_err());
    }

    #[test]
    fn test_rpc_request_parse2() {
        let rpc = "{\"id\":\"-8799978260242268161\",\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[\"0x0a2833616538386665333730633339333834666331366461326339653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String,), Error> = request.params.parse();
        assert!(params.is_ok());
    }

    #[test]
    // 交易添加valid_until_block后，兼容测试以前的交易。
    fn test_blocklimit_backword_compatibility() {
        let rpc = r#"{"jsonrpc":"2.0","method":"cita_sendTransaction","params":["0x1201311a85010a401201311a3b2239080a12350a2430356162636538642d316431662d343536352d396636342d62623164303236393365333910641a03303037220443495441280312417922853b51d097df76791aa10836942c66bc522c24c8804c93e9230fc67dde897bbed399fa0f9e9ac0abc598cd92215fb362b9e31251bf784511be61d045703e00"],"id":2}"#;
        let request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let params: Result<(String,), Error> = request.params.parse();
        assert!(params.is_ok());
    }

    #[test]
    fn eth_call_with_blockid_deserialization() {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_call","params":[{"from":"d46e8dd67c5d32be8058bb8eb970870f07244567","to":"b60e8dd61c5d32be8058bb8eb970870f07233155","data":"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"}, 22],"id":2}"#;
        let rpc_request: RpcRequest = serde_json::from_str(rpc).unwrap();

        let handler = MethodHandler;
        let request: Result<request::Request, Error> = handler.call(rpc_request);

        assert!(request.is_ok());
        let request = request.unwrap();
        let call = request.get_call();
        assert_eq!(call.get_from(), Hash160::from("0xd46e8dd67c5d32be8058bb8eb970870f07244567").to_vec().as_slice());
        assert_eq!(call.get_to(), Hash160::from("0xb60e8dd61c5d32be8058bb8eb970870f07233155").to_vec().as_slice());
        assert_eq!(call.get_data(),
                   Bytes("d46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"
                             .from_hex()
                             .unwrap())
                   .to_vec()
                   .as_slice());
        assert_eq!(call.get_height(), "\"0x16\"");
    }

    #[test]
    fn eth_call_deserialization() {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_call","params":[{"from":"d46e8dd67c5d32be8058bb8eb970870f07244567","to":"b60e8dd61c5d32be8058bb8eb970870f07233155","data":"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"}],"id":2}"#;
        let rpc_request: RpcRequest = serde_json::from_str(rpc).unwrap();

        let handler = MethodHandler;
        let request: Result<request::Request, Error> = handler.call(rpc_request);

        assert!(request.is_ok());
        let request = request.unwrap();
        let call = request.get_call();
        assert_eq!(call.get_from(), Hash160::from("0xd46e8dd67c5d32be8058bb8eb970870f07244567").to_vec().as_slice());
        assert_eq!(call.get_to(), Hash160::from("0xb60e8dd61c5d32be8058bb8eb970870f07233155").to_vec().as_slice());
        assert_eq!(call.get_data(),
                   Bytes("d46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675"
                             .from_hex()
                             .unwrap())
                   .to_vec()
                   .as_slice());
    }

    #[test]
    fn cita_get_log_deserialization() {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_getLogs","params":[{"fromBlock":1,"toBlock":2,"address":"8888f1f195afa192cfee860698584c030f4c9db1","topics": ["0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b", null, ["0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b", "0x0000000000000000000000000aff3454fce5edbc8cca8697c15331677e6ebccc"]]}],"id":2}"#;
        let rpc_request: RpcRequest = serde_json::from_str(rpc).unwrap();
        let handler = MethodHandler;
        let request: Result<request::Request, Error> = handler.get_logs(rpc_request.clone());

        assert!(request.is_ok());
        let request = request.unwrap();
        let filter = request.get_filter();
        let params: Result<(Filter,), Error> = rpc_request.params.clone().parse();
        assert_eq!(serde_json::to_string(&params.unwrap().0).unwrap(), filter);
    }
}
