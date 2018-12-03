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
use helper::{ReqSender, RpcMap};
use hyper::header::{
    AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlAllowOrigin,
    AccessControlMaxAge, ContentType, Headers, UserAgent,
};
use hyper::server::{Http, NewService, Request, Response, Service};
use hyper::{self, Method, StatusCode};
use jsonrpc_types::{request::RpcRequest as JsonrpcRequest, rpctypes::Id as RpcId};
use libproto::request::Request as ProtoRequest;
use net2;
use std::io;
use std::net::SocketAddr;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Handle, Timeout};
use unicase::Ascii;
use util::Mutex;

use crate::extractor::FutExtractor;
use crate::mq_publisher::{AccessLog as MQAccessLog, MQRequest, Publisher, TimeoutPublisher};
use crate::response::IntoResponse;
use crate::service_error::ServiceError;

const TCP_BACKLOG: i32 = 1024;
const CORS_CACHE: u32 = 86_400u32;

struct Inner {
    pub tx: ReqSender,
    pub responses: RpcMap,
    pub timeout: Duration,
    pub reactor_handle: Handle,
    pub http_headers: Headers,
}

pub struct Server {
    inner: Arc<Inner>,
}

pub struct NewServer {
    inner: Arc<Inner>,
}

impl NewService for NewServer {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Server;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Server {
            inner: Arc::clone(&self.inner),
        })
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
            .get::<UserAgent>()
            .map(ToString::to_string)
            .unwrap_or_else(|| String::from("unknown"));

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

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, http_req: Request) -> Self::Future {
        let sender = { self.inner.tx.lock().clone() };
        let responses = Arc::clone(&self.inner.responses);
        let timeout = self.inner.timeout;
        let reactor_handle = self.inner.reactor_handle.clone();
        let http_headers = self.inner.http_headers.clone();

        let mut access_log = AccessLog::new(http_req.method(), http_req.path(), &http_headers);

        match (http_req.method(), http_req.path()) {
            (&Method::Post, "/") => {
                let fut_resp = FutExtractor::<JsonrpcRequest>::extract_from(http_req)
                    .and_then(FutExtractor::<MQRequest>::extract_from)
                    .and_then({
                        let headers = http_headers.clone();

                        let timeout = Timeout::new(timeout, &reactor_handle.clone()).map_err(|e| {
                            error!("error create timeout: {}", e);
                            ServiceError::InternalServerError
                        });
                        let fut_timeout: FutureResult<
                            Timeout,
                            ServiceError,
                        > = timeout.into();

                        move |mq_req| {
                            // logging
                            access_log.set_rpc_info(RpcAccessLog::from(mq_req.access_log()));
                            info!("{}", access_log);

                            fut_timeout.and_then(|timeout| {
                                let timeout_responses = Arc::clone(&responses);
                                let pulibsher = Publisher::new(responses, sender, headers);
                                let pulibsher =
                                    TimeoutPublisher::new(pulibsher, timeout, timeout_responses);

                                pulibsher.publish(mq_req)
                            })
                        }
                    })
                    .then(move |resp| match resp {
                        Ok(resp) => Ok(resp),
                        Err(err) => Ok(err.into_response(http_headers.clone())),
                    });

                Box::new(fut_resp)
            }
            (&Method::Options, "/") => {
                info!("{}", access_log);
                handle_preflighted(http_headers)
            }
            _ => {
                info!("{}", access_log);
                Box::new(futures::future::ok(
                    Response::new()
                        .with_headers(http_headers)
                        .with_status(StatusCode::NotFound),
                ))
            }
        }
    }
}

fn handle_preflighted(mut headers: Headers) -> Box<Future<Item = Response, Error = hyper::Error>> {
    headers.set(ContentType::plaintext());
    headers.set(AccessControlAllowMethods(vec![
        Method::Post,
        Method::Options,
    ]));

    headers.set(AccessControlAllowHeaders(vec![
        Ascii::new("Origin".to_owned()),
        Ascii::new("Content-Type".to_owned()),
        Ascii::new("X-Requested-With".to_owned()),
        Ascii::new("User-Agent".to_owned()),
        Ascii::new("Accept".to_owned()),
    ]));
    headers.set(AccessControlMaxAge(CORS_CACHE));
    Box::new(futures::future::ok(Response::new().with_headers(headers)))
}

