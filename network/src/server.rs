use std::net::SocketAddr;
use std::{io, thread};
use std::sync::mpsc::Sender;

use futures::{BoxFuture, Future};
use futures::future::result;
use tokio_proto::TcpServer;
use tokio_service::Service;

use config::NetConfig;
use citaprotocol::{CitaProto, CitaRequest, CitaResponse};
use msghandle::net_msg_handler;

#[derive(Clone)]
pub struct MySender {
    tx: Sender<(String, CitaRequest)>,
}

impl MySender {
    pub fn new(tx: Sender<(String, CitaRequest)>) -> Self {
        MySender { tx: tx }
    }

    pub fn send(&self, msg: (String, CitaRequest)) {
        self.tx.send(msg).unwrap();
    }
}

unsafe impl Sync for MySender {}

struct Server {
    mysender: MySender,
}

impl Service for Server {
    type Request = CitaRequest;
    type Response = CitaResponse;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, io::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        result(net_msg_handler(req, &self.mysender)).boxed()
    }
}

pub fn start_server(config: &NetConfig, mysender: MySender) {
    let addr = format!("0.0.0.0:{}", config.port.unwrap());
    let addr = addr.parse::<SocketAddr>().unwrap();

    thread::spawn(move || {
                      info!("start server on {:?}!", addr);
                      TcpServer::new(CitaProto, addr).serve(move || {
                                                                Ok(Server {
                                                                       mysender: mysender.clone(),
                                                                   })
                                                            });
                  });
}
