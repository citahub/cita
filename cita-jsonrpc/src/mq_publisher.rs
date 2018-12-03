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

use std::sync::mpsc;

use futures::{future::Future, stream::FuturesOrdered, sync::oneshot};
use hyper::{header::Headers, server::Response as HyperResponse};
use jsonrpc_types::{
    request::Request as JsonRequest, response::Output as JsonrpcResponse, rpctypes::Id as JsonrpcId,
};
use libproto::request::Request as ProtoRequest;
use tokio_core::reactor::Timeout;

use crate::helper::{select_topic, RpcMap, TransferType};
use crate::response::{BatchFutureResponse, PublishFutResponse, SingleFutureResponse};
use crate::service_error::ServiceError;

#[derive(Debug)]
pub struct HybridRequest {
    pub json_req: JsonRequest,
    pub proto_req: ProtoRequest,
}

#[derive(Debug)]
pub enum MQRequest {
    Single(Box<HybridRequest>),
    Batch(Vec<HybridRequest>),
}

pub enum AccessLog {
    Single {
        id: JsonrpcId,
        method: Option<String>,
    },
    Batch {
        count: Option<usize>,
    },
}

impl MQRequest {
    pub fn access_log(&self) -> AccessLog {
        match self {
            MQRequest::Single(ref hybrid_req) => AccessLog::Single {
                id: hybrid_req.json_req.id.clone(),
                method: Some(hybrid_req.json_req.get_method().to_owned()),
            },
            MQRequest::Batch(ref hybrid_reqs) => AccessLog::Batch {
                count: Some(hybrid_reqs.len()),
            },
        }
    }
}

pub type ProtoReqSender = mpsc::Sender<(String, ProtoRequest)>;

pub struct Publisher {
    responses: RpcMap,
    sender: ProtoReqSender,
    headers: Headers,
}

impl Publisher {
    pub fn new(responses: RpcMap, sender: ProtoReqSender, headers: Headers) -> Self {
        Self {
            responses,
            sender,
            headers,
        }
    }

    pub fn publish(&mut self, req: MQRequest) -> PublishFutResponse {
        use futures::Stream;
        use std::iter::FromIterator;

        match req {
            MQRequest::Single(req) => {
                let rx = self.send_request(*req);

                let resp = SingleFutureResponse::new(rx, self.headers.clone());
                PublishFutResponse::Single(resp)
            }
            MQRequest::Batch(reqs) => {
                let rxs = reqs
                    .into_iter()
                    .map(|req| self.send_request(req))
                    .collect::<Vec<oneshot::Receiver<JsonrpcResponse>>>();

                let resp = BatchFutureResponse::new(
                    FuturesOrdered::from_iter(rxs).collect(),
                    self.headers.clone(),
                );
                PublishFutResponse::Batch(resp)
            }
        }
    }

    fn send_request(&mut self, hybrid_req: HybridRequest) -> oneshot::Receiver<JsonrpcResponse> {
        let (json_req, proto_req) = (hybrid_req.json_req, hybrid_req.proto_req);
        let (tx, rx) = oneshot::channel();
        let topic = select_topic(json_req.get_method());

        self.responses.lock().insert(
            proto_req.request_id.clone(),
            TransferType::HTTP((json_req.get_info(), tx)),
        );

        // NOTE: send failure is handled as timeout error
        let _ = self.sender.send((topic, proto_req));

        rx
    }
}

pub struct TimeoutPublisher {
    publisher: Publisher,
    timeout: Timeout,
    timeout_responses: RpcMap,
}

impl TimeoutPublisher {
    pub fn new(publisher: Publisher, timeout: Timeout, timeout_responses: RpcMap) -> Self {
        Self {
            publisher,
            timeout,
            timeout_responses,
        }
    }

    pub fn publish(
        mut self,
        req: MQRequest,
    ) -> Box<dyn Future<Item = HyperResponse, Error = ServiceError> + Send + 'static> {
        use futures::future::Either;
        use std::sync::Arc;

        let timeout_responses = Arc::clone(&self.timeout_responses);
        let (req_info, req_ids) = match req {
            MQRequest::Single(ref hybrid_req) => (
                Some(hybrid_req.json_req.get_info()),
                vec![hybrid_req.proto_req.request_id.clone()],
            ),
            MQRequest::Batch(ref hybrid_reqs) => (
                None,
                hybrid_reqs
                    .iter()
                    .map(|ref req| req.proto_req.request_id.clone())
                    .collect(),
            ),
        };

        let fut_resp =
            self.publisher
                .publish(req)
                .select2(self.timeout)
                .then(move |res| match res {
                    Ok(Either::A((mq_resp, _timeout))) => Ok(mq_resp),
                    Ok(Either::B((_reach_timeout, _no_resp))) => {
                        let mut guard = timeout_responses.lock();
                        for id in req_ids {
                            guard.remove(&id);
                        }
                        Err(ServiceError::MQRpcTimeout(req_info))
                    }
                    Err(Either::A((mq_rpc_err, _timeout))) => Err(mq_rpc_err),
                    Err(Either::B((_timeout_err, _mq_rpc_err))) => {
                        Err(ServiceError::InternalServerError)
                    }
                });

        Box::new(fut_resp)
    }
}
