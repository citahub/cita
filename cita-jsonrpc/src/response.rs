use futures::future::Future;
use futures::stream::{Collect, FuturesOrdered};
use futures::sync::oneshot;
use futures::{Async, Poll};
use hyper;
use hyper::header::Headers;
use hyper::server::Response;
use jsonrpc_types::response::Output;
use jsonrpc_types::response::RpcResponse;
use serde_json;

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

impl Future for SingleFutureResponse {
    type Item = Response;
    type Error = hyper::Error;

    fn poll(&mut self) -> Poll<Response, hyper::Error> {
        let e = match self.output.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(e)) => Ok(e),
            Err(e) => Err(e),
        };

        e.map(|resp_body| {
            Response::new()
                .with_headers(
                    self.headers
                        .take()
                        .expect("cannot poll SingleFutureResponse twice"),
                )
                .with_body(serde_json::to_vec(&resp_body).unwrap())
        }).map_err(|_| hyper::Error::Incomplete)
            .map(Async::Ready)
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

impl Future for BatchFutureResponse {
    type Item = Response;
    type Error = hyper::Error;

    fn poll(&mut self) -> Poll<Response, hyper::Error> {
        let e = match self.output.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(e)) => Ok(e),
            Err(e) => Err(e),
        };

        e.map(|resp_body| {
            Response::new()
                .with_headers(
                    self.headers
                        .take()
                        .expect("cannot poll BatchFutureResponse twice"),
                )
                .with_body(serde_json::to_vec(&RpcResponse::Batch(resp_body)).unwrap())
        }).map_err(|_| hyper::Error::Incomplete)
            .map(Async::Ready)
    }
}
