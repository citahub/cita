// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use connection::Task;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotResp};
use libproto::{Message, Response};
use std::convert::{Into, TryFrom, TryInto};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use Source;

/// Message forwarding, include p2p and local
pub struct NetWork {
    task_sender: Sender<Task>,
    tx_pub: Sender<(String, Vec<u8>)>,
    tx_sync: Sender<(Source, (String, Vec<u8>))>,
    tx_new_tx: Sender<(String, Vec<u8>)>,
    tx_consensus: Sender<(String, Vec<u8>)>,
    is_pause: Arc<AtomicBool>,
    connect_number: Arc<AtomicUsize>,
}

impl NetWork {
    pub fn new(
        task_sender: Sender<Task>,
        tx_pub: Sender<(String, Vec<u8>)>,
        tx_sync: Sender<(Source, (String, Vec<u8>))>,
        tx_new_tx: Sender<(String, Vec<u8>)>,
        tx_consensus: Sender<(String, Vec<u8>)>,
        is_pause: Arc<AtomicBool>,
        connect_number: Arc<AtomicUsize>,
    ) -> Self {
        NetWork {
            task_sender,
            tx_pub,
            tx_sync,
            tx_new_tx,
            tx_consensus,
            is_pause,
            connect_number,
        }
    }

    pub fn receiver(&self, source: Source, payload: (String, Vec<u8>)) {
        let (key, data) = payload;
        let rtkey = RoutingKey::from(&key);
        trace!("Network receive Msg from {:?}/{}", source, key);
        if self.is_pause.load(Ordering::SeqCst) && rtkey.get_sub_module() != SubModules::Snapshot {
            return;
        }
        match source {
            // Come from MQ
            Source::LOCAL => match rtkey {
                routing_key!(Chain >> Status) => {
                    let _ = self.tx_sync.send((source, (key, data)));
                }
                routing_key!(Chain >> SyncResponse) => {
                    let msg = Message::try_from(&data).unwrap();
                    self.task_sender
                        .send(Task::Broadcast((
                            routing_key!(Synchronizer >> SyncResponse).into(),
                            msg,
                        )))
                        .unwrap();
                }
                routing_key!(Jsonrpc >> RequestNet) => {
                    self.reply_rpc(&data);
                }
                routing_key!(Snapshot >> SnapshotReq) => {
                    info!("set disconnect and response");
                    self.snapshot_req(&data);
                }
                _ => {
                    error!("Unexpected key {} from {:?}", key, source);
                }
            },
            // Come from Netserver
            Source::REMOTE => match rtkey {
                routing_key!(Synchronizer >> Status)
                | routing_key!(Synchronizer >> SyncResponse) => {
                    let _ = self.tx_sync.send((source, (key, data)));
                }
                routing_key!(Synchronizer >> SyncRequest) => {
                    let _ = self
                        .tx_pub
                        .send((routing_key!(Net >> SyncRequest).into(), data));
                }
                routing_key!(Auth >> Request) => {
                    let _ = self
                        .tx_new_tx
                        .send((routing_key!(Net >> Request).into(), data));
                }
                routing_key!(Consensus >> SignedProposal) => {
                    let _ = self
                        .tx_consensus
                        .send((routing_key!(Net >> SignedProposal).into(), data));
                }
                routing_key!(Consensus >> RawBytes) => {
                    let _ = self
                        .tx_consensus
                        .send((routing_key!(Net >> RawBytes).into(), data));
                }
                _ => {
                    error!("Unexpected key {} from {:?}", key, source);
                }
            },
        }
    }

    fn snapshot_req(&self, data: &[u8]) {
        let mut msg = Message::try_from(data).unwrap();
        let req = msg.take_snapshot_req().unwrap();
        let mut resp = SnapshotResp::new();
        let mut send = false;
        match req.cmd {
            Cmd::Snapshot => {
                info!("[snapshot] receive cmd: Snapshot");
            }
            Cmd::Begin => {
                info!("[snapshot] receive cmd: Begin");
                self.is_pause.store(true, Ordering::SeqCst);
                resp.set_resp(Resp::BeginAck);
                resp.set_flag(true);
                send = true;
            }
            Cmd::Restore => {
                info!("[snapshot] receive cmd: Restore");
            }
            Cmd::Clear => {
                info!("[snapshot] receive cmd: Clear");
                resp.set_resp(Resp::ClearAck);
                resp.set_flag(true);
                send = true;
            }
            Cmd::End => {
                info!("[snapshot] receive cmd: End");
                self.is_pause.store(false, Ordering::SeqCst);
                resp.set_resp(Resp::EndAck);
                resp.set_flag(true);
                send = true;
            }
        }

        if send {
            let msg: Message = resp.into();
            self.tx_pub
                .send((
                    routing_key!(Net >> SnapshotResp).into(),
                    (&msg).try_into().unwrap(),
                ))
                .unwrap();
        }
    }

    pub fn reply_rpc(&self, data: &[u8]) {
        let mut msg = Message::try_from(data).unwrap();
        let req_opt = msg.take_request();
        {
            if let Some(mut ts) = req_opt {
                let mut response = Response::new();
                response.set_request_id(ts.take_request_id());
                if ts.has_peercount() {
                    let peercount = self.connect_number.load(Ordering::Relaxed);
                    response.set_peercount(peercount as u32);
                    let ms: Message = response.into();
                    self.tx_pub
                        .send((routing_key!(Net >> Response).into(), ms.try_into().unwrap()))
                        .unwrap();
                }
            } else {
                warn!("receive unexpected rpc data");
            }
        }
    }
}
