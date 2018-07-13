// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

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

//! Permission management.

use super::ContractCallExt;
use super::{encode_contract_name, to_address_vec, to_resource_vec};
use cita_types::{Address, H160, H256};
use libexecutor::executor::Executor;
use std::collections::HashMap;
use std::str::FromStr;
use types::reserved_addresses;

const ALLACCOUNTS: &'static [u8] = &*b"queryAllAccounts()";
const PERMISSIONS: &'static [u8] = &*b"queryPermissions(address)";
const RESOURCES: &'static [u8] = &*b"queryResource()";

lazy_static! {
    static ref ALLACCOUNTS_HASH: Vec<u8> = encode_contract_name(ALLACCOUNTS);
    static ref PERMISSIONS_HASH: Vec<u8> = encode_contract_name(PERMISSIONS);
    static ref RESOURCES_HASH: Vec<u8> = encode_contract_name(RESOURCES);
    static ref CONTRACT_ADDRESS: H160 = H160::from_str(reserved_addresses::AUTHORIZATION).unwrap();
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub struct Resource {
    pub cont: Address,
    pub func: Vec<u8>,
}

impl Resource {
    pub fn new(conf: Address, func: Vec<u8>) -> Self {
        Resource {
            cont: conf,
            func: func,
        }
    }

    pub fn set_cont(&mut self, addr: Address) {
        self.cont = addr;
    }

    pub fn get_cont(&self) -> Address {
        self.cont
    }

    pub fn set_func(&mut self, func: Vec<u8>) {
        self.func = func;
    }

    pub fn get_func(&self) -> &Vec<u8> {
        &self.func
    }
}

pub struct PermissionManagement;

impl PermissionManagement {
    pub fn load_account_permissions(executor: &Executor) -> HashMap<Address, Vec<Resource>> {
        let mut account_permissions = HashMap::new();
        let accounts = PermissionManagement::all_accounts(executor);

        trace!("ALl accounts: {:?}", accounts);
        for account in accounts {
            let permissions = PermissionManagement::permissions(executor, &(H256::from(account)));
            let mut resources = vec![];
            for permission in permissions {
                resources.extend(PermissionManagement::resources(executor, &permission));
            }
            account_permissions.insert(account, resources);
        }

        account_permissions
    }

    /// Account array
    pub fn all_accounts(executor: &Executor) -> Vec<Address> {
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &*ALLACCOUNTS_HASH.as_slice());
        trace!("All accounts output: {:?}", output);

        to_address_vec(&output)
    }

    pub fn get_super_admin_account(executor: &Executor) -> Option<Address> {
        let accounts = PermissionManagement::all_accounts(executor);
        if accounts.is_empty() {
            None
        } else {
            Some(accounts[0])
        }
    }

    /// Permission array
    pub fn permissions(executor: &Executor, param: &H256) -> Vec<Address> {
        let mut tx_data = PERMISSIONS_HASH.to_vec();
        tx_data.extend(param.to_vec());
        debug!("tx_data: {:?}", tx_data);
        let output = executor.call_method_latest(&*CONTRACT_ADDRESS, &tx_data.as_slice());
        debug!("Permissions output: {:?}", output);

        to_address_vec(&output)
    }

    /// Resources array
    pub fn resources(executor: &Executor, address: &Address) -> Vec<Resource> {
        let output = executor.call_method_latest(address, &*RESOURCES_HASH.as_slice());
        trace!("Resources output: {:?}", output);

        to_resource_vec(&output)
    }
}

