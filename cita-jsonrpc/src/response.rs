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
        })
        .map_err(|_| hyper::Error::Incomplete)
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
        })
        .map_err(|_| hyper::Error::Incomplete)
        .map(Async::Ready)
    }
}
