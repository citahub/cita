// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use libproto::blockchain::Proof;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::snapshot::{Cmd, Resp, SnapshotReq};
use libproto::{Message, TryFrom, TryInto};
use pubsub::channel::{Receiver, Sender};
use std::time::Duration;

enum Ack {
    Chain = 2,
    Executor = 3,
    Auth = 5,
    Consensus = 7,
    Net = 11,
}

impl From<SubModules> for Ack {
    fn from(sub_module: SubModules) -> Self {
        match sub_module {
            SubModules::Chain => Ack::Chain,
            SubModules::Executor => Ack::Executor,
            SubModules::Auth => Ack::Auth,
            SubModules::Consensus => Ack::Consensus,
            SubModules::Net => Ack::Net,
            _ => unreachable!(),
        }
    }
}

pub struct Postman {
    pub mq_req_receiver: Receiver<(String, Vec<u8>)>,
    pub mq_resp_sender: Sender<(String, Vec<u8>)>,
    pub command: String,
    pub start_height: u64,
    pub end_height: u64,
    pub file: String,
    pub proof: Proof,
    pub arrived: usize,
}

impl Postman {
    pub fn new(
        mq_req_receiver: Receiver<(String, Vec<u8>)>,
        mq_resp_sender: Sender<(String, Vec<u8>)>,
        command: String,
        start_height: u64,
        end_height: u64,
        file: String,
    ) -> Self {
        Self {
            mq_req_receiver,
            mq_resp_sender,
            command,
            start_height,
            end_height,
            file,
            arrived: 1,
            proof: Proof::new(),
        }
    }

    pub fn clear_message_bus(&self) {
        while self
            .mq_req_receiver
            .recv_timeout(Duration::new(2, 0))
            .is_ok()
        {}
    }

    pub fn serve(&mut self) -> Result<(), String> {
        match self.command.as_str() {
            "snapshot" => self.handle(Cmd::Snapshot, Resp::SnapshotAck),
            "restore" => Ok(())
                .and_then(|_| self.handle(Cmd::Begin, Resp::BeginAck))
                .and_then(|_| self.handle(Cmd::Restore, Resp::RestoreAck))
                .and_then(|_| self.handle(Cmd::Clear, Resp::ClearAck))
                .and_then(|_| self.handle(Cmd::End, Resp::EndAck)),
            _ => unreachable!(),
        }
    }

    fn recv(&self) -> (String, Vec<u8>) {
        self.mq_req_receiver
            .recv()
            .expect("listen message from message-bus")
    }

    // Sends specifc request and wait expected responses from all sub-modules
    fn handle(&mut self, cmd: Cmd, expected_ack: Resp) -> Result<(), String> {
        let mut req = SnapshotReq::new();
        req.set_cmd(cmd);
        req.set_start_height(self.start_height);
        req.set_end_height(self.end_height);
        req.set_file(self.file.clone());
        req.set_proof(self.proof.clone());

        self.send_request(req);
        self.wait(expected_ack)
    }

    fn wait(&mut self, expected_ack: Resp) -> Result<(), String> {
        // Wait until all sub-modules reponsed
        while !self.enough() {
            // 1. Listen messages from message-bus
            let (key, msg_vec) = self.recv();

            // 2. Assert message key
            let routing_key = RoutingKey::from(key);
            assert!(
                routing_key.is_msg_type(MsgType::SnapshotResp),
                "watch snapshot response only but get {}",
                routing_key,
            );

            // 3. Assert message body
            let mut message = Message::try_from(msg_vec).unwrap();
            let resp = message.take_snapshot_resp().unwrap();
            if resp.resp != expected_ack {
                warn!(
                    "receive unexpected ack, received({:?}) != expected({:?})",
                    resp.resp, expected_ack,
                );
                continue;
            }

            // 4. Return error if any
            let sub_module = routing_key.get_sub_module();
            info!("receive from {}, resp: {:?}", sub_module, resp,);
            if !resp.flag {
                return Err(format!("{} response error: {:?}", sub_module, resp,));
            }

            // 5. Update acks
            if sub_module == SubModules::Chain && resp.resp == Resp::RestoreAck {
                self.proof = resp.get_proof().clone();
                self.end_height = resp.get_height();
            }
            self.update(sub_module);
        }

        // Return Ok and reset counter after all sub-moduels responsed Ok
        self.reset();
        Ok(())
    }

    fn update(&mut self, sub_module: SubModules) {
        let ack: Ack = sub_module.into();
        let ack: usize = ack as usize;
        if self.arrived % ack != 0 {
            // avoid received duplicated responses
            self.arrived *= ack;
        }
    }

    fn reset(&mut self) {
        self.arrived = 1;
    }

    fn enough(&self) -> bool {
        let expeted = Ack::Chain as usize
            * Ack::Executor as usize
            * Ack::Auth as usize
            * Ack::Consensus as usize
            * Ack::Net as usize;
        expeted == self.arrived
    }

    fn send_request(&self, req: SnapshotReq) {
        info!("send Snapshot::{:?}", req.cmd);
        let message: Message = req.into();
        self.mq_resp_sender
            .send((
                routing_key!(Snapshot >> SnapshotReq).into(),
                (&message).try_into().unwrap(),
            ))
            .unwrap();
    }
}
