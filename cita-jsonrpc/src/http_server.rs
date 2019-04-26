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

use futures::future::{self as future, Future};
use hyper::header::{
    HeaderMap as Headers, HeaderName, HeaderValue, ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_MAX_AGE,
    CONTENT_TYPE, ORIGIN, USER_AGENT,
};
use hyper::service::{MakeService, Service};
use hyper::{Body, Method, Request, Response, StatusCode};
use jsonrpc_types::{rpc_request::RpcRequest as JsonrpcRequest, rpc_types::Id as RpcId};
use libproto::request::Request as ProtoRequest;
use pubsub::channel::Sender;
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use std::time::Duration;
use util::Mutex;

use crate::extractor::FutExtractor;
use crate::helper::{ReqSender, RpcMap};
use crate::http_header::{Origin, CONTENT_TYPE_JSON_STR, CONTENT_TYPE_PLAIN_TEXT_STR};
use crate::mq_publisher::{AccessLog as MQAccessLog, MQRequest, Publisher, TimeoutPublisher};
use crate::response::{HyperResponseExt, IntoResponse};

const TCP_BACKLOG: i32 = 1024;
const CORS_CACHE: u32 = 86_400u32;

struct Inner {
    pub tx: ReqSender,
    pub responses: RpcMap,
    pub timeout: Duration,
    pub http_headers: Headers,
}

pub struct Jsonrpc {
    inner: Arc<Inner>,
}

pub struct JsonrpcMakeService {
    inner: Arc<Inner>,
}

impl<Ctx> MakeService<Ctx> for JsonrpcMakeService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = Jsonrpc;
    type Future = Box<dyn Future<Item = Self::Service, Error = Self::Error> + Send>;
    type MakeError = hyper::Error;

    fn make_service(&mut self, _: Ctx) -> Self::Future {
        Box::new(future::ok(Jsonrpc {
            inner: Arc::clone(&self.inner),
        }))
    }
}

struct AccessLog {
    user_agent: String,
    http_method: Method,
    http_path: String,
    rpc_info: Option<RpcAccessLog>,
}

enum RpcAccessLog {
    Single(SingleRpcAccessLog),
    Batch(BatchRpcAccessLog),
}

struct SingleRpcAccessLog {
    id: RpcId,
    method: Option<String>,
}

struct BatchRpcAccessLog {
    count: Option<usize>,
}

impl From<MQAccessLog> for RpcAccessLog {
    fn from(mq_log: MQAccessLog) -> Self {
        match mq_log {
            MQAccessLog::Single { id, method } => {
                RpcAccessLog::Single(SingleRpcAccessLog { id, method })
            }
            MQAccessLog::Batch { count } => RpcAccessLog::Batch(BatchRpcAccessLog { count }),
        }
    }
}

impl AccessLog {
    pub fn new(http_method: &Method, http_path: &str, http_headers: &Headers) -> Self {
        let user_agent = http_headers
            .get(USER_AGENT)
            .and_then(|u| u.to_str().ok())
            .unwrap_or_else(|| "unknown")
            .to_owned();

        Self {
            user_agent,
            http_method: http_method.clone(),
            http_path: http_path.to_owned(),
            rpc_info: None,
        }
    }

    pub fn set_rpc_info(&mut self, rpc_acc_log: RpcAccessLog) {
        self.rpc_info = Some(rpc_acc_log);
    }
}

impl ::std::fmt::Display for AccessLog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "user-agent={}", self.user_agent)?;
        write!(f, ", http-method={}", self.http_method)?;
        write!(f, ", http-path={}", self.http_path)?;
        match self.rpc_info {
            Some(RpcAccessLog::Single(ref sl)) => {
                write!(f, ", rpc-type=single")?;
                write!(f, ", rpc-id={:?}", sl.id)?;
                if let Some(ref m) = sl.method {
                    write!(f, ", rpc-method={}", m)
                } else {
                    write!(f, ", rpc-method=unknown")
                }
            }
            Some(RpcAccessLog::Batch(ref bl)) => {
                write!(f, ", rpc-type=batch")?;
                if let Some(c) = bl.count {
                    write!(f, ", rpc-count={}", c)
                } else {
                    write!(f, ", rpc-count=-1")
                }
            }
            None => write!(f, ", rpc-type=unknown"),
        }
    }
}

impl Service for Jsonrpc {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, http_req: Request<Self::ReqBody>) -> Self::Future {
        let sender = { self.inner.tx.lock().clone() };
        let responses = Arc::clone(&self.inner.responses);
        let timeout = self.inner.timeout;
        let http_headers = self.inner.http_headers.clone();

