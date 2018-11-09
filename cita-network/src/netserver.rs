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
use native_tls;
use std::fs::File;
use std::io;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio;
use tokio::codec::Decoder;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio_tls::TlsAcceptor;
use Source;

const SERVER_CERT_NAME: &str = "server.pfx";
const SERVER_CERT_PASSWORD: &str = "server.tls.cita";

#[derive(Clone)]
pub struct NetServer {
    net_sender: Sender<(Source, CitaRequest)>,
    enable_tls: bool,
}

fn generate_tls_acceptor(path: &str, password: &str) -> Option<TlsAcceptor> {
    let mut file = File::open(path).unwrap();
    let mut pkcs12 = vec![];
    file.read_to_end(&mut pkcs12).unwrap();

    let pkcs12 = native_tls::Identity::from_pkcs12(&pkcs12, password).unwrap();

    //let acceptor = TlsAcceptor::new(pkcs12).unwrap();
    native_tls::TlsAcceptor::builder(pkcs12)
        .min_protocol_version(Some(native_tls::Protocol::Tlsv11))
        .max_protocol_version(Some(native_tls::Protocol::Tlsv12))
        .build()
        .ok()
        .and_then(|acceptor| Some(TlsAcceptor::from(acceptor)))
}

impl NetServer {
    pub fn new(net_sender: Sender<(Source, CitaRequest)>, enable_tls: bool) -> NetServer {
        NetServer {
            net_sender,
            enable_tls,
        }
    }

    pub fn server(self, addr: SocketAddr) {
        let mut tls_acceptor = None;
        if self.enable_tls {
            tls_acceptor = generate_tls_acceptor(SERVER_CERT_NAME, SERVER_CERT_PASSWORD);
            if tls_acceptor.is_none() {
                panic!("TLS Server Cert for acceptor is not ok");
            }
        };

        let tokio_tls_acceptor = Arc::new(tls_acceptor);

        let listener = TcpListener::bind(&addr).unwrap();
        let server = listener
            .incoming()
            .for_each(move |socket| {
                process(socket, self.net_sender.clone(), &tokio_tls_acceptor);
                Ok(())
            })
            .map_err(|err| {
                error!("accept error = {:?}", err);
            });

        tokio::run(server);
    }
}

fn process(
    socket: TcpStream,
    send: Sender<(Source, CitaRequest)>,
    acceptor: &Arc<Option<TlsAcceptor>>,
) {
    if let Some(ref acceptor) = *acceptor.clone() {
        let accept_task = acceptor
            .accept(socket)
            .and_then(|tls| {
                let (_tx, rx) = CitaCodec.framed(tls).split();
                let task = rx
                    .for_each(move |chunk| {
                        send.send((Source::REMOTE, chunk))
                            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                    })
                    .map_err(|e| error!("reading error = {:?}", e));
                tokio::spawn(task);
                Ok(())
            })
            .map_err(|err| {
                error!("server error {:?}", err);
            });
        tokio::spawn(accept_task);
    } else {
        let (_tx, rx) = CitaCodec.framed(socket).split();
        let task = rx
            .for_each(move |chunk| {
                send.send((Source::REMOTE, chunk))
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            })
            .map_err(|e| error!("reading error = {:?}", e));
        tokio::spawn(task);
    }
}

unsafe impl Send for NetServer {}
