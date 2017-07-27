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

use parking_lot::RwLock;
use byteorder::{BigEndian, ByteOrder};
use std::net::SocketAddr;
use std::time::Duration;
use std::thread;
use std::convert::AsRef;
use std::sync::Arc;
use std::io::prelude::*;
use std::net::TcpStream;
use config;
use libproto::communication;
use protobuf::Message;

const TIMEOUT: u64 = 15;

pub struct Connection {
    pub id_card: u32,
    pub peers_pair: Vec<(u32, SocketAddr, Arc<RwLock<Option<TcpStream>>>)>,
}

impl Connection {
    pub fn new(config: &config::NetConfig) -> Self {
        let peers = config.peers.as_ref().unwrap();
        let id_card = config.id_card.unwrap();
        let mut peers_pair = Vec::default();
        for peer in peers.iter() {
            let id_card: u32 = peer.id_card.unwrap();
            let addr = format!("{}:{}", peer.ip.clone().unwrap(), peer.port.unwrap());
            let addr = addr.parse::<SocketAddr>().unwrap();
            peers_pair.push((id_card, addr, Arc::new(RwLock::new(None))));
        }
        Connection {
            id_card,
            peers_pair,
        }
    }
}

pub fn do_connect(con: &Connection) {
    for &(_, addr, ref stream) in &con.peers_pair {
        let stream_lock = stream.clone();
        thread::spawn(move || loop {
                          {
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
    for &(id_card, _, ref stream) in &con.peers_pair {
        if is_send(id_card, origin, operate) {
            peers.push(id_card);
            send_msg(stream);
        }
    }

    info!("{:?} broadcast msg to nodes {:?} {:?}",
          con.id_card,
          operate,
          peers);
}

pub fn is_send(id_card: u32, origin: u32, operate: communication::OperateType) -> bool {
    let mut is_ok = false;
    if operate == communication::OperateType::BROADCAST {
        is_ok = true;
    } else if operate == communication::OperateType::SINGLE {
        if id_card == origin {
            is_ok = true;
        }
    } else if operate == communication::OperateType::SUBTRACT {
        if origin != id_card {
            is_ok = true;
        }
    }
    is_ok
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
