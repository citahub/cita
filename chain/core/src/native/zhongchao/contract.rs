///
/// zhongchao contract
///
///
///

use super::*;
use native::permission::action::ElementAction;
use std::str;
use util::ToPretty;

// 用户拥有的group name
pub struct UserGroup {
    groups: HashMap<String, Vec<String>>,
}

impl UserGroup {
    pub fn new() -> Self {
        UserGroup {
            groups: HashMap::new(),
        }
    }
}

pub struct GroupRole {
    roles: HashMap<String, Vec<String>>,
}

impl GroupRole {
    pub fn new() -> Self {
        GroupRole {
            roles: HashMap::new(),
        }
    }
}

pub struct ZcPermission {
    // Key is signature of function in the contract, value is contract function
    functions: HashMap<Signature, Box<Function>>,
    // Group that has 'creat' permission
    creator: Vec<String>,
    // Group that has 'send' permission
    sender: Vec<String>,
    // 申请加入含creat的Group的user
    applicant_of_creator: Vec<String>,
    // 申请加入含send的Group的user
    applicant_of_sender: Vec<String>,
    // user : groups
    groups: HashMap<String, Vec<String>>,
    // group : roles
    roles: HashMap<String, Vec<String>>,
}

impl Contract for ZcPermission {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        self.functions.get(hash)
    }
}

impl ZcPermission {
    pub fn new() -> Self {
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        contract.functions.insert(0, Box::new(ZcPermission::apply_group));
        contract.functions.insert(1, Box::new(ZcPermission::verify_group));
//        contract.functions.insert(2, Box::new(ZcPermission::query_group));
        contract
    }


    // apply to into/quit the group
    pub fn apply_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };

        let user = params.sender.to_hex();
        if let Some(ref data) = params.data {
            ZcPermission::into_temp(data.get(4..36).unwrap(), ext, &mut contract, user);
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // verify the application
    // fn verify_group(group: &string);
    pub fn verify_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };

        if let Some(ref data) = params.data {
            // let _ = ext.set_storage(H256::from(0), H256::from(data));
            ZcPermission::into_group(data.get(4..36).unwrap(), ext, &mut contract);
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // query the permission
    // data(0..4) is signature of function, (4..36) is group's name.
    pub fn query_group(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let user = params.sender.to_hex();
        let groups = contract.groups;
        match groups.get(&user) {
            Some(ref group) => {
               let _ug =  group.clone();
            },
            None => {}
        }

        Ok(GasLeft::Known(U256::from(0)))
    }

    // query the role of the group
    // fn query_role(group: String) -> Vec<String>
    pub fn query_role(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let roles = contract.roles;

        if let Some(ref data) = params.data {
            let group = data.get(4..36).unwrap();
            match roles.get(&String::from_utf8(group.to_vec()).unwrap()) {
                Some(ref role) => {
                    let _role = role.clone();
                },
                None => {}
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    // grant the role to a user
    // fn grant_role(group: &String, role: &Role, user: &String) -> bool
    pub fn grant_role(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let mut groups = contract.groups;
        let roles = contract.roles;

        if let Some(ref data) = params.data {
            let param_group = data.get(4..36).unwrap();
            let param_role = data.get(36..68).unwrap();
            let param_user = data.get(68..100).unwrap();

            match roles.get(&String::from_utf8(param_group.to_vec()).unwrap()) {
                Some(mut role) => {
                    if !role.contains(&String::from_utf8(param_role.to_vec()).unwrap()) {
                        return false;
                    }
                },
                None => { return false; },
            };

            match groups.get_mut(&String::from_utf8(param_user.to_vec()).unwrap()) {
                Some(group) => group.push(String::from_utf8(param_group.to_vec()).unwrap()),
                None => { return false; },
            };
        }
        true
    }

    // revoke the role to a user
    // fn revoke_role(group: &String, role: &Role, user: &String) -> bool;
    pub fn revoke_role(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let mut groups = contract.groups;
        let roles = contract.roles;

        if let Some(ref data) = params.data {
            let param_group = data.get(4..36).unwrap();
            let param_role = data.get(36..68).unwrap();
            let param_user = data.get(68..100).unwrap();

            match roles.get(&String::from_utf8(param_group.to_vec()).unwrap()) {
                Some(mut role) => {
                    if !role.contains(&String::from_utf8(param_role.to_vec()).unwrap()) {
                        return false;
                    }
                },
                None => { return false; }
            }

            match groups.get_mut(&String::from_utf8(param_user.to_vec()).unwrap()) {
                Some(group) => {
                    match group.remove_item(&String::from_utf8(param_user.to_vec()).unwrap()) {
                        Some(user) => { return true; },
                        None => { return false; }
                    }
                },
                None => { return false; }
            }
        }
        true
    }

    // quit the group
    // fn quit_group(group: &String) -> bool;
    pub fn quit_group(params: &ActionParams, ext: &mut Ext) -> bool {
        // TODO：后面用storage_at获取变量值，删除下面contract
        let mut contract = ZcPermission {
            functions: HashMap::<Signature, Box<Function>>::new(),
            creator: vec![],
            sender: vec![],
            applicant_of_creator: vec![],
            applicant_of_sender: vec![],
            groups: HashMap::<String, Vec<String>>::new(),
            roles: HashMap::<String, Vec<String>>::new(),
        };
        let mut groups = contract.groups;

        let user = params.sender.to_hex();
        if let Some(ref data) = params.data {
            let param_group = data.get(4..36).unwrap();
            match groups.get_mut(&user) {
                Some(vec_group) => {
                    match vec_group.remove_item(&String::from_utf8(param_group.to_vec()).unwrap()) {
                        Some(group) => { return true; },
                        None => { return false; }
                    }
                }
                None => { return false; }
            }
        }
        true
    }


    pub fn into_temp(group: &[u8], ext: &mut Ext, contract: &mut ZcPermission, user: String) -> bool {
        match str::from_utf8(group).unwrap() {
            "sender" => {
                contract.applicant_of_sender.push(user);
                true
            },
            "creator" => {
                contract.applicant_of_creator.push(user);
                false
            },
            _ => false,
        }
    }

    // verify the into application
    pub fn into_group(group: &[u8], ext: &mut Ext, contract: &mut ZcPermission) -> bool {
        match str::from_utf8(group).unwrap() {
            "sender" => {
                // ext.set_storage(H256::from(0), H256::from(data));
                for user in contract.applicant_of_sender.clone() {
                    contract.sender.push(user);
                }
                true
            }
            "creator" => {
                // ext.set_storage(H256::from(0), H256::from(data));
                for user in contract.applicant_of_creator.clone() {
                    contract.creator.push(user);
                }
                true
            },
            _ => false,
        }
    }
    // verify the quit application
//    pub fn quit_group(group: &[u8], ext: &mut Ext, contract: &mut ZcPermission) -> bool {
//        match str::from_utf8(group).unwrap() {
//            "sender" => {
//                // ext.set_storage(H256::from(0), H256::from(data));
//                for user in contract.applicant_of_sender.clone() {
//                    contract.sender.remove_item(&user);
//                }
//                true
//            },
//            "creator" => {
//                // ext.set_storage(H256::from(0), H256::from(data));
//                for user in contract.applicant_of_creator.clone() {
//                    contract.creator.remove_item(&user);
//                }
//                true
//            },
//            _ => false,
//        }
//    }

}
