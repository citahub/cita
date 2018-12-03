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

use futures::future::{Future, FutureResult};
use hyper::Request as HttpRequest;
use jsonrpc_types::request::{
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

impl FutExtractor<JsonrpcRequest> for HttpRequest {
    type Error = ServiceError;
    type Fut = ExtractFuture<JsonrpcRequest, Self::Error>;

    fn extract_from(self) -> Self::Fut {
        use futures::Stream;

        let fut_resp = self
            .body()
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
