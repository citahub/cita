// Copyright Cryptape Technologies LLC.
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

use crate::cita_executive::{call as ext_call, create as ext_create, CreateKind};
use cita_trie::DB;
use cita_types::{Address, H256, U256};
use cita_vm::evm;
use cita_vm::state::{State, StateObjectInfo};
use hashbrown::{HashMap, HashSet};
use hasher::Hasher;
use std::cell::RefCell;
use std::sync::Arc;

use crate::rs_contracts::storage::db_contracts::ContractsDB;

/// BlockDataProvider provides functions to get block's hash from chain.
///
/// Block data(only hash) are required to cita-vm from externalize database.
pub trait BlockDataProvider: Send + Sync {
    /// Function get_block_hash returns the block_hash of the specific block.
    fn get_block_hash(&self, number: &U256) -> H256;
}

/// BlockDataProviderMock is a mock for BlockDataProvider. We could use it in
/// tests or demos.
#[derive(Default)]
pub struct BlockDataProviderMock {
    data: HashMap<U256, H256>,
}

impl BlockDataProviderMock {
    /// Set blockhash for a specific block.
    pub fn set(&mut self, number: U256, hash: H256) {
        self.data.insert(number, hash);
    }
}

/// Impl.
impl BlockDataProvider for BlockDataProviderMock {
    fn get_block_hash(&self, number: &U256) -> H256 {
        *self.data.get(number).unwrap_or(&H256::zero())
    }
}

/// Store storages shared datas.
#[derive(Clone, Default, Debug)]
pub struct Store {
    pub refund: HashMap<Address, u64>, // For record refunds
    pub origin: HashMap<Address, HashMap<H256, H256>>, // For record origin value
    pub selfdestruct: HashSet<Address>, // For self destruct
    // Field inused used for garbage collection.
    //
    // Test:
    //   ./tests/jsondata/GeneralStateTests/stSStoreTest/sstore_combinations_initial0.json
    //   ./tests/jsondata/GeneralStateTests/stSStoreTest/sstore_combinations_initial1.json
    //   ./tests/jsondata/GeneralStateTests/stSStoreTest/sstore_combinations_initial2.json
    pub inused: HashSet<Address>,
    pub evm_context: evm::Context,
    pub evm_cfg: evm::InterpreterConf,
}

impl Store {
    /// Merge with sub store.
    pub fn merge(&mut self, other: Arc<RefCell<Self>>) {
        self.refund = other.borrow().refund.clone();
        self.origin = other.borrow().origin.clone();
        self.selfdestruct = other.borrow().selfdestruct.clone();
        self.inused = other.borrow().inused.clone();
    }

    /// When a account has been read or write, record a log
    /// to prove that it has dose.
    pub fn used(&mut self, address: Address) {
        if address == Address::zero() {
            return;
        }
        self.inused.insert(address);
    }
}

/// An implemention for evm::DataProvider
pub struct DataProvider<B> {
    pub block_provider: Arc<BlockDataProvider>,
    pub state_provider: Arc<RefCell<State<B>>>,
    pub store: Arc<RefCell<Store>>,
    pub contracts_db: Arc<ContractsDB>,
}

impl<B: DB> DataProvider<B> {
    /// Create a new instance. It's obvious.
    pub fn new(
        b: Arc<BlockDataProvider>,
        s: Arc<RefCell<State<B>>>,
        store: Arc<RefCell<Store>>,
        c: Arc<ContractsDB>,
    ) -> Self {
        DataProvider {
            block_provider: b,
            state_provider: s,
            store,
            contracts_db: c,
        }
    }
}

