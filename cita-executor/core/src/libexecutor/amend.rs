use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;

use cita_types::{H160, H256, U256};
use cita_vm::state::{State, StateObjectInfo};
use util::Bytes;

///amend the abi data
const AMEND_ABI: u32 = 1;
///amend the account code
const AMEND_CODE: u32 = 2;
///amend the kv of db
const AMEND_KV_H256: u32 = 3;
///amend get the value of db
const AMEND_GET_KV_H256: u32 = 4;
///amend account's balance
const AMEND_ACCOUNT_BALANCE: u32 = 5;

pub struct Amend<B> {
    state_provider: Arc<RefCell<State<B>>>,
}

impl<B: cita_trie::DB + 'static> Amend<B> {
    pub fn new(state: Arc<RefCell<State<B>>>) -> Self {
        Self {
            state_provider: state,
        }
    }

    pub fn call_amend_data(
        &mut self,
        value: U256,
        data: Option<Bytes>,
    ) -> Result<AmendResult, AmendError> {
        let amend_type = value.low_u32();
        match amend_type {
            AMEND_ABI => self.transact_set_abi(&(data.to_owned().unwrap())),
            AMEND_CODE => self.transact_set_code(&(data.to_owned().unwrap())),
            AMEND_KV_H256 => self.transact_set_kv_h256(&(data.to_owned().unwrap())),
            AMEND_GET_KV_H256 => self.transact_get_kv_h256(&(data.to_owned().unwrap())),
            AMEND_ACCOUNT_BALANCE => self.transact_set_balance(&(data.to_owned().unwrap())),
            _ => panic!("Unsupported amend type"),
        }
    }

    pub fn transact_set_abi(&mut self, data: &[u8]) -> Result<AmendResult, AmendError> {
        if data.len() <= 20 {
            return Err(AmendError::ABI);
        }

        let account = H160::from(&data[0..20]);
        let abi = &data[20..];
        info!("Set abi for contract address: {:?}", account);

        let b = self
            .state_provider
            .borrow_mut()
            .exist(&account)
            .map(|exists| {
                exists
                    && self
                        .state_provider
                        .borrow_mut()
                        .set_abi(&account, abi.to_vec())
                        .is_ok()
            })
            .unwrap_or(false);
        Ok(AmendResult::Set(b))
    }

    pub fn transact_set_code(&mut self, data: &[u8]) -> Result<AmendResult, AmendError> {
        if data.len() <= 20 {
            return Err(AmendError::Code);
        }
        let account = H160::from(&data[0..20]);
        let code = &data[20..];
        let b = self
            .state_provider
            .borrow_mut()
            .set_code(&account, code.to_vec())
            .is_ok();
        Ok(AmendResult::Set(b))
    }

    pub fn transact_set_kv_h256(&mut self, data: &[u8]) -> Result<AmendResult, AmendError> {
        let len = data.len();
        if len < 84 {
            return Err(AmendError::KV);
        }
        let loop_num: usize = (len - 20) / (32 * 2);
        let account = H160::from(&data[0..20]);

        for i in 0..loop_num {
            let base = 20 + 32 * 2 * i;
            let key = H256::from_slice(&data[base..base + 32]);
            let val = H256::from_slice(&data[base + 32..base + 32 * 2]);
            if self
                .state_provider
                .borrow_mut()
                .set_storage(&account, key, val)
                .is_err()
            {
                return Ok(AmendResult::Set(false));
            }
        }
        Ok(AmendResult::Set(true))
    }

    pub fn transact_get_kv_h256(&mut self, data: &[u8]) -> Result<AmendResult, AmendError> {
        let account = H160::from(&data[0..20]);
        let key = H256::from_slice(&data[20..52]);
        let b = self
            .state_provider
            .borrow_mut()
            .get_storage(&account, &key)
            .ok();
        Ok(AmendResult::Get(b))
    }

    pub fn transact_set_balance(&mut self, data: &[u8]) -> Result<AmendResult, AmendError> {
        if data.len() < 52 {
            return Err(AmendError::Balance);
        }
        let account = H160::from(&data[0..20]);
        let balance = U256::from(&data[20..52]);
        let b = self
            .state_provider
            .borrow_mut()
            .balance(&account)
            .and_then(|now_val| {
                if now_val >= balance {
                    self.state_provider
                        .borrow_mut()
                        .sub_balance(&account, now_val - balance)
                } else {
                    self.state_provider
                        .borrow_mut()
                        .add_balance(&account, balance - now_val)
                }
            })
            .is_ok();
        Ok(AmendResult::Set(b))
    }
}

pub enum AmendResult {
    Set(bool),
    Get(Option<H256>),
}

#[derive(Debug)]
pub enum AmendError {
    ABI,
    Balance,
    KV,
    Code,
}

impl fmt::Display for AmendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match self {
            AmendError::ABI => "Amend abi error".to_string(),
            AmendError::Balance => "Amend balance error".to_string(),
            AmendError::KV => "Amend kv error".to_string(),
            AmendError::Code => "Amend code error".to_string(),
        };
        write!(f, "{}", printable)
    }
}
