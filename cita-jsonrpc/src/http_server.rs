use error::ErrorCode;
use futures::future::{Either, Future};
use futures::stream::FuturesOrdered;
use futures::sync::oneshot;
use futures::{self, Stream};
use helper::{select_topic, ReqInfo, ReqSender, RpcMap, TransferType};
use hyper::header::{
    AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlAllowOrigin,
    AccessControlMaxAge, ContentType, Headers,
};
use hyper::server::{Http, NewService, Request, Response, Service};
use hyper::{self, Method, StatusCode};
use jsonrpc_types::method::{self, MethodHandler};
use jsonrpc_types::response::RpcFailure;
use jsonrpc_types::{Call, Error, RpcRequest};
use libproto::request as reqlib;
use net2;
use response::{BatchFutureResponse, SingleFutureResponse};
use serde_json;
use std::io;
use std::net::SocketAddr;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Handle, Timeout};
use unicase::Ascii;
use util::Mutex;

const TCP_BACKLOG: i32 = 1024;
const CORS_CACHE: u32 = 86_400u32;

struct Inner {
    pub tx: ReqSender,
    pub responses: RpcMap,
    pub timeout: Duration,
    pub reactor_handle: Handle,
    pub method_handler: method::MethodHandler,
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

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let sender = { self.inner.tx.lock().clone() };
        let responses = Arc::clone(&self.inner.responses);
        let timeout_responses = Arc::clone(&self.inner.responses);
        let method_handler = self.inner.method_handler;
        let timeout = self.inner.timeout;
        let reactor_handle = self.inner.reactor_handle.clone();
        let http_headers = self.inner.http_headers.clone();

