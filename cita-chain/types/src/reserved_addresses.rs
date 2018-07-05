// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

//! Define all Reserved Addresses.
//!
//! # All
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff000000
//!     End  : 0xffffffffffffffffffffffffffffffffffffffff
//!
//! ## Action Address:
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff010000
//!     End  : 0xffffffffffffffffffffffffffffffffff01ffff
//!
//! ### Normal Action Address
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff010000
//!     End  : 0xffffffffffffffffffffffffffffffffff0100ff
//!
//! ### Go Action Address
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff018000
//!     End  : 0xffffffffffffffffffffffffffffffffff018fff
//!
//! ## System Contracts
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff020000
//!     End  : 0xffffffffffffffffffffffffffffffffff02ffff
//!
//! ### Normal System Contracts
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff020000
//!     End  : 0xffffffffffffffffffffffffffffffffff0200ff
//!
//! ### Permission System Contracts
//!
//!     Start: 0xffffffffffffffffffffffffffffffffff021000
//!     End  : 0xffffffffffffffffffffffffffffffffff0210ff
//!

// Normal Action Address
pub const STORE_ADDRESS: &str = "ffffffffffffffffffffffffffffffffff010000";
pub const ABI_ADDRESS: &str = "ffffffffffffffffffffffffffffffffff010001";
pub const AMEND_ADDRESS: &str = "ffffffffffffffffffffffffffffffffff010002";
// Go Action Address
pub const GO_CONTRACT: &str = "ffffffffffffffffffffffffffffffffff018000";
pub const GO_CONTRACT_MIN: &str = "ffffffffffffffffffffffffffffffffff018001";
pub const GO_CONTRACT_MAX: &str = "ffffffffffffffffffffffffffffffffff018fff";
