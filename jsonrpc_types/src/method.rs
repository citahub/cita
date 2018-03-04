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

use super::{Call, Error, Params};
//#[warn(non_snake_case)]
use libproto::{request as reqlib, UnverifiedTransaction};
use rpctypes::{BlockNumber, BlockParamsByHash, BlockParamsByNumber, CallRequest, CountOrCode, Filter};
use rustc_serialize::hex::FromHex;
use serde_json;
use std::convert::TryFrom;
use util::{H160, H256, U256};
use util::ToPretty;
use util::clean_0x;
use uuid::Uuid;

pub mod method {
    pub const CITA_BLOCK_BUMBER: &str = "cita_blockNumber";
    pub const CITA_GET_BLOCK_BY_HASH: &str = "cita_getBlockByHash";
    pub const CITA_GET_BLOCK_BY_NUMBER: &str = "cita_getBlockByNumber";
    pub const CITA_GET_TRANSACTION: &str = "cita_getTransaction";
    pub const CITA_SEND_TRANSACTION: &str = "cita_sendTransaction";
    pub const NET_PEER_COUNT: &str = "net_peerCount";
    /// Executes a new message call immediately without creating a transaction on the block chain.
    /// Parameters
    /// 1. Object - The transaction call object
    /// from: DATA, 20 Bytes - (optional) The address the transaction is sent from.
    /// to: DATA, 20 Bytes - The address the transaction is directed to.
    /// data: DATA - (optional) Hash of the method signature and encoded parameters.
    /// 2. QUANTITY|TAG - integer block height, or the string "latest" or "earliest".
    pub const ETH_GET_TRANSACTION_COUNT: &str = "eth_getTransactionCount";
    pub const ETH_GET_CODE: &str = "eth_getCode";
    pub const ETH_CALL: &str = "eth_call";
    pub const ETH_GET_LOGS: &str = "eth_getLogs";
    pub const ETH_GET_TRANSACTION_RECEIPT: &str = "eth_getTransactionReceipt";

    /// filter
    pub const ETH_NEW_FILTER: &str = "eth_newFilter";
    pub const ETH_NEW_BLOCK_FILTER: &str = "eth_newBlockFilter";
    pub const ETH_UNINSTALL_FILTER: &str = "eth_uninstallFilter";
    pub const ETH_GET_FILTER_CHANGES: &str = "eth_getFilterChanges";
    pub const ETH_GET_FILTER_LOGS: &str = "eth_getFilterLogs";
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MethodHandler;
impl MethodHandler {
    pub fn params_len(&self, params: &Option<Params>) -> usize {
        if let &Some(ref params) = params {
            params.len()
        } else {
            0
        }
    }

    pub fn detach_requeired_params(&self, req_rpc: &Call) -> Result<Params, Error> {
        req_rpc
            .params
            .clone()
            .ok_or_else(|| Error::invalid_params("params is requeired"))
    }

    pub fn create_request(&self) -> reqlib::Request {
        let request_id = Uuid::new_v4().as_bytes().to_vec();
        let mut request = reqlib::Request::new();
        request.set_request_id(request_id);
        request
    }

    pub fn request(&self, rpc: &Call) -> Result<reqlib::Request, Error> {
        match rpc.method.as_str() {
            method::CITA_BLOCK_BUMBER => self.block_number(rpc),
            method::NET_PEER_COUNT => self.peer_count(rpc),
            method::CITA_GET_BLOCK_BY_HASH => self.get_block_by_hash(rpc),
            method::CITA_GET_BLOCK_BY_NUMBER => self.get_block_by_number(rpc),
            method::CITA_GET_TRANSACTION => self.get_transaction(rpc),
            method::ETH_CALL => self.call(rpc),
            method::ETH_GET_LOGS => self.get_logs(rpc),
            method::ETH_GET_TRANSACTION_RECEIPT => self.get_transaction_receipt(rpc),
            method::ETH_GET_TRANSACTION_COUNT => self.get_transaction_count(rpc),
            method::ETH_GET_CODE => self.get_code(rpc),
            method::CITA_SEND_TRANSACTION => self.send_transaction(rpc),

            method::ETH_NEW_FILTER => self.new_filter(rpc),

            method::ETH_NEW_BLOCK_FILTER => self.new_block_filter(rpc),

            method::ETH_UNINSTALL_FILTER => self.uninstall_filter(rpc),
            method::ETH_GET_FILTER_CHANGES => self.get_filter_changes(rpc),
            method::ETH_GET_FILTER_LOGS => self.get_filter_logs(rpc),

            _ => Err(Error::method_not_found()),
        }
    }

