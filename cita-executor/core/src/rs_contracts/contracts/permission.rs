use cita_types::Address;
use std::collections::BTreeMap;

pub type FuncSig = [u8; 4];

#[derive(Debug)]
pub struct PermissionContracts {
    contracts: BTreeMap<u64, Option<String>>,
}

#[derive(Debug)]
pub struct PermissionManager {
    permissions: BTreeMap<Address, Permission>,
    account_own_perms: BTreeMap<Address, Vec<Address>>,
    perm_own_accounts: BTreeMap<Address, Vec<Address>>,
}

impl PermissionManager {

    pub fn new_permission() {
        // 新建一个权限
    }

    pub fn delete_permission() {
        // 更新拥有该权限的用户，权限列表
    }

    pub fn update_permission_name() {}

    pub fn add_resources() {
        // 为某一个权限添加更多的资源
    }

    pub fn delete_resources() {
        // 删除某一个权限的资源
    }

    pub fn set_authorizations() {}

    pub fn set_authorization() {}

    pub fn cancel_authorizations() {}

    pub fn cancel_authorization() {}

    pub fn clear_authorization() {}

}

#[derive(Debug)]
pub struct Permission {
    name: String,
    resources: Vec<Resource>,
}


#[derive(Debug)]
pub struct Resource {
    addr: Address,
    func: FuncSig,
}

impl Permission {
    pub fn new(name: String, contracts: Vec<Address>, funcs: Vec<FuncSig>) -> Self {

    }

    pub fn add_resources() {}
    fn add_resource() {}
    pub fn delete_resources() {}
    fn delete_resource()
    pub fn update_name() {}

    pub fn in_permission() {}
    pub fn query_info() {}
    pub fn query_name() {}
    pub fn query_resource() {}

}







