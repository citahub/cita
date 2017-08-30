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

use base_hanlder::{BaseHandler, RpcResult};
use hyper::Post;
use hyper::server::{Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use jsonrpc_types::error::Error;
use jsonrpc_types::method;
use jsonrpc_types::response::{self as cita_response, RpcSuccess, RpcFailure};
use libproto::{blockchain, request};
use libproto::communication;
use parking_lot::{RwLock, Mutex};
use protobuf::Message;
use serde_json;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Read;
use std::result;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use util::H256;

impl BaseHandler for RpcHandler {}

pub struct RpcHandler {
    pub tx: Arc<Mutex<Sender<(String, Vec<u8>)>>>,
    pub responses: Arc<RwLock<HashMap<Vec<u8>, request::Response>>>,
    pub tx_responses: Arc<RwLock<HashMap<H256, blockchain::TxResponse>>>,
    pub sleep_duration: usize,
    pub timeout_count: usize,
    pub method_handler: method::MethodHandler,
}


impl RpcHandler {
    pub fn pase_url(&self, mut req: Request) -> Result<String, Error> {
        let uri = req.uri.clone();
        let method = req.method.clone();
        match uri {
            AbsolutePath(ref path) => {
                match (&method, &path[..]) {
                    (&Post, "/") => {
                        let mut body = String::new();
                        match req.read_to_string(&mut body) {
                            Ok(_) => Ok(body),
                            Err(_) => Err(Error::invalid_request()),//TODO
                        }
                    }
                    _ => result::Result::Err(Error::invalid_request()),
                }
            }
            _ => result::Result::Err(Error::invalid_request()),
        }
    }


    pub fn deal_req(&self, post_data: String) -> Result<RpcSuccess, RpcFailure> {
        match RpcHandler::into_json(post_data) {
            Err(err) => Err(RpcFailure::from(err)),
            Ok(rpc) => {
                let req_id = rpc.id.clone();
                let jsonrpc_version = rpc.jsonrpc.clone();
                let topic = RpcHandler::select_topic(&rpc.method);
                match self.method_handler.from_req(rpc)? {
                    method::RpcReqType::TX(tx) => {
                        let hash = tx.crypt_hash();
                        self.send_mq(topic, tx.into(), self.tx_responses.clone(), hash)
                            .map_err(|err_data| RpcFailure::from_options(req_id.clone(), jsonrpc_version.clone(), err_data))
                            .map(|data| {
                                     RpcSuccess {
                                         jsonrpc: jsonrpc_version,
                                         id: req_id,
                                         result: cita_response::ResponseBody::from(data), //TODO
                                     }
                                 })
                    }
                    method::RpcReqType::REQ(req) => {
                        let key = req.request_id.clone();
                        self.send_mq(topic, req.into(), self.responses.clone(), key)
                            .map_err(|err_data| RpcFailure::from_options(req_id.clone(), jsonrpc_version.clone(), err_data))
                            .map(|data| {
                                     RpcSuccess {
                                         jsonrpc: jsonrpc_version,
                                         id: req_id,
                                         result: cita_response::ResponseBody::from(data.result.expect("chain response error")), //TODO
                                     }
                                 })
                    }
                }
            }
        }
    }


    pub fn send_mq<K, V>(&self, topic: String, req: communication::Message, responses: Arc<RwLock<HashMap<K, V>>>, key: K) -> RpcResult<V>
    where
        K: Eq + Hash + Debug,
    {
        {
            let tx = self.tx.clone();
            tx.lock().send((topic, req.write_to_bytes().unwrap())).unwrap();
        }
        trace!("wait response {:?}", key);
        let mut timeout_count = 0;
        loop {
            timeout_count = timeout_count + 1;
            if timeout_count > self.timeout_count {
                //TODO
                return Err(Error::server_error(-32099, "system time out,please resend"));
            }
            thread::sleep(Duration::new(0, (self.sleep_duration * 1000000) as u32));
            if responses.read().contains_key(&key) {
                let mut responses = responses.write();
                if let Some(res) = responses.remove(&key) {
                    return Ok(res);
                } else {
                    //TODO
                    return Err(Error::invalid_params("duplicated transaction,please wait a while "));
                }
            }
        }
    }
}



impl Handler for RpcHandler {
    fn handle(&self, req: Request, res: Response) {
        //TODO 不允许在这里做业务处理。
        let data = match self.pase_url(req) {
            Err(err) => serde_json::to_string(&RpcFailure::from(err)),
            Ok(body) => {
                trace!("Request data {:?}", body);
                match self.deal_req(body) {
                    Ok(ret) => serde_json::to_string(&ret),
                    Err(err) => serde_json::to_string(&err),
                }
            }
        };

        //TODO
        trace!("respone data {:?}", data);
        res.send(data.unwrap().as_ref());
    }
}
