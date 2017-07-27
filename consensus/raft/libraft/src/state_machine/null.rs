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

/// A state machine with no states.
#[derive(Debug)]
pub struct NullStateMachine;

impl StateMachine for NullStateMachine {
    fn apply(&mut self, _command: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    fn query(&self, _query: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    fn snapshot(&self) -> Vec<u8> {
        Vec::new()
    }

    fn restore_snapshot(&mut self, _snapshot: Vec<u8>) {
        ()
    }
}
