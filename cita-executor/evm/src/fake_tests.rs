// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.
#![rustfmt_skip]

use env_info::EnvInfo;
use return_data::{GasLeft, ReturnData};
use schedule::Schedule;
use ext::{Ext, ContractCreateResult, MessageCallResult};
use call_type::CallType;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use util::*;
use cita_types::{Address, H256, U256};
use error;

pub struct FakeLogEntry {
    pub topics: Vec<H256>,
    pub data: Bytes,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum FakeCallType {
    Call,
    Create,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FakeCall {
    pub call_type: FakeCallType,
    pub gas: U256,
    pub sender_address: Option<Address>,
    pub receive_address: Option<Address>,
    pub value: Option<U256>,
    pub data: Bytes,
    pub code_address: Option<Address>,
}

/// Fake externalities test structure.
///
/// Can't do recursive calls.
#[derive(Default)]
pub struct FakeExt {
    pub sstore_clears: usize,
    pub depth: usize,
    pub store: HashMap<H256, H256>,
    pub blockhashes: HashMap<U256, H256>,
    pub codes: HashMap<Address, Arc<Bytes>>,
    pub logs: Vec<FakeLogEntry>,
    pub _suicides: HashSet<Address>,
    pub info: EnvInfo,
    pub schedule: Schedule,
    pub balances: HashMap<Address, U256>,
    pub calls: HashSet<FakeCall>,
}

// similar to the normal `finalize` function, but ignoring NeedsReturn.
pub fn test_finalize(res: Result<GasLeft, error::Error>) -> Result<U256, error::Error> {
    match res {
        Ok(GasLeft::Known(gas)) => Ok(gas),
        Ok(GasLeft::NeedsReturn{..}) => unimplemented!(), // since ret is unimplemented.
        Err(e) => Err(e),
    }
}

impl FakeExt {
    pub fn new() -> Self {
        FakeExt::default()
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule::new_v1()
    }
}

impl Ext for FakeExt {
    fn storage_at(&self, key: &H256) -> error::Result<H256> {
        Ok(self.store.get(key).unwrap_or(&H256::new()).clone())
    }

    fn set_storage(&mut self, key: H256, value: H256) -> error::Result<()> {
        self.store.insert(key, value);
        Ok(())
    }

    fn exists(&self, address: &Address) -> error::Result<bool> {
        Ok(self.balances.contains_key(address))
    }

    fn exists_and_not_null(&self, address: &Address) -> error::Result<bool> {
        Ok(self.balances.get(address).map_or(false, |b| !b.is_zero()))
    }

    fn origin_balance(&self) -> error::Result<U256> {
        unimplemented!()
    }

    fn balance(&self, address: &Address) -> error::Result<U256> {
        Ok(self.balances[address])
    }

    fn blockhash(&self, number: &U256) -> H256 {
        self.blockhashes.get(number).unwrap_or(&H256::new()).clone()
    }

    fn create(&mut self, gas: &U256, value: &U256, code: &[u8]) -> ContractCreateResult {
        self.calls.insert(FakeCall {
                              call_type: FakeCallType::Create,
                              gas: *gas,
                              sender_address: None,
                              receive_address: None,
                              value: Some(*value),
                              data: code.to_vec(),
                              code_address: None,
                          });
        ContractCreateResult::Failed
    }

    fn call(&mut self, gas: &U256, sender_address: &Address, receive_address: &Address, value: Option<U256>, data: &[u8], code_address: &Address, _output: &mut [u8], _call_type: CallType) -> MessageCallResult {

        self.calls.insert(FakeCall {
                              call_type: FakeCallType::Call,
                              gas: *gas,
                              sender_address: Some(sender_address.clone()),
                              receive_address: Some(receive_address.clone()),
                              value: value,
                              data: data.to_vec(),
                              code_address: Some(code_address.clone()),
                          });
        MessageCallResult::Success(*gas, ReturnData::empty())
    }

    fn extcode(&self, address: &Address) -> error::Result<Arc<Bytes>> {
        Ok(self.codes.get(address).unwrap_or(&Arc::new(Bytes::new())).clone())
    }

    fn extcodesize(&self, address: &Address) -> error::Result<usize> {
        Ok(self.codes.get(address).map_or(0, |c| c.len()))
    }

    fn log(&mut self, topics: Vec<H256>, data: &[u8]) -> error::Result<()> {
        Ok(self.logs.push(FakeLogEntry {
                           topics: topics,
                           data: data.to_vec(),
                       }))
    }

    fn ret(self, _gas: &U256, _data: &ReturnData, _apply_state: bool) -> error::Result<U256> {
        unimplemented!();
    }

    fn suicide(&mut self, _refund_address: &Address) -> error::Result<()> {
        unimplemented!();
    }

    fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    fn env_info(&self) -> &EnvInfo {
        &self.info
    }

    fn depth(&self) -> usize {
        self.depth
    }

    fn is_static(&self) -> bool {
         false
     }

    fn inc_sstore_clears(&mut self) {
        self.sstore_clears += 1;
    }
}
