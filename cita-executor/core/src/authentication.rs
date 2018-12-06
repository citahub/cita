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

use cita_types::{Address, H160};
use contracts::solc::{permission_management::contains_resource, Resource};
use executed::ExecutionError;
use libexecutor::sys_config::CheckOptions;
use std::collections::HashMap;
use std::str::FromStr;
use types::reserved_addresses;
use types::transaction::{Action, SignedTransaction};

/// Check the sender's permission
#[allow(unknown_lints, clippy::implicit_hasher)] // TODO clippy
pub fn check_permission(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    t: &SignedTransaction,
    options: CheckOptions,
) -> Result<(), ExecutionError> {
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
                    return Err(ExecutionError::TransactionMalformed(
                        "The length of transaction data is less than four bytes".to_string(),
                    ));
                }

                if address == group_management_addr {
                    if t.data.len() < 36 {
                        return Err(ExecutionError::TransactionMalformed(
                            "Data should have at least one parameter".to_string(),
                        ));
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
) -> Result<(), ExecutionError> {
    let cont = Address::from_str(reserved_addresses::PERMISSION_SEND_TX).unwrap();
    let func = vec![0; 4];
    let has_permission = has_resource(
        group_accounts,
        account_permissions,
        account,
        &cont,
        &func[..],
    );

    trace!("has send tx permission: {:?}", has_permission);

    if !has_permission {
        return Err(ExecutionError::NoTransactionPermission);
    }

    Ok(())
}

/// Check permission: create contract
fn check_create_contract(
    group_accounts: &HashMap<Address, Vec<Address>>,
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
) -> Result<(), ExecutionError> {
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
        return Err(ExecutionError::NoContractPermission);
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
) -> Result<(), ExecutionError> {
    let has_permission = has_resource(group_accounts, account_permissions, account, cont, func);

    trace!("has call contract permission: {:?}", has_permission);

    if !has_permission {
        return Err(ExecutionError::NoCallPermission);
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
) -> Result<(), ExecutionError> {
    let has_permission = contains_resource(account_permissions, account, *cont, func);

    trace!("Sender has call contract permission: {:?}", has_permission);

    if !has_permission && !contains_resource(account_permissions, param, *cont, func) {
        return Err(ExecutionError::NoCallPermission);
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
