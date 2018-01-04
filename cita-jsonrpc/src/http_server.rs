use error::ErrorCode;
use futures;
use futures::Stream;
use futures::future::Either;
use futures::future::Future;
use futures::sync::oneshot;
use helper::{select_topic, ReqInfo, ReqSender, RpcMap, TransferType};
use hyper;
use hyper::{Method, StatusCode};
use hyper::server::{Http, NewService, Request, Response, Service};
use jsonrpc_types::{method, Error, RpcRequest};
use jsonrpc_types::response::RpcFailure;
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
                        let id = rpc.id.clone();
                        let jsonrpc_version = rpc.jsonrpc.clone();
                        let topic = select_topic(&rpc.method);

                        match method_handler.request(rpc) {
                            Ok(req) => {
                                if let Ok(timeout) = Timeout::new(timeout, &reactor_handle) {
                                    let request_id = req.request_id.clone();
                                    let (tx, rx) = oneshot::channel();
                                    let req_info = (ReqInfo::new(jsonrpc_version.clone(), id.clone()), tx);
                                    {
                                        responses
                                            .lock()
                                            .insert(request_id.clone(), TransferType::HTTP(req_info));
                                    }
                                    let _ = sender.send((topic, req));

                                    let mq_resp = rx.map(|resp_body| Response::new().with_body(resp_body))
                                        .map_err(|_| hyper::Error::Incomplete);

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

                                    Either::A(resp)
                                } else {
                                    Either::B(futures::future::ok(
                                        Response::new().with_status(StatusCode::InternalServerError),
                                    ))
                                }
                            }
                            Err(e) => {
                                let resp_body = serde_json::to_vec(&RpcFailure::from_options(id, jsonrpc_version, e))
                                    .expect("should be serialize by serde_json");
                                Either::B(futures::future::ok(Response::new().with_body(resp_body)))
                            }
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
