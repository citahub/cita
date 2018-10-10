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

use citaprotocol::{CitaCodec, CitaRequest};
use futures::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use tokio;
use tokio::codec::Decoder;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use Source;

#[derive(Clone)]
pub struct NetServer {
    net_sender: Sender<(Source, CitaRequest)>,
}

impl NetServer {
    pub fn new(net_sender: Sender<(Source, CitaRequest)>) -> NetServer {
        NetServer { net_sender }
    }

    pub fn server(self, addr: SocketAddr) {
        let listener = TcpListener::bind(&addr).unwrap();
        let server = listener
            .incoming()
            .for_each(move |socket| {
                process(socket, self.net_sender.clone());
                Ok(())
            })
            .map_err(|err| {
                error!("accept error = {:?}", err);
            });

        tokio::run(server);
    }
}

fn process(socket: TcpStream, send: Sender<(Source, CitaRequest)>) {
    let (_tx, rx) = CitaCodec.framed(socket).split();
    let task = rx
        .for_each(move |chunk| {
            send.send((Source::REMOTE, chunk))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        })
        .map_err(|e| error!("reading error = {:?}", e));
    tokio::spawn(task);
}

unsafe impl Send for NetServer {}
unsafe impl Sync for NetServer {}