impl<B: DB + 'static> evm::DataProvider for DataProvider<B> {
    fn get_balance(&self, address: &Address) -> U256 {
        self.state_provider
            .borrow_mut()
            .balance(address)
            .unwrap_or_else(|_| U256::zero())
    }

    fn add_refund(&mut self, address: &Address, n: u64) {
        self.store
            .borrow_mut()
            .refund
            .entry(*address)
            .and_modify(|v| *v += n)
            .or_insert(n);
    }

    fn sub_refund(&mut self, address: &Address, n: u64) {
        debug!("ext.sub_refund {:?} {}", address, n);
        self.store
            .borrow_mut()
            .refund
            .entry(*address)
            .and_modify(|v| *v -= n)
            .or_insert(n);
    }

    fn get_refund(&self, address: &Address) -> u64 {
        self.store
            .borrow_mut()
            .refund
            .get(address)
            .map_or(0, |v| *v)
    }

    fn get_code_size(&self, address: &Address) -> u64 {
        self.state_provider
            .borrow_mut()
            .code_size(address)
            .unwrap_or(0) as u64
    }

    fn get_code(&self, address: &Address) -> Vec<u8> {
        self.state_provider
            .borrow_mut()
            .code(address)
            .unwrap_or_else(|_| vec![])
    }

    fn get_code_hash(&self, address: &Address) -> H256 {
        self.state_provider
            .borrow_mut()
            .code_hash(address)
            .unwrap_or_else(|_| H256::zero())
    }

    fn get_block_hash(&self, number: &U256) -> H256 {
        self.block_provider.get_block_hash(number)
    }

    fn get_storage(&self, address: &Address, key: &H256) -> H256 {
        self.state_provider
            .borrow_mut()
            .get_storage(address, key)
            .unwrap_or_else(|_| H256::zero())
    }

    fn set_storage(&mut self, address: &Address, key: H256, value: H256) {
        let a = self.get_storage(address, &key);
        self.store
            .borrow_mut()
            .origin
            .entry(*address)
            .or_insert_with(HashMap::new)
            .entry(key)
            .or_insert(a);
        if let Err(e) = self
            .state_provider
            .borrow_mut()
            .set_storage(address, key, value)
        {
            panic!("{}", e);
        }
    }

    fn get_storage_origin(&self, address: &Address, key: &H256) -> H256 {
        //self.store.borrow_mut().used(address.clone());
        match self.store.borrow_mut().origin.get(address) {
            Some(account) => match account.get(key) {
                Some(val) => *val,
                None => self.get_storage(address, key),
            },
            None => self.get_storage(address, key),
        }
    }

    fn set_storage_origin(&mut self, _address: &Address, _key: H256, _value: H256) {
        unimplemented!()
    }

    fn selfdestruct(&mut self, address: &Address, refund_to: &Address) -> bool {
        if self.store.borrow_mut().selfdestruct.contains(address) {
            return false;
        }
        //self.store.borrow_mut().used(refund_to.clone());
        self.store.borrow_mut().selfdestruct.insert(address.clone());
        let b = self.get_balance(address);

        if address != refund_to {
            self.state_provider
                .borrow_mut()
                .transfer_balance(address, refund_to, b)
                .unwrap();
        } else {
            // Must ensure that the balance of address which is suicide is zero.
            self.state_provider
                .borrow_mut()
                .sub_balance(address, b)
                .unwrap();
        }
        true
    }

    fn sha3(&self, data: &[u8]) -> H256 {
        From::from(&hasher::HasherKeccak::new().digest(data)[..])
    }

    fn is_empty(&self, address: &Address) -> bool {
        self.state_provider
            .borrow_mut()
            .is_empty(address)
            .unwrap_or(false)
    }

    fn exist(&self, address: &Address) -> bool {
        self.state_provider
            .borrow_mut()
            .exist(address)
            .unwrap_or(false)
    }

    fn call(
        &self,
        opcode: evm::OpCode,
        params: evm::InterpreterParams,
    ) -> (Result<evm::InterpreterResult, evm::Error>) {
        match opcode {
            evm::OpCode::CALL
            | evm::OpCode::CALLCODE
            | evm::OpCode::DELEGATECALL
            | evm::OpCode::STATICCALL => {
                let r = ext_call(
                    self.block_provider.clone(),
                    self.state_provider.clone(),
                    self.store.clone(),
                    self.contracts_db.clone(),
                    &params,
                );
                debug!("ext.call.result = {:?}", r);
                r.or(Err(evm::Error::CallError))
            }
            evm::OpCode::CREATE | evm::OpCode::CREATE2 => {
                let mut request = params.clone();
                request.nonce = self
                    .state_provider
                    .borrow_mut()
                    .nonce(&request.sender)
                    .or(Err(evm::Error::CallError))?;
                // Must inc nonce for sender
                // See: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-161.md
                self.state_provider
                    .borrow_mut()
                    .inc_nonce(&request.sender)
                    .or(Err(evm::Error::CallError))?;

                let r = match opcode {
                    evm::OpCode::CREATE => ext_create(
                        self.block_provider.clone(),
                        self.state_provider.clone(),
                        self.store.clone(),
                        self.contracts_db.clone(),
                        &request,
                        CreateKind::FromAddressAndNonce,
                    ),
                    evm::OpCode::CREATE2 => ext_create(
                        self.block_provider.clone(),
                        self.state_provider.clone(),
                        self.store.clone(),
                        self.contracts_db.clone(),
                        &request,
                        CreateKind::FromSaltAndCodeHash,
                    ),
                    _ => unimplemented!(),
                }
                .or(Err(evm::Error::CallError));
                debug!("ext.create.result = {:?}", r);
                r
            }
            _ => unimplemented!(),
        }
    }
}
