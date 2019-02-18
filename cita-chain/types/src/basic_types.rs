// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Ethcore basic typenames.

/// Type for a 2048-bit log-bloom, as used by our blocks.
pub use log_entry::LogBloom;

pub use log_blooms::LogBloomGroup;

/// Constant 2048-bit datum for 0. Often used as a default.
lazy_static! {
    pub static ref ZERO_LOGBLOOM: LogBloom = LogBloom::from([0x00; 256]);
}