    pub fn send_transaction(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let params = self.detach_requeired_params(req_rpc)?;
        let params: (String,) = params.parse()?;

        let data = clean_0x(&params.0);
        let un_tx = data.from_hex()
            .map_err(|_err| {
                let err_msg = format!("param not hex string : {:?}", _err);
                Error::parse_error_with_message(err_msg)
            })
            .and_then(|content| {
                UnverifiedTransaction::try_from(&content[..]).map_err(|_err| {
                    let err_msg = format!(
                        "parse protobuf UnverifiedTransaction data error : {:?}",
                        _err
                    );
                    Error::parse_error_with_message(err_msg)
                })
            })?;

        {
            let tx = un_tx.get_transaction();
            let to = clean_0x(tx.get_to());
            if to.len() != 40 && !to.is_empty() {
                return Err(Error::invalid_params(
                    "param 'to' length too short, or are you create contract?",
                ));
            } else {
                let _ = to.from_hex().map_err(|err| {
                    let err_msg = format!("param not hex string : {:?}", err);
                    Error::parse_error_with_message(err_msg)
                })?;
            }
            trace!(
                "SEND ProtoTransaction: nonce {:?}, block_limit {:?}, data {}, quota {:?}, to {:?}, hash {}",
                tx.get_nonce(),
                tx.get_valid_until_block(),
                tx.get_data().pretty(),
                tx.get_quota(),
                tx.get_to(),
                un_tx.crypt_hash().pretty()
            );
        }
        request.set_un_tx(un_tx);
        Ok(request)
    }

    pub fn peer_count(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 0 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let mut request = self.create_request();
        request.set_peercount(true);
        Ok(request)
    }

    pub fn block_number(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 0 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let mut request = self.create_request();
        request.set_block_number(true);
        Ok(request)
    }

    pub fn get_block_by_hash(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 2 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }
        let params = self.detach_requeired_params(req_rpc)?;
        let (hash, is_block): (H256, bool) = params.parse()?;
        let mut request = self.create_request();

