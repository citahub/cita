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

use super::{encode_contract_name, to_address_vec, to_resource_vec};
use super::ContractCallExt;
use cita_types::{Address, H160, H256};
use libexecutor::executor::Executor;
use std::collections::HashMap;

const ALLACCOUNTS: &'static [u8] = &*b"queryAllAccounts()";
const PERMISSIONS: &'static [u8] = &*b"queryPermissions(address)";
const RESOURCES: &'static [u8] = &*b"queryResource()";

lazy_static! {
    static ref ALLACCOUNTS_HASH: Vec<u8> = encode_contract_name(ALLACCOUNTS);
    static ref PERMISSIONS_HASH: Vec<u8> = encode_contract_name(PERMISSIONS);
    static ref RESOURCES_HASH: Vec<u8> = encode_contract_name(RESOURCES);
    static ref CONTRACT_ADDRESS: H160 = H160::from(0x13241b4);
}

#[derive(PartialEq, Clone, Default, Debug, Serialize, Deserialize)]
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
    use super::{PermissionManagement, Resource};
    use super::contains_resource;
    use cita_types::{Address, H160, H256};
    use std::collections::HashMap;
    use std::str::FromStr;
    use tests::helpers::init_executor;

    #[test]
    fn test_contains_resource() {
        let mut permission_resources: HashMap<Address, Vec<Resource>> = HashMap::new();
        let addr = Address::from(0x1);
        let resources = vec![
            Resource {
                cont: Address::from(0x13241b2),
                func: vec![0xf0, 0x36, 0xed, 0x56],
            },
            Resource {
                cont: Address::from(0x13241b3),
                func: vec![0xf0, 0x36, 0xed, 0x56],
            },
        ];
        permission_resources.insert(addr, resources);
        assert!(contains_resource(
            &permission_resources,
            &Address::from(0x1),
            Address::from(0x13241b2),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(contains_resource(
            &permission_resources,
            &Address::from(0x1),
            Address::from(0x13241b3),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(!contains_resource(
            &permission_resources,
            &Address::from(0x2),
            Address::from(0x13241b2),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(!contains_resource(
            &permission_resources,
            &Address::from(0x1),
            Address::from(0x13241b4),
            vec![0xf0, 0x36, 0xed, 0x56]
        ));
        assert!(!contains_resource(
            &permission_resources,
            &Address::from(0x1),
            Address::from(0x13241b2),
            vec![0xf0, 0x36, 0xed, 0x57]
        ));
    }

    #[test]
    fn test_all_accounts() {
        let executor = init_executor();
        let all_accounts: Vec<Address> = PermissionManagement::all_accounts(&executor);

        assert_eq!(
            all_accounts,
            vec![
                Address::from_str("9dcd6b234e2772c5451fd4ccf7582f4283140697").unwrap(),
                Address::from(0x13241b6),
            ]
        );
    }

    #[test]
    fn test_permissions() {
        let executor = init_executor();
        let super_admin = Address::from_str("9dcd6b234e2772c5451fd4ccf7582f4283140697").unwrap();
        let permissions: Vec<Address> = PermissionManagement::permissions(&executor, &(H256::from(super_admin)));
        assert_eq!(
            permissions,
            vec![
                Address::from(0x13241b5),
                Address::from(0x23241b5),
                Address::from(0x33241b5),
                Address::from(0x43241b5),
                Address::from(0x53241b5),
                Address::from(0x63241b5),
                Address::from(0x73241b5),
                Address::from(0x83241b5),
                Address::from(0x93241b5),
                Address::from(0xa3241b5),
                Address::from(0xb3241b5),
                Address::from(0xc3241b5),
                Address::from(0xd3241b5),
                Address::from(0x1),
                Address::from(0x2),
            ]
        );
    }

    #[test]
    fn test_resources() {
        let executor = init_executor();
        let permission = Address::from(0x13241b5);
        let resources: Vec<Resource> = PermissionManagement::resources(&executor, &permission);
        assert_eq!(
            resources,
            vec![
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xfc, 0x4a, 0x08, 0x9c],
                },
            ]
        );
    }

    #[test]
    fn test_resources_from_not_exist_permission() {
        let executor = init_executor();
        let permission = Address::from(0x13);
        let resources: Vec<Resource> = PermissionManagement::resources(&executor, &permission);
        assert_eq!(resources, vec![]);
    }

    #[test]
    fn test_load_account_permissions() {
        let executor = init_executor();
        let super_admin = Address::from_str("9dcd6b234e2772c5451fd4ccf7582f4283140697").unwrap();
        let account_permissions: HashMap<Address, Vec<Resource>> =
            PermissionManagement::load_account_permissions(&executor);
        assert_eq!(account_permissions.contains_key(&super_admin), true);
        assert_eq!(
            *account_permissions.get(&super_admin).unwrap(),
            vec![
                // newPermission
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xfc, 0x4a, 0x08, 0x9c],
                },
                // deletePermission
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x98, 0xa0, 0x5b, 0xb1],
                },
                // updatePermission
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xf0, 0x36, 0xed, 0x56],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x64, 0x46, 0xeb, 0xd8],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x53, 0x7b, 0xf9, 0xa3],
                },
                // setAuth
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x0f, 0x5a, 0xa9, 0xf3],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x52, 0xc5, 0xb4, 0xcc],
                },
                // cancelAuth
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x34, 0x82, 0xe0, 0xc9],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xa5, 0x92, 0x5b, 0x5b],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xba, 0x00, 0xab, 0x60],
                },
                // newRole
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0x55, 0x1e, 0xf8, 0x60],
                },
                // deleteRole
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0x54, 0xb0, 0x25, 0xc5],
                },
                // updateRole
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0x07, 0x73, 0xe6, 0xba],
                },
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0x17, 0xb2, 0xe3, 0x50],
                },
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0xd9, 0xc0, 0x90, 0xa0],
                },
                // setRole
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0xa3, 0x27, 0x10, 0xeb],
                },
                // cancelRole
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0xa8, 0x31, 0x94, 0x81],
                },
                Resource {
                    cont: H160::from_str("e3b5ddb80addb513b5c981e27bb030a86a8821ee").unwrap(),
                    func: vec![0xc6, 0x31, 0xe7, 0x58],
                },
                // newGroup
                Resource {
                    cont: H160::from_str("00000000000000000000000000000000013241c2").unwrap(),
                    func: vec![0xd7, 0xcd, 0x72, 0x09],
                },
                // deleteGroup
                Resource {
                    cont: H160::from_str("00000000000000000000000000000000013241c2").unwrap(),
                    func: vec![0xba, 0xeb, 0x8c, 0xad],
                },
                // updateGroup
                Resource {
                    cont: H160::from_str("00000000000000000000000000000000013241c2").unwrap(),
                    func: vec![0x2c, 0x84, 0xe3, 0x1f],
                },
                Resource {
                    cont: H160::from_str("00000000000000000000000000000000013241c2").unwrap(),
                    func: vec![0xd8, 0x6d, 0xf3, 0x33],
                },
                Resource {
                    cont: H160::from_str("00000000000000000000000000000000013241c2").unwrap(),
                    func: vec![0x7e, 0xaf, 0xcd, 0xb1],
                },
                // senTx
                Resource {
                    cont: Address::from(0x1),
                    func: vec![0, 0, 0, 0],
                },
                // createContract
                Resource {
                    cont: Address::from(0x2),
                    func: vec![0, 0, 0, 0],
                },
            ]
        );
    }
}
