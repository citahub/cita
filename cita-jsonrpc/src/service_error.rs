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

use hyper::{Body, HeaderMap as Headers, Response, StatusCode};
use jsonrpc_types::{rpc_request::RequestInfo, rpc_response::RpcFailure};
use serde_json;

use crate::response::{HyperResponseExt, IntoResponse};

const MSG_TIMEOUT_RESEND: &str = r#"{"err": "System timeout, please resend."}"#;
const MSG_INCOMPLETE_REQUEST: &str = r#"{"err": "Incomplete request, please resend."}"#;

#[derive(Debug)]
pub enum ServiceError {
    BodyConcatError(hyper::Error),
    JsonrpcSerdeError(serde_json::Error),
    JsonrpcPartCompleteError(RequestInfo, jsonrpc_types::Error),
    MQRpcTimeout(Option<RequestInfo>),
    MQResponsePollIncompleteError,
    InternalServerError,
}

impl IntoResponse for ServiceError {
    fn into_response(self, http_headers: Headers) -> Response<Body> {
        let new_response = |status_code: Option<StatusCode>, body: Option<Body>| {
            let resp = Response::default().with_headers(http_headers);

            match (status_code, body) {
                (Some(code), Some(body)) => resp.with_status(code).with_body(body),
                (Some(code), None) => resp.with_status(code),
                (None, Some(body)) => resp.with_body(body),
                (None, None) => resp,
            }
        };

        match self {
            ServiceError::BodyConcatError(e) => new_response(None, Some(Body::from(e.to_string()))),
            ServiceError::JsonrpcSerdeError(_) => new_response(Some(StatusCode::BAD_REQUEST), None),
            ServiceError::JsonrpcPartCompleteError(req_info, err) => {
                let failure = RpcFailure::from_options(req_info, err);
                let resp_body = serde_json::to_vec(&failure).unwrap_or_else(|e| {
                    error!("serde_json: {}", e);
                    MSG_INCOMPLETE_REQUEST.as_bytes().to_vec()
                });

                new_response(None, Some(Body::from(resp_body)))
            }
            ServiceError::MQRpcTimeout(req_info) => {
                let timeout_err = jsonrpc_types::Error::server_error(
                    error::ErrorCode::time_out_error(),
                    MSG_TIMEOUT_RESEND,
                );
                let failure = match req_info {
                    Some(info) => RpcFailure::from_options(info, timeout_err),
                    None => RpcFailure::from(timeout_err),
                };
                let resp_body = serde_json::to_vec(&failure).unwrap_or_else(|e| {
                    error!("serde_json: {}", e);
                    MSG_TIMEOUT_RESEND.as_bytes().to_vec()
                });

                new_response(None, Some(Body::from(resp_body)))
            }
            ServiceError::InternalServerError | ServiceError::MQResponsePollIncompleteError => {
                new_response(Some(StatusCode::INTERNAL_SERVER_ERROR), None)
            }
        }
    }
}
