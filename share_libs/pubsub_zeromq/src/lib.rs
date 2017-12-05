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

#[macro_use]
extern crate log;
extern crate zmq;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
//use util::log::error;
pub fn start_zeromq(name: &str, keys: Vec<&str>, tx: Sender<(String, Vec<u8>)>, rx: Receiver<(String, Vec<u8>)>) {
    let context = zmq::Context::new();
    //pub
    let publisher = context.socket(zmq::PUB).unwrap();
    match name {
        "network" => assert!(publisher.bind("tcp://*:5563").is_ok()),
        "chain" => assert!(publisher.bind("tcp://*:5564").is_ok()),
        "jsonrpc" => assert!(publisher.bind("tcp://*:5565").is_ok()),
        "consensus" => assert!(publisher.bind("tcp://*:5566").is_ok()),
        _ => error!("not hava {} module !", name),
    }

    let _ = thread::Builder::new()
        .name("publisher".to_string())
        .spawn(move || {
            loop {
                let ret = rx.recv();

                if ret.is_err() {
                    break;
                }
                let (topic, msg) = ret.unwrap();
                publisher
                    .send_multipart(&[&(topic.into_bytes())], zmq::SNDMORE)
                    .unwrap();
                publisher.send(&msg, 0).unwrap();
            }
        });


    //sub

    let network_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(network_subscriber.connect("tcp://localhost:5563").is_ok());

    let chain_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(chain_subscriber.connect("tcp://localhost:5564").is_ok());

    let jsonrpc_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(jsonrpc_subscriber.connect("tcp://localhost:5565").is_ok());

    let consensus_subscriber = context.socket(zmq::SUB).unwrap();
    assert!(network_subscriber.connect("tcp://localhost:5566").is_ok());

    let mut flag = 400;
    for topic in keys {
        flag = match name {
            "network" => {
                network_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                0
            }
            "chain" => {
                chain_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                1
            }
            "jsonrpc" => {
                jsonrpc_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                2
            }
            "consensus" => {
                consensus_subscriber
                    .set_subscribe(&topic.to_string().into_bytes())
                    .unwrap();
                3
            }
            _ => {
                error!("invalid  flag!");
                -1
            }
        }
    }

    let _ = thread::Builder::new()
        .name("subscriber".to_string())
        .spawn(move || {
            loop {
                match flag {
                    0 => {
                        let topic = network_subscriber.recv_string(0).unwrap().unwrap();
                        let msg = network_subscriber.recv_bytes(0).unwrap();
                        let _ = tx.send((topic, msg));
                    }

                    1 => {
                        let topic = chain_subscriber.recv_string(0).unwrap().unwrap();
                        let msg = chain_subscriber.recv_bytes(0).unwrap();
                        let _ = tx.send((topic, msg));
                    }

                    2 => {
                        let topic = jsonrpc_subscriber.recv_string(0).unwrap().unwrap();
                        let msg = jsonrpc_subscriber.recv_bytes(0).unwrap();
                        let _ = tx.send((topic, msg));
                    }

                    3 => {
                        let topic = consensus_subscriber.recv_string(0).unwrap().unwrap();
                        let msg = consensus_subscriber.recv_bytes(0).unwrap();
                        let _ = tx.send((topic, msg));
                    }

                    _ => {
                        break;
                    }
                }
            }
        });
}
