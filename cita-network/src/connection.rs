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

use bytes::BytesMut;
use citaprotocol::pubsub_message_to_network_message;
use config;
use config::NetConfig;
use libproto::{Message, OperateType};
use native_tls::{self, TlsConnector};
use notify::DebouncedEvent;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio;
use tokio::prelude::*;
use tokio::util::FutureExt;
use tokio_tls;

const TIMEOUT: u64 = 15;
const ROOT_CERT_FILE: &str = "rootCA.crt";

pub enum RealStream {
    CryptStream(tokio_tls::TlsStream<tokio::net::TcpStream>),
    NormalStream(TcpStream),
}

impl RealStream {
    pub fn shutdown(&mut self) -> io::Result<()> {
        match self {
            RealStream::CryptStream(ref mut tls) => tls.shutdown().map(|_| ()),
            RealStream::NormalStream(ref mut tcp) => tcp.shutdown(Shutdown::Both),
        }
    }
}

impl io::Write for RealStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            RealStream::CryptStream(ref mut tls) => tls.write(buf),
            RealStream::NormalStream(ref mut tcp) => tcp.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            RealStream::CryptStream(ref mut tls) => tls.flush(),
            RealStream::NormalStream(ref mut tcp) => tcp.flush(),
        }
    }
}

pub enum Task {
    Broadcast((String, Message)),
    Update(NetConfig),
    NewTCP((u32, SocketAddr, RealStream, String)),
}

/// Manager unconnected address
struct Manager {
    need_connect: Vec<(u32, SocketAddr, String)>,
    connect_receiver: Receiver<(u32, SocketAddr, String)>,
    task_sender: Sender<Task>,
    enable_tls: bool,
}

fn generate_tls_connector(path: &str) -> Option<TlsConnector> {
    let mut file = File::open(path).expect("Not has cert file");
    let mut pem = vec![];
    file.read_to_end(&mut pem).expect("Cert File read error");
    let root_ca =
        native_tls::Certificate::from_pem(&pem).expect("Cert file content not right, pem ?");

    native_tls::TlsConnector::builder()
        .min_protocol_version(Some(native_tls::Protocol::Tlsv11))
        .max_protocol_version(Some(native_tls::Protocol::Tlsv12))
        .add_root_certificate(root_ca)
        .build()
        .ok()
}

impl Manager {
    fn new(
        task_sender: Sender<Task>,
        connect_receiver: Receiver<(u32, SocketAddr, String)>,
        enable_tls: bool,
    ) -> Self {
        Manager {
            need_connect: Vec::new(),
            connect_receiver,
            task_sender,
            enable_tls,
        }
    }

    fn run(&mut self) {
        let (tls_connector, mut rt) = if self.enable_tls {
            let tls_connector = generate_tls_connector(ROOT_CERT_FILE);
            let rt = tokio::runtime::current_thread::Runtime::new().ok();
            if tls_connector.is_none() || rt.is_none() {
                panic!("TLS connector not generated");
            }
            (tls_connector, rt)
        } else {
            (None, None)
        };

        loop {
            while let Ok(message) = self.connect_receiver.try_recv() {
                self.need_connect.push(message);
            }
            let mut new_need_connect = Vec::new();
            while let Some((id, addr, common_name)) = self.need_connect.pop() {
                match tls_connector.clone() {
                    Some(tls_connect) => {
                        let common_name_clone = common_name.clone();
                        let task = tokio::net::TcpStream::connect(&addr)
                            .and_then(move |socket| {
                                tokio_tls::TlsConnector::from(tls_connect)
                                    .connect(&common_name_clone, socket)
                                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
                            })
                            .timeout(Duration::from_secs(TIMEOUT));
                        match rt.as_mut().unwrap().block_on(task) {
                            Ok(tls) => {
                                self.task_sender
                                    .send(Task::NewTCP((
                                        id,
                                        addr,
                                        RealStream::CryptStream(tls),
                                        common_name.clone(),
                                    )))
                                    .unwrap();
                            }
                            Err(e) => {
                                warn!("TLS connect {} failed, error: {}", addr, e);
                                new_need_connect.push((id, addr, common_name));
                            }
                        };
                    }
                    None => match TcpStream::connect_timeout(&addr, Duration::from_secs(TIMEOUT)) {
                        Ok(tcp) => {
                            self.task_sender
                                .send(Task::NewTCP((
                                    id,
                                    addr,
                                    RealStream::NormalStream(tcp),
                                    common_name.clone(),
                                )))
                                .unwrap();
                        }
                        Err(e) => {
                            warn!(
                                "Node{}, {} unable to establish connection, error: {}",
                                id, addr, e
                            );
                            new_need_connect.push((id, addr, common_name));
                        }
                    },
                }
            }
            self.need_connect = new_need_connect;

            if !self.need_connect.is_empty() {
                debug!(
                    "Complete a round of attempts to connect, \
                     left {} address for the next round of processing",
                    self.need_connect.len()
                );
            }

            thread::sleep(Duration::from_secs(TIMEOUT));
        }
    }
}

