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



// Raft's major components. See comments in code on usage and things.

use libraft::{Server, ServerId};
use log_store::*;
// A payload datatype. We're just using a simple enum. You can use whatever.
use machine::*;
use mio::EventLoop;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};

#[derive(Debug, RustcDecodable)]
pub struct Args {
    cmd_server: bool,

    // When creating a server you will necessarily need some sort of unique ID for it as well
    // as a list of peers. In this example we just accept them straight from args. You might
    // find it best to use a `toml` or `yaml` or `json` file.
    arg_id: Option<u64>,
    arg_node_address: Vec<String>,
}

/// A simple convenience method since this is an example and it should exit if given invalid params.
fn parse_addr(addr: &str) -> SocketAddr {
    addr.to_socket_addrs()
        .ok()
        .expect(&format!("unable to parse socket address: {}", addr))
        .next()
        .unwrap()
}

/// Creates a Raft server using the specified ID from the list of nodes.
pub fn server(args: &Args) -> (Server<Store, HashmapStateMachine>, EventLoop<Server<Store, HashmapStateMachine>>) {
    // Creating a raft server requires several things:

    // A persistent log implementation, which manages the persistent, replicated log...
    let persistent_log = Store::new();

    // A state machine which replicates state. This state should be the same on all nodes.
    let state_machine = HashmapStateMachine::new();

    // As well as a unique server id.
    let id = ServerId::from(args.arg_id.unwrap() + 1);
    println!("id:{:?}", id);
    let mut node_id: Vec<u64> = vec![];
    for i in 0..args.arg_node_address.len() {
        node_id.push(i as u64 + 1);
    }
    // ...  And a list of peers.
    let mut peers = node_id.iter()
                           .zip(args.arg_node_address.iter())
                           .map(|(&id, addr)| (ServerId::from(id), parse_addr(&addr)))
                           .collect::<HashMap<_, _>>();
    println!("peers:{:?}", peers);

    // The Raft Server will return an error if its ID is inside of its peer set. Don't do that.
    // Instead, take it out and use it!
    let addr = peers.remove(&id).unwrap();

    println!("addr:{:?}", addr);
    // Using all of the above components.
    // You probably shouldn't `.unwrap()` in production code unless you're totally sure it works
    // 100% of the time, all the time.
    //Server::spawn(id, addr, peers, persistent_log, state_machine).unwrap();
    Server::new(id, addr, peers, persistent_log, state_machine).unwrap()
}
