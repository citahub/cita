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

use std::collections::HashMap;
use std::str::FromStr;

use crate::types::transaction::{Action, SignedTransaction};
use cita_types::{Address, H160};

use crate::contracts::solc::{permission_management::contains_resource, Resource};
use crate::libexecutor::sys_config::CheckOptions;
use crate::types::errors::AuthenticationError;
use crate::types::reserved_addresses;

/// Check the sender's permission
#[allow(unknown_lints, clippy::implicit_hasher)] // TODO clippy
pub fn check_permission(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    t: &SignedTransaction,
    options: CheckOptions,
) -> Result<(), AuthenticationError> {
    let sender = *t.sender();
    // It's eth_call when the account is zero.
    // No need to check the options in case that the option is true.
    if sender == Address::zero() {
        return Ok(());
    }

    if options.send_tx_permission {
        check_send_tx(group_accounts, account_permissions, &sender)?;
    }

    match t.action {
        Action::Create => {
            if options.create_contract_permission {
                check_create_contract(group_accounts, account_permissions, &sender)?;
            }
        }
        Action::Call(address) => {
            if options.call_permission {
                let group_management_addr =
                    Address::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap();
                trace!("t.data {:?}", t.data);

                if t.data.is_empty() {
                    // Transfer transaction, no function call
                    return Ok(());
                }

                if t.data.len() < 4 {
                    return Err(AuthenticationError::InvalidTransaction);
                }

                if address == group_management_addr {
                    if t.data.len() < 36 {
                        return Err(AuthenticationError::InvalidTransaction);
                    }
                    check_origin_group(
                        account_permissions,
                        &sender,
                        &address,
                        &t.data[0..4],
                        &H160::from(&t.data[16..36]),
                    )?;
                }

                check_call_contract(
                    group_accounts,
                    account_permissions,
                    &sender,
                    &address,
                    &t.data[0..4],
                )?;
            }
        }
        _ => {}
    }

    Ok(())
}

/// Check permission: send transaction
fn check_send_tx(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
) -> Result<(), AuthenticationError> {
    let cont = Address::from_str(reserved_addresses::PERMISSION_SEND_TX).unwrap();
    let func = vec![0; 4];
    let has_permission = has_resource(
        group_accounts,
        account_permissions,
        account,
        &cont,
        &func[..],
    );

    trace!(
        "Account {:?} has send tx permission: {:?}",
        account,
        has_permission
    );

    if !has_permission {
        return Err(AuthenticationError::NoTransactionPermission);
    }

    Ok(())
}

/// Check permission: create contract
fn check_create_contract(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
) -> Result<(), AuthenticationError> {
    let cont = Address::from_str(reserved_addresses::PERMISSION_CREATE_CONTRACT).unwrap();
    let func = vec![0; 4];
    let has_permission = has_resource(
        group_accounts,
        account_permissions,
        account,
        &cont,
        &func[..],
    );

    trace!("has create contract permission: {:?}", has_permission);

    if !has_permission {
        return Err(AuthenticationError::NoContractPermission);
    }

    Ok(())
}

/// Check permission: call contract
fn check_call_contract(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: &Address,
    func: &[u8],
) -> Result<(), AuthenticationError> {
    let has_permission = has_resource(group_accounts, account_permissions, account, cont, func);

    trace!("has call contract permission: {:?}", has_permission);

    if !has_permission {
        return Err(AuthenticationError::NoCallPermission);
    }

    Ok(())
}

/// Check permission with parameter: origin group
fn check_origin_group(
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: &Address,
    func: &[u8],
    param: &Address,
) -> Result<(), AuthenticationError> {
    let has_permission = contains_resource(account_permissions, account, *cont, func);

    trace!("Sender has call contract permission: {:?}", has_permission);

    if !has_permission && !contains_resource(account_permissions, param, *cont, func) {
        return Err(AuthenticationError::NoCallPermission);
    }

    Ok(())
}

/// Check the account has resource
/// 1. Check the account has resource
/// 2. Check all account's groups has resource
fn has_resource(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: &Address,
    func: &[u8],
) -> bool {
    let groups = get_groups(group_accounts, account);
    trace!("groups in has resource {:?}", groups);

    if !contains_resource(account_permissions, account, *cont, func) {
        for group in groups {
            if contains_resource(account_permissions, &group, *cont, func) {
                return true;
            }
        }

        return false;
    }

    true
}

/// Get all sender's groups
fn get_groups(group_accounts: &HashMap<Address, Vec<Address>>, account: &Address) -> Vec<Address> {
    let mut groups: Vec<Address> = vec![];

    for (group, accounts) in group_accounts {
        if accounts.contains(account) {
            groups.push(*group);
        }
    }

    groups
}
