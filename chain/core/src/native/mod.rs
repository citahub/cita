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

use action_params::ActionParams;
use evm::{self, Ext, GasLeft};
use std::collections::HashMap;
use util::Address;

////////////////////////////////////////////////////////////////////////////////
pub mod storage;
pub type Signature = u32;

////////////////////////////////////////////////////////////////////////////////
pub trait ContractClone {
    fn clone_box(&self) -> Box<Contract>;
}

impl<T> ContractClone for T
where
    T: 'static + Contract + Clone,
{
    fn clone_box(&self) -> Box<Contract> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Contract> {
    fn clone(&self) -> Box<Contract> {
        self.clone_box()
    }
}

// Contract
pub trait Contract: Sync + Send + ContractClone {
    fn exec(&mut self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error>;
    fn create(&self) -> Box<Contract>;
}


////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Factory {
    contracts: HashMap<Address, Box<Contract>>,
}


impl Factory {
    pub fn new_contract(&self, address: Address) -> Option<Box<Contract>> {
        if let Some(contract) = self.contracts.get(&address) { Some(contract.create()) } else { None }
    }
    pub fn register(&mut self, address: Address, contract: Box<Contract>) {
        self.contracts.insert(address, contract);
    }
    pub fn unregister(&mut self, address: Address) {
        self.contracts.remove(&address);
    }
}

impl Default for Factory {
    fn default() -> Self {
        let factory = Factory { contracts: HashMap::new() };
        // here we register contracts with addresses defined in genesis.json.
        factory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::storage::{Scalar, Array, Map};
    use byteorder::{BigEndian, ByteOrder};
    use evm::tests::FakeExt;
    use util::{H256, U256, Address};

    ////////////////////////////////////////////////////////////////////////////////
    // NowPay
    #[derive(Clone)]
    pub struct NowPay {
        output: Vec<u8>,
        scalar_string: Scalar,
        array_u256: Array,
        map_u256: Map,
    }

    impl Contract for NowPay {
        fn exec(&mut self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
            let signature = BigEndian::read_u32(params.clone().data.unwrap().get(0..4).unwrap());
            match signature {
                0 => self.init(params, ext),
                1 => self.get(params, ext),
                _ => Err(evm::Error::OutOfGas),
            }
        }
        fn create(&self) -> Box<Contract> {
            Box::new(NowPay::default())
        }
    }

    impl Default for NowPay {
        fn default() -> Self {
            NowPay {
                output: Vec::new(),
                scalar_string: Scalar::new(H256::from(0)),
                array_u256: Array::new(H256::from(1)),
                map_u256: Map::new(H256::from(2)),
            }
        }
    }
    impl NowPay {
        fn get(&mut self, _params: ActionParams, _ext: &mut Ext) -> Result<GasLeft, evm::Error> {
            self.output.push(8);
            Ok(GasLeft::NeedsReturn(U256::from(100), self.output.as_slice()))
        }
        fn init(&mut self, _params: ActionParams, _ext: &mut Ext) -> Result<GasLeft, evm::Error> {
            Ok(GasLeft::Known(U256::from(100)))
        }
    }

    #[test]
    fn test_native_contract() {
        let mut factory = Factory::default();
        factory.register(Address::from(0), Box::new(NowPay::default()));

        let mut params = ActionParams::default();
        let input = vec![0, 0, 0, 1, 1, 2, 3, 4];
        params.data = Some(input);
        let mut ext = FakeExt::new();
        let mut contract = factory.new_contract(Address::from(0)).unwrap();
        let _ = contract.exec(params, &mut ext).unwrap();
    }
}
