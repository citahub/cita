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

use amqp::{Consumer, Channel, protocol, Basic};
use libproto::{MsgClass, parse_msg, key_to_id};
use pubsub::Pub;
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use threadpool::ThreadPool;

pub type TransType = (u32, u32, MsgClass);
pub type PubType = (String, Vec<u8>);

pub struct MQWork {
    tobepub: Receiver<PubType>,
    mqpub: Rc<Pub>,
}

impl MQWork {
    pub fn new(rx: Receiver<PubType>, _pub: Rc<Pub>) -> Self {
        MQWork { tobepub: rx, mqpub: _pub }
    }

    pub fn send2pub(tx: &Sender<PubType>, info: PubType) {
        tx.send(info).unwrap();
    }

    pub fn start(&mut self) {
        let innerrx = &self.tobepub;
        loop {
            select! {
                infos = innerrx.recv() =>  {
                    let (rkey, content)= infos.unwrap();
                    trace!("***** to be publish***** {}",rkey);
                    let _pub = Rc::get_mut(&mut self.mqpub).unwrap();
                    _pub.publish(rkey.as_str(),content);
                }
            }
        }
    }
}

pub struct MyHandler {
    pool: ThreadPool,
    tx: Sender<TransType>,
}

impl MyHandler {
    pub fn new(pool: ThreadPool, tx: Sender<(u32, u32, MsgClass)>) -> Self {
        MyHandler { pool: pool, tx: tx }
    }

    pub fn receive(pool: &ThreadPool, tx: &Sender<(u32, u32, MsgClass)>, id: u32, msg: Vec<u8>) {
        let tx = tx.clone();
        pool.execute(move || {
                         let (cmd_id, _, content) = parse_msg(msg.as_slice());
                         tx.send((id, cmd_id, content)).unwrap();
                     });
    }
}

impl Consumer for MyHandler {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, _: protocol::basic::BasicProperties, body: Vec<u8>) {
        //trace!("************ handle delivery id {:?} {:?} ",deliver.routing_key,deliver.delivery_tag);
        MyHandler::receive(&self.pool, &self.tx, key_to_id(&deliver.routing_key), body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}
