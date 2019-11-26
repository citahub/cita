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

use futures::future::Either;
use futures::sync::{mpsc, oneshot};
use futures::{Future, Sink, Stream};
use hyper;
use libproto::TryInto;
use parking_lot::{Mutex, RwLock};
use serde_json;
use std::convert::Into;
use tokio_core::reactor::{Core, Timeout};

use crate::configuration::UpStream;
use cita_types::{H256, U256};
use jsonrpc_types::{rpc_request, rpc_types};
use libproto::blockchain::UnverifiedTransaction;

#[derive(Debug)]
pub enum Error {
    BadStatus,
    Timeout,
    Parse,
}

type RpcSender =
    Mutex<mpsc::Sender<(hyper::Request, oneshot::Sender<Result<hyper::Chunk, Error>>)>>;

pub struct RpcClient {
    sender: RpcSender,
    uri: RwLock<hyper::Uri>,
}

impl RpcClient {
    pub fn create(upstream: &UpStream) -> ::std::sync::Arc<Self> {
        let tb = ::std::thread::Builder::new().name("RpcClient".to_string());
        let uri = upstream.url.parse::<hyper::Uri>().unwrap();
        let (tx, rx) =
            mpsc::channel::<(hyper::Request, oneshot::Sender<Result<hyper::Chunk, Error>>)>(65_535);
        let timeout_duration = upstream.timeout;

        let _tb = tb
            .spawn(move || {
                let mut core = Core::new().unwrap();
                let handle = core.handle();
                let client = hyper::Client::configure()
                    .connector(hyper::client::HttpConnector::new(4, &handle))
                    .keep_alive(false)
                    .build(&handle);

                let messages = rx.for_each(|(req, sender)| {
                    let timeout = Timeout::new(timeout_duration, &handle).unwrap();
                    let post = client.request(req).and_then(|res| res.body().concat2());

                    let work = post.select2(timeout).then(move |res| match res {
                        Ok(Either::A((got, _timeout))) => {
                            let _ = sender.send(Ok(got));
                            Ok(())
                        }
                        Ok(Either::B(_)) | Err(_) => {
                            let _ = sender.send(Err(Error::Timeout));
                            Ok(())
                        }
                    });

                    handle.spawn(work);
                    Ok(())
                });

                core.run(messages).unwrap();
            })
            .expect("Couldn't spawn a thread.");

        ::std::sync::Arc::new(RpcClient {
            sender: Mutex::new(tx),
            uri: RwLock::new(uri),
        })
    }

    pub fn do_post(&self, body: &str) -> Result<hyper::Chunk, Error> {
        let uri = { self.uri.read().clone() };
        trace!("Send body {:?} to {:?}.", body, uri);
        let mut req = hyper::Request::new(hyper::Method::Post, uri);
        req.headers_mut().set(hyper::header::ContentType::json());
        req.set_body(body.to_owned());
        let (tx, rx) = oneshot::channel();
        {
            let _ = self.sender.lock().start_send((req, tx));
        }
        match rx.wait() {
            Ok(res) => {
                let res = res.map_err(|_| Error::BadStatus)?;
                trace!("Get response {:?}.", res);
                Ok(res)
            }
            Err(_) => Err(Error::BadStatus),
        }
    }
}

// Pack the result type into a reply type, and parse result from the reply, and return the result.
// The user of this macro do NOT have to care about the inner reply type.
macro_rules! rpc_send_and_get_result_from_reply {
    ($upstream:ident, $request:ident, $result_type:path) => {{
        define_reply_type!(ReplyType, $result_type);
        let rpc_cli = RpcClient::create($upstream);
        let body: String = $request.into();
        let data = rpc_cli.do_post(&body)?;
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
