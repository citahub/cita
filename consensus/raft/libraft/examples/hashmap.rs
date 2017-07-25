//! This example demonstrates using Raft to implement a replicated hashmap over `n` servers and
//! interact with them over `m` clients.
//!
//! This example uses Serde serialization.
//!
//! Comments below will aim to be tailored towards Raft and it's usage. If you have any questions,
//! Please, just open an issue.
//!
//! TODO: For the sake of simplicity of this example, we don't implement a `Log` and just use a
//! simple testing one. We should improve this in the future.

// In order to use Serde we need to enable these nightly features.
#![feature(plugin)]
#![feature(custom_derive)]

extern crate libraft; // <--- Kind of a big deal for this!
extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate scoped_log;
extern crate docopt;
extern crate serde_json;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::net::{SocketAddr, ToSocketAddrs};
use std::collections::HashMap;

use serde_json::Value;
use docopt::Docopt;

// Raft's major components. See comments in code on usage and things.
use libraft::{
    Server,
    Client,
    state_machine,
    persistent_log,
    ServerId,
};
// A payload datatype. We're just using a simple enum. You can use whatever.
use Message::*;

// Using docopt we define the overall usage of the application.
static USAGE: &'static str = "
A replicated mutable hashmap. Operations on the register have serializable
consistency, but no durability (once all register servers are terminated the
map is lost).

Each register server holds a replica of the map, and coordinates with its
peers to update the maps values according to client commands. The register
is available for reading and writing only if a majority of register servers are
available.


Commands:

  get     Returns the current value of the key.

  put     Sets the current value of the key, and returns the previous
          value.

  cas     (compare and set) Conditionally sets the value of the key if the
          current value matches an expected value, returning true if the
          key was set.

  server  Starts a key server. Servers must be provided a unique ID and
          address (ip:port) at startup, along with the ID and address of all
          peer servers.

Usage:
  hashmap get <key> (<node-address>)...
  hashmap put <key> <new-value> (<node-address>)...
  hashmap cas <key> <expected-value> <new-value> (<node-address>)...
  hashmap server <id> (<node-address>)...
  hashmap (-h | --help)

Options:
  -h --help   Show a help message.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_server: bool,
    cmd_get: bool,
    cmd_put: bool,
    cmd_cas: bool,

    // When creating a server you will necessarily need some sort of unique ID for it as well
    // as a list of peers. In this example we just accept them straight from args. You might
    // find it best to use a `toml` or `yaml` or `json` file.
    arg_id: Option<u64>,
    arg_node_id: Vec<u64>,
    arg_node_address: Vec<String>,

    // In this example keys and values are associated. In your application you can model your data
    // however you please.
    arg_key: String,
    arg_new_value: String,
    arg_expected_value: String,
}

/// This is the defined message type for this example. For the sake of simplicity we don't go very
/// far with this. In a "real" application you may want to more distinctly distinguish between
/// data meant for `.query()` and data meant for `.propose()`.
#[derive(Serialize, Deserialize)]
pub enum Message {
    Get(String),
    Put(String, Value),
    Cas(String, Value, Value),
}