/// Manage p2p networks
pub struct Connections {
    id_card: u32,
    /// list of peer: id, addr, tcp_connect
    peers: HashMap<(u32, SocketAddr, String), RealStream>,
    pub is_pause: Arc<AtomicBool>,
    pub connect_number: Arc<AtomicUsize>,
    task_receiver: Receiver<Task>,
    connect_sender: Sender<(u32, SocketAddr, String)>,
}

impl Connections {
    pub fn new(config: &config::NetConfig) -> (Self, Sender<Task>) {
        let id_card = config.id_card.unwrap();
        let (task_sender, task_receiver) = channel();
        let (connect_sender, connect_receiver) = channel();

        let connect_task_sender = task_sender.clone();
        let enable_tls = config.enable_tls.unwrap_or(false);
        thread::spawn(move || {
            Manager::new(connect_task_sender, connect_receiver, enable_tls).run()
        });

        if let Some(peers) = config.peers.as_ref() {
            for peer in peers.iter() {
                let id_card: u32 = peer.id_card.unwrap();
                let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap())
                    .parse()
                    .unwrap();
                connect_sender
                    .send((id_card, addr, peer.common_name.clone().unwrap_or_default()))
                    .unwrap();
            }
        }

        (
            Connections {
                id_card,
                peers: HashMap::new(),
                is_pause: Arc::new(AtomicBool::new(false)),
                connect_number: Arc::new(AtomicUsize::new(0)),
                task_receiver,
                connect_sender,
            },
            task_sender,
        )
    }

    fn is_send(id_card: u32, origin: u32, operate: OperateType) -> bool {
        match operate {
            OperateType::Broadcast => true,
            OperateType::Single => id_card == origin,
            OperateType::Subtract => id_card != origin,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self
                .task_receiver
                .recv_timeout(Duration::from_secs(TIMEOUT))
            {
                Ok(task) => match task {
                    Task::Broadcast((key, message)) => {
                        if !self.is_pause.load(Ordering::SeqCst) {
                            self.broadcast(key, message)
                        }
                    }
                    Task::NewTCP(tcp) => {
                        self.peers.insert((tcp.0, tcp.1, tcp.3), tcp.2);
                        self.connect_number.fetch_add(1, Ordering::Relaxed);
                    }
                    Task::Update(config) => self.update(config),
                },
                Err(_) => {
                    self.heart_beat();
                }
            }
        }
    }

    fn update(&mut self, config: config::NetConfig) {
        // Update configuration
        match config.peers {
            Some(peers) => {
                let config_peers = peers
                    .into_iter()
                    .map(|peer| {
                        let id_card: u32 = peer.id_card.unwrap();
                        let addr = format!("{}:{}", peer.ip.unwrap(), peer.port.unwrap())
                            .parse()
                            .unwrap();
                        (id_card, addr, peer.common_name.unwrap_or_default())
                    })
                    .collect::<Vec<(u32, SocketAddr, String)>>();

                let remove_peers = self
                    .peers
                    .keys()
                    .filter(|peer| !config_peers.contains(&peer))
                    .map(|ref peer| {
                        info!("Remove peer {}, {},{}", peer.0, peer.1, peer.2);
                        (peer.0, peer.1, peer.2.clone())
                    })
                    .collect::<Vec<(u32, SocketAddr, String)>>();

                config_peers
                    .into_iter()
                    .filter(|peer| {
                        self.peers
                            .keys()
                            .find(|&current_peer| current_peer == peer)
                            .is_none()
                    })
                    .for_each(|peer| {
                        info!("Add peer {}, {}", peer.0, peer.1);
                        let _ = self.connect_sender.send(peer);
                    });

                self.close(Some(remove_peers), false);
            }
            None => {
                info!("clear all peers after update!");
                self.close::<Vec<(u32, SocketAddr, String)>>(None, false);
            }
        }
    }

    fn broadcast(&mut self, key: String, mut msg: Message) {
        let origin = msg.get_origin();
        let operate = msg.get_operate();
        msg.set_origin(self.id_card);

        trace!("Broadcast msg {:?} from key {}", msg, key);
        let msg_bytes: Vec<u8> = msg.try_into().unwrap();

        let mut buf = BytesMut::with_capacity(4 + 4 + 1 + key.len() + msg_bytes.len());
        pubsub_message_to_network_message(&mut buf, Some((key, msg_bytes)));

        let mut peers = Vec::new();
        let mut remove_peers = Vec::new();
        for (peer, stream) in self.peers.iter_mut() {
            if Connections::is_send(peer.0, origin, operate) {
                match stream.write(&buf) {
                    Ok(_) => {
                        let _ = stream.flush();
                        peers.push(peer.0);
                    }
                    Err(e) => {
                        warn!("Node{} {} is shutdown, err: {}", peer.0, peer.1, e);
                        remove_peers.push(peer.clone());
                    }
                };
            }
        }
        self.close(Some(remove_peers), true);
        trace!(
            "{:?} broadcast msg to nodes {:?} {:?}",
            self.id_card,
            operate,
            peers
        );
    }

    fn close<T: ::std::iter::IntoIterator<Item = (u32, SocketAddr, String)>>(
        &mut self,
        peers: Option<T>,
        need_reconnect: bool,
    ) {
        match peers {
            Some(peers) => peers.into_iter().for_each(|peer| {
                if let Some(mut stream) = self.peers.remove(&peer) {
                    let _ = stream.shutdown().map_err(|err| {
                        warn!("Shutdown {} - {} failed: {}", peer.0, peer.1, err);
                    });
                    if need_reconnect {
                        self.connect_sender.send(peer).unwrap();
                    }
                    self.connect_number.fetch_sub(1, Ordering::Relaxed);
                }
            }),
            None => {
                self.peers.iter_mut().for_each(|(peer, stream)| {
                    let _ = stream.shutdown().map_err(|err| {
                        warn!("Shutdown {} - {} failed: {}", peer.0, peer.1, err);
                    });
                });
                if need_reconnect {
                    self.peers.iter().for_each(|(ref peer, _)| {
                        self.connect_sender
                            .send((peer.0, peer.1, peer.2.clone()))
                            .unwrap();
                    })
                }
                self.peers.clear();
                self.connect_number.store(0, Ordering::Relaxed);
            }
        }
    }

    fn heart_beat(&mut self) {
        let mut buf = BytesMut::with_capacity(4 + 4);
        pubsub_message_to_network_message(&mut buf, None);
        let mut remove_peers = Vec::new();
        for (peer, stream) in self.peers.iter_mut() {
            match stream.write(&buf) {
                Ok(_) => {
                    let _ = stream.flush();
                }
                Err(e) => {
                    warn!(
                        "Node{} {} {} is shutdown, err: {}",
                        peer.0, peer.1, peer.2, e
                    );
                    remove_peers.push((peer.0, peer.1, peer.2.clone()));
                }
            };
        }
        self.close(Some(remove_peers), true);
    }
}

