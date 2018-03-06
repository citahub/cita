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
use super::encode_contract_name;
use ethabi::{decode, ParamType, Token};
use libexecutor::executor::Executor;
use std::collections::HashMap;
use util::{Address, H160, H256};

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
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &*ALLACCOUNTS_HASH.as_slice());
        trace!("All accounts output: {:?}", output);

        PermissionManagement::to_address_vec(&output)
    }

    /// Permission array
    pub fn permissions(executor: &Executor, param: &H256) -> Vec<Address> {
        let mut tx_data = PERMISSIONS_HASH.to_vec();
        tx_data.extend(param.to_vec());
        debug!("tx_data: {:?}", tx_data);
        let output = executor.call_contract_method(&*CONTRACT_ADDRESS, &tx_data.as_slice());
        debug!("Permissions output: {:?}", output);

        PermissionManagement::to_address_vec(&output)
    }

    /// Resources array
    /// TODO Result. Check []
    pub fn resources(executor: &Executor, address: &Address) -> Vec<Resource> {
        let output = executor.call_contract_method(address, &*RESOURCES_HASH.as_slice());
        trace!("Resources output: {:?}", output);

        PermissionManagement::to_resource_vec(&output)
    }

    fn to_address_vec(output: &[u8]) -> Vec<Address> {
        let mut decoded = decode(&[ParamType::Array(Box::new(ParamType::Address))], &output).unwrap();
        let addresses: Vec<Token> = decoded.remove(0).to_array().unwrap();
        let addresses: Vec<Address> = addresses
            .into_iter()
            .map(|de| Address::from(de.to_address().expect("decode address")))
            .collect();
        debug!("Decoded addresses: {:?}", addresses);
        addresses
    }

    fn to_resource_vec(output: &[u8]) -> Vec<Resource> {
        // Decode the address[] and bytes4[]
        let mut decoded = decode(
            &[
                ParamType::Array(Box::new(ParamType::Address)),
                ParamType::Array(Box::new(ParamType::FixedBytes(4))),
            ],
            &output,
        ).unwrap();
        trace!("Resource decode: {:?}", decoded);
        let cont_mapiter = decoded
            .remove(0)
            .to_array()
            .unwrap()
            .into_iter()
            .map(|de| Address::from(de.to_address().expect("decode address")));

        let func_mapiter = decoded
            .remove(0)
            .to_array()
            .unwrap()
            .into_iter()
            .map(|func| func.to_fixed_bytes().expect("decode fixed bytes"));

        cont_mapiter
            .zip(func_mapiter)
            .map(|(cont, func)| Resource::new(cont, func))
            .collect()
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
    use super::*;
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
        assert_eq!(
            contains_resource(
                &permission_resources,
                &Address::from(0x1),
                Address::from(0x13241b2),
                vec![0xf0, 0x36, 0xed, 0x56]
            ),
            true
        );
        assert_eq!(
            contains_resource(
                &permission_resources,
                &Address::from(0x1),
                Address::from(0x13241b3),
                vec![0xf0, 0x36, 0xed, 0x56]
            ),
            true
        );
        assert_eq!(
            contains_resource(
                &permission_resources,
                &Address::from(0x2),
                Address::from(0x13241b2),
                vec![0xf0, 0x36, 0xed, 0x56]
            ),
            false
        );
        assert_eq!(
            contains_resource(
                &permission_resources,
                &Address::from(0x1),
                Address::from(0x13241b4),
                vec![0xf0, 0x36, 0xed, 0x56]
            ),
            false
        );
        assert_eq!(
            contains_resource(
                &permission_resources,
                &Address::from(0x1),
                Address::from(0x13241b2),
                vec![0xf0, 0x36, 0xed, 0x57]
            ),
            false
        );
    }

    #[test]
    fn test_all_accounts() {
        let executor = init_executor();
        let all_accounts: Vec<Address> = PermissionManagement::all_accounts(&executor);

        assert_eq!(
            all_accounts,
            vec![
                Address::from_str("9dcd6b234e2772c5451fd4ccf7582f4283140697").unwrap(),
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
                    func: vec![0xf0, 0x36, 0xed, 0x56],
                },
            ]
        );
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
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xf0, 0x36, 0xed, 0x56],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x34, 0x82, 0xe0, 0xc9],
                },
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
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x0f, 0x5a, 0xa9, 0xf3],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0x34, 0x82, 0xe0, 0xc9],
                },
                Resource {
                    cont: Address::from(0x13241b2),
                    func: vec![0xa5, 0x92, 0x5b, 0x5b],
                },
                Resource {
                    cont: Address::from(0x1),
                    func: vec![0, 0, 0, 0],
                },
                Resource {
                    cont: Address::from(0x2),
                    func: vec![0, 0, 0, 0],
                },
            ]
        );
    }
}
