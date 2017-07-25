use std::sync::mpsc::Receiver;
use libproto::TopicMessage;
use protobuf::Message;
use pubsub::Pub;

pub struct Publisher {
    rx: Receiver<TopicMessage>,
}

impl Publisher {
    pub fn new(rx: Receiver<TopicMessage>) -> Self {
        Publisher { rx: rx }
    }

    pub fn run(&self, _pub: &mut Pub) {
        loop {
            let msg = self.rx.recv().unwrap();
            _pub.publish(&msg.0, msg.1.write_to_bytes().unwrap());
        }
    }
}