        serde_json::to_string(&BlockParamsByHash::new(hash.to_vec(), is_block))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_hash| {
                request.set_block_by_hash(block_hash);
                request
            })
    }

    pub fn get_block_by_number(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 2 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }
        let params = self.detach_requeired_params(req_rpc)?;
        let params: (BlockNumber, bool) = params.parse()?;
        let mut request = self.create_request();

        serde_json::to_string(&BlockParamsByNumber::new(params.0, params.1))
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|block_height| {
                request.set_block_by_height(block_height);
                request
            })
    }

    pub fn get_transaction(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let params = self.detach_requeired_params(req_rpc)?;
        let (hash,): (H256,) = params.parse()?;
        let mut request = self.create_request();
        request.set_transaction(hash.to_vec());
        Ok(request)
    }

    pub fn call(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        let mut request = self.create_request();
        let len = self.params_len(&req_rpc.params);

        let params = self.detach_requeired_params(req_rpc)?;

        let (base, id) = match len {
            0 => Err(Error::invalid_params("must have 1 or 2 param!")),
            1 => params
                .parse::<(CallRequest,)>()
                .map(|(base,)| (base, BlockNumber::default()))
                .map_err(|err| {
                    let err_msg = format!("param parse error : {:?}", err);
                    Error::parse_error_with_message(err_msg)
                }),
            2 => params.parse::<(CallRequest, BlockNumber)>().map_err(|err| {
                let err_msg = format!("param parse error : {:?}", err);
                Error::parse_error_with_message(err_msg)
            }),
            _ => Err(Error::invalid_params("have much param!")),
        }?;

        let mut call = reqlib::Call::new();
        call.set_from(base.from.unwrap_or_default().to_vec());
        call.set_to(base.to.to_vec());
        call.set_data(base.data.unwrap_or_default().vec());
        serde_json::to_string(&id)
            .map_err(|err| Error::invalid_params(err.to_string()))
            .map(|height| {
                call.set_height(height);
                request.set_call(call);
                request
            })
    }

    pub fn get_logs(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let params = self.detach_requeired_params(req_rpc)?;
        let (filter,): (Filter,) = params.parse()?;

        let mut request = self.create_request();
        request.set_filter(serde_json::to_string(&filter).unwrap());
        Ok(request)
    }

    pub fn get_transaction_receipt(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let params = self.detach_requeired_params(req_rpc)?;
        let (hash,): (H256,) = params.parse()?;

        let mut request = self.create_request();
        request.set_transaction_receipt(hash.to_vec());
        Ok(request)
    }

    pub fn get_transaction_count(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        let tx_count = self.code_or_count(req_rpc)?;
        let mut request = self.create_request();
        trace!("count = {:?}", tx_count);
        request.set_transaction_count(tx_count);
        Ok(request)
    }

    fn code_or_count(&self, req_rpc: &Call) -> Result<String, Error> {
        if 2 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let params = self.detach_requeired_params(req_rpc)?;
        let (address, number): (H160, BlockNumber) = params.parse()?;

        let count_code = CountOrCode::new(address.to_vec(), number);
        match serde_json::to_string(&count_code) {
            Ok(data) => Ok(data),
            Err(err) => Err(Error::invalid_params(err.to_string())), // return error information
        }
    }

    pub fn get_code(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        let code = self.code_or_count(req_rpc)?;
        let mut request = self.create_request();
        request.set_code(code);
        Ok(request)
    }

    pub fn new_filter(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }
        let params = self.detach_requeired_params(req_rpc)?;
        let (filter,): (Filter,) = params.parse()?;
        let mut request = self.create_request();

        let filter = serde_json::to_string(&filter).map_err(|err| Error::invalid_params(format!("{:?}", err)))?;
        request.set_new_filter(filter);
        Ok(request)
    }

    pub fn new_block_filter(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 0 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let mut request = self.create_request();
        request.set_new_block_filter(true);
        Ok(request)
    }

    pub fn uninstall_filter(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }
        let params = self.detach_requeired_params(req_rpc)?;
        let (filter_id,): (U256,) = params.parse()?;
        let mut request = self.create_request();

        trace!("uninstall_filter {:?}", filter_id);
        request.set_uninstall_filter(filter_id.into());
        Ok(request)
    }

    pub fn get_filter_changes(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }
        let params = self.detach_requeired_params(req_rpc)?;
        let (filter_id,): (U256,) = params.parse()?;

        let mut request = self.create_request();
        request.set_filter_changes(filter_id.into());
        Ok(request)
    }

    pub fn get_filter_logs(&self, req_rpc: &Call) -> Result<reqlib::Request, Error> {
        if 1 != self.params_len(&req_rpc.params) {
            return Err(Error::invalid_params_len());
        }

        let params = self.detach_requeired_params(req_rpc)?;
        let (filter_id,): (U256,) = params.parse()?;

        let mut request = self.create_request();
        request.set_filter_logs(filter_id.into());
        Ok(request)
    }
}

//TODO add mock?
#[cfg(test)]
mod tests {
    use super::*;
    use Id;
    use bytes::Bytes;
    use libproto::blockchain::{Transaction, UnverifiedTransaction};
    use libproto::request;
    use method::MethodHandler;
    use params::Params;
    use request::Version;
    use serde_json;
    use serde_json::Value;
    use std::convert::TryInto;
    use util::H160 as Hash160;
    use util::ToPretty;

    #[test]
    fn test_rpc_serialize() {
        let rpc_body = r#"{"jsonrpc":"2.0","method":"cita_blockNumber","params":[],"id":"1"}"#;
        let rpc: Call = serde_json::from_str(rpc_body).unwrap();

        assert_eq!(rpc.id, Id::Str("1".to_string()));
        assert_eq!(rpc.jsonrpc, Some(Version::V2));
        assert_eq!(rpc.method, "cita_blockNumber".to_string());
        assert_eq!(rpc.params.unwrap(), Params::None);
    }