        match (req.method(), req.path()) {
            (&Method::Post, "/") => {
                let mapping = req.body().concat2().and_then(move |chunk| {
                    if let Ok(rpc) = serde_json::from_slice::<RpcRequest>(&chunk) {
                        match rpc {
                            RpcRequest::Single(call) => {
                                match read_single(&call, method_handler, &http_headers) {
                                    Ok(req) => {
                                        if let Ok(timeout) = Timeout::new(timeout, &reactor_handle)
                                        {
                                            let id = call.id.clone();
                                            let jsonrpc_version = call.jsonrpc.clone();
                                            let request_id = req.request_id.clone();
                                            let mq_resp = handle_single(
                                                call,
                                                req,
                                                &responses,
                                                &sender,
                                                &http_headers,
                                            );

                                            let resp = mq_resp.select2(timeout).then(move |res| {
                                                match res {
                                                    Ok(Either::A((got, _timeout))) => Ok(got),
                                                    Ok(Either::B((_timeout_error, _get))) => {
                                                        {
                                                            timeout_responses
                                                                .lock()
                                                                .remove(&request_id);
                                                        }
                                                        let failure = RpcFailure::from_options(
                                                            id,
                                                            jsonrpc_version,
                                                            Error::server_error(
                                                                ErrorCode::time_out_error(),
                                                                "system time out, please resend",
                                                            ),
                                                        );
                                                        let resp_body =
                                                            serde_json::to_string(&failure).expect(
                                                                "should be serialize by serde_json",
                                                            );
                                                        Ok(Response::new()
                                                            .with_headers(http_headers)
                                                            .with_body(resp_body))
                                                    }
                                                    Err(Either::A((get_error, _timeout))) => {
                                                        Err(get_error)
                                                    }
                                                    Err(Either::B((timeout_error, _get))) => {
                                                        Err(From::from(timeout_error))
                                                    }
                                                }
                                            });

                                            Either::A(Either::A(resp))
                                        } else {
                                            Either::B(futures::future::ok(
                                                Response::new()
                                                    .with_headers(http_headers)
                                                    .with_status(StatusCode::InternalServerError),
                                            ))
                                        }
                                    }
                                    Err(resp) => Either::B(futures::future::ok(resp)),
                                }
                            }
                            RpcRequest::Batch(calls) => {
                                match read_batch(calls, method_handler, &http_headers) {
                                    Ok(reqs) => {
                                        let request_ids: Vec<Vec<u8>> = reqs.iter()
                                        .map(|&(ref _call, ref req)| req.request_id.clone())
                                        .collect();

                                        let mq_resp =
                                            handle_batch(reqs, &responses, &sender, &http_headers);

                                        if let Ok(timeout) = Timeout::new(timeout, &reactor_handle)
                                        {
                                            let resp = mq_resp.select2(timeout).then(move |res| {
                                                match res {
                                                    Ok(Either::A((got, _timeout))) => Ok(got),
                                                    Ok(Either::B((_timeout_error, _get))) => {
                                                        {
                                                            let mut guard =
                                                                timeout_responses.lock();
                                                            for request_id in request_ids {
                                                                guard.remove(&request_id);
                                                            }
                                                        }
                                                        let failure =
                                                            RpcFailure::from(Error::server_error(
                                                                ErrorCode::time_out_error(),
                                                                "system time out, please resend",
                                                            ));
                                                        let resp_body =
                                                            serde_json::to_string(&failure).expect(
                                                                "should be serialize by serde_json",
                                                            );
                                                        Ok(Response::new()
                                                            .with_headers(http_headers)
                                                            .with_body(resp_body))
                                                    }
                                                    Err(Either::A((get_error, _timeout))) => {
                                                        Err(get_error)
                                                    }
                                                    Err(Either::B((timeout_error, _get))) => {
                                                        Err(From::from(timeout_error))
                                                    }
                                                }
                                            });

                                            Either::A(Either::B(resp))
                                        } else {
                                            Either::B(futures::future::ok(
                                                Response::new()
                                                    .with_headers(http_headers)
                                                    .with_status(StatusCode::InternalServerError),
                                            ))
                                        }
                                    }
                                    Err(resp) => Either::B(futures::future::ok(resp)),
                                }
                            }
                        }
                    } else {
                        Either::B(futures::future::ok(
                            Response::new()
                                .with_headers(http_headers)
                                .with_status(StatusCode::BadRequest),
                        ))
                    }
                });
                let resp: Box<Future<Error = hyper::Error, Item = hyper::Response>> =
                    Box::new(mapping);
                resp
            }
            (&Method::Options, "/") => handle_preflighted(http_headers),
            _ => Box::new(futures::future::ok(
                Response::new()
                    .with_headers(http_headers)
                    .with_status(StatusCode::NotFound),
            )),
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
        Ascii::new("Accept".to_owned()),
    ]));
    headers.set(AccessControlMaxAge(CORS_CACHE));
    Box::new(futures::future::ok(Response::new().with_headers(headers)))
}

fn read_single(
    call: &Call,
    method_handler: MethodHandler,
    headers: &Headers,
) -> Result<reqlib::Request, Response> {
    match method_handler.request(call) {
        Ok(req) => Ok(req),
        Err(e) => {
            let resp_body = serde_json::to_vec(&RpcFailure::from_options(
                call.id.clone(),
                call.jsonrpc.clone(),
                e,
            )).expect("should be serialize by serde_json");
            Err(Response::new()
                .with_headers(headers.clone())
                .with_body(resp_body))
        }
    }
}

fn handle_single(
    call: Call,
    req: reqlib::Request,
    responses: &RpcMap,
    sender: &mpsc::Sender<(String, reqlib::Request)>,
    headers: &Headers,
) -> SingleFutureResponse {
    let request_id = req.request_id.clone();
    let (tx, rx) = oneshot::channel();
    let topic = select_topic(&call.method);
    let req_info = (ReqInfo::new(call.jsonrpc, call.id), tx);
    {
        responses
            .lock()
            .insert(request_id, TransferType::HTTP(req_info));
    }
    let _ = sender.send((topic, req));
    let headers = headers.clone();

    SingleFutureResponse::new(rx, headers)
}