/// Check the account contains the resource
pub fn contains_resource(
    account_permissions: &HashMap<Address, Vec<Resource>>,
    account: &Address,
    cont: Address,
    func: Vec<u8>,
) -> bool {
    match account_permissions.get(account) {
        Some(resources) => {
            let resource = Resource {
                cont: cont,
                func: func,
            };
            resources.iter().any(|res| *res == resource)
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {

    extern crate logger;
    extern crate mktemp;

    use super::contains_resource;
    use super::{PermissionManagement, Resource};
    use cita_types::{Address, H160, H256};
    use std::collections::HashMap;
    use std::str::FromStr;
    use tests::helpers::init_executor;
    use types::reserved_addresses;

    #[test]
    fn test_contains_resource() {
        let mut permission_resources: HashMap<Address, Vec<Resource>> = HashMap::new();
        let addr1 = Address::from(0x111);
        let addr2 = Address::from(0x222);
        let resources = vec![
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0xf0, 0x36, 0xed, 0x56],
            },
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_CREATOR).unwrap(),
                func: vec![0xf0, 0x36, 0xed, 0x56],
            },
        ];
        permission_resources.insert(addr1, resources);
        assert!(contains_resource(
            &permission_resources,
            &addr1,
            Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(contains_resource(
            &permission_resources,
            &addr1,
            Address::from_str(reserved_addresses::PERMISSION_CREATOR).unwrap(),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(!contains_resource(
            &permission_resources,
            &addr2,
            Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(!contains_resource(
            &permission_resources,
            &addr1,
            Address::from_str(reserved_addresses::AUTHORIZATION).unwrap(),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(!contains_resource(
            &permission_resources,
            &addr1,
            Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
            vec![0xf0, 0x36, 0xed, 0x57]
        ));
    }

    #[test]
    fn test_all_accounts() {
        let executor = init_executor(vec![(
            "Authorization.superAdmin",
            "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa1300000",
        )]);
        let all_accounts: Vec<Address> = PermissionManagement::all_accounts(&executor);

        assert_eq!(
            all_accounts,
            vec![
                Address::from_str("4b5ae4567ad5d9fb92bc9afd6a657e6fa1300000").unwrap(),
                Address::from_str(reserved_addresses::GROUP).unwrap(),
            ]
        );
    }

    #[test]
    fn test_permissions() {
        let executor = init_executor(vec![
            ((
                "Authorization.superAdmin",
                "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa1300000",
            )),
        ]);
        let super_admin = Address::from_str("4b5ae4567ad5d9fb92bc9afd6a657e6fa1300000").unwrap();
        let mut permissions: Vec<Address> =
            PermissionManagement::permissions(&executor, &(H256::from(super_admin)));
        permissions.sort();

        let mut expected_permissions = vec![
            Address::from_str(reserved_addresses::PERMISSION_NEW_PERMISSION).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_DELETE_PERMISSION).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_UPDATE_PERMISSION).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_SET_AUTH).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_CANCEL_AUTH).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_NEW_ROLE).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_DELETE_ROLE).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_UPDATE_ROLE).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_SET_ROLE).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_CANCEL_ROLE).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_NEW_GROUP).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_DELETE_GROUP).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_UPDATE_GROUP).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_SEND_TX).unwrap(),
            Address::from_str(reserved_addresses::PERMISSION_CREATE_CONTRACT).unwrap(),
        ];
        expected_permissions.sort();

        assert_eq!(permissions, expected_permissions);
    }

    #[test]
    fn test_resources() {
        let executor = init_executor(vec![]);
        let permission = Address::from_str(reserved_addresses::PERMISSION_NEW_PERMISSION).unwrap();
        let resources: Vec<Resource> = PermissionManagement::resources(&executor, &permission);
        assert_eq!(
            resources,
            vec![Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0xfc, 0x4a, 0x08, 0x9c],
            }]
        );
    }

    #[test]
    fn test_resources_from_not_exist_permission() {
        let executor = init_executor(vec![]);
        let permission = Address::from(0x13);
        let resources: Vec<Resource> = PermissionManagement::resources(&executor, &permission);
        assert_eq!(resources, vec![]);
    }

    #[test]
    fn test_load_account_permissions() {
        let executor = init_executor(vec![(
            "Authorization.superAdmin",
            "0x4b5ae4567ad5d9fb92bc9afd6a657e6fa1300000",
        )]);
        let super_admin = Address::from_str("4b5ae4567ad5d9fb92bc9afd6a657e6fa1300000").unwrap();
        let account_permissions: HashMap<Address, Vec<Resource>> =
            PermissionManagement::load_account_permissions(&executor);
        assert_eq!(account_permissions.contains_key(&super_admin), true);

        let mut resources = (*account_permissions.get(&super_admin).unwrap()).clone();
        resources.sort();

        let mut expected_resources = vec![
            // newPermission
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0xfc, 0x4a, 0x08, 0x9c],
            },
            // deletePermission
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0x98, 0xa0, 0x5b, 0xb1],
            },
            // updatePermission
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0xf0, 0x36, 0xed, 0x56],
            },
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0x64, 0x46, 0xeb, 0xd8],
            },
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0x53, 0x7b, 0xf9, 0xa3],
            },
            // setAuth
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0x0f, 0x5a, 0xa9, 0xf3],
            },
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0x52, 0xc5, 0xb4, 0xcc],
            },
            // cancelAuth
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0x34, 0x82, 0xe0, 0xc9],
            },
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0xa5, 0x92, 0x5b, 0x5b],
            },
            Resource {
                cont: Address::from_str(reserved_addresses::PERMISSION_MANAGEMENT).unwrap(),
                func: vec![0xba, 0x00, 0xab, 0x60],
            },
            // newRole
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0x55, 0x1e, 0xf8, 0x60],
            },
            // deleteRole
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0x54, 0xb0, 0x25, 0xc5],
            },
            // updateRole
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0x07, 0x73, 0xe6, 0xba],
            },
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0x17, 0xb2, 0xe3, 0x50],
            },
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0xd9, 0xc0, 0x90, 0xa0],
            },
            // setRole
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0xa3, 0x27, 0x10, 0xeb],
            },
            // cancelRole
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0xa8, 0x31, 0x94, 0x81],
            },
            Resource {
                cont: H160::from_str(reserved_addresses::ROLE_MANAGEMENT).unwrap(),
                func: vec![0xc6, 0x31, 0xe7, 0x58],
            },
            // newGroup
            Resource {
                cont: H160::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap(),
                func: vec![0xd7, 0xcd, 0x72, 0x09],
            },
            // deleteGroup
            Resource {
                cont: H160::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap(),
                func: vec![0xba, 0xeb, 0x8c, 0xad],
            },
            // updateGroup
            Resource {
                cont: H160::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap(),
                func: vec![0x2c, 0x84, 0xe3, 0x1f],
            },
            Resource {
                cont: H160::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap(),
                func: vec![0xd8, 0x6d, 0xf3, 0x33],
            },
            Resource {
                cont: H160::from_str(reserved_addresses::GROUP_MANAGEMENT).unwrap(),
                func: vec![0x7e, 0xaf, 0xcd, 0xb1],
            },
            // senTx
            Resource {
                cont: H160::from_str(reserved_addresses::PERMISSION_SEND_TX).unwrap(),
                func: vec![0, 0, 0, 0],
            },
            // createContract
            Resource {
                cont: H160::from_str(reserved_addresses::PERMISSION_CREATE_CONTRACT).unwrap(),
                func: vec![0, 0, 0, 0],
            },
        ];
        expected_resources.sort();

        assert_eq!(resources, expected_resources);
    }
}