impl Server {
    pub fn start(
        core: Core,
        listener: TcpListener,
        tx: mpsc::Sender<(String, ProtoRequest)>,
        responses: RpcMap,
        timeout: Duration,
        allow_origin: &Option<String>,
    ) {
        let mut headers = Headers::new();
        let origin = parse_origin(allow_origin);
        headers.set(ContentType::json());
        headers.set(origin);

        let new_service = NewServer {
            inner: Arc::new(Inner {
                tx: Mutex::new(tx),
                responses,
                timeout,
                reactor_handle: core.handle(),
                http_headers: headers,
            }),
        };
        let server = Http::new()
            .sleep_on_errors(Some(Duration::from_millis(50)))
            .keep_alive(true)
            .bind_listener(core, listener, new_service)
            .unwrap();
        server.run().unwrap();
    }
}

fn parse_origin(origin: &Option<String>) -> AccessControlAllowOrigin {
    match origin.as_ref().map(|s| s.trim()) {
        Some("*") => AccessControlAllowOrigin::Any,
        None | Some("") | Some("null") => AccessControlAllowOrigin::Null,
        Some(origin) => AccessControlAllowOrigin::Value(origin.to_string()),
    }
}

pub fn listener(addr: &SocketAddr, handle: &Handle) -> io::Result<TcpListener> {
    let listener = match *addr {
        SocketAddr::V4(_) => net2::TcpBuilder::new_v4()?,
        SocketAddr::V6(_) => net2::TcpBuilder::new_v6()?,
    };
    configure_tcp(&listener)?;
    listener.reuse_address(true)?;
    listener.bind(addr)?;
    listener
        .listen(TCP_BACKLOG)
        .and_then(|l| TcpListener::from_listener(l, addr, handle))
}

fn configure_tcp(tcp: &net2::TcpBuilder) -> io::Result<()> {
    use net2::unix::*;
    tcp.reuse_port(true)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{parse_origin, AccessControlAllowOrigin};

    #[test]
    fn test_parse_origin() {
        vec![
            (
                Some("abc".to_owned()),
                AccessControlAllowOrigin::Value("abc".to_owned()),
            ),
            (
                Some(" xyz ".to_owned()),
                AccessControlAllowOrigin::Value("xyz".to_owned()),
            ),
            (Some("*".to_owned()), AccessControlAllowOrigin::Any),
            (Some(" * ".to_owned()), AccessControlAllowOrigin::Any),
            (None, AccessControlAllowOrigin::Null),
        ]
        .into_iter()
        .for_each(|(origin, result)| {
            assert_eq!(parse_origin(&origin), result);
        });
    }
}

#[cfg(test)]
mod integration_test {
    use std::collections::HashMap;
    use std::str::FromStr;
    use std::sync::mpsc::channel;
    use std::thread;

    use uuid::Uuid;

