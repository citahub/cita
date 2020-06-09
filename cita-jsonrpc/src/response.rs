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

use futures::stream::{Collect, FuturesOrdered};
use futures::{future::Future, sync::oneshot, Async, Poll};
use hyper::{HeaderMap as Headers, Response as HyperResponse, StatusCode};
use jsonrpc_types::rpc_response::Output;

use crate::service_error::ServiceError;

pub type Response = HyperResponse<hyper::Body>;

// bring back hyper 0.11 api for easy of use
pub trait HyperResponseExt<T> {
    fn with_headers(self, headers: Headers) -> Self;
    fn with_body(self, body: T) -> Self;
    fn with_status(self, code: StatusCode) -> Self;
}

impl<T> HyperResponseExt<T> for HyperResponse<T> {
    fn with_headers(mut self, headers: Headers) -> Self {
        self.headers_mut().extend(headers);
        self
    }

    fn with_body(self, body: T) -> Self {
        self.map(|_| body)
    }

    fn with_status(mut self, code: StatusCode) -> Self {
        *self.status_mut() = code;
        self
    }
}

pub trait IntoResponse {
    fn into_response(self, _: Headers) -> Response;
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
                Err(ServiceError::MQResponsePollIncompleteError)
            }
        }?;

        let headers = self.headers().take().ok_or_else(|| {
            error!("pull {} future response twice", response_type);
            ServiceError::InternalServerError
        })?;

        let json_body = serde_json::to_vec(&resp).map_err(|err| {
            error!("json serde {} response: {}", response_type, err);
            ServiceError::InternalServerError
        })?;

        let resp = Response::default()
            .with_headers(headers)
            .with_body(hyper::Body::from(json_body));
        Ok(Async::Ready(resp))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyper_response_with_headers() {
        use crate::http_header::HeaderMapExt;
        use hyper::{header::*, Method};

        let mut headers = HeaderMap::new();
        let mut ext_headers = HeaderMap::new();
        let mut resp = Response::default();

        headers.insert(HOST, HeaderValue::from_static("citahub.com"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        ext_headers.insert_vec(
            ACCESS_CONTROL_ALLOW_METHODS,
            vec![Method::POST, Method::OPTIONS],
        );
        ext_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));

        resp = resp.with_headers(headers.clone());
        assert_eq!(resp.headers(), &headers);

        resp = resp.with_headers(ext_headers.clone());
        headers.extend(ext_headers);
        assert_eq!(resp.headers().len(), 3);
        assert_eq!(resp.headers(), &headers);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/plain"))
        );
    }

    #[test]
    fn test_hyper_response_with_body() {
        let mut resp = HyperResponse::new("");
        assert_eq!(resp.body(), &"");

        resp = resp.with_body("citahub");
        assert_eq!(resp.body(), &"citahub");
    }

    #[test]
    fn test_hyper_response_with_status() {
        let mut resp = Response::default();
        assert_eq!(resp.status(), StatusCode::OK);

        resp = resp.with_status(StatusCode::NOT_FOUND);
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}
