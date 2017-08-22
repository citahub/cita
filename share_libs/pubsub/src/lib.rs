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

#[cfg(feature = "pubsub_rabbitmq")]
extern crate pubsub_rabbitmq;
#[cfg(feature = "pubsub_zeromq")]
extern crate pubsub_zeromq;
extern crate dotenv

use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use dotenv::dotenv;

#[cfg(feature = "pubsub_rabbitmq")]
use pubsub_rabbitmq::start_rabbitmq;

#[cfg(feature = "pubsub_zeromq")]
use pubsub_rabbitmq::start_rabbitmq;


#[cfg(feature = "pubsub_rabbitmq")]
pub fn start_pubsub(name: &str, keys: Vec<&str>, tx: Sender<(String, Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    dotenv().ok();
    start_rabbitmq(name, keys, tx, rx);
}

#[cfg(feature = "pubsub_zeromq")]
pub fn start_pubsub(name: &str, keys: Vec<&str>, tx: Sender<(String, Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    dotenv().ok();
    start_zeromq(name, keys, tx, rx);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::mpsc::channel;
    #[test]
    fn basics() {
        let (ntx_sub, nrx_sub) = channel();
        let (ntx_pub, nrx_pub) = channel();
        start_pubsub("network", vec!["chain.newtx", "chain.newblk"], ntx_sub, nrx_pub);

        let (ctx_sub, crx_sub) = channel();
        let (ctx_pub, crx_pub) = channel();
        start_pubsub("chain", vec!["network.newtx", "network.newblk"], ctx_sub, crx_pub);

        ntx_pub.send(("network.newtx".to_string(), vec![1])).unwrap();
        ntx_pub.send(("network.newblk".to_string(), vec![2])).unwrap();

        ctx_pub.send(("chain.newtx".to_string(), vec![3])).unwrap();
        ctx_pub.send(("chain.newblk".to_string(), vec![4])).unwrap();

        assert_eq!(crx_sub.recv().unwrap(), ("network.newtx".to_string(), vec![1]));
        assert_eq!(crx_sub.recv().unwrap(), ("network.newblk".to_string(), vec![2]));

        assert_eq!(nrx_sub.recv().unwrap(), ("chain.newtx".to_string(), vec![3]));
        assert_eq!(nrx_sub.recv().unwrap(), ("chain.newblk".to_string(), vec![4]));
    }
}
