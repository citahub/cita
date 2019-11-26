// Copyright Cryptape Technologies LLC.
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

use futures::future::{Future, FutureResult};
use jsonrpc_proto::complete::CompleteInto;
use jsonrpc_types::rpc_request::{
    PartialRequest, Request as JsonRequest, RpcRequest as JsonrpcRequest,
};
use libproto::request::Request as ProtoRequest;

use crate::mq_publisher::{HybridRequest, MQRequest};
use crate::service_error::ServiceError;

pub trait FutExtractor<T> {
    type Error;
    type Fut: Future<Item = T, Error = Self::Error> + Send + 'static;

    fn extract_from(self) -> Self::Fut;
}

pub trait Extractor<T> {
    type Error;

    fn extract_from(self) -> Result<T, Self::Error>;
}

pub type ExtractFuture<T, E> = Box<dyn Future<Item = T, Error = E> + Send + 'static>;

impl FutExtractor<JsonrpcRequest> for hyper::Request<hyper::Body> {
    type Error = ServiceError;
    type Fut = ExtractFuture<JsonrpcRequest, Self::Error>;

    fn extract_from(self) -> Self::Fut {
        use futures::Stream;

        let fut_resp = self
            .into_body()
            .concat2()
            .map_err(ServiceError::BodyConcatError)
            .and_then(|chunk| {
                serde_json::from_slice::<JsonrpcRequest>(&chunk)
                    .map_err(ServiceError::JsonrpcSerdeError)
            });

        Box::new(fut_resp)
    }
}

impl Extractor<HybridRequest> for PartialRequest {
    type Error = ServiceError;

    fn extract_from(self) -> Result<HybridRequest, Self::Error> {
        let req_info = self.get_info();

        self.complete_and_into_proto()
            .map_err(|e| ServiceError::JsonrpcPartCompleteError(req_info, e))
            .map(|req: (JsonRequest, ProtoRequest)| HybridRequest {
                json_req: req.0,
                proto_req: req.1,
            })
    }
}

impl FutExtractor<MQRequest> for JsonrpcRequest {
    type Error = ServiceError;
    type Fut = ExtractFuture<MQRequest, Self::Error>;

    fn extract_from(self) -> Self::Fut {
        let fut_ret: FutureResult<MQRequest, ServiceError> = match self {
            JsonrpcRequest::Single(part_req) => Extractor::<HybridRequest>::extract_from(part_req)
                .map(|hybrid_req| MQRequest::Single(Box::new(hybrid_req))),
            JsonrpcRequest::Batch(part_reqs) => part_reqs
                .into_iter()
                .map(Extractor::<HybridRequest>::extract_from)
                .collect::<Result<Vec<HybridRequest>, ServiceError>>()
                .map(MQRequest::Batch),
        }
        .into();

        Box::new(fut_ret)
    }
}