    #[test]
    fn test_rpc_deserialize() {
        let rpc = Call {
            jsonrpc: Some(Version::V2),
            method: method::CITA_BLOCK_BUMBER.to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::Array(vec![])),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(
            rpc_body,
            r#"{"jsonrpc":"2.0","method":"cita_blockNumber","id":"2","params":[]}"#
        );
    }

    #[test]
    fn test_rpc_deserialize1() {
        let rpc = Call {
            jsonrpc: Some(Version::V2),
            method: method::CITA_BLOCK_BUMBER.to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::None),
        };

        let rpc_body = serde_json::to_string(&rpc).unwrap();
        assert_eq!(
            rpc_body,
            r#"{"jsonrpc":"2.0","method":"cita_blockNumber","id":"2","params":[]}"#
        );
    }

    #[test]
    fn test_rpc_into() {
        let rpc = Call {
            jsonrpc: Some(Version::V2),
            method: method::CITA_BLOCK_BUMBER.to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::Array(vec![])),
        };

        let handler = MethodHandler;
        let result: Result<request::Request, Error> = handler.block_number(&rpc);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_rpc_into_err() {
        let rpc = Call {
            jsonrpc: Some(Version::V2),
            method: method::ETH_GET_TRANSACTION_RECEIPT.to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::Array(vec![Value::from(2)])),
        };

        let handler = MethodHandler;
        let result: Result<request::Request, Error> = handler.get_transaction_receipt(&rpc);
        assert!(result.is_err());
    }

    #[test]
    fn test_rpc_into_err2() {
        let rpc = Call {
            jsonrpc: Some(Version::V2),
            method: "cita_xxx".to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::Array(vec![])),
        };

        let handler = MethodHandler;
        assert!(handler.request(&rpc).is_err());
    }

    #[test]
    fn test_cita_send_transaction() {
        let mut tx = Transaction::new();
        tx.set_to("0xb84a3067e31cbe3bebfcc16e2b3495838864b82a".to_string());
        tx.set_quota(23);
        tx.set_nonce("23".to_string());
        tx.set_valid_until_block(99999);
        let mut utx = UnverifiedTransaction::new();
        utx.set_transaction(tx);
        let utx_string: Vec<u8> = utx.try_into().unwrap();

        let rpc1 = Call {
            jsonrpc: Some(Version::V2),
            method: method::CITA_SEND_TRANSACTION.to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::Array(vec![
                Value::from(utx_string.to_hex().to_owned()),
            ])),
        };

        let rpc2 = Call {
            jsonrpc: Some(Version::V2),
            method: method::CITA_SEND_TRANSACTION.to_owned(),
            id: Id::Str("2".to_string()),
            params: Some(Params::Array(vec![
                Value::from(clean_0x(&utx_string.to_hex()).to_owned()),
            ])),
        };
        let handler = MethodHandler;
        let result1: Result<reqlib::Request, Error> = handler.send_transaction(&rpc1);
        let result2: Result<reqlib::Request, Error> = handler.send_transaction(&rpc2);
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_rpc_request_parse() {
        let rpc = "{\"id\":\"-8799978260242268161\",\
                   \"jsonrpc\":\"2.0\",\
                   \"method\":\"eth_call\",\
                   \"params\":[\"1\",\
                   \"0x0a283361653838666533373063333933383466633136646132633965373638\
                   6366356432343935623438120d31343932353139393038393631\"]}";

        let request: Call = serde_json::from_str(rpc).unwrap();
        let params: Result<(String, String), Error> = request.params.unwrap().parse();
        assert!(params.is_ok());
    }

    #[test]
    fn test_rpc_request_parse1() {
        let rpc = "{\"id\":\"-8799978260242268161\",\
                   \"jsonrpc\":\"2.0\",\
                   \"method\":\"eth_call\",\
                   \"params\":[\"0x0a28336165383866653337306333393338346663313664613263\
                   39653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: Call = serde_json::from_str(rpc).unwrap();
        let params: Result<(String, String), Error> = request.params.unwrap().parse();
        assert!(params.is_err());
    }

    #[test]
    fn test_rpc_request_parse2() {
        let rpc = "{\"id\":\"-8799978260242268161\",\
                   \"jsonrpc\":\"2.0\",\
                   \"method\":\"eth_call\",\
                   \"params\":[\"0x0a28336165383866653337306333393338346663313664613263\
                   39653736386366356432343935623438120d31343932353139393038393631\"]}";

        let request: Call = serde_json::from_str(rpc).unwrap();
        let params: Result<(String,), Error> = request.params.unwrap().parse();
        assert!(params.is_ok());
    }

    #[test]
    fn test_blocklimit_backword_compatibility() {
        let rpc = "{\"jsonrpc\":\"2.0\",\
                   \"method\":\"cita_sendTransaction\",\
                   \"params\":[\"0x1201311a85010a401201311a3b2239080a12350a24303561626\
                   36538642d316431662d343536352d396636342d6262316430323639336533391\
                   0641a03303037220443495441280312417922853b51d097df76791aa10836942\
                   c66bc522c24c8804c93e9230fc67dde897bbed399fa0f9e9ac0abc598cd92215\
                   fb362b9e31251bf784511be61d045703e00\"],\"id\":2}";
        let request: Call = serde_json::from_str(rpc).unwrap();
        let params: Result<(String,), Error> = request.params.unwrap().parse();
        assert!(params.is_ok());
    }

    #[test]
    fn eth_call_with_blockid_deserialization() {
        let rpc = "{\"jsonrpc\":\"2.0\",\
                   \"method\":\"eth_call\",\
                   \"params\":[{\"from\":\"d46e8dd67c5d32be8058bb8eb970870f07244567\",\
                   \"to\":\"b60e8dd61c5d32be8058bb8eb970870f07233155\",\
                   \"data\":\"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f0724456\
                   75058bb8eb970870f072445675\"},\
                   \"22\"],\"id\":2}";
        let rpc_request: Call = serde_json::from_str(rpc).unwrap();

        let handler = MethodHandler;
        let request: Result<request::Request, Error> = handler.call(&rpc_request);

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
                    .unwrap()
            ).vec()
                .as_slice()
        );
        assert_eq!(call.get_height(), "\"0x22\"");
    }

    #[test]
    fn eth_call_deserialization() {
        let rpc = "{\"jsonrpc\":\"2.0\",\
                   \"method\":\"eth_call\",\
                   \"params\":[{\"from\":\"d46e8dd67c5d32be8058bb8eb970870f07244567\",\
                   \"to\":\"b60e8dd61c5d32be8058bb8eb970870f07233155\",\
                   \"data\":\"0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f07\
                   2445675058bb8eb970870f072445675\"}],\
                   \"id\":2}";
        let rpc_request: Call = serde_json::from_str(rpc).unwrap();

        let handler = MethodHandler;
        let request: Result<request::Request, Error> = handler.call(&rpc_request);

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
                    .unwrap()
            ).vec()
                .as_slice()
        );
    }

    #[test]
    fn cita_get_log_deserialization() {
        let rpc = "{\"jsonrpc\":\"2.0\",\
                   \"method\":\"eth_getLogs\",\
                   \"params\":[{\"fromBlock\":\"0x1\",\
                   \"toBlock\":\"0x2\",\
                   \"address\":\"8888f1f195afa192cfee860698584c030f4c9db1\",\
                   \"topics\": [\"0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b\",\
                   null,\
                   [\"0x000000000000000000000000a94f5374fce5edbc8e2a8697c15331677e6ebf0b\",\
                   \"0x0000000000000000000000000aff3454fce5edbc8cca8697c15331677e6ebccc\"]]}],\
                   \"id\":2}";
        let rpc_request: Call = serde_json::from_str(rpc).unwrap();
        let handler = MethodHandler;
        let request: Result<request::Request, Error> = handler.get_logs(&rpc_request);

        assert!(request.is_ok());
        let request = request.unwrap();
        let filter = request.get_filter();
        let params: Result<(Filter,), Error> = rpc_request.params.unwrap().parse();
        assert_eq!(serde_json::to_string(&params.unwrap().0).unwrap(), filter);
    }
}
