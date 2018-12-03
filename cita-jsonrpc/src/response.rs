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

use futures::future::Future;
use futures::stream::{Collect, FuturesOrdered};
use futures::sync::oneshot;
use futures::{Async, Poll};
use hyper;
use hyper::header::Headers;
use hyper::server::Response;
use jsonrpc_types::response::Output;
use serde_json;

use crate::service_error::ServiceError;

pub trait IntoResponse {
    fn into_response(self, Headers) -> Response;
}

pub trait FutureResponse
where
    <<Self as FutureResponse>::Output as Future>::Error: std::fmt::Display,
    <<Self as FutureResponse>::Output as Future>::Item: serde::Serialize,
{
    type Output: Future;

    fn inner_output(&mut self) -> &mut Self::Output;
    fn headers(&mut self) -> &mut Option<Headers>;
    fn response_type() -> &'static str;

    fn poll_response(&mut self) -> Poll<Response, ServiceError> {
        let response_type = <Self as FutureResponse>::response_type();

        let resp = match self.inner_output().poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(resp)) => Ok(resp),
            Err(e) => {
                error!("pool {} response: {}", response_type, e);
                Err(ServiceError::MQResponsePollIncompleteError(
                    hyper::Error::Incomplete,
                ))
            }
        }?;

        let headers = self.headers().take().ok_or_else(|| {
            error!("pull {} future response twice", response_type);
            ServiceError::InternalServerError
        })?;

        let json_resp = serde_json::to_vec(&resp).map_err(|err| {
            error!("json serde {} response: {}", response_type, err);
            ServiceError::InternalServerError
        })?;

        Ok(Async::Ready(
            Response::new().with_headers(headers).with_body(json_resp),
        ))
    }
}

pub struct SingleFutureResponse {
    output: oneshot::Receiver<Output>,
    headers: Option<Headers>,
}

impl SingleFutureResponse {
    pub fn new(output: oneshot::Receiver<Output>, headers: Headers) -> SingleFutureResponse {
        SingleFutureResponse {
            output,
            headers: Some(headers),
        }
    }
}

impl FutureResponse for SingleFutureResponse {
    type Output = oneshot::Receiver<Output>;

    fn inner_output(&mut self) -> &mut Self::Output {
        &mut self.output
    }

    fn headers(&mut self) -> &mut Option<Headers> {
        &mut self.headers
    }

    fn response_type() -> &'static str {
        "single"
    }
}

impl Future for SingleFutureResponse {
    type Item = Response;
    type Error = ServiceError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.poll_response()
    }
}

type BatchOutput = Collect<FuturesOrdered<oneshot::Receiver<Output>>>;

pub struct BatchFutureResponse {
    output: BatchOutput,
    headers: Option<Headers>,
}

impl BatchFutureResponse {
    pub fn new(output: BatchOutput, headers: Headers) -> BatchFutureResponse {
        BatchFutureResponse {
            output,
            headers: Some(headers),
        }
    }
}

impl FutureResponse for BatchFutureResponse {
    type Output = BatchOutput;

    fn inner_output(&mut self) -> &mut Self::Output {
        &mut self.output
    }

    fn headers(&mut self) -> &mut Option<Headers> {
        &mut self.headers
    }

    fn response_type() -> &'static str {
        "batch"
    }
}

impl Future for BatchFutureResponse {
    type Item = Response;
    type Error = ServiceError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.poll_response()
    }
}

pub enum PublishFutResponse {
    Single(SingleFutureResponse),
    Batch(BatchFutureResponse),
}

impl Future for PublishFutResponse {
    type Item = Response;
    type Error = ServiceError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            PublishFutResponse::Single(resp) => resp.poll(),
            PublishFutResponse::Batch(resp) => resp.poll(),
        }
    }
}
