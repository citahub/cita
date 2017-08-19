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

pub mod wal;
pub mod votetime;
pub mod tendermint;
pub mod params;
pub mod voteset;
pub mod spec;
pub mod dispatchtx;
pub mod txwal;
pub mod txhandler;

pub use self::params::*;
pub use self::spec::*;
pub use self::tendermint::*;
pub use self::voteset::*;

pub use libproto::blockchain::{BlockHeader, Block, Transaction, BlockBody, Proof, Status};

use util::Address;
use util::H256;

pub trait BareHash {
    fn bare_hash(&self) -> H256;
}
