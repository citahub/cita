use common_types::errors::ContractError;
use common_types::reserved_addresses;
use cita_vm::evm::InterpreterParams;
use cita_vm::evm::InterpreterResult;
// use crate::contracts::Sysconfig;

use cita_types::{Address, U256, H256};
use common_types::context::Context;

use std::sync::Arc;

use crate::rs_contracts::storage::db_contracts::ContractsDB;
use crate::rs_contracts::contracts::contract::Contract;
use crate::rs_contracts::contracts::admin::AdminContract;
use crate::rs_contracts::contracts::price::PriceContract;
use crate::rs_contracts::contracts::perm_manager::PermStore;


use cita_vm::state::State;
use std::cell::RefCell;
use cita_trie::DB;
use std::collections::BTreeMap;

pub struct ContractsFactory<B> {
    // contracts: HashMap<Address, Box<Contract>>,
    state: Arc<RefCell<State<B>>>,
    contracts_db: Arc<ContractsDB>,
    admin_contract: AdminContract,
    price_contract: PriceContract,
    perm_store: PermStore,
}

impl<B: DB> ContractsFactory<B> {

    pub fn register(&mut self, address: Address, contract: String) {
        trace!("Register system contract address {:?} contract {:?}", address, contract);
        let mut updated_hash = H256::from(0).0;
        if address == Address::from(reserved_addresses::ADMIN) {
            updated_hash = self.admin_contract.init(contract, self.contracts_db.clone());
        } else if address == Address::from(reserved_addresses::PRICE_MANAGEMENT) {
            updated_hash = self.price_contract.init(contract, self.contracts_db.clone());
        }
        trace!("===> updated hash {:?}", updated_hash);
        // new a contract account, storage(key = height, value = hash(contracts))
        let _ = self.state.borrow_mut().new_contract(&address, U256::from(0), U256::from(0), vec![]);
        let _ = self.state.borrow_mut().set_storage(&address, H256::from(0), H256::from(updated_hash));
    }

    pub fn register_perms(&mut self, admin: Address, perm_contracts: BTreeMap<Address, String>) {
        trace!("Register permission contract {:?}", perm_contracts);
        self.perm_store.init(admin, perm_contracts, self.contracts_db.clone());
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
        }
    }

    pub fn is_rs_contract(&self, addr: &Address) -> bool {
        if *addr == Address::from(reserved_addresses::ADMIN) ||
            *addr == Address::from(reserved_addresses::PRICE_MANAGEMENT) ||
            * addr == Address::from(reserved_addresses::PERMISSION_MANAGEMENT) ||
            * addr == Address::from(reserved_addresses::AUTHORIZATION) || is_permssion_contract(*addr){
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
            return self.admin_contract.execute(&params, context, self.contracts_db.clone(), self.state.clone());
        } else if params.contract.code_address == Address::from(reserved_addresses::PRICE_MANAGEMENT) {
            return self.price_contract.execute(&params, context, self.contracts_db.clone(), self.state.clone());
        } else if params.contract.code_address == Address::from(reserved_addresses::PERMISSION_MANAGEMENT) ||
            params.contract.code_address == Address::from(reserved_addresses::AUTHORIZATION) ||
            is_permssion_contract(params.contract.code_address)
        {
            trace!("This a permission related contract");
            return self.perm_store.execute(&params, context, self.contracts_db.clone(), self.state.clone());
        }

        return Err(ContractError::AdminError(String::from(
            "not a valid address",
        )));
    }
}

pub fn is_permssion_contract(addr: Address) -> bool {
    if addr == Address::from(reserved_addresses::PERMISSION_SEND_TX) ||
    addr == Address::from(reserved_addresses::PERMISSION_CREATE_CONTRACT) ||
    addr == Address::from(reserved_addresses::PERMISSION_NEW_PERMISSION) ||
    addr == Address::from(reserved_addresses::PERMISSION_DELETE_PERMISSION) ||
    addr == Address::from(reserved_addresses::PERMISSION_UPDATE_PERMISSION) ||
    addr == Address::from(reserved_addresses::PERMISSION_SET_AUTH) ||
    addr == Address::from(reserved_addresses::PERMISSION_CANCEL_AUTH) ||
    addr == Address::from(reserved_addresses::PERMISSION_NEW_ROLE) ||
    addr == Address::from(reserved_addresses::PERMISSION_DELETE_ROLE) ||
    addr == Address::from(reserved_addresses::PERMISSION_UPDATE_ROLE) ||
    addr == Address::from(reserved_addresses::PERMISSION_SET_ROLE) ||
    addr == Address::from(reserved_addresses::PERMISSION_CANCEL_ROLE) ||
    addr == Address::from(reserved_addresses::PERMISSION_NEW_GROUP) ||
    addr == Address::from(reserved_addresses::PERMISSION_DELETE_GROUP) ||
    addr == Address::from(reserved_addresses::PERMISSION_UPDATE_GROUP) ||
    addr == Address::from(reserved_addresses::PERMISSION_NEW_NODE) ||
    addr == Address::from(reserved_addresses::PERMISSION_DELETE_NODE) ||
    addr == Address::from(reserved_addresses::PERMISSION_UPDATE_NODE) ||
    addr == Address::from(reserved_addresses::PERMISSION_ACCOUNT_QUOTA) ||
    addr == Address::from(reserved_addresses::PERMISSION_BLOCK_QUOTA) ||
    addr == Address::from(reserved_addresses::PERMISSION_BATCH_TX) ||
    addr == Address::from(reserved_addresses::PERMISSION_EMERGENCY_INTERVENTION) ||
    addr == Address::from(reserved_addresses::PERMISSION_QUOTA_PRICE) ||
    addr == Address::from(reserved_addresses::PERMISSION_VERSION) {
        return true;
    }
    false
}
