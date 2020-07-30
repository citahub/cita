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

use libproto::TryInto;
use parking_lot::RwLock;
use std::convert::Into;

use crate::configuration::UpStream;
use cita_types::{H256, U256};
use jsonrpc_types::{rpc_request, rpc_types};
use libproto::blockchain::UnverifiedTransaction;

#[derive(Debug)]
pub enum Error {
    BadStatus,
    Parse,
}

pub struct RpcClient {
    uri: RwLock<hyper::Uri>,
}

impl RpcClient {
    pub fn create(upstream: &UpStream) -> ::std::sync::Arc<Self> {
        let uri = upstream.url.parse::<hyper::Uri>().unwrap();

        ::std::sync::Arc::new(RpcClient {
            uri: RwLock::new(uri),
        })
    }

    pub fn do_post(&self, body: String) -> Result<Vec<u8>, Error> {
        let uri = { self.uri.read().clone() };
        trace!("Send body {:?} to {:?}.", body, uri);
        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(uri)
            .header("content-type", "application/json")
            .body(hyper::Body::from(body))
            .unwrap();

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let data = rt.block_on(async {
            let client = hyper::Client::new();

            let resp = client.request(req).await.unwrap();
            hyper::body::to_bytes(resp.into_body()).await.unwrap()
        });
        Ok(data.to_vec())
    }
}

// Pack the result type into a reply type, and parse result from the reply, and return the result.
// The user of this macro do NOT have to care about the inner reply type.
macro_rules! rpc_send_and_get_result_from_reply {
    ($upstream:ident, $request:ident, $result_type:path) => {{
        define_reply_type!(ReplyType, $result_type);
        let rpc_cli = RpcClient::create($upstream);
        let body: String = $request.into();
        let data = rpc_cli.do_post(body.clone())?;
        let reply: ReplyType = serde_json::from_slice(&data).map_err(|_| {
            error!(
                "send {:?} return error: {:?}",
                &body,
                ::std::str::from_utf8(&data)
            );
            Error::Parse
        })?;
        trace!("get reply {:?}.", reply);
        reply.result
    }};
}

macro_rules! define_reply_type {
    ($reply_type:ident, $result_type:path) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct $reply_type {
            pub jsonrpc: Option<rpc_types::Version>,
            pub id: rpc_types::Id,
            pub result: $result_type,
        }
    };
}

pub fn cita_get_transaction_proof(upstream: &UpStream, tx_hash: H256) -> Result<Vec<u8>, Error> {
    let req = rpc_request::GetTransactionProofParams::new(tx_hash.into()).into_request(1);
    let result = rpc_send_and_get_result_from_reply!(upstream, req, rpc_types::Data);
    Ok(result.into())
}

pub fn cita_block_number(upstream: &UpStream) -> Result<U256, Error> {
    let req = rpc_request::BlockNumberParams::new().into_request(1);
    let result = rpc_send_and_get_result_from_reply!(upstream, req, U256);
    Ok(result)
}

pub fn cita_get_metadata(upstream: &UpStream) -> Result<rpc_types::MetaData, Error> {
    let height = rpc_types::BlockNumber::latest();
    let req = rpc_request::GetMetaDataParams::new(height).into_request(1);
    let result = rpc_send_and_get_result_from_reply!(upstream, req, rpc_types::MetaData);
    Ok(result)
}

pub fn cita_send_transaction(
    upstream: &UpStream,
    utx: &UnverifiedTransaction,
) -> Result<H256, Error> {
    let tx_bytes: Vec<u8> = utx.try_into().unwrap();
    let req = rpc_request::SendRawTransactionParams::new(tx_bytes.into()).into_request(1);
    let result = rpc_send_and_get_result_from_reply!(upstream, req, rpc_types::TxResponse);
    if result.status.to_uppercase() == "OK" {
        Ok(result.hash)
    } else {
        Err(Error::BadStatus)
    }
}
