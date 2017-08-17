// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

use citaprotocol::{CitaProto, CitaRequest, CitaResponse};

use config::NetConfig;

use futures::{BoxFuture, Future};
use futures::future::result;
use msghandle::net_msg_handler;
use std::{io, thread};
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use tokio_proto::TcpServer;
use tokio_service::Service;

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
                      TcpServer::new(CitaProto, addr).serve(move || Ok(Server { mysender: mysender.clone() }));
                  });
}