    use super::*;
    use futures::{sync::oneshot, Stream};
    use jsonrpc_types;
    use jsonrpc_types::response::Output;
    use libproto::protos;
    use serde_json;

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
        tx: mpsc::Sender<(String, ProtoRequest)>,
        timeout: u64,
        allow_origin: Option<&str>,
    ) -> Serve {
        let addr = "127.0.0.1:0".parse().unwrap();
        let tx = tx.clone();

        let timeout = Duration::from_secs(timeout);
        let allow_origin = allow_origin.map(|s| s.to_owned());
        let (addr_tx, addr_rx) = ::std::sync::mpsc::channel();
        let thread_handle = thread::Builder::new()
            .name(format!("test-server-{}", Uuid::new_v4()))
            .spawn(move || {
                let core = Core::new().unwrap();
                let handle = core.handle();
                let listener = listener(&addr, &handle).unwrap();
                let addr = listener.local_addr().unwrap().clone();
                let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
                addr_tx.send((addr, shutdown_tx)).unwrap();

                // Server::start(core, listener, tx, responses, timeout, &allow_origin);
                let mut headers = Headers::new();
                let origin = parse_origin(&allow_origin);
                headers.set(ContentType::json());
                headers.set(origin);
                let new_service = NewServer {
                    inner: Arc::new(Inner {
                        tx: Mutex::new(tx),
                        responses,
                        timeout,
                        reactor_handle: core.handle(),
                        http_headers: headers,
                    }),
                };
                let server = Http::new()
                    .keep_alive(true)
                    .sleep_on_errors(Some(Duration::from_millis(10)))
                    .bind_listener(core, listener, new_service)
                    .unwrap();
                server.run_until(shutdown_rx.then(|_| Ok(()))).unwrap();
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
        use std::io::Write;

        // For message forwarding
        let (tx_relay, rx_relay) = channel();
        let backlog_capacity = 256;
        let responses = Arc::new(Mutex::new(HashMap::with_capacity(backlog_capacity)));
        let serve = start_server(responses.clone(), tx_relay, 3, Some("*"));

        let http_responses = responses.clone();
        let (tx_quit, rx_quit) = channel();
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
                            let _ = sender.send(Output::from(content, req_info));
                        }
                        TransferType::WEBSOCKET((req_info, sender)) => {
                            let _ = sender.send(
                                serde_json::to_string(&Output::from(content, req_info)).unwrap(),
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

        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = hyper::Client::configure().keep_alive(true).build(&handle);

        let mut works: Vec<Box<Future<Item = (), Error = _>>> = vec![];
        let uri = hyper::Uri::from_str(
            format!("http://{}:{}/", serve.addr.ip(), serve.addr.port()).as_str(),
        )
        .unwrap();

        let req = hyper::Request::<hyper::Body>::new(Method::Post, uri.clone());
        let work_empty = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 400);
            Ok(())
        });

        let req = hyper::Request::<hyper::Body>::new(Method::Options, uri.clone());
        let work_options = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            let headers = resp.headers().clone();
            assert_eq!(
                headers.get::<ContentType>(),
                Some(&ContentType::plaintext())
            );
            assert_eq!(
                headers.get::<AccessControlAllowMethods>(),
                Some(&AccessControlAllowMethods(vec![
                    Method::Post,
                    Method::Options,
                ]))
            );
            assert_eq!(
                headers.get::<AccessControlAllowHeaders>(),
                Some(&AccessControlAllowHeaders(vec![
                    Ascii::new("Origin".to_owned()),
                    Ascii::new("Content-Type".to_owned()),
                    Ascii::new("X-Requested-With".to_owned()),
                    Ascii::new("User-Agent".to_owned()),
                    Ascii::new("Accept".to_owned()),
                ]))
            );
            assert_eq!(
                headers.get::<AccessControlMaxAge>(),
                Some(&AccessControlMaxAge(CORS_CACHE))
            );
            Ok(())
        });

        let request_str = r#"{
            "jsonrpc": "2.0",
            "id": null,
            "params": ["0x000000000000000000000000000000000000000000000000000000000000000a"]
        }"#;
        let rpcreq =
            serde_json::from_str::<jsonrpc_types::request::PartialRequest>(request_str).unwrap();
        let data = serde_json::to_string(&JsonrpcRequest::Single(rpcreq)).unwrap();
        let mut req = hyper::Request::<hyper::Body>::new(Method::Post, uri.clone());
        req.set_body(data);
        let work_method_not_found = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            resp.body()
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
        let mut req = hyper::Request::<hyper::Body>::new(Method::Post, uri.clone());
        req.set_body(data);
        let work_peercount = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            resp.body()
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
        let mut req = hyper::Request::<hyper::Body>::new(Method::Post, uri.clone());
        req.set_body(data);
        let work_peercount_batch = client.request(req).and_then(|resp| {
            assert_eq!(resp.status().as_u16(), 200);
            resp.body()
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
        core.run(futures::future::join_all(works)).unwrap();

        tx_quit.send(()).unwrap();
        receiver.join().unwrap();
    }
}
