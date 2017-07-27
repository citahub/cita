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
