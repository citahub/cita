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

#![allow(unused_must_use)]
use hyper::Post;
use hyper::server::{Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use hyper::uri::RequestUri;
use hyper::method::Method;
use libproto::{blockchain, request, TopicMessage};
use std::sync::{RwLock, Arc, Mutex};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::io::{self, Read};
use serde_json;
use jsonrpc_types::rpc_request::RpcRequest;
use jsonrpc_types::RpcError;
use std::convert::TryInto;
use jsonrpc_types::rpc_response::{self as cita_response, ResErrBody, RpcResponse, ResErr};
use util::hash::H256;
use std::thread;
use std::time::Duration;
use jsonrpc_types::rpc_request::ReqType;
use libproto::{communication, Response_oneof_result};
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;
use std::result;
use jsonrpc_types::Id;
use std::fmt;

pub type RpcResult<T> = result::Result<T, ErrorCode>;

// TODO: 对用户更友好的错误提示。错误码提示修改 https://github.com/ethereum/wiki/wiki/JSON-RPC-Error-Codes-Improvement-Proposal
// 比如可以提示期待3或2个参数，收到4个参数等等
// eg: {"jsonrpc":"2.0","error":{"code":-32602,"message":"unknown field `input`, expected one of `from`, `to`, `gasPrice`, `gas`, `value`, `data`, `nonce`","data":null},"id":1}
// {"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid length.","data":null},"id":1}
// {"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid format","data":null},"id":1}
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    // TODO: Fix, JSON错误码和Http错误码区分
    TimeOut = 326009,
    InvalidReq = -32600,
    MethodErr = -32601,
    DuplicatedTx = -32602,
    // TODO: InternalError
    DbError = -32603,
    AuthErr = -32604,
    InvalidParams = -32605,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn read_to_string(mut req: Request) -> io::Result<String> {
    let mut s = String::new();
    req.read_to_string(&mut s)?;
    Ok(s)
}

pub struct RpcHandler {
    pub tx: Arc<Mutex<Sender<TopicMessage>>>,
    pub responses: Arc<RwLock<HashMap<Vec<u8>, request::Response>>>,
    pub tx_responses: Arc<RwLock<HashMap<H256, blockchain::TxResponse>>>,
    pub sleep_duration: u32,
    pub timeout_count: u32,
}


impl RpcHandler {
    pub fn is_valid_url(url: RequestUri, method: Method) -> RpcResult<()> {
        match url {
            AbsolutePath(ref path) => {
                match (&method, &path[..]) {
                    (&Post, "/") => result::Result::Ok(()),
                    _ => result::Result::Err(ErrorCode::InvalidReq),
                }
            }
            _ => result::Result::Err(ErrorCode::InvalidReq),
        }
    }

    pub fn is_valid_param<T>(req_rpc: &Result<T, RpcError>) -> RpcResult<()> {
        match *req_rpc {
            Err(RpcError::NotFound) => Err(ErrorCode::MethodErr),
            Err(RpcError::InvalidParams) => Err(ErrorCode::InvalidParams),
            Ok(_) => Ok(()),
        }
    }

    pub fn select_topic(method: &String) -> String {
        let topic = if method.starts_with("cita_send") {
                "jsonrpc.new_tx"
            } else if method.starts_with("cita") || method.starts_with("eth") {
                "jsonrpc.request"
            } else if method.starts_with("net_") {
                "jsonrpc.net"
            } else {
                "jsonrpc"
            }
            .to_string();
        topic
    }

    pub fn deal_other_req(&self, rpc: RpcRequest) -> RpcResult<String> {
        let proto_req: result::Result<request::Request, RpcError> = rpc.clone().try_into();
        trace!("deal_other_req display {:?} ", proto_req);
        RpcHandler::is_valid_param(&proto_req)?;
        let proto_req = proto_req.unwrap();
        let req_id = proto_req.request_id.clone();
        let recv_msg = self.send_mq(RpcHandler::select_topic(&rpc.method),
                                    proto_req.into(),
                                    self.responses.clone(),
                                    req_id)?;
        let mut rpc_response = cita_response::RpcResult {
            id: rpc.id.clone(),
            jsonrpc: rpc.jsonrpc.clone(),
            result: cita_response::ResponseBody::Null,
            error: ResErrBody::default(),
        };
        
        rpc_response.result = cita_response::ResponseBody::from(recv_msg.result.unwrap_or(Response_oneof_result::none(false)));
        Ok(serde_json::to_string(&RpcResponse::from(rpc_response)).unwrap())
    }

    fn deal_tx(&self, rpc: RpcRequest) -> RpcResult<String> {
        let rpc_tx = rpc.clone();
        let proto_ts: Result<blockchain::Transaction, RpcError> = rpc_tx.try_into();
        RpcHandler::is_valid_param(&proto_ts)?;
        let tx = proto_ts.unwrap();
        // self.check_tx_auth(&tx)?;
        let hash = tx.sha3();
        let recv_msg = self.send_mq(RpcHandler::select_topic(&rpc.method),
                                    tx.into(),
                                    self.tx_responses.clone(),
                                    hash);
        let mut rpc_response = cita_response::RpcResult {
            id: rpc.id.clone(),
            jsonrpc: rpc.jsonrpc.clone(),
            result: cita_response::ResponseBody::Null,
            error: ResErrBody::default(),
        };
        rpc_response.result = cita_response::ResponseBody::from(recv_msg.unwrap_or_default());
        Ok(serde_json::to_string(&RpcResponse::from(rpc_response)).unwrap())
    }

    pub fn send_mq<K, V>(&self,
                         topic: String,
                         req: communication::Message,
                         responses: Arc<RwLock<HashMap<K, V>>>,
                         key: K)
                         -> RpcResult<V>
        where K: Eq + Hash + Debug
    {
        self.tx.lock().unwrap().send((topic, req.into())).unwrap();
        trace!("wait response {:?}", key);
        let mut timeout_count = 0;
        loop {
            timeout_count = timeout_count + 1;
            if timeout_count > self.timeout_count {
                return Err(ErrorCode::TimeOut);
            }
            thread::sleep(Duration::new(0, self.sleep_duration * 1000000));
            if responses.read().unwrap().contains_key(&key) {
                let mut responses = responses.write().unwrap();
                if let Some(res) = responses.remove(&key) {
                    return Ok(res);
                } else {
                    return Err(ErrorCode::DuplicatedTx);
                }
            }
        }
    }
}


impl Handler for RpcHandler {
    fn handle(&self, req: Request, res: Response) {
        let uri = req.uri.clone();
        let method = req.method.clone();
        let body = read_to_string(req).unwrap();
        trace!("receive Request method: {:?}, body: {:?}", method, body);
        let mut id = Id::Null;
        let mut jsonrpc_version = Option::None;
        let ret = match RpcHandler::is_valid_url(uri, method) {
            Err(code) => Err(code),
            Ok(_) => {
                let rpc: Result<RpcRequest, serde_json::Error> = serde_json::from_str(&body);
                match rpc {
                    Err(_err_msg) => Err(ErrorCode::InvalidParams),
                    Ok(rpc) => {
                        id = rpc.id.clone();
                        jsonrpc_version = rpc.jsonrpc.clone();
                        let topic = RpcHandler::select_topic(&rpc.method);
                        info!("-----rpc dispacth topic: {:?}-----", topic);
                        match rpc.req_type() {
                            ReqType::TX => self.deal_tx(rpc),
                            ReqType::OTHER => self.deal_other_req(rpc),
                        }
                    }
                }
            }
        };

        let data = match ret {
            Err(err_code) => {
                let err_msg: String = format!("{:?}", err_code);
                let rpc_response = cita_response::RpcResult {
                    id: id,
                    jsonrpc: jsonrpc_version,
                    result: cita_response::ResponseBody::Null,
                    error: ResErrBody {
                        code: err_code as isize,
                        message: err_msg,
                    },
                };
                serde_json::to_string(&ResErr::from(rpc_response)).unwrap()
            }
            Ok(msg) => msg,
        };
        trace!("--------return data ={:?}---", data);
        res.send(data.as_bytes());
    }
}

#[cfg(test)]
mod test {
    use super::RpcHandler;
    use super::Method;
    use hyper::uri::RequestUri::AbsolutePath;
    use super::RpcError;

    #[test]
    fn test_get_topic() {
        assert_eq!(RpcHandler::select_topic(&"net_work".to_string()),
                   "jsonrpc.net".to_string());
        assert_eq!(RpcHandler::select_topic(&"cita_send".to_string()),
                   "jsonrpc.new_tx".to_string());
        assert_eq!(RpcHandler::select_topic(&"cita".to_string()),
                   "jsonrpc.request".to_string());
        assert_eq!(RpcHandler::select_topic(&"eth".to_string()),
                   "jsonrpc.request".to_string());
        assert_eq!(RpcHandler::select_topic(&"123".to_string()),
                   "jsonrpc".to_string());
    }

    #[test]
    fn test_is_valid_url() {
        assert!(RpcHandler::is_valid_url(AbsolutePath("/where?q=now".to_string()), Method::Post)
                    .is_err());
        assert!(RpcHandler::is_valid_url(AbsolutePath("where/".to_string()), Method::Post)
                    .is_err());
        assert!(RpcHandler::is_valid_url(AbsolutePath("/".to_string()), Method::Post).is_ok());
    }

    #[test]
    fn test_is_valid_param() {
        assert!(RpcHandler::is_valid_param::<()>(&Err(RpcError::NotFound)).is_err());
        assert!(RpcHandler::is_valid_param::<()>(&Err(RpcError::InvalidParams)).is_err());
        assert!(RpcHandler::is_valid_param::<()>(&Ok(())).is_ok());
    }
}