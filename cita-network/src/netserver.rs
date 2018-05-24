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
use futures::future::result;
use futures::{BoxFuture, Future};
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use tokio_proto::TcpServer;
use tokio_service::{NewService, Service};
use Source;

#[derive(Clone)]
pub struct NetServer {
    net_sender: Sender<(Source, CitaRequest)>,
}

impl Service for NetServer {
    type Request = CitaRequest;
    type Response = CitaResponse;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, io::Error>;

    fn call(&self, payload: Self::Request) -> Self::Future {
        trace!("SERVER get msg: {:?}", payload);
        self.net_sender.send((Source::REMOTE, payload));
        result(Ok(None)).boxed()
    }
}

impl NewService for NetServer {
    type Request = CitaRequest;
    type Response = CitaResponse;
    type Error = io::Error;
    type Instance = Self;
    /// Create and return a new service value.
    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

impl NetServer {
    pub fn new(net_sender: Sender<(Source, CitaRequest)>) -> NetServer {
        NetServer {
            net_sender: net_sender,
        }
    }

    pub fn server(self, addr: SocketAddr) {
        TcpServer::new(CitaProto, addr).serve(self);
    }
}

unsafe impl Send for NetServer {}
unsafe impl Sync for NetServer {}
