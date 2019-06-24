// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use crate::builtin::Builtin;
use crate::engines::Engine;
use cita_types::Address;
use std::collections::BTreeMap;

/// An engine which does not provide any consensus mechanism and does not seal blocks.
pub struct NullEngine {
    builtins: BTreeMap<Address, Builtin>,
}

impl NullEngine {
    /// Returns new instance of NullEngine with default VM Factory
    pub fn new(builtins: BTreeMap<Address, Builtin>) -> Self {
        NullEngine { builtins }
    }
}

impl Default for NullEngine {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Engine for NullEngine {
    fn name(&self) -> &str {
        "NullEngine"
    }

    fn builtins(&self) -> &BTreeMap<Address, Builtin> {
        &self.builtins
    }
}