        let http_path = http_req.uri().path().to_owned();
        let mut access_log = AccessLog::new(http_req.method(), &http_path, &http_headers);

        match (http_req.method(), http_path.as_ref()) {
            (&Method::POST, "/") => {
                let fut_resp = FutExtractor::<JsonrpcRequest>::extract_from(http_req)
                    .and_then(FutExtractor::<MQRequest>::extract_from)
                    .and_then({
                        let headers = http_headers.clone();

                        move |mq_req| {
                            // logging
                            access_log.set_rpc_info(RpcAccessLog::from(mq_req.access_log()));
                            info!("{}", access_log);

                            let timeout_responses = Arc::clone(&responses);
                            let pulibsher = Publisher::new(responses, sender, headers);
                            let pulibsher =
                                TimeoutPublisher::new(pulibsher, timeout, timeout_responses);

                            pulibsher.publish(mq_req)
                        }
                    })
                    .then(move |resp| match resp {
                        Ok(resp) => Ok(resp),
                        Err(err) => Ok(err.into_response(http_headers.clone())),
                    });

                Box::new(fut_resp)
            }
            (&Method::OPTIONS, "/") => {
                info!("{}", access_log);
                let resp = Response::default().with_headers(handle_preflighted(http_headers));

                Box::new(future::ok(resp))
            }
            _ => {
                info!("{}", access_log);
                let resp = Response::default()
                    .with_headers(http_headers)
                    .with_status(StatusCode::NOT_FOUND);

                Box::new(future::ok(resp))
            }
        }
    }
}

fn handle_preflighted(mut headers: Headers) -> Headers {
    use crate::http_header::{HeaderMapExt, X_REQUESTED_WITH_STR};

    let x_requested_with = HeaderName::from_static(X_REQUESTED_WITH_STR);
    let plain_text = HeaderValue::from_static(CONTENT_TYPE_PLAIN_TEXT_STR);
    let cors_cache = HeaderValue::from(CORS_CACHE);
    let allow_methods = vec![Method::POST, Method::OPTIONS];
    let allow_headers = vec![ORIGIN, CONTENT_TYPE, x_requested_with, USER_AGENT, ACCEPT];

    headers.insert(CONTENT_TYPE, plain_text);
    headers.insert_vec(ACCESS_CONTROL_ALLOW_METHODS, allow_methods);
    headers.insert_vec(ACCESS_CONTROL_ALLOW_HEADERS, allow_headers);
    headers.insert(ACCESS_CONTROL_MAX_AGE, cors_cache);

    headers
}

pub type JsonrpcServer = hyper::Server<hyper::server::conn::AddrIncoming, JsonrpcMakeService>;
pub struct Server {
    addr: SocketAddr,
    jsonrpc: JsonrpcServer,
}

impl Server {
    pub fn create(
        addr: &SocketAddr,
        tx: Sender<(String, ProtoRequest)>,
        responses: RpcMap,
        timeout: u64,
        allow_origin: &Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = listener_from_socket_addr(&addr)?;
        let addr = listener.local_addr()?;
        let timeout = Duration::from_secs(timeout);
        let json = HeaderValue::from_static(CONTENT_TYPE_JSON_STR);
        let allow_origin = Origin::from_config(allow_origin)?;

        let mut http_headers = Headers::new();
        http_headers.insert(CONTENT_TYPE, json);
        http_headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, allow_origin);

        let make_jsonrpc_svc = JsonrpcMakeService {
            inner: Arc::new(Inner {
                tx: Mutex::new(tx),
                responses,
                timeout,
                http_headers,
            }),
        };

        // NOTE: sleep_on_errors is turned on by default
        hyper::Server::from_tcp(listener)
            .map(|builder| builder.http1_keepalive(true).serve(make_jsonrpc_svc))
            .map(|jsonrpc| Self { addr, jsonrpc })
            .map_err(Box::from)
    }

    // used in test code
    #[allow(dead_code)]
    pub fn local_addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn jsonrpc(self) -> JsonrpcServer {
        self.jsonrpc
    }
}

pub fn listener_from_socket_addr(addr: &SocketAddr) -> std::io::Result<TcpListener> {
    use net2::unix::UnixTcpBuilderExt;

    let listener = match *addr {
        SocketAddr::V4(_) => net2::TcpBuilder::new_v4()?,
        SocketAddr::V6(_) => net2::TcpBuilder::new_v6()?,
    };
    listener.reuse_port(true)?;
    listener.reuse_address(true)?;
    listener.bind(addr)?;
    listener.listen(TCP_BACKLOG)
}

