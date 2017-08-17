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



use state_machine::StateMachine;
use std::fmt::{self, Debug};
use std::sync::mpsc;


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
        self.tx.send(command.to_vec()).map(|_| Vec::new()).unwrap_or(b"An error occured."[..].into())
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