fn read_batch(
    calls: Vec<Call>,
    method_handler: MethodHandler,
    headers: &Headers,
) -> Result<Vec<(Call, reqlib::Request)>, Response> {
    let mut reqs = Vec::with_capacity(calls.len());
    for call in calls {
        match method_handler.request(&call) {
            Ok(req) => {
                reqs.push((call, req));
            }
            Err(_) => {
                return Err(Response::new()
                    .with_headers(headers.clone())
                    .with_status(StatusCode::BadRequest))
            }
        }
    }
    Ok(reqs)
}

fn handle_batch(
    reqs: Vec<(Call, reqlib::Request)>,
    responses: &RpcMap,
    sender: &mpsc::Sender<(String, reqlib::Request)>,
    headers: &Headers,
) -> BatchFutureResponse {
    use std::iter::FromIterator;
    let mut rxs = Vec::with_capacity(reqs.len());
    for (call, req) in reqs {
        let request_id = req.request_id.clone();
        let topic = select_topic(&call.method);
        let (tx, rx) = oneshot::channel();
        let req_info = (ReqInfo::new(call.jsonrpc, call.id), tx);
        {
            responses
                .lock()
                .insert(request_id, TransferType::HTTP(req_info));
        }
        let _ = sender.send((topic, req));
        rxs.push(rx);
    }
    let headers = headers.clone();

    BatchFutureResponse::new(FuturesOrdered::from_iter(rxs).collect(), headers)
}

impl Server {
    pub fn start(
        core: Core,
        listener: TcpListener,
        tx: mpsc::Sender<(String, reqlib::Request)>,
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
                responses: responses,
                timeout: timeout,
                reactor_handle: core.handle(),
                method_handler: method::MethodHandler,
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
    match origin.as_ref().map(|s| s.trim().as_ref()) {
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
    use super::*;

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
        ].into_iter()
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
    use jsonrpc_types;
    use jsonrpc_types::response::Output;
    use libproto::protos;

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
        tx: mpsc::Sender<(String, reqlib::Request)>,
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
                        responses: responses,
                        timeout: timeout,
                        reactor_handle: core.handle(),
                        method_handler: method::MethodHandler,
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
                            let _ =
                                sender.send(Output::from(content, req_info.id, req_info.jsonrpc));
                        }
                        TransferType::WEBSOCKET((req_info, sender)) => {
                            let _ = sender.send(
                                serde_json::to_string(&Output::from(
                                    content,
                                    req_info.id,
                                    req_info.jsonrpc,
                                )).unwrap(),
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
        ).unwrap();

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
                    Ascii::new("Accept".to_owned()),
                ]))
            );
            assert_eq!(
                headers.get::<AccessControlMaxAge>(),
                Some(&AccessControlMaxAge(CORS_CACHE))
            );
            Ok(())
        });

        let params_str = r#"[{"from":"foo", "to":"bar"}, 99]"#;
        let params = serde_json::from_str::<jsonrpc_types::Params>(params_str).unwrap();
        let rpc_call = Call {
            jsonrpc: Some(jsonrpc_types::Version::V2),
            method: "test_method".to_owned(),
            id: jsonrpc_types::Id::Null,
            params: Some(params),
        };
        let data = serde_json::to_string(&RpcRequest::Single(rpc_call)).unwrap();
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
            json!({"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":74})
        );
        let mut req = hyper::Request::<hyper::Body>::new(Method::Post, uri.clone());
        req.set_body(data);
        let work_net_peercount = client.request(req).and_then(|resp| {
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
                {"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":74},
                {"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":75}
            ])
        );
        let mut req = hyper::Request::<hyper::Body>::new(Method::Post, uri.clone());
        req.set_body(data);
        let work_net_peercount_batch = client.request(req).and_then(|resp| {
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
        works.push(Box::new(work_net_peercount));
        works.push(Box::new(work_net_peercount_batch));
        core.run(futures::future::join_all(works)).unwrap();

        tx_quit.send(()).unwrap();
        receiver.join().unwrap();
    }
}
