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
use libproto::blockchain::SignedTransaction;
use libproto::communication::{Message, MsgType};
use libproto::key_to_id;
use protobuf::core::parse_from_bytes;
use std::sync::mpsc::Sender;
use threadpool::ThreadPool;
use util::snappy;

pub type TransType = (u32, bool, SignedTransaction);

pub struct TxHandler {
    pool: ThreadPool,
    tx: Sender<TransType>,
}

impl TxHandler {
    pub fn new(pool: ThreadPool, tx: Sender<TransType>) -> Self {
        TxHandler { pool: pool, tx: tx }
    }

    pub fn receive(pool: &ThreadPool, tx: &Sender<TransType>, id: u32, msg: Vec<u8>) {
        let tx = tx.clone();
        pool.execute(move || {
            let mut msg = parse_from_bytes::<Message>(msg.as_ref()).unwrap();
            let content_msg = msg.take_content();
            let content_msg = snappy::cita_decompress(content_msg);
            match msg.get_field_type() {
                MsgType::TX => {
                    let mut trans = parse_from_bytes::<SignedTransaction>(&content_msg).unwrap();
                    let ret = trans.recover();
                    tx.send((id, ret, trans)).unwrap();
                }
                _ => info!("recv msg type[{:?}] error", msg.get_field_type()),
            };
        });
    }
}

impl Consumer for TxHandler {
    fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, _: protocol::basic::BasicProperties, body: Vec<u8>) {
        //trace!("************ handle delivery id {:?} {:?} ",deliver.routing_key,deliver.delivery_tag);
        TxHandler::receive(&self.pool, &self.tx, key_to_id(&deliver.routing_key), body);
        let _ = channel.basic_ack(deliver.delivery_tag, false);
    }
}
