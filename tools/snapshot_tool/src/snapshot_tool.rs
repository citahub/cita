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

use libproto::blockchain::Proof;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq};
use libproto::Message;
use std::convert::{TryFrom, TryInto};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
enum AckType {
    Chain,
    Executor,
    Auth,
    Consensus,
    Net,
}

impl From<SubModules> for AckType {
    fn from(sub_modules: SubModules) -> Self {
        match sub_modules {
            SubModules::Chain => AckType::Chain,
            SubModules::Executor => AckType::Executor,
            SubModules::Auth => AckType::Auth,
            SubModules::Consensus => AckType::Consensus,
            SubModules::Net => AckType::Net,
            _ => {
                error!("Wrong submodule: {:?}.", sub_modules);
                AckType::Chain
            }
        }
    }
}

#[derive(Clone, Copy)]
struct AckStatus {
    have_received: bool,
    is_succeed: bool,
}

impl Default for AckStatus {
    fn default() -> Self {
        AckStatus {
            have_received: false,
            is_succeed: false,
        }
    }
}

#[derive(Clone)]
struct GotAcks {
    // (bool, bool) = (whether or not received response, whether or not succeed)
    chain: AckStatus,
    executor: AckStatus,
    auth: AckStatus,
    consensus: AckStatus,
    net: AckStatus,
}

impl Default for GotAcks {
    fn default() -> Self {
        let ack_status = AckStatus::default();

        GotAcks {
            chain: ack_status,
            executor: ack_status,
            auth: ack_status,
            consensus: ack_status,
            net: ack_status,
        }
    }
}

impl GotAcks {
    fn get_position(&self, ack: AckType) -> &AckStatus {
        match ack {
            AckType::Chain => &self.chain,
            AckType::Executor => &self.executor,
            AckType::Auth => &self.auth,
            AckType::Consensus => &self.consensus,
            AckType::Net => &self.net,
        }
    }
    fn get_mut_position(&mut self, ack: AckType) -> &mut AckStatus {
        match ack {
            AckType::Chain => &mut self.chain,
            AckType::Executor => &mut self.executor,
            AckType::Auth => &mut self.auth,
            AckType::Consensus => &mut self.consensus,
            AckType::Net => &mut self.net,
        }
    }
    // set ack with received msgs.
    pub fn set(&mut self, ack_type: AckType, is_succeed: bool) {
        let p = self.get_mut_position(ack_type);
        *p = AckStatus {
            have_received: true,
            is_succeed,
        };
    }

    // reset ack
    pub fn reset(&mut self, ack_type: AckType) {
        let p = self.get_mut_position(ack_type);
        *p = AckStatus::default();
    }

    // whether or not received response.
    pub fn get(&self, ack_type: AckType) -> bool {
        self.get_position(ack_type).have_received
    }

    // whether or not received response and the result is succeed.
    pub fn is_succeed(&self, ack_type: AckType) -> bool {
        let p = self.get_position(ack_type);
        p.have_received && p.is_succeed
    }
}

pub struct SnapShot {
    ctx_pub: Sender<(String, Vec<u8>)>,
    start_height: u64,
    end_height: u64,
    file: String,
    acks: GotAcks,
    proof: Proof,
    restore_height: u64,
}

impl SnapShot {
    pub fn new(
        ctx_pub: Sender<(String, Vec<u8>)>,
        start_height: u64,
        end_height: u64,
        file: String,
    ) -> Self {
        SnapShot {
            ctx_pub,
            start_height,
            end_height,
            file,
            acks: GotAcks::default(),
            proof: Proof::new(),
            restore_height: 0,
        }
    }

    // parse resp data
    pub fn parse_data(&mut self, key: &str, msg_vec: &[u8]) -> bool {
        let mut msg = Message::try_from(msg_vec).unwrap();

        let routing_key = RoutingKey::from(key);

        if routing_key.is_msg_type(MsgType::SnapshotResp) {
            self.parse_resp(&mut msg, routing_key)
        } else {
            error!("error MsgClass!!!!");
            false
        }
    }

