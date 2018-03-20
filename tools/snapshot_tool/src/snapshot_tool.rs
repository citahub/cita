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

use libproto::Message;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq, SnapshotResp};
use std::convert::{TryFrom, TryInto};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
//use std::thread;
//use std::time::Duration;

#[derive(Clone)]
pub struct SnapShot {
    ctx_pub: Sender<(String, Vec<u8>)>,
    start_height: u64,
    end_height: u64,
    file: String,
    ack_count: Arc<RwLock<u8>>,
}

impl SnapShot {
    pub fn new(ctx_pub: Sender<(String, Vec<u8>)>, start_height: u64, end_height: u64, file: String) -> Self {
        SnapShot {
            ctx_pub: ctx_pub,
            start_height: start_height,
            end_height: end_height,
            file: file,
            ack_count: Arc::new(RwLock::new(0)),
        }
    }

    // parse resp data
    pub fn parse_data(self, key: String, msg_vec: Vec<u8>) -> bool {
        let mut msg = Message::try_from(&msg_vec).unwrap();
        let _origin = msg.get_origin();
        let _content_ext = msg.take_content();

        if RoutingKey::from(&key).is_msg_type(MsgType::SnapshotResp) {
            let resp = msg.take_snapshot_resp().unwrap();
            self.parse_resp(resp)
        } else {
            error!("error MsgClass!!!!");
            false
        }
    }

    pub fn parse_resp(self, resp: SnapshotResp) -> bool {
        info!("receive resp ack: {:?}", resp.resp);
        let snap_shot = match resp.resp {
            Resp::SnapshotAck => {
                // snapshot over
                info!("receive snapshot ack, snapshot end !");
                return true;
            }
            Resp::BeginAck => {
                // send clear
                info!("receive begin ack");
                let mut ack_count = self.ack_count.write().unwrap();
                *ack_count += 1;
                info!("receive ack_count: {:?}", *ack_count);
                if *ack_count >= 3 {
                    //key_list elements number
                    *ack_count = 0;
                    self.clone().clear()
                } else {
                    return false;
                }
            }
            Resp::ClearAck => {
                // send restore
                info!("receive clear ack");
                let mut ack_count = self.ack_count.write().unwrap();
                *ack_count += 1;
                if *ack_count >= 3 {
                    *ack_count = 0;
                    self.clone().restore()
                } else {
                    return false;
                }
            }
            Resp::RestoreAck => {
                // send end
                info!("receive restore ack");
                self.clone().end()
            }
            Resp::EndAck => {
                // restore over
                info!("receive restore end ack, restore end !");
                return true;
            }
        };
        self.send_cmd(snap_shot);
        false
    }

    // 发送snapshot命令
    pub fn snapshot(self) -> SnapshotReq {
        let mut snap_shot = SnapshotReq::new();
        snap_shot.set_cmd(Cmd::Snapshot);
        snap_shot.set_start_height(self.start_height);
        snap_shot.set_end_height(self.end_height);
        snap_shot.set_file(self.file.clone());
        self.send_cmd(snap_shot.clone());
        snap_shot
    }

    // send begin
    pub fn begin(self) -> SnapshotReq {
        let mut snap_shot = SnapshotReq::new();
        snap_shot.set_cmd(Cmd::Begin);
        self.send_cmd(snap_shot.clone());
        snap_shot
    }

    // send clear
    pub fn clear(self) -> SnapshotReq {
        let mut snap_shot = SnapshotReq::new();
        snap_shot.set_cmd(Cmd::Clear);
        snap_shot
    }

    // send restore
    pub fn restore(self) -> SnapshotReq {
        let mut snap_shot = SnapshotReq::new();
        snap_shot.set_cmd(Cmd::Restore);
        snap_shot.set_file(self.file.clone());
        snap_shot
    }

    // send end
    pub fn end(self) -> SnapshotReq {
        let mut snap_shot = SnapshotReq::new();
        snap_shot.set_cmd(Cmd::End);
        snap_shot
    }

    pub fn send_cmd(self, snap_shot: SnapshotReq) {
        let msg: Message = snap_shot.into();
        info!("tool send_cmd");
        self.ctx_pub
            .send((
                routing_key!(Snapshot >> SnapshotReq).into(),
                (&msg).try_into().unwrap(),
            ))
            .unwrap();
    }
}
