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

use bincode::{serialize, deserialize, Infinite};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Command {
    SpawnBlk(u64, Vec<u8>),
    PoolSituation(u64, Option<Vec<u8>>, Option<Vec<u8>>),
}

pub fn encode(cmd: &Command) -> Vec<u8> {
    serialize(cmd, Infinite).unwrap()
}

pub fn decode(bin: &[u8]) -> Command {
    deserialize(bin).unwrap()
}