#[cfg(test)]
mod integration_test {
    use pubsub::channel::{self, Sender};
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::thread;

    use uuid::Uuid;

    use super::*;
    use futures::{sync::oneshot, Stream};
    use jsonrpc_proto::response::OutputExt;
    use jsonrpc_types;
    use jsonrpc_types::rpc_response::Output;
    use libproto::protos;
    use serde_json;
    use tokio_core::reactor::Core;

    use helper::TransferType;

    struct Serve {
        pub addr: SocketAddr,
        pub shutdown_signal: Option<oneshot::Sender<()>>,
        pub thread: Option<thread::JoinHandle<()>>,
    }

    impl Drop for Serve {
        fn drop(&mut self) {
            drop(self.shutdown_signal.take());
            self.thread.take().unwrap().join().unwrap();
        }
    }

    fn start_server(
        responses: RpcMap,
        tx: Sender<(String, ProtoRequest)>,
        timeout: u64,
        allow_origin: Option<String>,
    ) -> Serve {
        let addr = "127.0.0.1:0".parse().unwrap();
        let tx = tx.clone();

        let (addr_tx, addr_rx) = channel::unbounded();
        let thread_handle = thread::Builder::new()
            .name(format!("test-server-{}", Uuid::new_v4()))
            .spawn(move || {
                let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
                let server = Server::create(&addr, tx, responses, timeout, &allow_origin).unwrap();

                let addr = server.local_addr();
                addr_tx.send((addr, shutdown_tx)).unwrap();

                let jsonrpc_server = server
                    .jsonrpc()
                    .with_graceful_shutdown(shutdown_rx)
                    .map_err(|err| eprintln!("server err {}", err));

                let mut rt = tokio::runtime::Builder::new()
                    .core_threads(2)
                    .build()
                    .unwrap();
                rt.spawn(jsonrpc_server);

                tokio_executor::enter()
                    .unwrap()
                    .block_on(rt.shutdown_on_idle())
                    .unwrap();
            })
            .unwrap();
        let (addr, shutdown_tx) = addr_rx.recv().unwrap();
        Serve {
            addr,
            shutdown_signal: Some(shutdown_tx),
            thread: Some(thread_handle),
        }
    }

