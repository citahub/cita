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

use cita_types::Address;
use evm::action_params::ActionParams;
use evm::{self, Ext, GasLeft};
use std::collections::HashMap;
use std::str::FromStr;
use types::reserved_addresses;

////////////////////////////////////////////////////////////////////////////////
pub type Signature = u32;
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
    fn exec(&mut self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error>;
    fn create(&self) -> Box<Contract>;
}

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct Factory {
    contracts: HashMap<Address, Box<Contract>>,
}

impl Factory {
    pub fn new_contract(&self, address: Address) -> Option<Box<Contract>> {
        if let Some(contract) = self.contracts.get(&address) {
            Some(contract.create())
        } else {
            None
        }
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
        let mut factory = Factory {
            contracts: HashMap::new(),
        };
        // here we register contracts with addresses defined in genesis.json.
        {
            use super::crosschain_verify::CrossChainVerify;
            factory.register(
                Address::from_str(reserved_addresses::NATIVE_CROSS_CHAIN_VERIFY).unwrap(),
                Box::new(CrossChainVerify::default()),
            );
        }
        #[cfg(test)]
        {
            use super::storage::SimpleStorage;
            factory.register(
                Address::from_str(reserved_addresses::NATIVE_SIMPLE_STORAGE).unwrap(),
                Box::new(SimpleStorage::default()),
            );
        }
        #[cfg(feature = "privatetx")]
        {
            use super::zk_privacy::ZkPrivacy;
            factory.register(
                Address::from_str(reserved_addresses::NATIVE_ZK_PRIVACY).unwrap(),
                Box::new(ZkPrivacy::default()),
            );
        }
        factory
    }
}
