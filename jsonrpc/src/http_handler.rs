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

use base_hanlder::{BaseHandler, ReqInfo, RpcMap, TransferType};
use error::ErrorCode;
use hyper::Post;
use hyper::server::{Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use jsonrpc_types::{method, Error, RpcRequest};
use jsonrpc_types::response::RpcFailure;
use libproto::request as reqlib;
use serde_json;
use std::io::Read;
use std::result;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use util::Mutex;

impl BaseHandler for HttpHandler {}

pub struct HttpHandler {
    pub tx: Arc<Mutex<mpsc::Sender<(String, reqlib::Request)>>>,
    pub responses: RpcMap,
    pub timeout: usize,
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
                            Err(_) => Err(Error::invalid_request()), //TODO
                        }
                    }
                    _ => result::Result::Err(Error::invalid_request()),
                }
            }
            _ => result::Result::Err(Error::invalid_request()),
        }
    }

    pub fn deal_req(&self, post_data: String) -> Result<String, RpcFailure> {
        match HttpHandler::into_rpc(post_data) {
            Err(err) => Err(RpcFailure::from(err)),
            Ok(rpc) => self.send_mq(rpc),
        }
    }

    pub fn send_mq(&self, rpc: RpcRequest) -> Result<String, RpcFailure> {
        let id = rpc.id.clone();
        let jsonrpc_version = rpc.jsonrpc.clone();
        let topic = HttpHandler::select_topic(&rpc.method);
        match self.method_handler.request(rpc) {
            Ok(req) => {
                let request_id = req.request_id.clone();
                trace!("wait response {:?}", request_id);
                let (tx, rx) = mpsc::channel();
                {
                    self.responses.lock().insert(
                        request_id.clone(),
                        TransferType::HTTP((
                            ReqInfo {
                                id: id.clone(),
                                jsonrpc: jsonrpc_version.clone(),
                            },
                            tx,
                        )),
                    );
                    self.tx.lock().send((topic, req));
                }

                if let Ok(res) = rx.recv_timeout(Duration::from_secs(self.timeout as u64)) {
                    Ok(res)
                } else {
                    self.responses.lock().remove(&request_id);
                    Err(RpcFailure::from_options(
                        id,
                        jsonrpc_version,
                        Error::server_error(ErrorCode::time_out_error(), "system time out,please resend"),
                    ))
                }
            }

            Err(err) => Err(RpcFailure::from_options(id, jsonrpc_version, err)),
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
                    Ok(ret) => Ok(ret),
                    Err(err) => serde_json::to_string(&err),
                }
            }
        };

        //TODO
        trace!("JsonRpc respone data {:?}", data);
        res.send(
            data.expect("return client's respone data unwrap error")
                .as_ref(),
        );
    }
}