/// Just a plain old boring "parse args and dispatch" call.
fn main() {
    let _ = env_logger::init();
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if args.cmd_server {
        server(&args);
    } else if args.cmd_get {
        get(&args);
    } else if args.cmd_put {
        put(&args);
    } else if args.cmd_cas {
        cas(&args);
    }
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
fn server(args: &Args) {
    // Creating a raft server requires several things:

    // A persistent log implementation, which manages the persistent, replicated log...
    let persistent_log = persistent_log::MemLog::new();

    // A state machine which replicates state. This state should be the same on all nodes.
    let state_machine = HashmapStateMachine::new();

    // As well as a unique server id.
    let id = ServerId::from(args.arg_id.unwrap());
    println!("id:{:?}", id);
    println!("node_id:{:?}, node_address:{:?}", args.arg_node_id, args.arg_node_address);
    let mut node_id: Vec<u64> = vec![];
    for i in 0..args.arg_node_address.len() {
        node_id.push(i as u64 + 1);
    }
    // ...  And a list of peers.
    let mut peers = node_id
                    .iter()
                    .zip(args.arg_node_address.iter())
                    .map(|(&id, addr)| (ServerId::from(id), parse_addr(&addr)))
                    .collect::<HashMap<_,_>>();
    println!("peers:{:?}", peers);

    // The Raft Server will return an error if its ID is inside of its peer set. Don't do that.
    // Instead, take it out and use it!
    let addr = peers.remove(&id).unwrap();

    println!("addr:{:?}", addr);
    // Using all of the above components.
    // You probably shouldn't `.unwrap()` in production code unless you're totally sure it works
    // 100% of the time, all the time.
    Server::run(id, addr, peers, persistent_log, state_machine).unwrap();
}

/// Gets a value for a given key from the provided Raft cluster.
fn get(args: &Args) {
    // Clients necessarily need to now the valid set of nodes which they can talk to.
    // This is both so they can try to talk to all the nodes if some are failing, and so that it
    // can verify that it's not being lead astray somehow in redirections on leadership changes.
    let cluster = args.arg_node_address.iter()
        .map(|v| parse_addr(&v))
        .collect();

    // Clients can be stored and reused, or used once and discarded.
    // There is very small overhead in connecting a new client to a cluster as it must discover and
    // identify itself to the leader.
    let mut client = Client::new(cluster);

    // In this example `serde::json` is used to serialize and deserialize messages.
    // Since Raft accepts `[u8]` the way you structure your data, the serialization method you
    // choose, and how you interpret that data is entirely up to you.
    let payload = serde_json::to_string(&Message::Get(args.arg_key.clone())).unwrap();

    // A query executes **immutably** on the leader of the cluster and does not pass through the
    // persistent log. This is intended for querying the current state of the state machine.
    let response = client.query(payload.as_bytes()).unwrap();

    // A response will block until it's query is complete. This is intended and expected behavior
    // based on the papers specifications.
    println!("{}", String::from_utf8(response).unwrap())
}

/// Sets a value for a given key in the provided Raft cluster.
fn put(args: &Args) {
    // Same as above.
    let cluster = args.arg_node_address.iter()
        .map(|v| parse_addr(&v))
        .collect();

    let mut client = Client::new(cluster);

    let new_value = serde_json::to_value(&args.arg_new_value).unwrap();
    let payload = serde_json::to_string(&Message::Put(args.arg_key.clone(), new_value)).unwrap();

    // A propose will go through the persistent log and mutably modify the state machine in some
    // way. This is **much** slower than `.query()`.
    let response = client.propose(payload.as_bytes()).unwrap();

    // A response will block until it's proposal is complete. This is intended and expected behavior
    // based on the papers specifications.
    println!("{}", String::from_utf8(response).unwrap())
}

/// Compares and sets a value for a given key in the provided Raft cluster if the value is what is
/// expected.
fn cas(args: &Args) {
    // Same as above.
    let cluster = args.arg_node_address.iter()
        .map(|v| parse_addr(&v))
        .collect();

    let mut client = Client::new(cluster);

    let new_value = serde_json::to_value(&args.arg_new_value).unwrap();
    let expected_value = serde_json::to_value(&args.arg_expected_value).unwrap();
    let payload = serde_json::to_string(&Message::Cas(args.arg_key.clone(), expected_value, new_value)).unwrap();

    let response = client.propose(payload.as_bytes()).unwrap();

    println!("{}", String::from_utf8(response).unwrap())
}

/// A state machine that holds a hashmap.
#[derive(Debug)]
pub struct HashmapStateMachine {
    map: HashMap<String, Value>,
}

/// Implement anything you want... A `new()` is generally a great idea.
impl HashmapStateMachine {
    pub fn new() -> HashmapStateMachine {
        HashmapStateMachine {
            map: HashMap::new(),
        }
    }
}

/// Implementing `state_machine::StateMachine` allows your application specific state machine to be
/// used in Raft. Feel encouraged to base yours of one of ours in these examples.
impl state_machine::StateMachine for HashmapStateMachine {

    /// `apply()` is called on when a client's `.propose()` is commited and reaches the state
    /// machine. At this point it is durable and is going to be applied on at least half the nodes
    /// within the next couple round trips.
    fn apply(&mut self, new_value: &[u8]) -> Vec<u8> {
        scoped_info!("Applying {:?}", String::from_utf8_lossy(new_value));
        // Deserialize
        let string = String::from_utf8_lossy(new_value);
        let message = serde_json::from_str(&string).unwrap();

        // Handle
        let response = match message {
            Get(key) => {
                let old_value = &self.map.get(&key).map(|v| v.clone());
                serde_json::to_string(old_value)
            },
            Put(key, value) => {
                let old_value = &self.map.insert(key, value);
                serde_json::to_string(old_value)
            },
            Cas(key, old_check, new) => {
                if *self.map.get(&key).unwrap() == old_check {
                    let _ = self.map.insert(key, new);
                    serde_json::to_string(&true)
                } else {
                    serde_json::to_string(&false)
                }
            },
        };

        // Respond.
        response.unwrap().into_bytes()
    }

    /// `query()` is called on when a client's `.query()` is recieved. It does not go through the
    /// persistent log, it does not mutate the state of the state machine, and it is intended to be
    /// fast.
    fn query(&self, query: &[u8]) -> Vec<u8> {
        scoped_info!("Querying {:?}", String::from_utf8_lossy(query));
        // Deserialize
        let string = String::from_utf8_lossy(query);
        let message = serde_json::from_str(&string).unwrap();

        // Handle
        let response = match message {
            Get(key) => {
                let old_value = &self.map.get(&key).map(|v| v.clone());
                serde_json::to_string(old_value)
            },
            _ => panic!("Can't do mutating requests in query"),
        };

        // Respond.
        response.unwrap().into_bytes()
    }

    fn snapshot(&self) -> Vec<u8> {
        serde_json::to_string(&self.map)
            .unwrap()
            .into_bytes()
    }

    fn restore_snapshot(&mut self, snapshot_value: Vec<u8>) {
        self.map = serde_json::from_str(&String::from_utf8_lossy(&snapshot_value)).unwrap();
        ()
    }
}
