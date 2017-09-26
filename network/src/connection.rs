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

use byteorder::{BigEndian, ByteOrder};
use config;
use libproto::communication;
use protobuf::Message;
use std::convert::AsRef;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use util::RwLock;

const TIMEOUT: u64 = 15;
//表示线程已经运行
type ThreadRuned = Arc<RwLock<bool>>;
//已经运行的线程是否结束
type ThreadExit = Arc<RwLock<bool>>;

pub struct Connection {
    pub id_card: u32,
    pub peers_pair: Vec<(u32, SocketAddr, Arc<RwLock<Option<TcpStream>>>, ThreadRuned, ThreadExit)>,
    pub remove_addr: Arc<RwLock<Vec<SocketAddr>>>,
}

impl Connection {
    pub fn new(config: &config::NetConfig) -> Self {
        let id_card = config.id_card.unwrap();
        let mut peers_pair = Vec::default();
        match config.peers.as_ref() {
            Some(peers) => {
                for peer in peers.iter() {
                    let id_card: u32 = peer.id_card.unwrap();
                    let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap());
                    let addr = addr.parse::<SocketAddr>().unwrap();
                    peers_pair.push((id_card, addr, Arc::new(RwLock::new(None)), Arc::new(RwLock::new(false)), Arc::new(RwLock::new(false))));
                }
            }
            None => (),

        }

        Connection {
            id_card,
            peers_pair,
            remove_addr: Arc::new(RwLock::new(Vec::default())),
        }
    }

    pub fn update(&mut self, config: &config::NetConfig) {
        //添加更新的配置到self
        match config.peers.as_ref() {
            Some(peers) => {
                let peers_addr: Vec<SocketAddr> = self.peers_pair.clone().into_iter().map(|peers_pair| peers_pair.1).rev().collect();
                let mut config_addr = Vec::new();
                for peer in peers.iter() {
                    let id_card: u32 = peer.id_card.unwrap();
                    let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap());
                    let addr = addr.parse::<SocketAddr>().unwrap();
                    config_addr.push(addr);
                    if peers_addr.contains(&addr) {
                        continue;
                    }
                    self.peers_pair
                        .push((id_card, addr, Arc::new(RwLock::new(None)), Arc::new(RwLock::new(false)), Arc::new(RwLock::new(false))));
                }
                for &(_, addr, _, _, ref thread_exit) in &self.peers_pair {
                    if config_addr.contains(&addr) {
                        continue;
                    }
                    let thread_exit = &mut *thread_exit.as_ref().write();
                    *thread_exit = true;
                }
            }
            None => (),
        }
    }

    pub fn del_peer(&mut self) {
        let remove_addr = &mut *self.remove_addr.as_ref().write();
        if !remove_addr.is_empty() {
            let mut index = 0;
            let mut len = self.peers_pair.len();
            loop {
                if index >= len {
                    break;
                }
                if remove_addr.contains(&self.peers_pair[index].1) {
                    self.peers_pair.remove(index);
                    len = self.peers_pair.len();
                } else {
                    index += 1;
                }
            }
            remove_addr.clear();
        }
    }
}


pub fn do_connect(con: &Connection) {
    for &(_, addr, ref stream, ref thread_run, ref thread_exit) in &con.peers_pair {
        let stream_lock = stream.clone();
        let thread_run_lock = thread_run.clone();
        let thread_exit_lock = thread_exit.clone();
        if *thread_run.read() {
            continue;
        }

        let remove_addr_lock = con.remove_addr.clone();

        thread::spawn(move || loop {
                          {
                              if *thread_exit_lock.read() {
                                  let remove_addr = &mut *remove_addr_lock.as_ref().write();
                                  trace!("{:?} thread exit", addr);
                                  remove_addr.push(addr);
                                  let stream_opt = &mut *stream_lock.as_ref().write();
                                  *stream_opt = None;
                                  break;
                              }
                              {
                                  let thread_run = &mut *thread_run_lock.as_ref().write();
                                  *thread_run = true;
                              }

                              let stream_opt = &mut *stream_lock.as_ref().write();
                              if stream_opt.is_none() {
                                  trace!("connet {:?}", addr);
                                  let stream = TcpStream::connect(addr).ok();
                                  *stream_opt = stream;
                              }

                              let mut need_reconnect = false;
                              if let Some(ref mut stream) = stream_opt.as_mut() {
                                  trace!("handshake with {:?}!", addr);
                                  let mut header = [0; 8];
                                  BigEndian::write_u64(&mut header, 0xDEADBEEF00000000 as u64);
                                  let res = stream.write(&header);
                                  if res.is_err() {
                                      warn!("handshake with {:?} error!", addr);
                                      need_reconnect = true;
                                  }
                              }

                              if need_reconnect {
                                  *stream_opt = None;
                              }
                          }

                          let ten_sec = Duration::from_millis(TIMEOUT * 1000);
                          thread::sleep(ten_sec);
                          trace!("after sleep retry connect {:?}!", addr);
                      });
    }
}

pub fn broadcast(con: &Connection, mut msg: communication::Message) {
    let origin = msg.get_origin();
    let operate = msg.get_operate();
    msg.set_origin(con.id_card);

    trace!("broadcast msg {:?} ", msg);
    let msg = msg.write_to_bytes().unwrap();
    let request_id = 0xDEADBEEF00000000 + msg.len();
    let mut encoded_request_id = [0; 8];
    BigEndian::write_u64(&mut encoded_request_id, request_id as u64);
    let mut buf = Vec::new();
    buf.extend(&encoded_request_id);
    buf.extend(msg);
    let send_msg = move |stream: &Arc<RwLock<Option<TcpStream>>>| {
        let streams_lock = stream.clone();
        let stream_opt = &mut (*streams_lock.as_ref().write());
        if let Some(ref mut stream) = stream_opt.as_mut() {
            let _ = stream.write(&buf);
        }
    };
    let mut peers = vec![];
    for &(id_card, _, ref stream, _, _) in &con.peers_pair {
        if is_send(id_card, origin, operate) {
            peers.push(id_card);
            send_msg(stream);
        }
    }

    info!("{:?} broadcast msg to nodes {:?} {:?}", con.id_card, operate, peers);
}

pub fn is_send(id_card: u32, origin: u32, operate: communication::OperateType) -> bool {
    operate == communication::OperateType::BROADCAST || (operate == communication::OperateType::SINGLE && id_card == origin) || (operate == communication::OperateType::SUBTRACT && origin != id_card)
}

pub fn start_client(con: Arc<RwLock<Connection>>, rx: Receiver<communication::Message>) {
    thread::spawn(move || {
                      info!("start client!");
                      loop {
                          let msg = rx.recv().unwrap();
                          let con = &*con.as_ref().read();
                          broadcast(&con, msg);
                      }
                  });
}

#[cfg(test)]
mod test {
    use super::is_send;
    use libproto::communication;
    #[test]
    fn is_seng_mag() {
        assert!(is_send(0, 0, communication::OperateType::BROADCAST));
        assert!(is_send(0, 1, communication::OperateType::BROADCAST));

        assert!(is_send(0, 0, communication::OperateType::SINGLE));
        assert!(!is_send(0, 1, communication::OperateType::SINGLE));

        assert!(!is_send(0, 0, communication::OperateType::SUBTRACT));
        assert!(is_send(0, 1, communication::OperateType::SUBTRACT));
    }
}
