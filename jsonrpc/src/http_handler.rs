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
use hyper::status::StatusCode;
use hyper::server::{Handler, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;
use jsonrpc_types::{method, Error, RpcRequest};
use jsonrpc_types::response::RpcFailure;
use libproto::request as reqlib;
use serde_json::{self, from_value, Value};
use std::sync::{mpsc, Arc};
use std::time::Duration;
use util::Mutex;

impl BaseHandler for HttpHandler {}

pub struct HttpHandler {
    pub tx: Arc<Mutex<mpsc::Sender<(String, reqlib::Request)>>>,
    pub responses: RpcMap,
    pub timeout: u64,
    pub method_handler: method::MethodHandler,
}

impl HttpHandler {
    pub fn send_mq(&self, rpc: RpcRequest) -> String {
        let id = rpc.id.clone();
        let jsonrpc_version = rpc.jsonrpc.clone();
        let topic = HttpHandler::select_topic(&rpc.method);
        self.method_handler
            .request(rpc)
            .map(|req| {
                let request_id = req.request_id.clone();
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
                rx.recv_timeout(Duration::from_secs(self.timeout))
                    .unwrap_or_else(|_| {
                        self.responses.lock().remove(&request_id);
                        let failure = RpcFailure::from_options(
                            id.clone(),
                            jsonrpc_version.clone(),
                            Error::server_error(
                                ErrorCode::time_out_error(),
                                "system time out, please resend",
                            ),
                        );
                        serde_json::to_string(&failure).expect("should be serialize by serde_json")
                    })
            })
            .unwrap_or_else(|err| {
                serde_json::to_string(&RpcFailure::from_options(id, jsonrpc_version, err))
                    .expect("should be serialize by serde_json")
            })
    }
}



impl Handler for HttpHandler {
    fn handle(&self, request: Request, mut response: Response) {
        if request.uri != AbsolutePath(String::from("/")) {
            *response.status_mut() = StatusCode::NotFound;
            return;
        }
        if request.method != Post {
            *response.status_mut() = StatusCode::BadRequest;
            return;
        }
        let result: Result<String, StatusCode> = serde_json::from_reader::<_, Value>(request)
            .map(|json| {
                trace!("recv: {}", json);
                from_value(json.clone())
                    .map(|item: RpcRequest| {
                        // single request
                        self.send_mq(item)
                    })
                    .or_else(|_| {
                        from_value(json.clone())
                            .map(|items: Vec<RpcRequest>| {
                                // batch request
                                let mut first = true;
                                items.into_iter().map(|item| self.send_mq(item)).fold(
                                    String::from("["),
                                    |last, item| {
                                        if first {
                                            first = false;
                                            last + item.as_ref()
                                        } else {
                                            last + "," + item.as_ref()
                                        }
                                    },
                                ) + "]"
                            })
                            .map_err(|err| {
                                warn!("failed to parse batch request: {}", err);
                                StatusCode::BadRequest
                            })
                    })
            })
            .map_err(|err| {
                warn!("failed to parse into Value: {}", err);
                StatusCode::BadRequest
            })
            .and_then(|result| match result {
                Ok(text) => Ok(text),
                Err(err) => Err(err),
            });

        result
            .map_err(|status| {
                *response.status_mut() = status;
            })
            .map(|text| {
                trace!("send: {}", text);
                response.send(text.as_ref());
            });
    }
}
