use std::collections::HashMap;

use serde_json::Value;
use serde_json;
// Raft's major components. See comments in code on usage and things.
use libraft::state_machine;
use self::Message::*;

/// This is the defined message type for this example. For the sake of simplicity we don't go very
/// far with this. In a "real" application you may want to more distinctly distinguish between
/// data meant for `.query()` and data meant for `.propose()`.
#[derive(Serialize, Deserialize)]
pub enum Message {
    Get(String),
    Put(String, Value),
    Cas(String, Value, Value),
}

/// A state machine that holds a hashmap.
#[derive(Debug)]
pub struct HashmapStateMachine {
    map: HashMap<String, Value>,
}

/// Implement anything you want... A `new()` is generally a great idea.
impl HashmapStateMachine {
    pub fn new() -> HashmapStateMachine {
        HashmapStateMachine { map: HashMap::new() }
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
            }
            Put(key, value) => {
                let old_value = &self.map.insert(key, value);
                serde_json::to_string(old_value)
            }
            Cas(key, old_check, new) => {
                if *self.map.get(&key).unwrap() == old_check {
                    let _ = self.map.insert(key, new);
                    serde_json::to_string(&true)
                } else {
                    serde_json::to_string(&false)
                }
            }
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
            }
            _ => panic!("Can't do mutating requests in query"),
        };

        // Respond.
        response.unwrap().into_bytes()
    }

    fn snapshot(&self) -> Vec<u8> {
        serde_json::to_string(&self.map).unwrap().into_bytes()
    }

    fn restore_snapshot(&mut self, snapshot_value: Vec<u8>) {
        self.map = serde_json::from_str(&String::from_utf8_lossy(&snapshot_value)).unwrap();
        ()
    }
}