pub fn manage_connect(config_path: &str, rx: Receiver<DebouncedEvent>, task_send: Sender<Task>) {
    let config = String::from(config_path);

    thread::spawn(move || loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path_buf) | DebouncedEvent::Write(path_buf) => {
                    if path_buf.is_file() {
                        let file_name = path_buf.file_name().unwrap().to_str().unwrap();
                        if file_name == config.as_str() {
                            info!("file {} changed, will auto reload!", file_name);
                            let config = NetConfig::new(config.as_str());
                            let _ = task_send.send(Task::Update(config));
                        }
                    }
                }
                _ => trace!("file notify event: {:?}", event),
            },
            Err(e) => warn!("watch error: {:?}", e),
        }
    });
}

#[cfg(test)]
mod test {
    use super::Connections;
    use libproto::OperateType;
    #[test]
    fn is_send_msg() {
        assert!(Connections::is_send(0, 0, OperateType::Broadcast));
        assert!(Connections::is_send(0, 1, OperateType::Broadcast));

        assert!(Connections::is_send(0, 0, OperateType::Single));
        assert!(!Connections::is_send(0, 1, OperateType::Single));

        assert!(!Connections::is_send(0, 0, OperateType::Subtract));
        assert!(Connections::is_send(0, 1, OperateType::Subtract));
    }
}
