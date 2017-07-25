use std::fmt::{self, Debug};
use std::sync::mpsc;

use state_machine::StateMachine;


/// A state machine that simply redirects all commands to a channel.
///
/// This state machine is chiefly meant for testing.
pub struct ChannelStateMachine {
    tx: mpsc::Sender<Vec<u8>>,
}

impl ChannelStateMachine {
    pub fn new() -> (ChannelStateMachine, mpsc::Receiver<Vec<u8>>) {
        let (tx, recv) = mpsc::channel();
        (ChannelStateMachine { tx: tx }, recv)
    }
}

impl StateMachine for ChannelStateMachine {
    fn apply(&mut self, command: &[u8]) -> Vec<u8> {
        self.tx
            .send(command.to_vec())
            .map(|_| Vec::new())
            .unwrap_or(b"An error occured."[..].into())
    }

    fn query(&self, _query: &[u8]) -> Vec<u8> {
        unimplemented!()
    }

    fn snapshot(&self) -> Vec<u8> {
        Vec::new()
    }

    fn restore_snapshot(&mut self, _snapshot: Vec<u8>) -> () {
        ()
    }
}

impl Debug for ChannelStateMachine {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "ChannelStateMachine")
    }
}
