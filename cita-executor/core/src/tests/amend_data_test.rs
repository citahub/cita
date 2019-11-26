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

use std::cell::RefCell;
use std::sync::Arc;

use crate::cita_executive::CitaExecutive;
use crate::libexecutor::{economical_model::EconomicalModel, sys_config::BlockSysConfig};
use crate::tests::helpers::get_temp_state;
use crate::types::context::Context;
use crate::types::transaction::Action;

use crate::data_provider::BlockDataProviderMock;
use cita_types::{Address, H256, U256};
use core::transaction::{SignedTransaction, Transaction};
use types::Bytes;

fn build_transaction(
    data: &Vec<u8>,
    value: U256,
    use_super_admin: bool,
) -> (SignedTransaction, BlockSysConfig) {
    let mut tx = Transaction::default();
    tx.action = Action::AmendData;
    tx.data = data.to_vec();
    tx.value = value;
    tx.gas = U256::from(100_000);

    let signed_tx = tx.fake_sign(Address::random());
    let mut block_config = BlockSysConfig::default();
    if use_super_admin {
        let sender = signed_tx.sender();
        block_config.super_admin_account = Some(*sender);
    }
    (signed_tx, block_config)
}

#[test]
fn test_amend_tool() {
    let state = get_temp_state();
    let context = Context::default();

    let mut e = CitaExecutive::new(
        Arc::new(BlockDataProviderMock::default()),
        Arc::new(RefCell::new(state)),
        &context,
        EconomicalModel::default(),
    );

    let (key, value) = (H256::from(42), H256::from(42));
    let storage_address: Address = "0000000000000000000000000000000000055555".into();

    let mut data: Bytes = storage_address.to_vec();
    data.append(&mut key.to_vec());
    data.append(&mut value.to_vec());

    // Sender is not super admin
    // `value=3` means the operation of amending kv
    let (tx, config) = build_transaction(&data, U256::from(3), false);
    assert!(e.exec(&tx, &config).is_err());

    // Sender is super admin
    let (tx, config) = build_transaction(&data, U256::from(3), true);
    assert!(e.exec(&tx, &config).is_ok());

    let mut data: Bytes = storage_address.to_vec();
    data.append(&mut key.to_vec());

    // Get value from key use transact interface
    // `value=4` means the operation of getting value from key.
    let (tx, config) = build_transaction(&data, U256::from(4), true);
    let res = e.exec(&tx, &config).unwrap();
    assert!(e.exec(&tx, &config).is_ok());
    assert_eq!(res.output, value.to_vec());
}
