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
use notify::DebouncedEvent;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Write;
use std::net::SocketAddr;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const TIMEOUT: u64 = 15;

pub enum Task {
    Broadcast((String, Message)),
    Update(NetConfig),
    NewTCP((u32, SocketAddr, TcpStream)),
}

/// Manager unconnected address
struct Manager {
    need_connect: Vec<(u32, SocketAddr)>,
    connect_receiver: Receiver<(u32, SocketAddr)>,
    task_sender: Sender<Task>,
}

impl Manager {
    fn new(task_sender: Sender<Task>, connect_receiver: Receiver<(u32, SocketAddr)>) -> Self {
        Manager {
            need_connect: Vec::new(),
            connect_receiver,
            task_sender,
        }
    }

    fn run(&mut self) {
        loop {
            while let Ok(message) = self.connect_receiver.try_recv() {
                self.need_connect.push(message);
            }
            let mut new_need_connect = Vec::new();
            for (id, addr) in self.need_connect.iter() {
                match TcpStream::connect_timeout(addr, Duration::from_secs(TIMEOUT)) {
                    Ok(tcp) => {
                        self.task_sender
                            .send(Task::NewTCP((*id, *addr, tcp)))
                            .unwrap();
                    }
                    Err(e) => {
                        warn!(
                            "Node{}, {} unable to establish connection, error: {}",
                            id, addr, e
                        );
                        new_need_connect.push((*id, *addr));
                    }
                }
            }
            self.need_connect = new_need_connect;

            if !self.need_connect.is_empty() {
                trace!(
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
    peers: HashMap<(u32, SocketAddr), TcpStream>,
    pub is_pause: Arc<AtomicBool>,
    pub connect_number: Arc<AtomicUsize>,
    task_receiver: Receiver<Task>,
    connect_sender: Sender<(u32, SocketAddr)>,
}

impl Connections {
    pub fn new(config: &config::NetConfig) -> (Self, Sender<Task>) {
        let id_card = config.id_card.unwrap();
        let (task_sender, task_receiver) = channel();
        let (connect_sender, connect_receiver) = channel();

        let connect_task_sender = task_sender.clone();
        thread::spawn(move || Manager::new(connect_task_sender, connect_receiver).run());

        if let Some(peers) = config.peers.as_ref() {
            for peer in peers.iter() {
                let id_card: u32 = peer.id_card.unwrap();
                let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap())
                    .parse()
                    .unwrap();
                connect_sender.send((id_card, addr)).unwrap();
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
                        self.peers.insert((tcp.0, tcp.1), tcp.2);
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
                        (id_card, addr)
                    })
                    .collect::<Vec<(u32, SocketAddr)>>();

                let remove_peers = self
                    .peers
                    .keys()
                    .filter(|peer| !config_peers.contains(&peer))
                    .map(|&peer| {
                        info!("Remove peer {}, {}", peer.0, peer.1);
                        peer
                    })
                    .collect::<Vec<(u32, SocketAddr)>>();

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
                self.close::<Vec<(u32, SocketAddr)>>(None, false);
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
                        remove_peers.push(*peer);
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

    fn close<T: ::std::iter::IntoIterator<Item = (u32, SocketAddr)>>(
        &mut self,
        peers: Option<T>,
        need_reconnect: bool,
    ) {
        match peers {
            Some(peers) => peers.into_iter().for_each(|peer| {
                if let Some(stream) = self.peers.remove(&peer) {
                    let _ = stream.shutdown(Shutdown::Both).map_err(|err| {
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
                    let _ = stream.shutdown(Shutdown::Both).map_err(|err| {
                        warn!("Shutdown {} - {} failed: {}", peer.0, peer.1, err);
                    });
                });
                if need_reconnect {
                    self.peers.iter().for_each(|(&peer, _)| {
                        self.connect_sender.send(peer).unwrap();
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
                    warn!("Node{} {} is shutdown, err: {}", peer.0, peer.1, e);
                    remove_peers.push((peer.0, peer.1));
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
