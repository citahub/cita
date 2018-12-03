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

use hyper::{header::Headers, server::Response, StatusCode};
use jsonrpc_types::{request::RequestInfo, response::RpcFailure};
use serde_json;

use crate::response::IntoResponse;

const MSG_TIMEOUT_RESEND: &str = r#"{"err": "System timeout, please resend."}"#;
const MSG_INCOMPLETE_REQUEST: &str = r#"{"err": "Incomplete request, please resend."}"#;

#[derive(Debug)]
pub enum ServiceError {
    BodyConcatError(hyper::Error),
    JsonrpcSerdeError(serde_json::Error),
    JsonrpcPartCompleteError(RequestInfo, jsonrpc_types::Error),
    MQRpcTimeout(Option<RequestInfo>),
    MQResponsePollIncompleteError(hyper::Error),
    InternalServerError,
}

impl IntoResponse for ServiceError {
    fn into_response(self, http_headers: Headers) -> Response {
        let new_response = |status_code: Option<StatusCode>, body: Option<Vec<u8>>| {
            let resp = Response::new().with_headers(http_headers);

            match (status_code, body) {
                (Some(code), Some(body)) => resp.with_status(code).with_body(body),
                (Some(code), None) => resp.with_status(code),
                (None, Some(body)) => resp.with_body(body),
                (None, None) => resp,
            }
        };

        match self {
            ServiceError::BodyConcatError(e) | ServiceError::MQResponsePollIncompleteError(e) => {
                new_response(None, Some(e.to_string().into_bytes()))
            }
            ServiceError::JsonrpcSerdeError(_) => new_response(Some(StatusCode::BadRequest), None),
            ServiceError::JsonrpcPartCompleteError(req_info, err) => {
                let failure = RpcFailure::from_options(req_info, err);
                let resp_body = serde_json::to_vec(&failure).unwrap_or_else(|e| {
                    error!("serde_json: {}", e);
                    MSG_INCOMPLETE_REQUEST.as_bytes().to_vec()
                });

                new_response(None, Some(resp_body))
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

                new_response(None, Some(resp_body))
            }
            ServiceError::InternalServerError => {
                new_response(Some(StatusCode::InternalServerError), None)
            }
        }
    }
}
