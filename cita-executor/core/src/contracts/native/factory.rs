use crate::types::reserved_addresses;
use cita_types::Address;
use cita_vm::evm::DataProvider;
use cita_vm::evm::Error as EVMError;
use cita_vm::evm::InterpreterResult;
use evm::action_params::ActionParams;
use std::collections::HashMap;
use std::str::FromStr;

// FixMe: Just for Mock, Use config in cita-executive.rs later
#[derive(Clone, Debug)]
pub struct Config {
    pub block_gas_limit: u64, // gas limit for a block.
    pub check_nonce: bool,
}

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
    fn exec(
        &mut self,
        params: &ActionParams,
        ext: &mut DataProvider,
    ) -> Result<InterpreterResult, EVMError>;

    fn create(&self) -> Box<Contract>;
}

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
        // #[cfg(test)]
        {
            use super::storage::SimpleStorage;
            factory.register(
                Address::from_str(reserved_addresses::NATIVE_SIMPLE_STORAGE).unwrap(),
                Box::new(SimpleStorage::default()),
            );
        }
        // #[cfg(feature = "privatetx")]
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
