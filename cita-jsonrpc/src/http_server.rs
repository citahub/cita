use error::ErrorCode;
use futures;
use futures::Stream;
use futures::future::Either;
use futures::future::Future;
use futures::stream::Collect;
use futures::stream::FuturesOrdered;
use futures::sync::oneshot;
use helper::{select_topic, ReqInfo, ReqSender, RpcMap, TransferType};
use hyper;
use hyper::{Method, StatusCode};
use hyper::server::{Http, NewService, Request, Response, Service};
use jsonrpc_types::{method, Call, Error, RpcRequest};
use jsonrpc_types::method::MethodHandler;
use jsonrpc_types::response::{RpcFailure, RpcResponse};
use jsonrpc_types::response::Output;
use libproto::request as reqlib;
use net2;
use serde_json;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Duration;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Core, Handle};
use tokio_core::reactor::Timeout;
use util::Mutex;

const TCP_BACKLOG: i32 = 1024;

struct Inner {
    pub tx: ReqSender,
    pub responses: RpcMap,
    pub timeout: Duration,
    pub reactor_handle: Handle,
    pub method_handler: method::MethodHandler,
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

        match (req.method(), req.path()) {
            (&Method::Post, "/") => {
                let mapping = req.body().concat2().and_then(move |chunk| {
                    if let Ok(rpc) = serde_json::from_slice::<RpcRequest>(&chunk) {
                        match rpc {
                            RpcRequest::Single(call) => match read_single(&call, method_handler) {
                                Ok(req) => {
                                    if let Ok(timeout) = Timeout::new(timeout, &reactor_handle) {
                                        let id = call.id.clone();
                                        let jsonrpc_version = call.jsonrpc.clone();
                                        let request_id = req.request_id.clone();
                                        let future_result = handle_single(call, req, responses, sender);
                                        let mq_resp = future_result.map_err(|_| hyper::Error::Incomplete);

                                        let resp = mq_resp.select2(timeout).then(move |res| match res {
                                            Ok(Either::A((got, _timeout))) => Ok(got),
                                            Ok(Either::B((_timeout_error, _get))) => {
                                                {
                                                    timeout_responses.lock().remove(&request_id);
                                                }
                                                let failure = RpcFailure::from_options(
                                                    id,
                                                    jsonrpc_version,
                                                    Error::server_error(
                                                        ErrorCode::time_out_error(),
                                                        "system time out, please resend",
                                                    ),
                                                );
                                                let resp_body = serde_json::to_string(&failure)
                                                    .expect("should be serialize by serde_json");
                                                Ok(Response::new().with_body(resp_body))
                                            }
                                            Err(Either::A((get_error, _timeout))) => Err(get_error),
                                            Err(Either::B((timeout_error, _get))) => Err(From::from(timeout_error)),
                                        });

                                        Either::A(Either::A(resp))
                                    } else {
                                        Either::B(futures::future::ok(
                                            Response::new().with_status(StatusCode::InternalServerError),
                                        ))
                                    }
                                }
                                Err(resp) => Either::B(futures::future::ok(resp)),
                            },
                            RpcRequest::Batch(calls) => match read_batch(calls, method_handler) {
                                Ok(reqs) => {
                                    let request_ids: Vec<Vec<u8>> = reqs.iter()
                                        .map(|&(ref _call, ref req)| req.request_id.clone())
                                        .collect();

                                    let future_result = handle_batch(reqs, responses, sender);

                                    if let Ok(timeout) = Timeout::new(timeout, &reactor_handle) {
                                        let mq_resp = future_result.map_err(|_| hyper::Error::Incomplete);

                                        let resp = mq_resp.select2(timeout).then(move |res| match res {
                                            Ok(Either::A((got, _timeout))) => Ok(got),
                                            Ok(Either::B((_timeout_error, _get))) => {
                                                {
                                                    let mut guard = timeout_responses.lock();
                                                    for request_id in request_ids {
                                                        guard.remove(&request_id);
                                                    }
                                                }
                                                let failure = RpcFailure::from(Error::server_error(
                                                    ErrorCode::time_out_error(),
                                                    "system time out, please resend",
                                                ));
                                                let resp_body = serde_json::to_string(&failure)
                                                    .expect("should be serialize by serde_json");
                                                Ok(Response::new().with_body(resp_body))
                                            }
                                            Err(Either::A((get_error, _timeout))) => Err(get_error),
                                            Err(Either::B((timeout_error, _get))) => Err(From::from(timeout_error)),
                                        });

                                        Either::A(Either::B(resp))
                                    } else {
                                        Either::B(futures::future::ok(
                                            Response::new().with_status(StatusCode::InternalServerError),
                                        ))
                                    }
                                }
                                Err(resp) => Either::B(futures::future::ok(resp)),
                            },
                        }
                    } else {
                        Either::B(futures::future::ok(
                            Response::new().with_status(StatusCode::BadRequest),
                        ))
                    }
                });
                let resp: Box<Future<Error = hyper::Error, Item = hyper::Response>> = Box::new(mapping);
                resp
            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}

fn read_single(call: &Call, method_handler: MethodHandler) -> Result<reqlib::Request, Response> {
    match method_handler.request(call) {
        Ok(req) => Ok(req),
        Err(e) => {
            let resp_body = serde_json::to_vec(&RpcFailure::from_options(
                call.id.clone(),
                call.jsonrpc.clone(),
                e,
            )).expect("should be serialize by serde_json");
            Err(Response::new().with_body(resp_body))
        }
    }
}

type SingleFutureMap = futures::future::Map<oneshot::Receiver<Output>, fn(Output) -> Response>;

fn handle_single(
    call: Call,
    req: reqlib::Request,
    responses: RpcMap,
    sender: mpsc::Sender<(String, reqlib::Request)>,
) -> SingleFutureMap {
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
    rx.map(|resp_body| Response::new().with_body(serde_json::to_vec(&resp_body).unwrap()))
}

fn read_batch(calls: Vec<Call>, method_handler: MethodHandler) -> Result<Vec<(Call, reqlib::Request)>, Response> {
    let mut reqs = Vec::with_capacity(calls.len());
    for call in calls {
        match method_handler.request(&call) {
            Ok(req) => {
                reqs.push((call, req));
            }
            Err(_) => return Err(Response::new().with_status(StatusCode::BadRequest)),
        }
    }
    Ok(reqs)
}

type BatchFutureMap = futures::future::Map<
    Collect<FuturesOrdered<oneshot::Receiver<Output>>>,
    fn(Vec<Output>) -> Response,
>;

fn handle_batch(
    reqs: Vec<(Call, reqlib::Request)>,
    responses: RpcMap,
    sender: mpsc::Sender<(String, reqlib::Request)>,
) -> BatchFutureMap {
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
    FuturesOrdered::from_iter(rxs)
        .collect()
        .map(|resp_body| Response::new().with_body(serde_json::to_vec(&RpcResponse::Batch(resp_body)).unwrap()))
}

impl Server {
    pub fn start(
        core: Core,
        listener: TcpListener,
        tx: mpsc::Sender<(String, reqlib::Request)>,
        responses: RpcMap,
        timeout: Duration,
    ) {
        let new_service = NewServer {
            inner: Arc::new(Inner {
                tx: Mutex::new(tx),
                responses: responses,
                timeout: timeout,
                reactor_handle: core.handle(),
                method_handler: method::MethodHandler,
            }),
        };
        let mut server = Http::new()
            .keep_alive(true)
            .bind_listener(core, listener, new_service)
            .unwrap();
        server.no_proto();
        server.run().unwrap();
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