    fn parse_resp(&mut self, msg: &mut Message, routing_key: RoutingKey) -> bool {
        let sub_module = routing_key.get_sub_module();

        let snapshot_resp = msg.take_snapshot_resp().unwrap();

        self.acks.set(sub_module.into(), snapshot_resp.flag);
        info!("snapshot_resp = {:?}", snapshot_resp);

        match snapshot_resp.resp {
            Resp::SnapshotAck => {
                info!("receive snapshot ack");
                self.acks.get(AckType::Chain) && self.acks.get(AckType::Executor)
            }
            Resp::BeginAck => {
                info!("receive begin ack");
                if self.acks.get(AckType::Auth)
                    && self.acks.get(AckType::Consensus)
                    && self.acks.get(AckType::Net)
                {
                    self.acks.reset(AckType::Auth);
                    self.acks.reset(AckType::Consensus);
                    self.acks.reset(AckType::Net);
                    self.restore();
                }

                false
            }
            Resp::RestoreAck => {
                info!("receive restore ack, sub_module = {:?}", sub_module);
                if sub_module == SubModules::Chain {
                    self.proof = snapshot_resp.get_proof().clone();
                    self.restore_height = snapshot_resp.get_height();
                }

                if self.acks.get(AckType::Chain) && self.acks.get(AckType::Executor) {
                    if !self.acks.is_succeed(AckType::Chain)
                        || !self.acks.is_succeed(AckType::Executor)
                    {
                        self.end();
                    } else {
                        self.clear();
                    }

                    self.acks.reset(AckType::Chain);
                    self.acks.reset(AckType::Executor);
                }

                false
            }
            Resp::ClearAck => {
                info!("receive clear ack");
                if self.acks.get(AckType::Auth)
                    && self.acks.get(AckType::Consensus)
                    && self.acks.get(AckType::Net)
                {
                    self.acks.reset(AckType::Auth);
                    self.acks.reset(AckType::Consensus);
                    self.acks.reset(AckType::Net);
                    self.end();
                }

                false
            }
            Resp::EndAck => {
                info!("receive restore end ack, restore end !");
                self.acks.get(AckType::Auth)
                    && self.acks.get(AckType::Consensus)
                    && self.acks.get(AckType::Net)
            }
        }
        //self.send_cmd(&snap_shot);
        //false
    }

    // 发送snapshot命令
    pub fn snapshot(&self) {
        let mut req = SnapshotReq::new();
        req.set_cmd(Cmd::Snapshot);
        req.set_start_height(self.start_height);
        req.set_end_height(self.end_height);
        req.set_file(self.file.clone());
        self.send_cmd(&req);
    }

    // send begin
    pub fn begin(&self) {
        let mut req = SnapshotReq::new();
        req.set_cmd(Cmd::Begin);
        self.send_cmd(&req);
    }

    // send clear
    pub fn clear(&self) {
        let mut req = SnapshotReq::new();
        req.set_cmd(Cmd::Clear);
        self.send_cmd(&req);
    }

    // send restore
    pub fn restore(&self) {
        let mut req = SnapshotReq::new();
        req.set_cmd(Cmd::Restore);
        req.set_file(self.file.clone());
        self.send_cmd(&req);
    }

    // send end
    pub fn end(&self) {
        thread::sleep(Duration::new(5, 0));
        let mut req = SnapshotReq::new();
        req.set_cmd(Cmd::End);
        req.set_proof(self.proof.clone());
        req.set_end_height(self.restore_height);
        self.send_cmd(&req);
    }

    pub fn send_cmd(&self, snapshot_req: &SnapshotReq) {
        info!("send cmd: {:?}", snapshot_req.cmd);
        let msg: Message = snapshot_req.clone().into();
        self.ctx_pub
            .send((
                routing_key!(Snapshot >> SnapshotReq).into(),
                (&msg).try_into().unwrap(),
            ))
            .unwrap();
    }
}
