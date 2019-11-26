use cita_vm::evm::InterpreterParams;
use cita_vm::evm::InterpreterResult;
use common_types::errors::ContractError;
use common_types::reserved_addresses;
// use crate::contracts::Sysconfig;
use crate::rs_contracts::contracts::utils::is_permssion_contract;

use cita_types::{Address, H256, U256};
use common_types::context::Context;

use std::sync::Arc;

use crate::rs_contracts::contracts::admin::AdminContract;
use crate::rs_contracts::contracts::contract::Contract;
use crate::rs_contracts::contracts::emergency_intervention::EmergContract;
use crate::rs_contracts::contracts::perm_manager::PermStore;
use crate::rs_contracts::contracts::price::PriceContract;
use crate::rs_contracts::storage::db_contracts::ContractsDB;

use cita_trie::DB;
use cita_vm::state::State;
use std::cell::RefCell;
use std::collections::BTreeMap;

pub struct ContractsFactory<B> {
    // contracts: HashMap<Address, Box<Contract>>,
    state: Arc<RefCell<State<B>>>,
    contracts_db: Arc<ContractsDB>,
    admin_contract: AdminContract,
    price_contract: PriceContract,
    perm_store: PermStore,
    emerg_contract: EmergContract,
}

impl<B: DB> ContractsFactory<B> {
    pub fn register(&mut self, address: Address, contract: String) {
        trace!(
            "Register system contract address {:?} contract {:?}",
            address,
            contract
        );
        let mut updated_hash = H256::from(0).0;
        if address == Address::from(reserved_addresses::ADMIN) {
            updated_hash = self
                .admin_contract
                .init(contract, self.contracts_db.clone());
        } else if address == Address::from(reserved_addresses::PRICE_MANAGEMENT) {
            updated_hash = self
                .price_contract
                .init(contract, self.contracts_db.clone());
        } else if address == Address::from(reserved_addresses::EMERGENCY_INTERVENTION) {
            updated_hash = self
                .emerg_contract
                .init(contract, self.contracts_db.clone());
        }
        trace!("===> updated hash {:?}", updated_hash);
        // new a contract account, storage(key = height, value = hash(contracts))
        let _ =
            self.state
                .borrow_mut()
                .new_contract(&address, U256::from(0), U256::from(0), vec![]);
        let _ =
            self.state
                .borrow_mut()
                .set_storage(&address, H256::from(0), H256::from(updated_hash));
    }

    pub fn register_perms(&mut self, admin: Address, perm_contracts: BTreeMap<Address, String>) {
        trace!("Register permission contract {:?}", perm_contracts);
        self.perm_store
            .init(admin, perm_contracts, self.contracts_db.clone());
    }
}

impl<B: DB> ContractsFactory<B> {
    pub fn new(state: Arc<RefCell<State<B>>>, contracts_db: Arc<ContractsDB>) -> Self {
        ContractsFactory {
            state: state,
            contracts_db: contracts_db,
            admin_contract: AdminContract::default(),
            price_contract: PriceContract::default(),
            perm_store: PermStore::default(),
            emerg_contract: EmergContract::default(),
        }
    }

    pub fn is_rs_contract(&self, addr: &Address) -> bool {
        if *addr == Address::from(reserved_addresses::ADMIN)
            || *addr == Address::from(reserved_addresses::PRICE_MANAGEMENT)
            || *addr == Address::from(reserved_addresses::PERMISSION_MANAGEMENT)
            || *addr == Address::from(reserved_addresses::AUTHORIZATION)
            || *addr == Address::from(reserved_addresses::EMERGENCY_INTERVENTION)
            || is_permssion_contract(*addr)
        {
            return true;
        }
        false
    }

    pub fn works(
        &self,
        params: &InterpreterParams,
        context: &Context,
    ) -> Result<InterpreterResult, ContractError> {
        if params.contract.code_address == Address::from(reserved_addresses::ADMIN) {
            return self.admin_contract.execute(
                &params,
                context,
                self.contracts_db.clone(),
                self.state.clone(),
            );
        } else if params.contract.code_address
            == Address::from(reserved_addresses::PRICE_MANAGEMENT)
        {
            return self.price_contract.execute(
                &params,
                context,
                self.contracts_db.clone(),
                self.state.clone(),
            );
        } else if params.contract.code_address
            == Address::from(reserved_addresses::EMERGENCY_INTERVENTION)
        {
            return self.emerg_contract.execute(
                &params,
                context,
                self.contracts_db.clone(),
                self.state.clone(),
            );
        } else if params.contract.code_address
            == Address::from(reserved_addresses::PERMISSION_MANAGEMENT)
            || params.contract.code_address == Address::from(reserved_addresses::AUTHORIZATION)
            || is_permssion_contract(params.contract.code_address)
        {
            trace!("This a permission related contract");
            return self.perm_store.execute(
                &params,
                context,
                self.contracts_db.clone(),
                self.state.clone(),
            );
        }

        return Err(ContractError::AdminError(String::from(
            "not a valid address",
        )));
    }
}
