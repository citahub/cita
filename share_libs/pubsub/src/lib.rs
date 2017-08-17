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

extern crate amqp;
extern crate util;
use amqp::{Basic, Session, Consumer, Channel, Table, protocol};
use std::thread;
pub struct PubSub {
    session: Session,
    channel_no: u16,
}

pub const AMQP_URL: &'static str = "AMQP_URL";
impl PubSub {
    pub fn new() -> Self {
        let amqp_url = std::env::var(AMQP_URL).expect(format!("{} must be set", AMQP_URL).as_str());
        let session = match Session::open_url(&amqp_url) {
            Ok(session) => session,
            Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
        };
        PubSub { session: session, channel_no: 1 }
    }

    pub fn start_sub<T>(&mut self, name: &str, keys: Vec<&str>, callback: T)
    where
        T: Consumer + 'static,
    {
        let mut channel = self.session.open_channel(self.channel_no).ok().expect("Can't open channel");
        let _ = channel.basic_prefetch(10);
        self.channel_no += 1;
        channel.exchange_declare("cita", "topic", false, true, false, false, false, Table::new()).unwrap();

        //queue: &str, passive: bool, durable: bool, exclusive: bool, auto_delete: bool, nowait: bool, arguments: Table
        channel.queue_declare(name.clone(), false, true, false, false, false, Table::new()).unwrap();

        for key in keys {
            channel.queue_bind(name.clone(), "cita", key, false, Table::new()).unwrap();
        }
        //queue: &str, consumer_tag: &str, no_local: bool, no_ack: bool, exclusive: bool, nowait: bool, arguments: Table
        channel.basic_consume(callback, name.clone(), "", false, false, false, false, Table::new()).unwrap();

        thread::spawn(move || {
                          channel.start_consuming();
                          let _ = channel.close(200, "Bye");
                      });
    }

    pub fn get_pub(&mut self) -> Pub {
        let _pub = Pub::new(self.channel_no);
        self.channel_no += 1;
        _pub
    }
}


pub struct Pub {
    channel: Channel,
}

impl Pub {
    pub fn new(id: u16) -> Self {
        let amqp_url = std::env::var(AMQP_URL).expect(format!("{} must be set", AMQP_URL).as_str());
        let mut session = match Session::open_url(&amqp_url) {
            Ok(session) => session,
            Err(error) => panic!("failed to open url {} : {:?}", amqp_url, error),
        };
        let mut channel = session.open_channel(id).ok().expect("Can't open channel");

        let _ = channel.basic_prefetch(10);

        Pub { channel: channel }
    }

    pub fn publish(&mut self, routing_key: &str, msg: Vec<u8>) {
        let _ = self.channel.basic_publish("cita",
                                           routing_key,
                                           false,
                                           false,
                                           protocol::basic::BasicProperties {
                                               content_type: Some("text".to_string()),
                                               ..Default::default()
                                           },
                                           msg);
    }
}

impl Drop for Pub {
    fn drop(&mut self) {
        let _ = self.channel.close(200, "Bye");
    }
}

#[cfg(test)]
mod test {
    extern crate amqp;
    extern crate dotenv;
    use super::PubSub;
    use amqp::{Consumer, Channel, protocol, Basic};

    struct MyHandler {
        count: u32,
    }

    impl MyHandler {
        pub fn new() -> Self {
            MyHandler { count: 0 }
        }
    }

    impl Consumer for MyHandler {
        fn handle_delivery(&mut self, channel: &mut Channel, deliver: protocol::basic::Deliver, _: protocol::basic::BasicProperties, body: Vec<u8>) {
            self.count += 1;
            println!("get msg {:?} id {:?} payload {:?}", self.count, deliver.routing_key, body);
            let _ = channel.basic_ack(deliver.delivery_tag, false);
        }
    }

    #[test]
    fn basics() {
        println!("pubsub test begin!");
        dotenv::dotenv().ok();
        let mut net_pubsub = PubSub::new();
        let mut chain_pubsub = PubSub::new();
        net_pubsub.start_sub("network", vec!["chain.newtx", "chain.newblk"], MyHandler::new());
        chain_pubsub.start_sub("chain", vec!["network.newtx", "network.newblk"], MyHandler::new());

        let mut net_pub = net_pubsub.get_pub();
        let mut chain_pub = chain_pubsub.get_pub();
        net_pub.publish("network.newtx", vec![1]);
        net_pub.publish("network.newblk", vec![2]);
        chain_pub.publish("chain.newtx", vec![3]);
        chain_pub.publish("chain.newblk", vec![4]);
    }
}
