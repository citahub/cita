// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::str::FromStr;

use crate::cita_executive::VmExecParams;
use crate::types::context::Context;
use crate::types::errors::NativeError;
use crate::types::reserved_addresses;

use cita_types::Address;
use cita_vm::evm::DataProvider;
use cita_vm::evm::InterpreterResult;

pub type Signature = u32;
pub trait ContractClone {
    fn clone_box(&self) -> Box<dyn Contract>;
}

impl<T> ContractClone for T
where
    T: 'static + Contract + Clone,
{
    fn clone_box(&self) -> Box<dyn Contract> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Contract> {
    fn clone(&self) -> Box<dyn Contract> {
        self.clone_box()
    }
}

// Contract
pub trait Contract: Sync + Send + ContractClone {
    fn exec(
        &mut self,
        params: &VmExecParams,
        context: &Context,
        data_provider: &mut dyn DataProvider,
    ) -> Result<InterpreterResult, NativeError>;

    fn create(&self) -> Box<dyn Contract>;
}

#[derive(Clone)]
pub struct Factory {
    contracts: HashMap<Address, Box<dyn Contract>>,
}

impl Factory {
    pub fn new_contract(&self, address: Address) -> Option<Box<dyn Contract>> {
        if let Some(contract) = self.contracts.get(&address) {
            Some(contract.create())
        } else {
            None
        }
    }
    pub fn register(&mut self, address: Address, contract: Box<dyn Contract>) {
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
        {
            use super::hello::HelloWorld;
            factory.register(
                Address::from_str("0000000000000000000000000000000000000500").unwrap(),
                Box::new(HelloWorld::default()),
            );
        }
        #[cfg(test)]
        {
            use super::simple_storage::SimpleStorage;
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
