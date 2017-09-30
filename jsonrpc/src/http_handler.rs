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

use base_hanlder::BaseHandler;
use hyper::Post;
use hyper::server::{Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use jsonrpc_types::{RpcRequest, method};
use jsonrpc_types::error::Error;
use jsonrpc_types::response::{RpcSuccess, RpcFailure, Output};
use libproto::request as reqlib;
use libproto::response; 
use parking_lot::{RwLock, Mutex};
//use protobuf::Message;
use serde_json;
use std::collections::HashMap;
use std::io::Read;
use std::result;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

impl BaseHandler for HttpHandler {}

pub struct HttpHandler {
    pub tx: Arc<Mutex<Sender<(String, reqlib::Request)>>>,
    //TODO 定时清理工作
    pub responses: Arc<RwLock<HashMap<Vec<u8>, response::Response>>>,
    pub sleep_duration: usize,
    pub timeout_count: usize,
    pub method_handler: method::MethodHandler,
}

impl HttpHandler {
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
        match HttpHandler::into_rpc(post_data) {
            Err(err) => Err(RpcFailure::from(err)),
            Ok(rpc) => self.send_mq(rpc),
        }
    }

    pub fn send_mq(&self, rpc: RpcRequest) -> Result<RpcSuccess, RpcFailure> {
        let id = rpc.id.clone();
        let jsonrpc_version = rpc.jsonrpc.clone();
        let topic = HttpHandler::select_topic(&rpc.method);
        match self.method_handler.from_req(rpc) {
            Ok(req) => {
                let request_id = req.request_id.clone();
                //let msg: communication::Message = req.into();
                {
                    //self.tx.lock().send((topic, msg.write_to_bytes().unwrap())).unwrap();
                    self.tx.lock().send((topic, req)).unwrap();
                }
                trace!("wait response {:?}", String::from_utf8(request_id.clone()));
                let mut timeout_count = 0;
                loop {
                    timeout_count = timeout_count + 1;
                    if timeout_count > self.timeout_count {
                        return Err(RpcFailure::from_options(id, jsonrpc_version, Error::server_error(-32099, "system time out,please resend")));
                    }
                    thread::sleep(Duration::new(0, (self.sleep_duration * 1000000) as u32));
                    if self.responses.read().contains_key(&request_id) {
                        let value = {
                            self.responses.write().remove(&request_id)
                        };
                        if let Some(res) = value {
                            match Output::from(res, id, jsonrpc_version) {
                                Output::Success(success) => return Ok(success),
                                Output::Failure(failure) => return Err(failure),
                            }
                        }
                    }
                }
            }

            Err(err) => {
                Err(RpcFailure::from_options(id, jsonrpc_version, err))
            }
        }
    }
}



impl Handler for HttpHandler {
    fn handle(&self, req: Request, res: Response) {
        //TODO 不允许在这里做业务处理。
        let data = match self.pase_url(req) {
            Err(err) => serde_json::to_string(&RpcFailure::from(err)),
            Ok(body) => {
                trace!("JsonRpc recive raw Request data {:?}", body);
                match self.deal_req(body) {
                    Ok(ret) => serde_json::to_string(&ret),
                    Err(err) => serde_json::to_string(&err),
                }
            }
        };

        //TODO
        trace!("JsonRpc respone data {:?}", data);
        res.send(data.expect("return client's respone data unwrap error").as_ref());
    }
}