    #[test]
    fn test_server() {
        use crate::http_header::{HeaderValueExt, X_REQUESTED_WITH_STR};
        use std::io::Write;

        // For message forwarding
        let (tx_relay, rx_relay) = channel::unbounded();
        let backlog_capacity = 256;
        let responses = Arc::new(Mutex::new(HashMap::with_capacity(backlog_capacity)));
        let serve = start_server(responses.clone(), tx_relay, 3, Some(String::from("*")));

        let http_responses = responses.clone();
        let (tx_quit, rx_quit) = channel::unbounded();
        let receiver = thread::spawn(move || loop {
            if let Ok((_topic, req)) = rx_relay.try_recv() {
                let value = { http_responses.lock().remove(&req.request_id) };

                let mut content = protos::response::Response::new();
                content.set_request_id(req.request_id);
                content.set_code(0);
                content.set_tx_state(format!(
                    "{}",
                    json!({
                        "status": "test tx state",
                        "hash": "0xb38e5b6572b2613cab8088f93e6835576209f2b796104779b4a43fa5adc737af"
                    })
                ));

                if let Some(val) = value {
                    match val {
                        TransferType::HTTP((req_info, sender)) => {
                            let _ = sender.send(Output::from_res_info(content, req_info));
                        }
                        TransferType::WEBSOCKET((req_info, sender)) => {
                            let _ = sender.send(
                                serde_json::to_string(&Output::from_res_info(content, req_info))
                                    .unwrap(),
                            );
                        }
                    }
                } else {
                    warn!("receive lost request_id {:?}", content.request_id);
                }
            } else {
                if rx_quit.try_recv().is_ok() {
                    break;
                }
                thread::sleep(Duration::from_millis(10));
            }
        });

        let client = hyper::Client::builder().keep_alive(true).build_http();

        let mut works: Vec<Box<Future<Item = (), Error = _>>> = vec![];
        let uri = hyper::Uri::from_str(
            format!("http://{}:{}/", serve.addr.ip(), serve.addr.port()).as_str(),
        )
        .unwrap();

        let req = hyper::Request::post(uri.clone())
            .body(hyper::Body::empty())
            .unwrap();
        let work_empty = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 400);
            Ok(())
        });

        let req = hyper::Request::options(uri.clone())
            .body(hyper::Body::empty())
            .unwrap();
        let work_options = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            let headers = resp.headers().clone();
            assert_eq!(
                headers.get(CONTENT_TYPE),
                Some(&HeaderValue::from_static(CONTENT_TYPE_PLAIN_TEXT_STR))
            );
            assert_eq!(
                headers.get(ACCESS_CONTROL_ALLOW_METHODS),
                Some(&HeaderValue::from_vec(vec![Method::POST, Method::OPTIONS]))
            );
            let x_requested_with = HeaderName::from_static(X_REQUESTED_WITH_STR);
            let expect_headers = vec![ORIGIN, CONTENT_TYPE, x_requested_with, USER_AGENT, ACCEPT];
            assert_eq!(
                headers.get(ACCESS_CONTROL_ALLOW_HEADERS),
                Some(&HeaderValue::from_vec(expect_headers))
            );
            assert_eq!(
                headers.get(ACCESS_CONTROL_MAX_AGE),
                Some(&HeaderValue::from(CORS_CACHE))
            );
            Ok(())
        });

        let request_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let rpcreq =
            serde_json::from_str::<jsonrpc_types::rpc_request::PartialRequest>(request_str)
                .unwrap();
        let data = serde_json::to_string(&JsonrpcRequest::Single(rpcreq)).unwrap();
        let req = hyper::Request::post(uri.clone())
            .body(hyper::Body::from(data))
            .unwrap();
        let work_method_not_found = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            resp.into_body()
                .fold(vec![], |mut buf, chunk| {
                    buf.write(chunk.as_ref()).unwrap();
                    futures::future::ok(buf).map_err(|e: hyper::Error| e)
                })
                .and_then(|buf| {
                    let rv: serde_json::Value = serde_json::from_slice(&buf).unwrap();
                    let error_code = rv
                        .as_object()
                        .unwrap()
                        .get("error")
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .get("code")
                        .unwrap()
                        .as_i64()
                        .unwrap();
                    // message: "Method not found"
                    assert_eq!(error_code, -32601);
                    Ok(())
                })
        });

        let data = format!(
            "{}",
            json!({"jsonrpc":"2.0","method":"peerCount","params":[],"id":74})
        );
        let req = hyper::Request::post(uri.clone())
            .body(hyper::Body::from(data))
            .unwrap();
        let work_peercount = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            resp.into_body()
                .fold(vec![], |mut buf, chunk| {
                    buf.write(chunk.as_ref()).unwrap();
                    futures::future::ok(buf).map_err(|e: hyper::Error| e)
                })
                .and_then(|buf| {
                    let rv: serde_json::Value = serde_json::from_slice(&buf).unwrap();
                    let request_id = rv.as_object().unwrap().get("id").unwrap().as_i64().unwrap();
                    assert_eq!(request_id, 74);
                    Ok(())
                })
        });

        let data = format!(
            "{}",
            json!([
                {"jsonrpc":"2.0","method":"peerCount","params":[],"id":74},
                {"jsonrpc":"2.0","method":"peerCount","params":[],"id":75}
            ])
        );
        let req = hyper::Request::post(uri.clone())
            .body(hyper::Body::from(data))
            .unwrap();
        let work_peercount_batch = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            resp.into_body()
                .fold(vec![], |mut buf, chunk| {
                    buf.write(chunk.as_ref()).unwrap();
                    futures::future::ok(buf).map_err(|e: hyper::Error| e)
                })
                .and_then(|buf| {
                    let rv: serde_json::Value = serde_json::from_slice(&buf).unwrap();
                    vec![74, 75].into_iter().enumerate().for_each(|(i, value)| {
                        let request_id = rv
                            .as_array()
                            .unwrap()
                            .get(i)
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .get("id")
                            .unwrap()
                            .as_i64()
                            .unwrap();
                        assert_eq!(request_id, value);
                    });
                    Ok(())
                })
        });

        works.push(Box::new(work_empty));
        works.push(Box::new(work_options));
        works.push(Box::new(work_method_not_found));
        works.push(Box::new(work_peercount));
        works.push(Box::new(work_peercount_batch));

        let mut core = Core::new().unwrap();
        core.run(futures::future::join_all(works)).unwrap();

        tx_quit.send(()).unwrap();
        // explicitly drop Hyper::Client before shutdown test server,
        // otherwise graceful shutdown maybe blocked then cause deadlock
        // when we join that server's thread.
        //
        // Reference: https://github.com/hyperium/hyper/issues/1668
        drop(client);
        receiver.join().unwrap();
    }
}
