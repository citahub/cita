// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::Duration;

use futures::{future::Future, stream::FuturesOrdered, sync::oneshot};
use hyper::HeaderMap as Headers;
use jsonrpc_types::{
    rpc_request::Request as JsonRequest, rpc_response::Output as JsonrpcResponse,
    rpc_types::Id as JsonrpcId,
};
use libproto::request::Request as ProtoRequest;
use pubsub::channel::Sender;
use tokio_timer::{clock, Delay};

use crate::helper::{select_topic, RpcMap, TransferType};
use crate::response::{BatchFutureResponse, PublishFutResponse, SingleFutureResponse};
use crate::service_error::ServiceError;
type HyperResponse = hyper::Response<hyper::Body>;

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

pub type ProtoReqSender = Sender<(String, ProtoRequest)>;

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
    timeout: Duration,
    timeout_responses: RpcMap,
}

impl TimeoutPublisher {
    pub fn new(publisher: Publisher, timeout: Duration, timeout_responses: RpcMap) -> Self {
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

        let timeout = Delay::new(clock::now() + self.timeout);
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

        let fut_resp = self
            .publisher
            .publish(req)
            .select2(timeout)
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
