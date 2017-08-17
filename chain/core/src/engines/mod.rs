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

use builtin::Builtin;
use native;
use std::collections::{BTreeMap, HashMap};
use util::{Address, U256, BytesRef};

pub trait Engine: Sync + Send {
    /// The name of this engine.
    fn name(&self) -> &str;

    /// Builtin-contracts we would like to see in the chain.
    /// (In principle these are just hints for the engine since that has the last word on them.)
    fn builtins(&self) -> &BTreeMap<Address, Builtin>;

    // TODO: builtin contract routing - to do this properly, it will require removing the built-in configuration-reading logic
    // from Spec into here and removing the Spec::builtins field.
    /// Determine whether a particular address is a builtin contract.
    fn is_builtin(&self, a: &Address) -> bool {
        self.builtins().contains_key(a)
    }
    /// Determine the code execution cost of the builtin contract with address `a`.
    /// Panics if `is_builtin(a)` is not true.
    fn cost_of_builtin(&self, a: &Address, input: &[u8]) -> U256 {
        self.builtins().get(a).expect("queried cost of nonexistent builtin").cost(input.len())
    }
    /// Execution the builtin contract `a` on `input` and return `output`.
    /// Panics if `is_builtin(a)` is not true.
    fn execute_builtin(&self, a: &Address, input: &[u8], output: &mut BytesRef) {
        self.builtins().get(a).expect("attempted to execute nonexistent builtin").execute(input, output);
    }
    fn register(&mut self, addr: Address, contract: Box<native::Contract>);
    fn unregister(&mut self, addr: Address) -> Option<Box<native::Contract>>;
    fn get_native_contract(&self, addr: &Address) -> Option<&Box<native::Contract>>;
}

/// An engine which does not provide any consensus mechanism and does not seal blocks.
pub struct NullEngine {
    builtins: BTreeMap<Address, Builtin>,
    contracts: HashMap<Address, Box<native::Contract>>,
}

impl NullEngine {
    /// Returns new instance of NullEngine with default VM Factory
    pub fn new(builtins: BTreeMap<Address, Builtin>) -> Self {
        let mut engine = NullEngine {
            builtins: builtins,
            contracts: HashMap::new(),
        };
        engine.register(Address::from(0x400), Box::new(native::NowPay::new()));
        engine
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

    fn register(&mut self, addr: Address, contract: Box<native::Contract>) {
        self.contracts.insert(addr, contract);
    }

    fn unregister(&mut self, addr: Address) -> Option<Box<native::Contract>> {
        self.contracts.remove(&addr)
    }

    fn get_native_contract(&self, addr: &Address) -> Option<&Box<native::Contract>> {
        self.contracts.get(addr)
    }
}
